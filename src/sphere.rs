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
        let distance1 = {
            let distance = base + sqrt_delta;
            if distance > config::MIN_RAY_DISTANCE {
                Some(distance)
            } else {
                None
            }
        };
        let distance2 = {
            let distance = base - sqrt_delta;
            if distance > config::MIN_RAY_DISTANCE {
                Some(distance)
            } else {
                None
            }
        };
        let min_distance = match (distance1, distance2) {
            (Some(d1), Some(d2)) => Some(d1.min(d2)),
            (Some(d1), None) => Some(d1),
            (None, Some(d2)) => Some(d2),
            (None, None) => None,
        };

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
