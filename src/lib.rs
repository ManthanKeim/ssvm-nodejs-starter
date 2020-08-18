use std::io;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn dbtp(x1:f64,y1:f64)->f64{
    return (x1/y1)
}
