use wasm_bindgen::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

mod webgl_utils;
mod internal;
mod shader;

/// 30 frames per 1000 miliseconds.
const FRAME_TIMEOUT: f64 = 1000.0 / 30.0;

/// We need a hook to print rust compilation error to browser console.
/// Otherwise we will have only 'Runtime error' message in console.
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

/// Main loop.
/// Rerenders screen with given timeout `FRAME_TIMEOUT`.
#[wasm_bindgen]
pub fn main_loop() -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut last_update_time = js_sys::Date::now();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        webgl_utils::request_animation_frame(f.borrow().as_ref().unwrap());
        if js_sys::Date::now() - last_update_time > FRAME_TIMEOUT {
            internal::engine::render().unwrap();
            last_update_time = js_sys::Date::now();
        }
    }) as Box<dyn FnMut()>));

    webgl_utils::request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
