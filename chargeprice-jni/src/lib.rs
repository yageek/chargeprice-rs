#[cfg(target_os = "android")]
use android_logger::Config;
use chargeprice_ffi::client::FFIClient;

mod jni_cache;

// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::{objects::JValue, sys::jobject, JNIEnv};

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JString};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::jlong;
use log::{error, trace};

#[cfg(target_os = "android")]
use log::Level;

use jni_cache::runtime_adapter;

#[cfg(target_os = "android")]
fn native_activity_create() {
    android_logger::init_once(
        Config::default()
            .with_min_level(Level::Trace) // limit log level
            .with_tag("chargeprice-jni-android"), // logs will show under mytag tag
    );

    // std::env::set_var("HTTP_PROXY", "http://10.0.2.2:3128");
    // std::env::set_var("HTTPS_PROXY", "http://10.0.2.2:3128");
}

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
    _env: JNIEnv,
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
    _class: JClass,
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

    let client = input as *mut FFIClient;
    let client = unsafe { client.as_ref().unwrap() };

    trace!("Calling Web service...");

    client.load_vehicules(move |result| {
        // Use the `JavaVM` interface to attach a `JNIEnv` to the current thread.
        let env = jvm.attach_current_thread().unwrap();

        match result {
            Ok(v) => {
                trace!("Conversion starting...");
                // We instantiate an ArrayList
                let array_list_class = env
                    .find_class("java/util/ArrayList")
                    .expect("expect Array list");

                let array_list = env
                    .new_object(array_list_class, "()V", &[])
                    .expect("error during array init");

                for r in v.data().iter() {
                    trace!("Loop...");
                    // We convert elements to java
                    let identifier = env.new_string(r.id()).expect("valid string");
                    let brand = env.new_string(r.attributes().brand()).expect("valid brand");
                    let man = env
                        .new_string(r.relationships().unwrap().manufacturer_id())
                        .expect("valid brand");

                    // We create one element
                    trace!("Constructor...");

                    let new_vehicule = env
                        .new_object_unchecked(
                            &runtime_adapter::vehicule_class(),
                            runtime_adapter::vehicule_init(),
                            &[
                                JValue::Object(identifier.into()),
                                JValue::Object(brand.into()),
                                JValue::Object(man.into()),
                            ],
                        )
                        .expect("valid element");

                    trace!("Adding elements...");
                    let _ = env
                        .call_method(
                            array_list,
                            "add",
                            "(Ljava/lang/Object;)Z",
                            &[JValue::Object(new_vehicule)],
                        )
                        .expect("correct initialisation");
                }

                trace!("Call callback...");
                env.call_method(
                    &callback,
                    "onVehiculeSuccess",
                    "(Ljava/util/ArrayList;)V",
                    &[JValue::Object(array_list)],
                )
                .expect("valid callback");
            }

            Err(e) => {
                error!("An errors occurs during fetching the elements: {:?}", e);

                let message = env.new_string(format!("{:?}", e)).expect("valid string");
                env.call_method(
                    &callback,
                    "onVehiculeError",
                    "(Ljava/lang/String;)V",
                    &[JValue::Object(message.into())],
                )
                .expect("valid call");
            }
        }
        trace!("[END] Loading vehicules...");
    });
}
