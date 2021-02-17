use std::{
    collections::HashMap,
    ffi::{c_void, CStr, CString},
    fmt::Debug,
    sync::{Arc, Mutex},
};

use futures::{
    future::{AbortHandle, Abortable},
    Future,
};
use libc::{c_char, size_t};
use log::{error, trace};

use chargeprice::{
    api::VehiculeResponse,
    client::{APIClient, APIError},
};

use crate::RUN_TIME;

use super::vehicule::Vehicule;

/// A simple wrapper for cancellable
#[repr(transparent)]
pub struct Cancellable {
    id: u64,
}
// ErrorCode
/// The trait providing error code
trait ErrorCode: std::fmt::Display {
    fn error_code(&self) -> i32;
}

impl ErrorCode for APIError {
    fn error_code(&self) -> i32 {
        match self {
            APIError::Network(_) => -1000,
            APIError::APIAuthenticationError(_) => -2000,
            APIError::APIClientError { response: _ } => -3000,
            APIError::APIUnknownResponse { response: _ } => -3500,
        }
    }
}
// FFIClient
/// An FFI wrapper for the APIClient
#[derive(Debug)]
pub struct FFIClient {
    inner: Arc<APIClient>,
    cancellables: Arc<Mutex<HashMap<u64, AbortHandle>>>,
}

impl FFIClient {
    /// Creates
    pub fn new(api_key: &str, flavor: &str) -> Result<Self, APIError> {
        let client = APIClient::new_with_agent_flavor(api_key, flavor)?;
        Ok(FFIClient {
            inner: Arc::new(client),
            cancellables: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    fn execute_request<T, C, F>(&self, fut: F, callback: C) -> Cancellable
    where
        C: FnOnce(Result<T, APIError>) -> () + Send + 'static,
        F: Future<Output = Result<T, APIError>> + Send + 'static,
    {
        let (abort_handle, abort_registration) = AbortHandle::new_pair();
        let map = Arc::clone(&self.cancellables);
        let mut map = map.lock().unwrap();
        let size = map.len() as u64;

        let after = Arc::clone(&self.cancellables);
        let future = Abortable::new(
            async move {
                let res = fut.await;
                callback(res);

                let mut after = after.lock().unwrap();
                after.remove_entry(&size);
            },
            abort_registration,
        );
        map.insert(size, abort_handle);

        (*RUN_TIME).spawn(future);
        Cancellable { id: size }
    }

    pub fn load_vehicules<C>(&self, callback: C) -> Cancellable
    where
        C: FnOnce(Result<VehiculeResponse, APIError>) -> () + Send + 'static,
    {
        let client = Arc::clone(&self.inner);
        self.execute_request(async move { client.load_vehicules().await }, callback)
    }
}

// FFI Callbacks
#[repr(C)]
pub struct ArrayCallback<S, E> {
    context: *mut c_void,
    on_success: extern "C" fn(context: *mut c_void, arg: *const S, length: size_t),
    on_error: extern "C" fn(context: *mut c_void, arg: *const E),
}

unsafe impl<S, E> Send for ArrayCallback<S, E> {}

#[repr(C)]
#[derive(Debug)]
pub struct FFIError {
    code: i32,
    message: *const c_char,
}

impl Drop for FFIError {
    fn drop(&mut self) {
        unsafe {
            let _ = CString::from_raw(self.message as *mut c_char);
        }
    }
}

impl From<APIError> for FFIError {
    fn from(e: APIError) -> Self {
        FFIError {
            code: e.error_code(),
            message: CString::new(format!("{}", e)).unwrap().into_raw(),
        }
    }
}
// FFI API

#[no_mangle]
pub extern "C" fn chargeprice_create_api_client(
    key: *const c_char,
    flavor: *const c_char,
) -> *mut FFIClient {
    assert!(!key.is_null());

    let key = unsafe { CStr::from_ptr(key) };
    let flavor = unsafe { CStr::from_ptr(flavor) };

    match FFIClient::new(&key.to_string_lossy(), &flavor.to_string_lossy()) {
        Ok(client) => Box::into_raw(Box::new(client)),
        Err(err) => {
            error!("error creating the client: {}", err);
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn chargeprice_free_api_client(client: *mut FFIClient) {
    assert!(!client.is_null());
    Box::from_raw(client);
}

#[no_mangle]
pub extern "C" fn chargeprice_cancel(client: *mut FFIClient, cancellable: Cancellable) {
    assert!(!client.is_null());

    let ref_client = unsafe { client.as_ref() }.unwrap();

    let map = Arc::clone(&ref_client.cancellables);
    let mut map = map.lock().unwrap();
    if let Some(value) = map.remove(&cancellable.id) {
        trace!("{} cancelling...", cancellable.id);
        value.abort();
    }
}

#[no_mangle]
pub extern "C" fn chargeprice_get_vehicles(
    client: *mut FFIClient,
    callback: ArrayCallback<Vehicule, FFIError>,
) -> Cancellable {
    assert!(!client.is_null());

    // We assume that the provided client is not null here
    let ref_client = unsafe { client.as_ref() }.unwrap();
    ref_client.load_vehicules(move |result| match result {
        Ok(r) => {
            trace!("OK - received {} vehicules", r.data().len());
            let elements: Vec<Vehicule> = r.data().iter().map(|r| r.into()).collect();
            let converted: *const Vehicule = elements.as_ptr();
            (callback.on_success)(callback.context, converted, elements.len());
        }
        Err(e) => {
            trace!("Err - received {} vehicules", e);
            let e: FFIError = e.into();
            (callback.on_error)(callback.context, &e);
        }
    })
}

#[no_mangle]
pub extern "C" fn chargeprice_init_log() {
    env_logger::init();
}
