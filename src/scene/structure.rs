use crate::scene::TransformData;

pub struct Scene {
    child_nodes: Vec<Box<SceneNode>>,
    drawables: Vec<DrawableObjectData>
}

pub struct SceneNode {
    transform: TransformData,
    scene: Scene
}

pub enum DrawableObjectData {
    Sphere { radius: f32 },
}

pub struct DrawableObject {
    pub position: glm::Vec3,
    pub drawable: DrawableObjectData
}
