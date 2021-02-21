//!  Cachigng JNI methods. Heavily inspired. See link below.
//! See: https://github.com/exonum/exonum-java-binding/blob/4e00cd8cbae198ac0e8a49cb1405092537f306bc/exonum-java-binding/core/rust/src/utils/jni_cache.rs

use std::ffi::c_void;

use jni::{
    objects::{GlobalRef, JMethodID},
    sys::{jint, JNI_VERSION_1_6},
    JNIEnv, JavaVM,
};
use log::trace;
use parking_lot::Once;

static INIT: Once = Once::new();

static mut MODEL_VEHICULE_CLASS: Option<GlobalRef> = None;
static mut MODEL_VEHICULE_INIT: Option<JMethodID> = None;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _: *mut c_void) -> jint {
    trace!("JNI OnLoad !!!");
    let env = vm.get_env().expect("Cannot get reference to the JNIEnv");

    #[cfg(target_os = "android")]
    native_activity_create();

    init_cache(&env);
    JNI_VERSION_1_6
}

fn init_cache(env: &JNIEnv) {
    INIT.call_once(|| unsafe { cache_method(env) });
}

fn check_cache_initialized() {
    if !INIT.state().done() {
        panic!("JNI cache is not initialized")
    }
}

unsafe fn cache_method(env: &JNIEnv) {
    MODEL_VEHICULE_CLASS = get_class(&env, "app/chargeprice/api/Vehicule");
    MODEL_VEHICULE_INIT = get_method_id(
        &env,
        "app/chargeprice/api/Vehicule",
        "<init>",
        "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
    );
}
/// Produces `JMethodID` for a particular method dealing with its lifetime.
///
/// Always returns `Some(method_id)`, panics if method not found.
fn get_method_id(env: &JNIEnv, class: &str, name: &str, sig: &str) -> Option<JMethodID<'static>> {
    let method_id = env
        .get_method_id(class, name, sig)
        // we need this line to erase lifetime in order to save underlying raw pointer in static
        .map(|mid| mid.into_inner().into())
        .unwrap_or_else(|_| {
            panic!(
                "Method {} with signature {} of class {} not found",
                name, sig, class
            )
        });
    Some(method_id)
}
/// Returns cached class reference.
///
/// Always returns Some(class_ref), panics if class not found.
fn get_class(env: &JNIEnv, class: &str) -> Option<GlobalRef> {
    let class = env
        .find_class(class)
        .unwrap_or_else(|_| panic!("Class {} not found", class));
    Some(env.new_global_ref(class).unwrap())
}

pub mod runtime_adapter {
    use jni::objects::JMethodID;

    use super::*;

    pub fn vehicule_init() -> JMethodID<'static> {
        check_cache_initialized();
        unsafe { MODEL_VEHICULE_INIT.unwrap() }
    }

    pub fn vehicule_class() -> GlobalRef {
        check_cache_initialized();
        unsafe { MODEL_VEHICULE_CLASS.clone().unwrap() }
    }
}
