use std::io;
use wasm_bindgen::prelude::*;
use std::num::sqrt;
use std::num::powi;

#[wasm_bindgen]
pub fn dbtp(x1:f64,y1:f64,x2:f64,y2:f64)->f64{
    return ((((x2-x1).powi(2)) + ((y2-y1).powi(2))).sqrt())
}
