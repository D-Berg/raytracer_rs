
#![allow(dead_code)]
mod vec3;
mod color;

use color::Color;

use crate::vec3::Vec3;
use crate::color::print_color;

type Point3 = Vec3;


fn main() {


    // image 

    let image_width: u32 = 256;
    let image_height: u32 = 256;


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
