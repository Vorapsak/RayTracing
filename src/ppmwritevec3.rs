//use vector3::Vector3;
use test;

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny); // file header I assume

	for j in (0..ny).rev() {
		for i in 0..nx{
//			let color: vector3 = vector3::new( 
//			(i as f64) / (nx as f64) * 255.99, 
//			(j as f64) / (ny as f64) * 255.99, 
//			0.2 * 255.99 );
//			let color = Vector3::new(0.0, 0.0, 1.0);
//			println!("{} {} {}", color.x as i32, color.y as i32, color.z as i32);
			println!("{}", hi());
		}
	}
}
