
#![allow(dead_code)]
mod vec3;
mod ray;
mod image;


use std::f64;

use image::Image;
use ray::Ray;
use vec3::Point3;

use crate::vec3::Vec3;



fn main() {


    // image 
    let mut image = Image::new(800, (16, 9));

    let image_width = image.width;
    let image_height = image.height;

    dbg!(&image_height, &image_width);
    

    // camera--------------------
    
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * image.ratio;
    let camera_center: Point3 = Point3::zeros();

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_width, 0.0);

    let pixel_delta_u = &viewport_u / (image_width as f64);
    let pixel_delta_v = &viewport_v / (image_height as f64);
    

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = &camera_center 
        - &Vec3::new(0.0 , 0.0, focal_length)
        - &viewport_u / 2.0
        - &viewport_v / 2.0;

    let pixel00_loc = &viewport_upper_left + 0.5 * (&pixel_delta_u + &pixel_delta_v);


    // render---------------------
    for j in 0..image_height {

        let lines_remaing = image_width - j;
        eprint!("\rScanlines remaining: {}", &lines_remaing);

        for i in 0..image_width {

            let pixel_center = &pixel00_loc 
                + (i as f64 * &pixel_delta_u) 
                + (j as f64 * &pixel_delta_v);

            let ray_direction = &pixel_center - &camera_center;

            let r = Ray::new(camera_center.clone(), ray_direction);

            let pixel_color = r.color();

            image.pixel_color.push(pixel_color);

        }
    }


    eprintln!("\rDone rendering.                       ");

    
    // Print image
    println!("{}", &image);

}


#[cfg(test)]
mod test {
}
