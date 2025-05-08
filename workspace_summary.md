# Workspace Summary

## Directory Structure

```
- Cargo.lock
- Cargo.toml
- README.md
- examples/
- src/
  - cell/
    - mod.rs
    - spatial_cell.rs
    - xyz.rs
    - zfxy.rs
  - coords/
    - mod.rs
    - types.rs
  - lib.rs
  - projection/
    - mod.rs
    - web_mercator.rs
```

## File Contents

### Cargo.toml

```toml
[package]
name = "spatial-id-rs"
version = "0.1.0"
edition = "2021"

[dependencies]

```

### src/lib.rs

```rust
pub mod cell;
pub mod coords;
pub mod projection;

```

### src/cell/mod.rs

```rust
{{ ... }}
pub mod spatial_cell;
pub mod xyz;
pub mod zfxy;

```

### src/cell/spatial_cell.rs

```rust
use std::fmt::Debug;
use std::hash::Hash;

pub type Vertices2D<T> = [T; 4];
pub type Vertices3D<T> = [T; 8];

pub trait SpatialCell: Debug + Clone + Copy + PartialEq + Eq + Hash {
    type Coord: Copy + Default + Debug;
    type Vertices: Copy + Default + Debug + AsRef<[Self::Coord]>;

    fn centroid(&self) -> Self::Coord;
    fn vertices(&self) -> Self::Vertices;
    fn bbox(&self) -> (Self::Coord, Self::Coord);
}

```

### src/cell/xyz.rs

```rust
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

```

### src/cell/zfxy.rs

