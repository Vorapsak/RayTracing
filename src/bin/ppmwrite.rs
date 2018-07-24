fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny); // file header I assume

	for j in (0..ny).rev() {
		for i in 0..nx{
			println!("{} {} {}", ((i as f64) / (nx as f64) * 255.99) as i32, ((j as f64) / (ny as f64) * 255.99) as i32, (0.2 * 255.99) as i32);
		}
	}
}
