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
