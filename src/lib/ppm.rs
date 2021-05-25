mod ppm;
use std::fs::File;

use File::create;

pub fn write(f: String, data: Vector3){
	let mut file = create(f).unwrap();
}