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
use std::ptr;
use std::sync::Arc;
use actix_web::{HttpRequest, Responder, http::StatusCode};



use crate::models::View;
use crate::timing::Timer;


// pub static mut JSE: Arc<JSEngine> = JSEngine::init().unwrap();



pub fn exec_js(view: View) -> String {
    println!("Exec js");
    let mut timer = Timer::new();
    let mut JSE: Arc<JSEngine> = JSEngine::init().unwrap();
    timer.checkpoint("Engine init");
    let rt = Runtime::new(JSE);
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

        rooted!(in(cx) let mut rval = UndefinedValue());
        rt.evaluate_script(global.handle(), "({ 'a': 7 })",
            "test", 1, rval.handle_mut());

        timer.checkpoint("Finish exec");

        println!("Returns object? {:?}", rval.is_object());
        

    }


    // let mut response = Response::new(Body::from("JS"));
    return String::from("From JS")
}