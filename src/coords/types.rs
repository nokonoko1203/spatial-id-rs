use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LatLon {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LatLonAlt {
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,
}
