use glm::Vec3;
use serde_json::{Error, Value};



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
}