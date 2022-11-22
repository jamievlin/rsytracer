pub mod structure;
pub mod builder;

use glm::Vec3;

pub struct Rotation {
    pitch: f32,
    yaw: f32,
    roll: f32
}

pub struct Transform {
    translate: Vec3,
    rotate: Rotation,
    scale: Vec3
}

enum TransformData {
    Transform,
    TransfMat(glm::Mat3)
}