
#![allow(dead_code)]
mod vec3;
mod ray;
mod image;


use image::Image;

use crate::vec3::Vec3;



fn main() {


    // image 
    let mut image = Image::new(800, (16, 9));

    let image_width = image.width;
    let image_height = image.height;

    dbg!(&image_height, &image_width);
    

    // render
    for j in 0..image_width {

        let lines_remaing = image_width - j;
        eprint!("\rScanlines remaining: {}", &lines_remaing);

        for i in 0..image_width {

            let pixel_color = Vec3::new(
                (i as f64) / ((image_width - 1) as f64),
                (j as f64) / ((image_height - 1) as f64), 
                0.0
            );

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
