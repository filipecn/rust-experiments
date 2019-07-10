mod core;
mod geometry;
use std::f32;
use std::rc::Rc;

fn color(r: &mut geometry::Ray, world: &core::Hitable, depth: i32) -> geometry::Vec3 {
    let mut rec = core::HitRecord::new();
    if world.hit(r, 0.001f32, f32::INFINITY, &mut rec) == true {
        let mut scattered = geometry::Ray::new(
            geometry::Vec3::new(0f32, 0f32, 0f32),
            geometry::Vec3::new(0f32, 0f32, 0f32),
        );
        let mut attenuation = geometry::Vec3::new(0f32, 0f32, 0f32);
        match rec.mat {
            None => return geometry::Vec3::new(0f32, 0f32, 0f32),
            Some(ref mat) => {
                if depth < 50 && mat.scatter(r, &rec, &mut attenuation, &mut scattered) == true {
                    return attenuation * color(&mut scattered, world, depth + 1);
                } else {
                    return geometry::Vec3::new(0f32, 0f32, 0f32);
                }
            }
        }
    }
    let unit_direction = geometry::normalize(r.d);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * geometry::Vec3::new(1f32, 1f32, 1f32) + t * geometry::Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    let mut world = core::HitList::new();
    world.list = vec![
        Box::new(core::SphereObject {
            center: geometry::Vec3::new(0f32, 0f32, -1f32),
            radius: 0.5,
            mat: Some(Rc::new(core::materials::Lambertian::new(
                geometry::Vec3::new(0.8, 0.3, 0.3),
            ))),
        }),
        Box::new(core::SphereObject {
            center: geometry::Vec3::new(0f32, -100.5f32, -1f32),
            radius: 100.0,
            mat: Some(Rc::new(core::materials::Lambertian::new(
                geometry::Vec3::new(0.8, 0.8, 0.0),
            ))),
        }),
        Box::new(core::SphereObject {
            center: geometry::Vec3::new(1f32, 0f32, -1f32),
            radius: 0.5,
            mat: Some(Rc::new(core::materials::Metal::new(
                geometry::Vec3::new(0.8, 0.6, 0.2),
            ))),
        }),
       Box::new(core::SphereObject {
            center: geometry::Vec3::new(-1f32, 0f32, -1f32),
            radius: 0.5,
            mat: Some(Rc::new(core::materials::Metal::new(
                geometry::Vec3::new(0.8, 0.8, 0.8),
            ))),
        }),
    ];
    let cam = core::Camera::new(
        geometry::Vec3::new(-2f32, 2f32, 1f32),
        geometry::Vec3::new(0f32, 0f32, -1f32),
        geometry::Vec3::new(0f32, 1f32, 0f32),
        90f32,
        nx as f32 / ny as f32,
    );
    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = geometry::Vec3::new(0f32, 0f32, 0f32);
            for _ in 0..ns {
                let u: f32 = i as f32 / nx as f32;
                let v: f32 = j as f32 / ny as f32;
                let mut r = cam.ray(u, v);
                col += color(&mut r, &world, 0);
            }
            col /= ns as f32;
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
