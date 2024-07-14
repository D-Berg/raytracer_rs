
#![allow(dead_code)]
mod vec3;
mod color;
mod ray;

use color::Color;

use crate::vec3::Vec3;
use crate::color::print_color;



fn main() {


    // image 

    let aspect_ratio: f64 = 16.0 / 9.0;

    let image_width: u32 = 800;

    let image_height = (image_width as f64 / aspect_ratio) as u32;
    dbg!(&image_width, &image_height);


    // render

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {

        let lines_remaing = image_width - j;
        eprint!("\rScanlines remaining: {}", &lines_remaing);

        for i in 0..image_width {

            let pixel_color = Color::new(
                (i as f64) / ((image_width - 1) as f64),
                (j as f64) / ((image_height - 1) as f64), 
                0.0
            );


            print_color(&pixel_color);

        }
    }

    eprintln!("\rDone rendering.                       ");

    

}


#[cfg(test)]
mod test {
}
