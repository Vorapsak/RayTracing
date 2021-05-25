use utils::vector3::*;
use std::fmt;

#[derive(Copy, Clone)]
pub struct Ray {
	pub origin: Vector3,
	pub direction: Vector3
}

impl Ray {
	pub fn new(origin: Vector3, direction: Vector3) -> Ray {
		Ray { origin: origin, direction: direction }
	}
	
	pub fn point_at(self, t: f64) -> Vector3 {
		return self.origin + self.direction * t
	}
}

impl fmt::Display for Ray {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}), ({})", self.origin, self.direction)
	}
}

pub fn main() {
	let r = Ray::new(Vector3::zero(), Vector3::new(1.0, 0.0, 0.0));
	let uv = as_unit_vector(Vector3::new(1.0, 1.0, 1.0));
	println!("{}", r);
	println!("{}", uv.x);
}
