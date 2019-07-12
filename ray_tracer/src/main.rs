mod core;
mod geometry;
use rand::Rng;
use std::f32;
use std::rc::Rc;

fn color(r: &mut geometry::Ray, world: &core::Hitable, depth: i32) -> geometry::Vec3 {
    let mut rec = core::HitRecord::new();
    if world.hit(r, 0.001, f32::INFINITY, &mut rec) == true {
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

fn rand() -> f32 {
    rand::thread_rng().gen()
}

fn random_scene(world: &mut core::HitList) {
    world.list.push(Box::new(core::SphereObject {
        center: geometry::Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat: Some(Rc::new(core::materials::Lambertian::new(
            geometry::Vec3::new(0.5, 0.5, 0.5),
        ))),
    }));
    world.list.push(Box::new(core::SphereObject {
        center: geometry::Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        mat: Some(Rc::new(core::materials::Dielectric::new(1.5))),
    }));
    world.list.push(Box::new(core::SphereObject {
        center: geometry::Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat: Some(Rc::new(core::materials::Lambertian::new(
            geometry::Vec3::new(0.5, 0.5, 0.5),
        ))),
    }));
    world.list.push(Box::new(core::SphereObject {
        center: geometry::Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        mat: Some(Rc::new(core::materials::Metal::new(
            geometry::Vec3::new(0.7, 0.6, 0.5),
            0.0,
        ))),
    }));
    for a in -11..12 {
        for b in -11..12 {
            let choose_mat: f32 = rand::thread_rng().gen();
            let center = geometry::Vec3::new(a as f32 + 0.9 * rand(), 0.2, b as f32 + 0.9 * rand());
            if (center - geometry::Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    world.list.push(Box::new(core::SphereObject {
                        center,
                        radius: 0.2,
                        mat: Some(Rc::new(core::materials::Lambertian::new(
                            geometry::Vec3::new(rand() * rand(), rand() * rand(), rand() * rand()),
                        ))),
                    }));
                } else if choose_mat < 0.95 {
                    world.list.push(Box::new(core::SphereObject {
                        center,
                        radius: 0.2,
                        mat: Some(Rc::new(core::materials::Metal::new(
                            geometry::Vec3::new(
                                0.5 * (1.0 + rand()),
                                0.5 * (1.0 + rand()),
                                0.5 * (1.0 + rand()),
                            ),
                            0.5 * rand(),
                        ))),
                    }));
                } else {
                    world.list.push(Box::new(core::SphereObject {
                        center,
                        radius: 0.2,
                        mat: Some(Rc::new(core::materials::Dielectric::new(1.5))),
                    }));
                }
            }
        }
    }
}

fn main() {
    let nx = 800;
    let ny = 400;
    let ns = 100;
    let mut world = core::HitList::new();
    random_scene(&mut world);
    let lookfrom = geometry::Vec3::new(12.0, 1.2, 4.0);
    let lookat = geometry::Vec3::new(2.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture: f32 = 0.03;
    let cam = core::Camera::new(
        lookfrom,
        lookat,
        geometry::Vec3::new(0.0, 1.0, 0.0),
        30.0,
        nx as f32 / ny as f32,
        aperture,
        dist_to_focus,
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
