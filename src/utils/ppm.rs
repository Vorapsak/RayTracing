use std::{fs::File, io::Write};
use utils::vector3::Vector3;

pub fn write(f: String, _data: &[u8]){
	let mut file = File::create(f).unwrap();
	file.write(_data);
}