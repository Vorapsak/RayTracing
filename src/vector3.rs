use std::ops::{Add, Sub, Neg, Mul};

//#[derive(Copy, Clone)]
pub struct Vector3 {
	pub x : f64,
	pub y : f64,
	pub z : f64
}

//pub fn dot(a: vector3, b: vector3) -> f64 {
//	
//}

impl Vector3 {
	pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
		Vector3 {x: x, y: y, z: z}
	}
	
	pub fn zero() -> Vector3 {
		Vector3::new(0.0, 0.0, 0.0)
	}
	
	pub fn length(self) -> f64 {
		return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}
	
	pub fn squared_length(self) -> f64 {
		return self.x * self.x + self.y * self.y + self.z * self.z
	}		
}

/*
impl Add for Vector3 {
	type Output = Vector3;
	
	fn add (self, other: Vector3) -> Vector3 {
		Vector3 {self.x + other.x, self.y + other.y, self.z + other.z}
	}
}

impl Sub for Vector3 {
	type Output = Vector3;
	
	fn sub (self, other: Vector3) -> Vector3 {
		Vector3 {self.x - other.x, self.y - other.y, self.z - other.z}
	}
}

impl Mul<f64> for Vector3 {
	type Output = Vector3;
	
	fn mul (self, factor: f64) -> Vector3 {
		Vector3 {self.x * factor, self.y * factor, self.z * factor}
	}
}

impl Neg for Vector3 {
	type Output = Vector3;
	
	fn neg (self) -> Vector3 {
		Vector3 {-self.x, -self.y, -self.z}
	}
}
*/