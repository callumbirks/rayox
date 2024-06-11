use std::{
    f32::consts::PI,
    fs::File,
    io::{BufWriter, Write},
};

mod sphere;
mod vec;

type Vec3f = vec::Vec3<f32>;

struct Sphere {
    center: Vec3f,
    radius: f32,
    sqr_radius: f32,
    surface_color: Vec3f,
    emission: Vec3f,
    transparency: f32,
    reflection: f32,
}

struct Ray {
    origin: Vec3f,
    direction: Vec3f,
}

impl Sphere {
    /// Find the intersection points of the given ray within the sphere.
    /// Intersection points are given as float, distance along the ray.
    fn intersect(&self, ray: &Ray) -> Option<(f32, f32)> {
        // Line from sphere center to ray origin
        let l: Vec3f = self.center - ray.origin;
        // Distance from sphere center to ray origin, in direction of ray
        let tca: f32 = l.dot_product(ray.direction);
        // If `tca` is negative, sphere center is behind ray origin
        if tca < 0_f32 {
            return None;
        }
        // Square distance from sphere center to ray, perpendicular to ray
        let d2 = l.dot_product(l) - tca * tca;
        // If distance > radius, the ray lies outside the sphere
        if d2 > self.sqr_radius {
            return None;
        }
        // Distance from `d` to intersection point
        let thc: f32 = (self.sqr_radius - d2).sqrt();
        Some((tca - thc, tca + thc))
    }
}

fn mix(a: f32, b: f32, mix: f32) -> f32 {
    b * mix + a * (1_f32 - mix)
}

const MAX_RAY_DEPTH: usize = 5;

fn trace(ray: Ray, spheres: &[Sphere], depth: usize) -> Vec3f {
    // Find the first sphere which the ray intersects
    let mut near: (f32, Option<&Sphere>) = (f32::INFINITY, None);
    for sphere in spheres {
        if let Some((mut t0, t1)) = sphere.intersect(&ray) {
            // If the first intersection point lies behind the ray origin, then the first
            // intersection is the same as the second.
            if t0 < 0_f32 {
                t0 = t1;
            }
            if t0 < near.0 {
                near = (t0, Some(sphere));
            }
        }
    }

    // No intersection - return background color
    let Some(near_sphere) = near.1 else {
        return Vec3f::new_uniform(2.0);
    };

    // Point of intersection
    let hit_point: Vec3f = ray.origin + ray.direction * near.0;
    let mut hit_normal: Vec3f = (hit_point - near_sphere.center).normalized();

    let bias: f32 = 1e-4;

    let is_inside = if ray.direction.dot_product(hit_normal) > 0.0 {
        hit_normal = -hit_normal;
        true
    } else {
        false
    };

    let surface_color = if depth < MAX_RAY_DEPTH
        && (near_sphere.transparency > 0.0 || near_sphere.reflection > 0.0)
    {
        let facing_ratio = -ray.direction.dot_product(hit_normal);
        let fresnel_effect = mix((1.0 - facing_ratio).powi(3), 1.0, 0.1);

        let reflect_dir = ray.direction - hit_normal * 2.0 * ray.direction.dot_product(hit_normal);
        let reflect_dir = reflect_dir.normalized();
        let reflect_origin = hit_point + hit_normal * bias;
        let reflection = trace(
            Ray {
                origin: reflect_origin,
                direction: reflect_dir,
            },
            spheres,
            depth + 1,
        );
        let refraction = if near_sphere.transparency > 0.0 {
            let ior: f32 = 1.1;
            let eta: f32 = if is_inside { ior } else { 1.0 / ior };
            let cosi = -hit_normal.dot_product(ray.direction);
            let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
            let refract_dir = ray.direction * eta + hit_normal * (eta * cosi - k.sqrt());
            let refract_dir = refract_dir.normalized();
            let refract_origin = hit_point - hit_normal * bias;
            trace(
                Ray {
                    origin: refract_origin,
                    direction: refract_dir,
                },
                spheres,
                depth + 1,
            )
        } else {
            Vec3f::new_uniform(0.0)
        };
        (reflection * fresnel_effect
            + refraction * (1.0 - fresnel_effect) * near_sphere.transparency)
            * near_sphere.surface_color
    } else {
        let mut surface_color = Vec3f::new_uniform(0.0);
        for (i, sphere) in spheres.iter().enumerate() {
            if sphere.emission.x > 0.0 {
                let mut transmission = Vec3f::new_uniform(1.0);
                let light_dir = (sphere.center - hit_point).normalized();
                let light_origin = hit_point + hit_normal * bias;
                let light_ray = Ray {
                    origin: light_origin,
                    direction: light_dir,
                };
                for (j, other_sphere) in spheres.iter().enumerate() {
                    if i == j {
                        continue;
                    };
                    if other_sphere.intersect(&light_ray).is_some() {
                        transmission = Vec3f::new_uniform(0.0);
                        break;
                    }
                }
                surface_color += near_sphere.surface_color
                    * transmission
                    * 0_f32.max(hit_normal.dot_product(light_dir))
                    * sphere.emission;
            }
        }
        surface_color
    };

    surface_color + near_sphere.emission
}

fn render(spheres: &[Sphere]) -> std::io::Result<()> {
    const WIDTH: usize = 640;
    const HEIGHT: usize = 480;
    let inv_width = 1.0 / WIDTH as f32;
    let inv_height = 1.0 / HEIGHT as f32;
    const FOV: f32 = 30.0;
    const ASPECT_RATIO: f32 = WIDTH as f32 / HEIGHT as f32;
    let angle = f32::tan(PI * 0.5 * FOV / 180.0);

    let mut image = [Vec3f::default(); WIDTH * HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..HEIGHT {
            let pixel = &mut image[x + y];
            let xx = (2.0 * ((x as f32 + 0.5) * inv_width) - 1.0) * angle * ASPECT_RATIO;
            let yy = (1.0 - 2.0 * ((y as f32 + 0.5) * inv_height)) * angle;
            let ray_dir = Vec3f {
                x: xx,
                y: yy,
                z: -1.0,
            }
            .normalized();
            let ray = Ray {
                origin: Vec3f::new_uniform(0.0),
                direction: ray_dir,
            };
            *pixel = trace(ray, spheres, 0);
        }
    }

    let file = File::open("raytraced.ppm")?;
    let mut buf_writer = BufWriter::new(file);

    Ok(())
}

fn main() {
    println!("Hello, world!");
}
