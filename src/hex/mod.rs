//! Hexagonal map module.
//!
//! Based on the excellent tutorials written respectively by
//! Jasper Flick and by Amit Patel:
//!
//! - https://catlikecoding.com/unity/tutorials/hex-map
//! - https://www.redblobgames.com/grids/hexagons

mod hex_cell;
mod hex_grid;

/// Hexagon's outer radius.
const OUTER_RADIUS: f32 = 10.;

/// Hexagon's inner radius.
/// The inner radius is equals to √3/2 times the outer radius.
/// In our case it's equals to 5 * √3.
const INNER_RADIUS: f32 = OUTER_RADIUS * 0.866_025_4;

/// Hexagon's inner diameter.
const INNER_DIAMETER: f32 = INNER_RADIUS * 2.;

/// Hexagon's corners.
/// Orientation is pointy side up ⬡.
const CORNERS: [[f32; 3]; 6] = [
    [0., 0., OUTER_RADIUS],
    [INNER_RADIUS, 0., 0.5 * OUTER_RADIUS],
    [INNER_RADIUS, 0., -0.5 * OUTER_RADIUS],
    [0., 0., -OUTER_RADIUS],
    [-INNER_RADIUS, 0., -0.5 * OUTER_RADIUS],
    [-INNER_RADIUS, 0., 0.5 * OUTER_RADIUS],
];

pub use self::{hex_cell::HexCell, hex_grid::HexGrid};
