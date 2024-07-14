
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


            let r: f32 = (i as f32) / ((image_width - 1) as f32);
            let g: f32 = (j as f32) / ((image_height - 1) as f32);
            let b: f32 = 0.0;


            let ir: u32 = (255.0 * r) as u32;
            let ig: u32 = (255.0 * g) as u32;
            let ib: u32 = (255.0 * b) as u32;

            println!("{ir} {ig} {ib}");
        }
    }

    eprintln!("\rDone rendering.                       ");

    

}
