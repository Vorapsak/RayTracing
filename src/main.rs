use std::fs::File;
use std::io::prelude::*;

fn main() {
    let nx = 200;
    let ny = 100;
	let mut file = File::create("img.ppm").unwrap();
    file.write(format!("P3\n{} {}\n255\n", nx, ny).as_bytes()).ok();

	for j in (0..ny).rev() {
		for i in 0..nx{
			let ir = ((i as f64) / (nx as f64) * 255.99) as i32;
			let ig = ((j as f64) / (ny as f64) * 255.99) as i32;
			let ib = (0.2 * 255.99) as i32;
			file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes()).ok();
		}
	}

}
