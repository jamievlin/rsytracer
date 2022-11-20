/**
 * Implementation based on "An Improved Illumination Model for Shaded Display",
 * https://dl.acm.org/doi/pdf/10.1145/358876.358882
 */
mod config;
mod scene;
mod sphere;
mod types;

use crate::scene::Scene;
use crate::sphere::Sphere;
use crate::types::{Color, Float, Material, PointLight, Ray, Vec3};
use image::{Rgb, RgbImage};
use log::info;

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

fn main() {
    setup_logger().unwrap();

    // +x is right, +y is up, +z is in
    let scene = Scene {
        objects: vec![
            Box::new(Sphere {
                origin: Vec3::new(0.0, 0.0, 20.0),
                radius: 10.0,
                material: Material {
                    Ia: Color::new(0.01, 0.01, 0.01),
                    kd: Color::new(0.1, 0.1, 0.1),
                    ks: Color::new(0.1, 0.1, 0.1),
                    kt: Color::repeat(1.0),
                    kn: 0.995,
                },
            }),
            Box::new(Sphere {
                origin: Vec3::new(0.0, 10.0, 25.0),
                radius: 10.0,
                material: Material {
                    Ia: Color::new(0.0, 0.1, 0.1),
                    kd: Color::new(0.0, 1.0, 1.0),
                    ks: Color::new(0.0, 0.5, 0.5),
                    kt: Color::repeat(0.0),
                    kn: 1.0,
                },
            }),
            Box::new(Sphere {
                origin: Vec3::new(0.0, 0.0, 300.0),
                radius: 265.0,
                material: Material {
                    Ia: Color::new(0.1, 0.0, 0.0),
                    kd: Color::new(0.9, 0.1, 0.1),
                    ks: Color::new(1.0, 1.0, 1.0),
                    kt: Color::repeat(0.0),
                    kn: 1.0,
                },
            }),
        ],
        lights: vec![PointLight {
            color: Color::repeat(1.0),
            origin: Vec3::new(3.0, 10.0, 10.0),
            intensity: 200.0,
            radius: 1.0,
        }],
    };

    info!("Rendering scene");
    let mut buffer = RgbImage::new(config::IMAGE_WIDTH, config::IMAGE_HEIGHT);
    for x in 0..buffer.width() {
        for y in 0..buffer.height() {
            let ray_origin = Vec3::new(
                (x as Float) / (buffer.width() as Float) * (config::IMAGE_PLANE_WIDTH * 2.0)
                    - config::IMAGE_PLANE_WIDTH,
                (y as Float) / (buffer.height() as Float) * (config::IMAGE_PLANE_HEIGHT * 2.0)
                    - config::IMAGE_PLANE_HEIGHT,
                config::IMAGE_PLANE_Z,
            );
            let ray_dir = (config::FOCAL_POINT - ray_origin).normalize();
            let color = scene.cast_ray(
                &Ray {
                    origin: ray_origin,
                    dir: ray_dir,
                },
                0,
            );
            let scaled = color.map(|c| {
                (c * u8::MAX as Float)
                    .max(u8::MIN as Float)
                    .min(u8::MAX as Float) as u8
            });
            buffer.put_pixel(x, y, Rgb::from([scaled.x, scaled.y, scaled.z]));
        }
    }
    info!("Saving image to {}", config::IMAGE_FILEPATH);
    buffer.save(config::IMAGE_FILEPATH).unwrap();
}
