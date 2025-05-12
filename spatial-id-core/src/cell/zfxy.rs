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

    pub fn to_lat_lon_alt(&self) -> LatLonAlt {
        let n = 2.0f64.powi(self.zoom as i32);
        let vz = MAX_ALT / n;
        let (lat, lon) =
            crate::projection::web_mercator::tile_xy_to_latlon(self.x, self.y, self.zoom);
        let alt = self.floor as f64 * vz;
        LatLonAlt { lat, lon, alt }
    }

    pub fn to_tile_hash(&self) -> String {
        if self.zoom == 0 {
            return "".to_string();
        }

        let original_floor_sign = self.floor < 0;
        let mut current_tile = ZFXY {
            zoom: self.zoom,
            floor: self.floor.abs(),
            x: self.x,
            y: self.y,
        };
        let mut out = String::new();

        while current_tile.zoom > 0 {
            let parent = current_tile
                .get_parent()
                .expect("Parent should exist for zoom > 0");

            let children_of_parent = parent.get_children();

            let position_in_parent = children_of_parent.iter().position(|&child| {
                child.floor == current_tile.floor
                    && child.x == current_tile.x
                    && child.y == current_tile.y
                    && child.zoom == current_tile.zoom
            });

            match position_in_parent {
                Some(index) => {
                    out.insert_str(0, &(index + 1).to_string());
                }
                None => {
                    panic!("Could not find tile within parent's children during hash generation");
                }
            }
            current_tile = parent;
        }

        if original_floor_sign {
            out.insert(0, '-');
        }
        out
    }

    pub fn to_spatial_id_str(&self) -> String {
        format!("/{}/{}/{}/{}", self.zoom, self.floor, self.x, self.y)
    }

    pub fn bvol_cover(min_coord: LatLonAlt, max_coord: LatLonAlt, zoom: u8) -> Vec<ZFXY> {
        let Some((min_x, min_y)) = crate::projection::web_mercator::latlon_to_tile_xy(
            max_coord.lat,
            min_coord.lon,
            zoom.into(),
        ) else {
            panic!(
                "Invalid min_coord for tile conversion in bvol_cover (calculating min_x, min_y)"
            );
        };
        let Some((max_x, max_y)) = crate::projection::web_mercator::latlon_to_tile_xy(
            min_coord.lat,
            max_coord.lon,
            zoom.into(),
        ) else {
            panic!(
                "Invalid max_coord for tile conversion in bvol_cover (calculating max_x, max_y)"
            );
        };

        let min_floor = min_coord.alt.floor() as i32;
        let max_floor = max_coord.alt.floor() as i32;

        let mut cover = Vec::new();

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let start_floor = min_floor;
                let end_floor = max_floor;

                if start_floor <= end_floor {
                    for floor in start_floor..=end_floor {
                        cover.push(ZFXY { zoom, floor, x, y });
                    }
                }
            }
        }

        cover
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coords::types::LatLonAlt;

    #[test]
    fn test_centroid() {
        let cell = ZFXY {
            zoom: 1,
            floor: 0,
            x: 0,
            y: 0,
        };
        assert_eq!(
            cell.centroid(),
            LatLonAlt {
                lat: 66.51326044311186,
                lon: -90.0,
                alt: 8388608.0,
            }
        );
    }

    #[test]
    fn test_vertices() {
        let cell = ZFXY {
            zoom: 1,
            floor: 0,
            x: 0,
            y: 0,
        };
        let expected_vertices = [
            LatLonAlt {
                lat: 0.0,
                lon: -180.0,
                alt: 0.0,
            },
            LatLonAlt {
                lat: 0.0,
                lon: 0.0,
                alt: 0.0,
            },
            LatLonAlt {
                lat: 85.0511287798066,
                lon: 0.0,
                alt: 0.0,
            },
            LatLonAlt {
                lat: 85.0511287798066,
                lon: -180.0,
                alt: 0.0,
            },
            LatLonAlt {
                lat: 0.0,
                lon: -180.0,
                alt: 16777216.0,
            },
            LatLonAlt {
                lat: 0.0,
                lon: 0.0,
                alt: 16777216.0,
            },
            LatLonAlt {
                lat: 85.0511287798066,
                lon: 0.0,
                alt: 16777216.0,
            },
            LatLonAlt {
                lat: 85.0511287798066,
                lon: -180.0,
                alt: 16777216.0,
            },
        ];
        assert_eq!(cell.vertices(), expected_vertices);
    }

    #[test]
    fn test_from_lat_lon_alt() {
        let cell = ZFXY::from_lat_lon_alt(0.0, 0.0, 10.0, 25);
        assert_eq!(cell.zoom, 25);
        assert_eq!(cell.floor, 10);
        assert_eq!(cell.x, 16777216);
        assert_eq!(cell.y, 16777216);
    }

    #[test]
    fn test_to_lat_lon_alt() {
        let cell = ZFXY {
            zoom: 25,
            floor: 10,
            x: 16777216,
            y: 16777216,
        };
        let expected_lat_lon_alt = LatLonAlt {
            lat: 0.0,
            lon: 0.0,
            alt: 10.0,
        };
        assert_eq!(cell.to_lat_lon_alt(), expected_lat_lon_alt);
    }

    #[test]
    fn test_to_tile_hash() {
        let lat = 0.0;
        let lon = 0.0;
        let alt = 10.0;
        let zoom = 25;
        let cell = ZFXY::from_lat_lon_alt(lat, lon, alt, zoom);

        let expected_hash = "4111111111111111111115151";
        let actual_hash = cell.to_tile_hash();
        assert_eq!(actual_hash, expected_hash);

        let cell = ZFXY {
            zoom: 18,
            floor: 0,
            x: 232851,
            y: 103211,
        };
        let expected_hash = "244113223421323144";
        let actual_hash = cell.to_tile_hash();
        assert_eq!(actual_hash, expected_hash);
    }

    #[test]
    fn test_to_spatial_id_str() {
        let lat = 0.0;
        let lon = 0.0;
        let alt = 10.0;
        let zoom = 25;
        let cell = ZFXY::from_lat_lon_alt(lat, lon, alt, zoom);

        let expected_id = "/25/10/16777216/16777216";
        let actual_id = cell.to_spatial_id_str();
        assert_eq!(actual_id, expected_id);
    }
}
