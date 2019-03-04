#[macro_use]
extern crate clap;
extern crate rayon;

use clap::{App, Arg};
use failure::Error;
use graphics_codex::*;
use rayon::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use std::path::Path;

const USE_MULTI_THREADING: bool = true;

const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const PKG_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const PKG_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const MANIFEST_DIR: &'static str = env!("CARGO_MANIFEST_DIR");

fn main() -> Result<(), Error> {
    let matches = App::new(PKG_NAME)
        .version(PKG_VERSION)
        .author(PKG_AUTHORS)
        .about(PKG_DESCRIPTION)
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .value_name("WIDTH")
                .help("Rendered / displayed width")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .value_name("HEIGHT")
                .help("Rendered / displayed height")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("vfov")
                .short("y")
                .long("vfov")
                .value_name("VFOV")
                .help("Vertical field of view")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("znear")
                .short("z")
                .long("znear")
                .value_name("ZNEAR")
                .help("Distance from the camera to the image/near-clipping plane")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("obj")
                .long("obj")
                .value_name("OBJ")
                .help("Wavefront OBJ file to load")
                .takes_value(true),
        )
        .get_matches();

    let width: usize = value_t!(matches.value_of("width"), usize).unwrap_or(800);
    let height: usize = value_t!(matches.value_of("height"), usize).unwrap_or(600);
    let v_fov: f32 = radians(value_t!(matches.value_of("vfov"), f32).unwrap_or(45.0f32));
    let z_near: f32 = value_t!(matches.value_of("znear"), f32).unwrap_or(-0.5f32);
    let obj: String = value_t!(matches.value_of("obj"), String)
        .unwrap_or(String::from("assets/models/teapot.obj"));

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(PKG_NAME, width as u32, height as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(Some(PixelFormatEnum::ABGR8888), width as u32, height as u32)
        .unwrap();
    let rect = Some(Rect::new(0, 0, width as u32, height as u32));

    canvas.set_draw_color(Color::RGB(81, 81, 81));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let camera = pinhole_camera(z_near, v_fov);
    let mut image = image(width, height);

    let mut scene = scene();
    // scene
    //     .spheres
    //     .push(sphere(point3(-2.0f32, 1.5f32, -5.0f32), 0.5f32));
    // scene.planes.push(plane(
    //     point3(0.0f32, 0.0f32, -5.0f32),
    //     vector3(1.0f32, 0.0f32, 0.2f32).normalize(),
    // ));
    // scene.triangles.push(triangle(
    //     point3(-1f32, -1f32, -5f32),
    //     point3(1f32, -1f32, -5f32),
    //     point3(0f32, 1f32, -5f32),
    // ));
    scene.add_obj(&Path::new(MANIFEST_DIR).join(obj));
    let mesh_offset = vector3(0f32, -35f32, -150f32);
    for msh in &mut scene.meshes {
        // println!("Before: {}", msh);
        msh.translation = msh.translation + mesh_offset;
        // println!("After: {}", msh);
    }

    let mut frame_count = 0;
    'mainloop: loop {
        frame_count = frame_count + 1;
        println!("Frame #{}", frame_count);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                _ => {}
            }
        }
        render(&scene, &camera, &mut image);
        show(&mut image, rect, &mut texture, &mut canvas)?;
    }

    Ok(())
}

fn render(scene: &Scene, camera: &PinholeCamera, image: &mut Image) {
    let w = image.width;
    let h = image.height;
    if USE_MULTI_THREADING {
        image
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, radiance)| {
                let x = index % w;
                let y = index / w;
                let ray = camera.get_primary_ray(x as f32, y as f32, w as i32, h as i32);
                *radiance = l_i(scene, ray);
            });
    } else {
        for y in 0..h {
            for x in 0..w {
                // let radiance = match (x, y) {
                //     (_, _) if 0 < y && y < 20 && 0 < x && x < 20 => {
                //         Radiance::new(1.0f32, 0.0f32, 0.0f32)
                //     }
                //     (_, _) if 0 < y && y < 20 && 100 < x && x < 120 => {
                //         Radiance::new(0.0f32, 1.0f32, 0.0f32)
                //     }
                //     (_, _) if 100 < y && y < 120 && 0 < x && x < 20 => {
                //         Radiance::new(0.0f32, 0.0f32, 1.0f32)
                //     }
                //     (_, _) if 100 < y && y < 120 && 100 < x && x < 120 => {
                //         Radiance::new(1.0f32, 1.0f32, 1.0f32)
                //     }
                //     _ => Radiance::new(0.0f32, 0.0f32, 0.0f32),
                // };
                // image.set(x, y, radiance);
                let ray = camera.get_primary_ray(x as f32, y as f32, w as i32, h as i32);
                image.set(x, y, l_i(scene, ray));
            }
        }
    }
}

