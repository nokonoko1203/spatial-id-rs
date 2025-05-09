use pyo3::prelude::*;
use spatial_id_rs::cell::zfxy::ZFXY;

#[pyfunction]
#[pyo3(text_signature = "(lat, lon, alt, zoom)")]
fn generate_spatial_id(lat: f64, lon: f64, alt: f64, zoom: u8) -> PyResult<String> {
    let cell = ZFXY::from_lat_lon_alt(lat, lon, alt, zoom);
    Ok(cell.to_spatial_id_str())
}

#[pymodule]
fn spatial_id_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_spatial_id, m)?)?;
    Ok(())
}
