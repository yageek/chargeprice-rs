use std::{sync::mpsc, thread};

use chargeprice_ffi::client::FFIClient;
// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::{
    sys::{jobject, jvalue},
    JNIEnv,
};

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JString};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::jlong;
use log::error;

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_app_chargeprice_api_Client_createNewClient(
    env: JNIEnv,
    // This is the class that owns our static method. It's not going to be used,
    // but still must be present to match the expected signature of a static
    // native method.
    _class: JClass,
    input: JString,
) -> jlong {
    let key: String = env
        .get_string(input)
        .expect("couldnt't get the serial key")
        .into();

    let client = match FFIClient::new(&key, "ffi-java") {
        Ok(client) => client,
        Err(e) => {
            error!("Error during initialization: {}", e);
            return 0;
        }
    };

    let ptr = Box::into_raw(Box::new(client));
    ptr as jlong
}

// This keeps Rust from "mangling" the name and making it unique for this
// crate.

#[no_mangle]
pub extern "system" fn Java_app_chargeprice_api_Client_releaseClient(
    env: JNIEnv,
    // This is the class that owns our static method. It's not going to be used,
    // but still must be present to match the expected signature of a static
    // native method.
    _class: JClass,
    input: jlong,
) {
    assert!(input != 0);
    let client = input as *mut FFIClient;
    unsafe {
        Box::from_raw(client);
    }
}

#[no_mangle]
pub extern "system" fn Java_app_chargeprice_api_Client_loadVehicules(
    env: JNIEnv,
    class: JClass,
    input: jlong,
    cb: jobject,
) {
    assert!(input != 0);

    // `JNIEnv` cannot be sent across thread boundaries. To be able to use JNI
    // functions in other threads, we must first obtain the `JavaVM` interface
    // which, unlike `JNIEnv` is `Send`.
    let jvm = env.get_java_vm().unwrap();

    // We need to obtain global reference to the `callback` object before sending
    // it to the thread, to prevent it from being collected by the GC.
    let callback = env.new_global_ref(cb).unwrap();

    // Use channel to prevent the Java program to finish before the thread
    // has chance to start.
    let (tx, rx) = mpsc::channel();

    let client = input as *mut FFIClient;
    let client = unsafe { client.as_ref().unwrap() };

    client.load_vehicules(|result| {
        let _ = thread::spawn(move || {
            // Signal that the thread has started.
            tx.send(()).unwrap();

            // Use the `JavaVM` interface to attach a `JNIEnv` to the current thread.
            let env = jvm.attach_current_thread().unwrap();

            match result {
                Ok(v) => {
                    // Now we need to convert each rust object into JNI
                    let vehicule_class = env
                        .find_class("app/chargeprice/api/Vehicule")
                        .expect("Vehicule class found");
                    let jni_obj = v.data().into_iter().map(|r| {

                        let identifier = env.new_string(r.id()).expect("valid string");
                        let brand = env.new_string(r.attributes().brand()).expect("valid brand");

                        let new = env.call_method(
                            vehicule_class,
                            "<init>",
                            "(Ljava/lang/String;Ljava/lang/String;)Lapp/chargeprice/client/Vehicule;",
                            &[identifier.into_inner(), brand],
                        );
                    });
                    let _ = env.call_method(
                        cb,
                        "onSuccess",
                        "(Lapp/chargeprice/api/ClientListener;)V",
                        &[],
                    );
                }

                Err(_) => {}
            }
            // The current thread is detached automatically when `env` goes out of scope.
        });
    });

    // Wait until the thread has started.
    rx.recv().unwrap();
}