fn l_i(scene: &Scene, ray: Ray) -> Radiance {
    if let Some(s) = find_first_intersection(scene, ray) {
        return s.radiance;
    } else {
        return Radiance::new(0.0f32, 0.0f32, 0.0f32);
    }
}

fn find_first_intersection(scene: &Scene, ray: Ray) -> Option<Surfel> {
    let mut min_t = std::f32::MAX;
    let mut min_surfel: Option<Surfel> = None;
    for sphere in &scene.spheres {
        if let Some(s) = ray_sphere_intersect(&ray, sphere) {
            if s.t < min_t {
                min_t = s.t;
                min_surfel = Some(s);
            }
        }
    }
    for plane in &scene.planes {
        if let Some(s) = ray_plane_intersect(&ray, plane) {
            if s.t < min_t {
                min_t = s.t;
                min_surfel = Some(s);
            }
        }
    }
    for triangle in &scene.triangles {
        if let Some(s) = ray_triangle_intersect(&ray, triangle) {
            if s.t < min_t {
                min_t = s.t;
                min_surfel = Some(s);
            }
        }
    }
    for mesh in &scene.meshes {
        for triangle in mesh {
            if let Some(s) = ray_triangle_intersect(&ray, &triangle) {
                if s.t < min_t {
                    min_t = s.t;
                    min_surfel = Some(s);
                }
            }
        }
    }
    min_surfel
}

fn ray_sphere_intersect(ray: &Ray, sphere: &Sphere) -> Option<Surfel> {
    let v = ray.origin - sphere.center; // 3 sub
    if v.magnitude() < sphere.radius {
        // 3 mul, 2 add, 1 sqrt, 1 cmp
        return None;
    }
    let a = ray.direction.dot(&ray.direction); // 3 mul, 2 add
    let b = 2.0f32 * ray.direction.dot(&v); //4 mul, 2 add
    let c = v.dot(&v) - sphere.radius * sphere.radius; // 4 mul 2 add 1 sub
    let discriminant = b * b - 4.0f32 * a * c; // 3 mul, 1 sub
    if discriminant > 0.0f32 {
        // 1 cmp
        let discriminant_sqrt = discriminant.sqrt(); // 1 sqrt
        let t0 = (-b + discriminant_sqrt) * 0.5f32; // 1 add, 1 mul
        let t1 = (-b - discriminant_sqrt) * 0.5f32; // 1 add, 1 mul
        let t = t0.min(t1); // 1 cmp
        let x = ray.at_t(t); // 3 mul, 3 add
        let p = (x - sphere.center).normalize();
        let up = vector3(0f32, 1f32, 0f32);
        let theta = up.dot(&p).acos(); // angle from the y axis
        let phi = vector3(p.x, 0f32, p.z)
            .normalize()
            .cross(&vector3(1f32, 0f32, 0f32))
            .magnitude()
            .asin();
        return Some(surfel(x, p, t, Radiance::new(theta, phi, 1f32))); // 3 sub, 3 mul, 2 add, 1 sqrt, 1 div
                                                                       // 26 mul, 23 add, 3 cmp, 3 sqrt
    }
    None
}