```rust
use std::f64::consts::PI;
use std::fmt::Debug;

use super::spatial_cell::{SpatialCell, Vertices3D};
use crate::coords::types::LatLonAlt;

const MAX_ALT: f64 = 33_554_432.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ZFXY {
    pub zoom: u8,
    pub floor: i32,
    pub x: u32,
    pub y: u32,
}

impl SpatialCell for ZFXY {
    type Coord = LatLonAlt;
    type Vertices = Vertices3D<Self::Coord>;

    fn centroid(&self) -> Self::Coord {
        let n = 2.0f64.powi(self.zoom as i32);
        let lon_deg = ((self.x as f64 + 0.5) / n) * 360.0 - 180.0;
        let lat_rad = (PI * (1.0 - 2.0 * (self.y as f64 + 0.5) / n)).sinh().atan();
        let lat_deg = lat_rad.to_degrees();
        let vz = MAX_ALT / n;
        let alt = (self.floor as f64 + 0.5) * vz;
        Self::Coord {
            lat: lat_deg,
            lon: lon_deg,
            alt,
        }
    }

    fn vertices(&self) -> Self::Vertices {
        let n = 2.0f64.powi(self.zoom as i32);
        let vz = MAX_ALT / n;

        let (lat_nw, lon_nw) =
            crate::projection::web_mercator::tile_xy_to_latlon(self.x, self.y, self.zoom);
        let (lat_se, lon_se) =
            crate::projection::web_mercator::tile_xy_to_latlon(self.x + 1, self.y + 1, self.zoom);

        let alt_bottom = self.floor as f64 * vz;
        let alt_top = (self.floor as f64 + 1.0) * vz;

        let lat_n = lat_nw;
        let lat_s = lat_se;
        let lon_w = lon_nw;
        let lon_e = lon_se;

        [
            Self::Coord {
                lat: lat_s,
                lon: lon_w,
                alt: alt_bottom,
            },
            Self::Coord {
                lat: lat_s,
                lon: lon_e,
                alt: alt_bottom,
            },
            Self::Coord {
                lat: lat_n,
                lon: lon_e,
                alt: alt_bottom,
            },
            Self::Coord {
                lat: lat_n,
                lon: lon_w,
                alt: alt_bottom,
            },
            Self::Coord {
                lat: lat_s,
                lon: lon_w,
                alt: alt_top,
            },
            Self::Coord {
                lat: lat_s,
                lon: lon_e,
                alt: alt_top,
            },
            Self::Coord {
                lat: lat_n,
                lon: lon_e,
                alt: alt_top,
            },
            Self::Coord {
                lat: lat_n,
                lon: lon_w,
                alt: alt_top,
            },
        ]
    }

    fn bbox(&self) -> (Self::Coord, Self::Coord) {
        let (lat_nw, lon_nw) =
            crate::projection::web_mercator::tile_xy_to_latlon(self.x, self.y, self.zoom);
        let (lat_se, lon_se) =
            crate::projection::web_mercator::tile_xy_to_latlon(self.x + 1, self.y + 1, self.zoom);

        let min_coord = Self::Coord {
            lat: lat_se,
            lon: lon_nw,
            alt: self.floor as f64,
        };
        let max_coord = Self::Coord {
            lat: lat_nw,
            lon: lon_se,
            alt: (self.floor + 1) as f64,
        };
        (min_coord, max_coord)
    }
}

impl ZFXY {
    fn get_parent(&self) -> Option<ZFXY> {
        if self.zoom == 0 {
            return None;
        }
        let parent_floor_abs = self.floor.abs() / 2;
        Some(ZFXY {
            zoom: self.zoom - 1,
            floor: parent_floor_abs,
            x: self.x / 2,
            y: self.y / 2,
        })
    }

    fn get_children(&self) -> [ZFXY; 8] {
        let child_zoom = self.zoom + 1;
        let px = self.x * 2;
        let py = self.y * 2;
        let pf_abs = self.floor.abs() * 2;

        [
            ZFXY {
                zoom: child_zoom,
                floor: pf_abs,
                x: px,
                y: py,
            },
            ZFXY {
                zoom: child_zoom,
                floor: pf_abs,
                x: px + 1,
                y: py,
            },
            ZFXY {
                zoom: child_zoom,
                floor: pf_abs,
                x: px,
                y: py + 1,
            },
            ZFXY {
                zoom: child_zoom,
                floor: pf_abs,
                x: px + 1,
                y: py + 1,
            },
            ZFXY {
                zoom: child_zoom,
                floor: pf_abs + 1,
                x: px,
                y: py,
            },
            ZFXY {
                zoom: child_zoom,
                floor: pf_abs + 1,
                x: px + 1,
                y: py,
            },
            ZFXY {
                zoom: child_zoom,
                floor: pf_abs + 1,
                x: px,
                y: py + 1,
            },
            ZFXY {
                zoom: child_zoom,
                floor: pf_abs + 1,
                x: px + 1,
                y: py + 1,
            },
        ]
    }

    pub fn from_lat_lon_alt(lat: f64, lon: f64, alt: f64, zoom: u8) -> Self {
        let Some((x, y)) =
            crate::projection::web_mercator::latlon_to_tile_xy(lat, lon, zoom.into())
        else {
            panic!("Invalid lat/lon for tile conversion in from_lat_lon_alt");
        };
        let floor = alt.floor() as i32;
        ZFXY { zoom, floor, x, y }
    }

    pub fn to_tile_hash(&self) -> String {
        if self.zoom == 0 {
            return "".to_string();
        }
        // ... (remaining code for to_tile_hash) ...
    }

    pub fn to_spatial_id_str(&self) -> String {
        // ... (code for to_spatial_id_str) ...
    }

    pub fn bvol_cover(min_coord: LatLonAlt, max_coord: LatLonAlt, zoom: u8) -> Vec<ZFXY> {
        // ... (code for bvol_cover) ...
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // ... (test functions) ...
}

```

### src/coords/mod.rs

```rust
{{ ... }}
pub mod types;

```

### src/coords/types.rs

```rust
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

```

### src/projection/mod.rs

```rust
{{ ... }}
pub mod web_mercator;

```

### src/projection/web_mercator.rs

```rust
use std::f64::consts::PI;

pub const MAX_LATITUDE: f64 = 85.05112877980659;

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

pub fn tile_xy_to_latlon(x: u32, y: u32, zoom: u8) -> (f64, f64) {
    let n = 2.0f64.powi(zoom as i32);
    let lon_deg = (x as f64 / n) * 360.0 - 180.0;

    let lat_rad = (PI * (1.0 - 2.0 * (y as f64) / n)).sinh().atan();
    let lat_deg = lat_rad.to_degrees();

    (lat_deg, lon_deg)
}
