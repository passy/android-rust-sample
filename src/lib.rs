use jni::objects::{JObject, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn Java_me_passy_rustexample_MainActivity_hello(
    env: JNIEnv,
    _this: JObject,
    j_recipient: JString,
) -> jstring {
    let recipient =
        CString::from(unsafe { CStr::from_ptr(env.get_string(j_recipient).unwrap().as_ptr()) });

    let output = env
        .new_string(format!("Hello {}!", recipient.to_str().unwrap()))
        .unwrap();
    output.into_inner()
}
