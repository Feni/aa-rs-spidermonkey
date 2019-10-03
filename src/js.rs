use mozjs::glue::RUST_JSID_IS_STRING;
use mozjs::glue::RUST_JSID_TO_STRING;
use mozjs::jsapi::GetPropertyKeys;
use mozjs::jsapi::JSITER_OWNONLY;
use mozjs::jsapi::JS_NewGlobalObject;
use mozjs::jsapi::JS_StringEqualsAscii;
use mozjs::jsapi::OnNewGlobalHookOption;
use mozjs::jsval::UndefinedValue;
use mozjs::rust::IdVector;
use mozjs::rust::JSEngine;
// use mozjs::rust::RealmOptions;   // Newer, unpublished mozjs version
use mozjs::jsapi::CompartmentOptions;
use mozjs::rust::Runtime;
use mozjs::rust::SIMPLE_GLOBAL_CLASS;
use mozjs::rust::ToString;
use std::ptr;
use std::sync::Arc;
use actix_web::{HttpRequest, Responder, http::StatusCode};


use mozjs::rust::wrappers::JS_EncodeStringToUTF8;


use crate::models::View;
use crate::timing::Timer;

use std::ffi::CStr;
use std::str;

use mozjs::conversions::jsstr_to_string;
// pub static mut JSE: Arc<JSEngine> = JSEngine::init().unwrap();



pub fn exec_js(js: Arc<JSEngine>, view: View) -> String {
    println!("Exec js");
    let mut timer = Timer::new();
    // let mut JSE: Arc<JSEngine> = JSEngine::init().unwrap();
    // timer.checkpoint("Engine init");
    let rt = Runtime::new(js);
    timer.checkpoint("Runtime init");
    let cx = rt.cx();
    timer.checkpoint("Context init");
    // let options = RealmOptions::default();

    unsafe {
        rooted!(in(cx) let global =
            JS_NewGlobalObject(cx, &SIMPLE_GLOBAL_CLASS, ptr::null_mut(),
                               OnNewGlobalHookOption::FireOnNewGlobalHook,
                               &CompartmentOptions::default())
        );
        timer.checkpoint("Global context");
        let code = view.content.unwrap();
        println!("Exec {}", &code);

        rooted!(in(cx) let mut rval = UndefinedValue());
        rt.evaluate_script(global.handle(), &code,
            "test", 1, rval.handle_mut());

        timer.checkpoint("Finish exec");

        println!("Returns object? {:?}", rval.is_object());

        let mut jsstr = ToString(cx, rval.handle());
        let rstr = jsstr_to_string(cx, jsstr);
        // println!("{}", &rstr);


        // let mptr = JS_EncodeStringToUTF8(cx, rval.handle());
        // let mstr = CStr::from_ptr(rstr);
        
        // println!("{}", str::from_utf8(mstr.to_bytes()).unwrap());
        // println!("{}", str::from_utf8(mstr.to_bytes()).unwrap());

        return String::from(&rstr);
    }


    // let mut response = Response::new(Body::from("JS"));
    // return String::from("From JS")
}