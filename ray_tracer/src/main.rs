mod common;
mod core;
mod geometry;
use std::f32;
use std::rc::Rc;

fn color(r: &mut geometry::Ray, world: &core::Hitable, depth: i32) -> geometry::Vec3 {
    let mut rec = core::HitRecord::new();
    if world.hit(r, 0.001, f32::INFINITY, &mut rec) == true {
        let mut scattered = geometry::Ray::new(
            geometry::Vec3::new(0f32, 0f32, 0f32),
            geometry::Vec3::new(0f32, 0f32, 0f32),
            r.time,
        );
        let mut attenuation = geometry::Vec3::new(0f32, 0f32, 0f32);
        match rec.mat {
            None => return geometry::Vec3::new(0f32, 0f32, 0f32),
            Some(ref mat) => {
                let emitted = mat.emitted(rec.u, rec.v, &rec.p);
                if depth < 50 && mat.scatter(r, &rec, &mut attenuation, &mut scattered) == true {
                    return emitted + attenuation * color(&mut scattered, world, depth + 1);
                } else {
                    return emitted;
                }
            }
        }
    }
    geometry::Vec3::new(0.0, 0.0, 0.0)
    // let unit_direction = geometry::normalize(r.d);
    // let t = 0.5 * (unit_direction.y + 1.0);
    // (1.0 - t) * geometry::Vec3::new(1f32, 1f32, 1f32) + t * geometry::Vec3::new(0.5, 0.7, 1.0)
}

