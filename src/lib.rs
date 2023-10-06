extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use image;
use rqrr;

// 看这篇博文https://betterprogramming.pub/webassembly-is-it-as-scary-as-it-sounds-b0c38fb2d9c8

#[wasm_bindgen]
pub fn decode_qr(bytes: &[u8]) -> String {
    let img = match image::load_from_memory(&bytes) {
        Ok(v) => v,
        Err(_e) => return format!("{}", "[Error] Failed when trying to load image"),
    };

    let img = img.to_luma();

    // Prepare for detection
    let mut img = rqrr::PreparedImage::prepare(img);
    // Search for grids, without decoding
    let grids = img.detect_grids();

    if grids.len() != 1 {
        panic!( "[Error] No QR code detected in image");
    }

    // Decode the grid
    let (_meta, content) = match grids[0].decode() {
        Ok(v) => v,
        Err(_e) =>  panic!("[Error] Failed decoding the image"),
    };

    return format!("{}", content);
}