fn ray_plane_intersect(ray: &Ray, plane: &Plane) -> Option<Surfel> {
    let denominator = ray.direction.dot(&plane.normal); // 3 mul, 2 add
    if denominator > -EPSILON {
        // 1 cmp
        // den 0 and num 0 then in the plane
        // den 0 num non-zero then parallel
        // den > 0 then ray arriving from behind
        // println!(
        //     "w {:?} n {:?} w.n {}",
        //     ray.direction, plane.normal, denominator
        // );
        return None;
    }
    let numerator = (plane.p - ray.origin).dot(&plane.normal); // 3 sub, 3 mul, 2 add
    let t = numerator / denominator; // 1 div
    if t < 0.0f32 {
        // 1 cmp
        // in the past
        // println!("num {} den {} t = num/den {}", numerator, denominator, t);
        // println!("p {:?} c {:?} n {:?} (p - c).n {}", ray.origin, plane.p, plane.normal, numerator);
        // println!(
        //     "w {:?} n {:?} w.n {}",
        //     ray.direction, plane.normal, denominator
        // );
        return None;
    }
    let x = ray.at_t(t); // 3 mul, 3 sub
    Some(surfel(x, plane.normal, t, Radiance::new(1f32, 1f32, 1f32)))
    // 10 mul, 10 sub, 2 cmp
}

fn ray_triangle_intersect(ray: &Ray, triangle: &Triangle) -> Option<Surfel> {
    let v0 = triangle.vertices[0];
    let v1 = triangle.vertices[1];
    let v2 = triangle.vertices[2];

    let e1 = v1 - v0; // 3 sub
    let e2 = v2 - v0; // 3 sub
    let n = e1.cross(&e2).normalize(); // 6 mul, 3 sub, 3 mul, 2 add, 1 sqrt

    let q = ray.direction.cross(&e2); // 6 mul, 3 sub
    let a = e1.dot(&q); // 3 mul, 2 add

    if n.dot(&ray.direction) >= 0f32 || a.abs() < EPSILON {
        // 3 mul, 2 add, 1 cmp, 1 abs, 1 cmp
        // if ray.origin.x == 0f32 && ray.origin.y == 0f32 {
        //     println!("n {:?} w {:?}", n, ray.direction);
        //     println!("n.w {} abs(a) {}", n.dot(&ray.direction), a.abs());
        // }
        return None;
    }

    let s = (ray.origin - v0) / a; // 3 sub, 3 mul
    let r = s.cross(&e1); // 6 mul, 3 sub

    let b0 = s.dot(&q); // 3 mul, 2 add
    let b1 = r.dot(&ray.direction); // 3 mul, 2 add
    let b = vector3(b0, b1, 1f32 - (b0 + b1)); // 2 add
    if b.x < 0f32 || b.y < 0f32 || b.z < 0f32 {
        // 3 cmp
        // if ray.origin.x == 0f32 && ray.origin.y == 0f32 {
        //     println!("b {},{},{}", b.x, b.y, b.z);
        // }
        return None;
    }

    let t = e2.dot(&r); // 3 mul, 2 add
    if t < 0f32 {
        // 1 cmp
        // if ray.origin.x == 0f32 && ray.origin.y == 0f32 {
        //     println!("t {}", t);
        // }
        return None;
    }
    let (x, radiance) = interpolate(&triangle.vertices, &triangle.uvs, &b); // 9 sub, 9 mul, 9 add
    Some(surfel(x, n, t, radiance))
    // 50 add, 48 mul, 6 cmp, 1 sqrt, 1 abs
}

fn interpolate(
    v: &[Point3<f32>; 3],
    uv: &[Point2<f32>; 3],
    b: &Vector3<f32>,
) -> (Point3<f32>, Radiance) {
    let zero2 = point2(0f32, 0f32);
    let zero3 = point3(0f32, 0f32, 0f32);
    let mut x = zero3;
    let mut texcoord = zero2;

    for i in 0..3 {
        x = x + (v[i] - zero3) * b.data[i]; // 3 sub, 3 mul, 3 add
        texcoord = texcoord + (uv[i] - zero2) * b.data[i];
    }

    (x, Radiance::new(texcoord.x, texcoord.y, 1f32))
}

fn show(
    image: &mut Image,
    rect: Option<Rect>,
    texture: &mut Texture,
    canvas: &mut Canvas<Window>,
) -> Result<(), Error> {
    let stride = image.width * 4;
    texture.update(rect, image.as_argb8888(), stride)?;
    canvas.copy(&texture, rect, rect).unwrap();
    canvas.present();
    Ok(())
}