fn two_spheres(world: &mut core::HitList, camera: &mut core::Camera, ratio: f32) {
    world.list.push(Rc::new(core::SphereObject {
        center: geometry::Vec3::new(0.0, -10.0, 0.0),
        radius: 10.0,
        mat: Some(Rc::new(core::materials::Lambertian::new(Some(Rc::new(
            core::texture::CheckerTexture {
                odd: Some(Rc::new(core::texture::ConstantTexture {
                    color: geometry::Vec3::new(0.2, 0.3, 0.1),
                })),
                even: Some(Rc::new(core::texture::ConstantTexture {
                    color: geometry::Vec3::new(0.9, 0.9, 0.9),
                })),
            },
        ))))),
    }));
    world.list.push(Rc::new(core::SphereObject {
        center: geometry::Vec3::new(0.0, 10.0, 0.0),
        radius: 10.0,
        mat: Some(Rc::new(core::materials::Lambertian::new(Some(Rc::new(
            core::texture::CheckerTexture {
                odd: Some(Rc::new(core::texture::ConstantTexture {
                    color: geometry::Vec3::new(0.2, 0.3, 0.1),
                })),
                even: Some(Rc::new(core::texture::ConstantTexture {
                    color: geometry::Vec3::new(0.9, 0.9, 0.9),
                })),
            },
        ))))),
    }));
    let lookfrom = geometry::Vec3::new(13.0, 2.0, 3.0);
    let lookat = geometry::Vec3::new(1.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture: f32 = 0.0;
    *camera = core::Camera::build(
        lookfrom,
        lookat,
        geometry::Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );
}

fn two_perlin_spheres(world: &mut core::HitList, camera: &mut core::Camera, ratio: f32) {
    let noise_tex = Rc::new(core::NoiseTexture {
        noise: Some(Rc::new(common::Perlin::new())),
        scale: 0.5,
    });
    world.list.push(Rc::new(core::SphereObject {
        center: geometry::Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat: Some(Rc::new(core::materials::Lambertian::new(Some(
            noise_tex.clone(),
        )))),
    }));
    world.list.push(Rc::new(core::SphereObject {
        center: geometry::Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        mat: Some(Rc::new(core::materials::Lambertian::new(Some(
            noise_tex.clone(),
        )))),
    }));
    let lookfrom = geometry::Vec3::new(13.0, 2.0, 3.0);
    let lookat = geometry::Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture: f32 = 0.0;
    *camera = core::Camera::build(
        lookfrom,
        lookat,
        geometry::Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );
}

fn simple_light(world: &mut core::HitList, camera: &mut core::Camera, ratio: f32) {
    let noise_tex = Rc::new(core::NoiseTexture {
        noise: Some(Rc::new(common::Perlin::new())),
        scale: 4.0,
    });
    let diffuse_light_mat = Rc::new(core::DiffuseLight {
        emit: Some(Rc::new(core::ConstantTexture {
            color: geometry::Vec3::new(4.0, 4.0, 4.0),
        })),
    });
    world.list.push(Rc::new(core::SphereObject {
        center: geometry::Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat: Some(Rc::new(core::materials::Lambertian::new(Some(
            noise_tex.clone(),
        )))),
    }));
    world.list.push(Rc::new(core::SphereObject {
        center: geometry::Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        mat: Some(Rc::new(core::materials::Lambertian::new(Some(
            noise_tex.clone(),
        )))),
    }));
    world.list.push(Rc::new(core::XYRect {
        x0: 3.0,
        x1: 5.0,
        y0: 1.0,
        y1: 3.0,
        k: -2.0,
        mat: Some(diffuse_light_mat.clone()),
    }));
    let lookfrom = geometry::Vec3::new(13.0, 2.0, 3.0);
    let lookat = geometry::Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture: f32 = 0.0;
    *camera = core::Camera::build(
        lookfrom,
        lookat,
        geometry::Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );
}

fn cornell_box(world: &mut core::HitList, camera: &mut core::Camera, ratio: f32) {
    let red = Rc::new(core::Lambertian::new(Some(Rc::new(
        core::ConstantTexture {
            color: geometry::Vec3::new(0.65, 0.05, 0.05),
        },
    ))));
    let white = Rc::new(core::Lambertian::new(Some(Rc::new(
        core::ConstantTexture {
            color: geometry::Vec3::new(0.73, 0.73, 0.73),
        },
    ))));
    let green = Rc::new(core::Lambertian::new(Some(Rc::new(
        core::ConstantTexture {
            color: geometry::Vec3::new(0.12, 0.45, 0.15),
        },
    ))));
    let light = Rc::new(core::DiffuseLight {
        emit: Some(Rc::new(core::ConstantTexture {
            color: geometry::Vec3::new(15.0, 15.0, 15.0),
        })),
    });
    world.list.push(Rc::new(core::FlipNormals {
        ptr: Some(Rc::new(core::YZRect {
            y0: 0.0,
            y1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 555.0,
            mat: Some(green.clone()),
        })),
    }));
    world.list.push(Rc::new(core::YZRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        mat: Some(red.clone()),
    }));
    world.list.push(Rc::new(core::XZRect {
        x0: 213.0,
        x1: 343.0,
        z0: 277.0,
        z1: 332.0,
        k: 554.0,
        mat: Some(light.clone()),
    }));
    world.list.push(Rc::new(core::FlipNormals {
        ptr: Some(Rc::new(core::XZRect {
            x0: 0.0,
            x1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 555.0,
            mat: Some(white.clone()),
        })),
    }));
    world.list.push(Rc::new(core::XZRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        mat: Some(white.clone()),
    }));
    world.list.push(Rc::new(core::FlipNormals {
        ptr: Some(Rc::new(core::XYRect {
            x0: 0.0,
            x1: 555.0,
            y0: 0.0,
            y1: 555.0,
            k: 555.0,
            mat: Some(white.clone()),
        })),
    }));
    world.list.push(Rc::new(core::BoxObject::create(
        geometry::Vec3::new(130.0,0.0,65.0),
        geometry::Vec3::new(295.0,165.0,230.0),
        white.clone())
    ));
    world.list.push(Rc::new(core::BoxObject::create(
        geometry::Vec3::new(265.0,0.0,295.0),
        geometry::Vec3::new(430.0,330.0,460.0),
        white.clone())
    ));
    let lookfrom = geometry::Vec3::new(278.0, 278.0, -800.0);
    let lookat = geometry::Vec3::new(278.0, 278.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture: f32 = 0.0;
    *camera = core::Camera::build(
        lookfrom,
        lookat,
        geometry::Vec3::new(0.0, 1.0, 0.0),
        40.0,
        ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );
}

