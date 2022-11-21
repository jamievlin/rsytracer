use crate::{
    config,
    types::{Float, Intersection, Material, Object, Ray, Vec3},
};

pub struct Sphere {
    pub origin: Vec3,
    pub radius: Float,
    pub material: Material,
}

impl Object for Sphere {
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<Intersection<'a>> {
        // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let diff = ray.origin - self.origin;
        let delta = ray.dir.dot(&diff).powi(2) - diff.norm_squared() + self.radius.powi(2);

        if delta < 0.0 {
            return None;
        }

        let sqrt_delta = delta.sqrt();
        let base = -ray.dir.dot(&diff);

        let min_distance = [base + sqrt_delta, base - sqrt_delta]
            .into_iter()
            .filter(|&distance| distance > config::MIN_RAY_DISTANCE)
            .min_by(Float::total_cmp);

        match min_distance {
            Some(min_distance) => Some(Intersection {
                distance: min_distance,
                material: &self.material,
                normal: ((ray.origin + min_distance * ray.dir) - self.origin).normalize(),
            }),
            None => None,
        }
    }
}
