use spatial_id_rs::cell::zfxy::ZFXY;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmLatLon {
    pub lat: f64,
    pub lon: f64,
}

#[wasm_bindgen]
impl WasmLatLon {
    #[wasm_bindgen(constructor)]
    pub fn new(lat: f64, lon: f64) -> WasmLatLon {
        WasmLatLon { lat, lon }
    }
}

#[wasm_bindgen]
pub fn generate_spatial_id(lat: f64, lon: f64, alt: f64, zoom: u8) -> String {
    let cell = ZFXY::from_lat_lon_alt(lat, lon, alt, zoom);
    cell.to_spatial_id_str()
}
