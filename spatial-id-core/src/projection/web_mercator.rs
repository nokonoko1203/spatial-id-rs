use std::f64::consts::PI;

pub const MAX_LATITUDE: f64 = 85.05112877980659;

#[inline]
pub fn latlon_to_tile_xy(lat: f64, lon: f64, zoom: u32) -> Option<(u32, u32)> {
    let lat_clamped = lat.clamp(-MAX_LATITUDE, MAX_LATITUDE);
    let lat_rad = lat_clamped.to_radians();
    let n = (2.0_f64).powi(zoom as i32);

    let tile_x_f = (lon + 180.0) / 360.0 * n;

    let tile_y_f = (1.0 - lat_rad.tan().asinh() / PI) / 2.0 * n;

    let tile_x = tile_x_f.floor().max(0.0) as u32;
    let tile_y = tile_y_f.floor().max(0.0) as u32;

    let max_tile_index = (1u32 << zoom).saturating_sub(1u32);
    Some((tile_x.min(max_tile_index), tile_y.min(max_tile_index)))
}

#[inline]
pub fn tile_xy_to_latlon(x: u32, y: u32, zoom: u8) -> (f64, f64) {
    let n = 2.0f64.powi(zoom as i32);
    let lon_deg = (x as f64 / n) * 360.0 - 180.0;

    let lat_rad = (PI * (1.0 - 2.0 * (y as f64) / n)).sinh().atan();
    let lat_deg = lat_rad.to_degrees();

    (lat_deg, lon_deg)
}
