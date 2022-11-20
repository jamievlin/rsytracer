use nalgebra_glm as glm;

pub type Float = f64;
pub type Vec3 = glm::DVec3;
pub type Color = Vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

pub struct PointLight {
    pub origin: Vec3,
    pub color: Color,
    pub intensity: Float,
    pub radius: Float,
}

#[allow(non_snake_case)]
pub struct Material {
    pub Ia: Color,
    pub kd: Color,
    pub ks: Color,
    pub kt: Color,
    pub kn: Float,
}

pub struct Intersection<'a> {
    pub distance: Float,
    pub normal: Vec3,
    pub material: &'a Material,
}

pub trait Object {
    /**
    Finds the first point of intersection between the object and a ray.
    Intersections near the ray should be ignored to account for floating-point
    errors.
    */
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<Intersection<'a>>;
}
