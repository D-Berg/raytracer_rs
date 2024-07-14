use std::fmt::Display;

use crate::vec3::Vec3;


pub struct Image {
    pub width: u32,
    pub height: u32,
    aspect_ratio: (u32, u32),
    pub pixel_color: Vec<Vec3>
}


impl Image {

    pub fn new(width: u32, aspect_ratio: (u32, u32)) -> Self {

        let ratio = aspect_ratio.0 as f64 / aspect_ratio.1 as f64;
        let height = (width as f64 / ratio) as u32;

        let n_pixes: usize = (height * width) as usize;

        Image {
            width,
            height,
            aspect_ratio,
            pixel_color: Vec::with_capacity(n_pixes)

        }

    }
    
}


impl Display for Image {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {


        write!(f,"P3\n").expect("Print Image Failed");
        write!(f, "{} {}\n", self.width, self.height).expect("Print Image Failed");
        write!(f, "255").expect("Print Image Failed");

        let mut r: f64;
        let mut g: f64;
        let mut b: f64;

        let mut rbyte: u32;
        let mut gbyte: u32; 
        let mut bbyte: u32; 


        for pixel in self.pixel_color.iter() {

            r = pixel.x();
            g = pixel.y();
            b = pixel.z();


            rbyte = (255.999 * r) as u32;
            gbyte = (255.999 * g) as u32;
            bbyte = (255.999 * b) as u32;

            // SLOW PART
            writeln!(f, "{} {} {}", rbyte, gbyte, bbyte).expect("Print Image Failed");

        }

        Ok(())
    }
    
}




