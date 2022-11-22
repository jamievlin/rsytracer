use crate::scene::TransformData;

struct Scene {
    child_nodes: Vec<Box<SceneNode>>,
    drawables: Vec<Box<dyn Drawable>>
}

struct SceneNode {
    transform: TransformData,
    scene: Scene
}

trait Drawable {

}

