use std::fmt::Debug;

use super::spatial_cell::{SpatialCell, Vertices2D};
use crate::coords::types::LatLon;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XYZTile {
    pub zoom: u8,
    pub x: u32,
    pub y: u32,
}

impl SpatialCell for XYZTile {
    type Coord = LatLon;
    type Vertices = Vertices2D<Self::Coord>;

    #[inline]
    fn centroid(&self) -> Self::Coord {
        let n = 2.0f64.powi(self.zoom as i32);
        let lon_deg = ((self.x as f64 + 0.5) / n) * 360.0 - 180.0;
        let lat_rad = (std::f64::consts::PI * (1.0 - 2.0 * (self.y as f64 + 0.5) / n))
            .sinh()
            .atan();
        let lat_deg = lat_rad.to_degrees();
        Self::Coord {
            lat: lat_deg,
            lon: lon_deg,
        }
    }

    #[inline]
    fn vertices(&self) -> Self::Vertices {
        let (lat_nw, lon_nw) =
            crate::projection::web_mercator::tile_xy_to_latlon(self.x, self.y, self.zoom);
        let (lat_se, lon_se) =
            crate::projection::web_mercator::tile_xy_to_latlon(self.x + 1, self.y + 1, self.zoom);

        let lat_n = lat_nw;
        let lat_s = lat_se;
        let lon_w = lon_nw;
        let lon_e = lon_se;

        [
            Self::Coord {
                lat: lat_s,
                lon: lon_w,
            },
            Self::Coord {
                lat: lat_s,
                lon: lon_e,
            },
            Self::Coord {
                lat: lat_n,
                lon: lon_e,
            },
            Self::Coord {
                lat: lat_n,
                lon: lon_w,
            },
        ]
    }

    #[inline]
    fn bbox(&self) -> (Self::Coord, Self::Coord) {
        let (lat_nw, lon_nw) =
            crate::projection::web_mercator::tile_xy_to_latlon(self.x, self.y, self.zoom);
        let (lat_se, lon_se) =
            crate::projection::web_mercator::tile_xy_to_latlon(self.x + 1, self.y + 1, self.zoom);

        let min_coord = Self::Coord {
            lat: lat_se,
            lon: lon_nw,
        };
        let max_coord = Self::Coord {
            lat: lat_nw,
            lon: lon_se,
        };
        (min_coord, max_coord)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::spatial_cell::SpatialCell;
    use crate::coords::types::LatLon;

    const EPSILON: f64 = 1e-6;

    #[test]
    fn test_centroid() {
        let tile = XYZTile {
            zoom: 0,
            x: 0,
            y: 0,
        };
        let centroid = tile.centroid();
        assert!((centroid.lat - 0.0).abs() < EPSILON);
        assert!((centroid.lon - 0.0).abs() < EPSILON);

        let tile = XYZTile {
            zoom: 1,
            x: 0,
            y: 0,
        };
        let centroid = tile.centroid();
        assert!(centroid.lat > 0.0);
        assert!(centroid.lon < 0.0);

        let tile = XYZTile {
            zoom: 1,
            x: 1,
            y: 1,
        };
        let centroid = tile.centroid();
        assert!(centroid.lat < 0.0);
        assert!(centroid.lon > 0.0);
    }

    #[test]
    fn test_vertices() {
        let tile = XYZTile {
            zoom: 1,
            x: 0,
            y: 0,
        };
        let vertices = tile.vertices();

        let expected_sw = LatLon {
            lat: 0.0,
            lon: -180.0,
        };
        let expected_se = LatLon { lat: 0.0, lon: 0.0 };
        let expected_ne = LatLon {
            lat: 85.05112877980659,
            lon: 0.0,
        };
        let expected_nw = LatLon {
            lat: 85.05112877980659,
            lon: -180.0,
        };

        assert!((vertices[0].lat - expected_sw.lat).abs() < EPSILON);
        assert!((vertices[0].lon - expected_sw.lon).abs() < EPSILON);
        assert!((vertices[1].lat - expected_se.lat).abs() < EPSILON);
        assert!((vertices[1].lon - expected_se.lon).abs() < EPSILON);
        assert!((vertices[2].lat - expected_ne.lat).abs() < EPSILON);
        assert!((vertices[2].lon - expected_ne.lon).abs() < EPSILON);
        assert!((vertices[3].lat - expected_nw.lat).abs() < EPSILON);
        assert!((vertices[3].lon - expected_nw.lon).abs() < EPSILON);
    }

    #[test]
    fn test_bbox() {
        let tile = XYZTile {
            zoom: 1,
            x: 0,
            y: 0,
        };
        let (min_coord, max_coord) = tile.bbox();

        let expected_min = LatLon {
            lat: 0.0,
            lon: -180.0,
        };
        let expected_max = LatLon {
            lat: 85.05112877980659,
            lon: 0.0,
        };

        assert!((min_coord.lat - expected_min.lat).abs() < EPSILON);
        assert!((min_coord.lon - expected_min.lon).abs() < EPSILON);
        assert!((max_coord.lat - expected_max.lat).abs() < EPSILON);
        assert!((max_coord.lon - expected_max.lon).abs() < EPSILON);
    }
}
