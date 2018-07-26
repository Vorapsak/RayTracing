mod vector3;
use vector3::Vector3;

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


pub fn main() {
	let r = Ray::new(Vector3::zero(), Vector3::new(1.0, 0.0, 0.0));
	println!("{}", r.point_at(5.0).x);
}
