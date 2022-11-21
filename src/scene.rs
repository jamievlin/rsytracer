use crate::config;
use crate::types::{Color, Intersection, Object, PointLight, Ray, Vec3};

pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<PointLight>,
}

impl Scene {
    fn first_intersection(&self, ray: &Ray) -> Option<Intersection> {
        self.objects
            .iter()
            .filter_map(|object| object.intersect(&ray))
            .min_by(|a, b| a.distance.total_cmp(&b.distance))
    }

    fn is_shadowed(&self, light: &PointLight, pos: &Vec3) -> bool {
        // Cast a ray from the object to the light and check for any
        // intersections between the two. We cast from the object to the light,
        // instead of the other way around, so that the intersection at "pos"
        // is ignored.
        let ray = Ray {
            origin: *pos,
            dir: (light.origin - pos).normalize(),
        };
        let max_distance_squared = (light.origin - pos).norm_squared();
        self.objects
            .iter()
            .filter_map(|object| object.intersect(&ray))
            .any(|intersection| {
                intersection.distance * intersection.distance <= max_distance_squared
            })
    }

    fn diffuse(&self, light: &PointLight, origin: &Vec3, normal: &Vec3) -> Color {
        let diff = light.origin - origin;
        let dir = diff.normalize();
        light.color * light.intensity * dir.dot(normal).max(0.0)
            / diff.norm_squared().max(light.radius * light.radius)
    }

    #[allow(non_snake_case)]
    pub fn cast_ray(&self, incident_ray: &Ray, recursion_depth: i32) -> Color {
        if recursion_depth > config::MAX_RECURSION_DEPTH {
            return Color::default();
        }

        if let Some(intersection) = self.first_intersection(incident_ray) {
            let (reflected_ray, refracted_ray) = split_ray(incident_ray, &intersection);

            let material = &intersection.material;
            let origin = incident_ray.origin + intersection.distance * incident_ray.dir;

            let N = intersection.normal;

            let D: Color = self
                .lights
                .iter()
                .filter(|&light| !self.is_shadowed(light, &origin))
                .map(|light| self.diffuse(light, &origin, &N))
                .sum();

            let S = self.cast_ray(&reflected_ray, recursion_depth + 1);
            let T = refracted_ray
                .map(|refracted_ray| self.cast_ray(&refracted_ray, recursion_depth + 1))
                .unwrap_or_default();

            return material.Ia
                + material.kd.component_mul(&D)
                + material.ks.component_mul(&S)
                + material.kt.component_mul(&T);
        }

        return Color::default();
    }
}

#[allow(non_snake_case)]
fn split_ray(incident_ray: &Ray, intersection: &Intersection) -> (Ray, Option<Ray>) {
    let V = incident_ray.dir;
    // "Since these equations assume that V- N is less than zero,
    // the intersection processor must adjust the sign of N so
    // that it points to the side of the surface from which the
    // intersecting ray is incident. It must likewise adjust the
    // index of refraction to account for the sign change.""
    let (N, kn) = {
        let N = intersection.normal;
        let kn = intersection.material.kn;
        if V.dot(&N) > 0.0 {
            (-N, 1.0 / kn)
        } else {
            (N, kn)
        }
    };

    let V_prime = V / V.dot(&N).abs();
    let R = V_prime + 2.0 * N;
    let kf = 1.0 / (kn * kn * V_prime.norm_squared() - (V_prime + N).norm_squared()).sqrt();
    let kf = if kf.is_finite() { Some(kf) } else { None };
    let P = kf.map(|kf| kf * (N + V_prime) - N);

    let intersection_origin = incident_ray.origin + intersection.distance * incident_ray.dir;
    let reflected_ray = Ray {
        origin: intersection_origin,
        dir: R.normalize(),
    };
    let refracted_ray = P.map(|P| Ray {
        origin: intersection_origin,
        dir: P.normalize(),
    });
    (reflected_ray, refracted_ray)
}