fn random_scene(world: &mut core::HitList, camera: &mut core::Camera, ratio: f32) {
    world.list.push(Rc::new(core::SphereObject {
        center: geometry::Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat: Some(Rc::new(core::materials::Lambertian::new(Some(Rc::new(
            core::texture::CheckerTexture {
                odd: Some(Rc::new(core::texture::ConstantTexture {
                    color: geometry::Vec3::new(0.2, 0.3, 0.1),
                })),
                even: Some(Rc::new(core::texture::ConstantTexture {
                    color: geometry::Vec3::new(0.9, 0.9, 0.9),
                })),
            },
        ))))),
    }));
    world.list.push(Rc::new(core::SphereObject {
        center: geometry::Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        mat: Some(Rc::new(core::materials::Dielectric::new(1.5))),
    }));
    world.list.push(Rc::new(core::SphereObject {
        center: geometry::Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat: Some(Rc::new(core::materials::Lambertian::new(Some(Rc::new(
            core::texture::ConstantTexture {
                color: geometry::Vec3::new(0.5, 0.5, 0.5),
            },
        ))))),
    }));
    world.list.push(Rc::new(core::SphereObject {
        center: geometry::Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        mat: Some(Rc::new(core::materials::Metal::new(
            geometry::Vec3::new(0.7, 0.6, 0.5),
            0.0,
        ))),
    }));
    for a in -11..12 {
        for b in -11..12 {
            let choose_mat: f32 = common::rand();
            let center = geometry::Vec3::new(
                a as f32 + 0.9 * common::rand(),
                0.2,
                b as f32 + 0.9 * common::rand(),
            );
            if (center - geometry::Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    world.list.push(Rc::new(core::MovingSphereObject {
                        center0: center,
                        center1: center + geometry::Vec3::new(0.0, 0.5 * common::rand(), 0.0),
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        mat: Some(Rc::new(core::materials::Lambertian::new(Some(Rc::new(
                            core::texture::ConstantTexture {
                                color: geometry::Vec3::new(
                                    common::rand() * common::rand(),
                                    common::rand() * common::rand(),
                                    common::rand() * common::rand(),
                                ),
                            },
                        ))))),
                    }));
                } else if choose_mat < 0.95 {
                    world.list.push(Rc::new(core::SphereObject {
                        center,
                        radius: 0.2,
                        mat: Some(Rc::new(core::materials::Metal::new(
                            geometry::Vec3::new(
                                0.5 * (1.0 + common::rand()),
                                0.5 * (1.0 + common::rand()),
                                0.5 * (1.0 + common::rand()),
                            ),
                            0.5 * common::rand(),
                        ))),
                    }));
                } else {
                    world.list.push(Rc::new(core::SphereObject {
                        center,
                        radius: 0.2,
                        mat: Some(Rc::new(core::materials::Dielectric::new(1.5))),
                    }));
                }
            }
        }
    }
    let lookfrom = geometry::Vec3::new(12.0, 1.2, 4.0);
    let lookat = geometry::Vec3::new(2.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture: f32 = 0.03;
    *camera = core::Camera::build(
        lookfrom,
        lookat,
        geometry::Vec3::new(0.0, 1.0, 0.0),
        30.0,
        ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );
}

fn main() {
    let nx = 200;
    let ny = 200;
    let ns = 100;
    let mut world = core::HitList::new();
    let mut cam = core::Camera::new();
    // random_scene(&mut world, &mut cam, nx as f32 / ny as f32);
    // two_spheres(&mut world, &mut cam, nx as f32 / ny as f32);
    // two_perlin_spheres(&mut world, &mut cam, nx as f32 / ny as f32);
    // simple_light(&mut world, &mut cam, nx as f32 / ny as f32);
    cornell_box(&mut world, &mut cam, nx as f32 / ny as f32);
    let mut bvh = core::BVHNode::new(&mut world.list[..], 0f32, 1f32);
    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = geometry::Vec3::new(0f32, 0f32, 0f32);
            for _ in 0..ns {
                let u: f32 = i as f32 / nx as f32;
                let v: f32 = j as f32 / ny as f32;
                let mut r = cam.ray(u, v);
                col += color(&mut r, &world, 0);
                // col += color(&mut r, &bvh, 0);
            }
            col /= ns as f32;
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
