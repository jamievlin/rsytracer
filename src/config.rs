use crate::types::{Float, Vec3};

pub const MIN_RAY_DISTANCE: Float = 0.1;
pub const MAX_RECURSION_DEPTH: i32 = 3;

pub const IMAGE_PLANE_Z: Float = 0.0;
pub const IMAGE_PLANE_WIDTH: Float = 1.0;
pub const IMAGE_PLANE_HEIGHT: Float = 1.0;
pub const FOCAL_POINT: Vec3 = Vec3::new(0.0, 0.0, 1.0);

pub const IMAGE_WIDTH: u32 = 1080;
pub const IMAGE_HEIGHT: u32 = 1080;
pub const IMAGE_FILEPATH: &str = "images/out.png";
