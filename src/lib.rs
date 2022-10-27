use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use core::future::Future;
use std::pin::Pin;
use std::task::{Poll, RawWaker, RawWakerVTable, Waker, Context};

unsafe fn waker_clone(data: *const ()) -> RawWaker {
    unimplemented!()
}

unsafe fn waker_wake(data: *const ()) {
    unimplemented!()
}

unsafe fn waker_wake_by_ref(data: *const ()) {
    unimplemented!()
}

unsafe fn waker_drop(data: *const ()) {
    unimplemented!()
}

#[wasm_bindgen]
pub async fn get_rand() -> Option<i32> {
    let mut fut = wasm_bindgen_futures::JsFuture::from(web_sys::window()?.crypto().ok()?.subtle().generate_key_with_object(
        &web_sys::AesKeyGenParams::new("AES-GCM", 256),
        true,
        &js_sys::Array::from(&JsValue::from("encrypt")),
    ).ok()?);
    let x = ();
    let vtable = Box::leak(Box::new(RawWakerVTable::new(
        waker_clone,
        waker_wake,
        waker_wake_by_ref,
        waker_drop,
    )));
    let raw_waker = RawWaker::new(&x as *const (), vtable);
    let waker = unsafe { Waker::from_raw(raw_waker) };
    let mut ctx = Context::from_waker(&waker);
    loop {
        match core::future::Future::poll(Pin::new(&mut fut), &mut ctx) {
            Poll::Ready(x) => return Some(3),
            Poll::Pending => {}
        }
    }
}