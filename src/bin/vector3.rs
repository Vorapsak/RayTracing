use std::ops::{Add, Sub, Neg, Mul};

#[derive(Copy, Clone)]
pub struct vector3 {
	pub x : f64,
	pub y : f64,
	pub z : f64
}

//pub fn dot(a: vector3, b: vector3) -> f64 {
//	
//}

impl vector3 {
	pub fn new(x: f64, y: f64, z: f64) -> vector3 {
		vector3 {x: x, y: y, z: z}
	}
	
/*	pub fn zero() -> vector3 {
		vector3::new(0.0, 0.0, 0.0);
	}
	
	pub fn length(self) -> f64 {
		return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
	}
	
	pub fn squared_length(self) -> f64 {
		return self.x * self.x + self.y * self.y + self.z * self.z;
	}
*/		
}

/*
impl Add for vector3 {
	type Output = vector3;
	
	fn add (self, other: vector3) -> vector3 {
		vector3 {self.x + other.x, self.y + other.y, self.z + other.z}
	}
}

impl Sub for vector3 {
	type Output = vector3;
	
	fn sub (self, other: vector3) -> vector3 {
		vector3 {self.x - other.x, self.y - other.y, self.z - other.z}
	}
}

impl Mul<f64> for vector3 {
	type Output = vector3;
	
	fn mul (self, factor: f64) -> vector3 {
		vector3 {self.x * factor, self.y * factor, self.z * factor}
	}
}

impl Neg for vector3 {
	type Output = vector3;
	
	fn neg (self) -> vector3 {
		vector3 {-self.x, -self.y, -self.z}
	}
}
*/
