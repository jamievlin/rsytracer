use glm::Vec3;
use serde_json::{Error, Map, Value};

use crate::scene::structure::{DrawableObject, DrawableObjectData, SceneNode};

impl TryFrom<&Map<String, Value>> for DrawableObject {
    type Error = &'static str;

    fn try_from(val: &Map<String, Value>) -> Result<Self, Self::Error> {
        let raw_position =
            val["position"].as_array()
            .ok_or("Position cannot be loaded as array!")?;

        let position = build_vec3(raw_position)
            .ok_or("Cannot construct position vec3!")?;

        let drawable_data = val["data"]
            .as_object()
            .ok_or("Drawable data cannot be loaded as array!")?;

        let drawable = match val["type"].as_str() {
            Some("sphere") => build_sphere_data(drawable_data),
            _ => None
        }.ok_or("Cannot build drawable data!")?;

        return Ok(DrawableObject { position, drawable });
    }
}

fn build_sphere_data(json_val: &Map<String, Value>) -> Option<DrawableObjectData> {
    return match json_val["radius"].as_f64() {
        Some(radius) => {
            Some(DrawableObjectData::Sphere { radius: radius as f32 })
        },
        _ => None
    }
}

pub fn build_scene(json_data: &String) -> serde_json::Result<()> {
    let json_val: Value = serde_json::from_str(json_data)?;

    return Ok(());
}

fn build_vec3(json_val: &Vec<Value>) -> Option<Vec3> {
    match json_val.as_slice() {
        [x, y, z] => match (x.as_f64(), y.as_f64(), z.as_f64()) {
            (Some(fx), Some(fy), Some(fz)) =>
                Some(Vec3::new(fx as f32, fy as f32, fz as f32)),
            _ => None
        },
        _ => None
    }
}

// tests

#[cfg(test)]
mod tests {
    use serde_json::{Error, Value};
    use crate::scene::builder;
    use crate::scene::structure::{DrawableObject, DrawableObjectData};

    fn check_f32_equals(x: f32, y: f32) -> bool {
        return (x - y).abs() < f32::EPSILON;
    }

    #[test]
    fn test_build_vec3() -> Result<(), String> {
        let json_data = "[2.5, 3.3, 4.6]";
        let json_val: Value = serde_json::from_str(json_data)
            .ok().ok_or("Cannot load json")?;
        let vec_result = builder::build_vec3(
            json_val.as_array().ok_or("Cannot convert value to array!")?
        ).ok_or("Cannot build vec3")?;

        return match vec_result.as_slice() {
            [x, y, z] => {
                assert!(
                    check_f32_equals(x.clone(), 2.5_f32)
                    && check_f32_equals(y.clone(), 3.3_f32)
                    && check_f32_equals(z.clone(), 4.6_f32));
                Ok(())
            },
            _ => Err(String::from("Cannot convert vec_result into [f32;3]!"))
        };
    }

    #[test]
    fn test_build_sphere() -> Result<(), String> {
        let json_data = r#" {
          "position": [0.0, 0.1, 0.2],
          "type": "sphere",
          "data": {
            "radius": 1.6
          }
        }
        "#;

        let json_val: Value = serde_json::from_str(json_data)
            .ok().ok_or("Cannot load json")?;

        let drawable_object_json = json_val
            .as_object()
            .ok_or("Json data not in Object(Map<String, Value>) type!")?;

        let drawable_object = DrawableObject::try_from(drawable_object_json)?;

        return match &drawable_object.drawable {
            DrawableObjectData::Sphere { radius } => {
                let expected_radius = 1.6_f32;
                if check_f32_equals(radius.clone(), expected_radius) {
                    Ok(())
                } else {
                    Err(format!(
                        "Radius is not equal. Actual: {}, Expected: {}", radius, expected_radius))
                }
            },
            _ => Err(String::from("Object is not of sphere type!"))
        };
    }
}
