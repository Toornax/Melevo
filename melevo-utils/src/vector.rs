use std::ops::{Add, Index, IndexMut};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vector<const N: usize, T=f64> {
	data: [T; N],
}


impl<const N: usize, T> Vector<N, T> {	
	pub fn new(data: [T; N]) -> Self {
		Self {
			data
		}
	}

	pub fn iter(&self) -> std::slice::Iter<'_, T> {
		self.data.iter()
	}

	pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
		self.data.iter_mut()
	}
}

// Access operators
impl<const N: usize, T> std::ops::Index<usize> for Vector<N, T> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		&self.data[index]
	}
}

impl<const N: usize, T> std::ops::IndexMut<usize> for Vector<N, T> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.data[index]
	}
}

impl<const N: usize, T> IntoIterator for Vector<N, T> {
	type Item = T;
	type IntoIter = std::array::IntoIter<T, N>;

	fn into_iter(self) -> Self::IntoIter {
		self.data.into_iter()
	}
}

// Maths operators
impl<const N: usize, T> std::ops::Add for Vector<N, T>
	where T: Copy + std::ops::Add<Output = T>,
{
	type Output = Self;

	fn add(mut self, rhs: Self) -> Self::Output {
		for (a, b) in self.data.iter_mut().zip(rhs.data.iter()) {
			*a = *a + *b;
		}

		self
	}
}

impl<const N: usize, T> std::ops::AddAssign for Vector<N, T>
	where T: Copy + std::ops::Add<Output = T>,
{
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}

impl<const N: usize, T> std::ops::Sub for Vector<N, T>
	where T: Copy + std::ops::Sub<Output = T>,
{
	type Output = Self;

	fn sub(mut self, rhs: Self) -> Self::Output {
		for (a, b) in self.data.iter_mut().zip(rhs.data.iter()) {
			*a = *a - *b;
		}

		self
	}
}

impl<const N: usize, T> std::ops::SubAssign for Vector<N, T>
	where T: Copy + std::ops::Sub<Output = T>,
{
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs;
	}
}

impl<const N: usize, T> std::ops::Mul for Vector<N, T>
	where T: Copy + std::ops::Mul<Output = T>,
{
	type Output = Self;

	fn mul(mut self, rhs: Self) -> Self::Output {
		for (a, b) in self.data.iter_mut().zip(rhs.data.iter()) {
			*a = *a * *b;
		}

		self
	}
}

impl<const N: usize, T> std::ops::MulAssign for Vector<N, T>
	where T: Copy + std::ops::Mul<Output = T>,
{
	fn mul_assign(&mut self, rhs: Self) {
		*self = *self * rhs;
	}
}

impl<const N: usize, T> std::ops::Mul<T> for Vector<N, T>
	where T: Copy + std::ops::Mul<Output = T>,
{
	type Output = Self;

	fn mul(mut self, rhs: T) -> Self::Output {
		for val in self.data.iter_mut() {
			*val = *val * rhs;
		}

		self
	}
}

impl<const N: usize, T> std::ops::MulAssign<T> for Vector<N, T>
	where T: Copy + std::ops::Mul<Output = T>,
{
	fn mul_assign(&mut self, rhs: T) {
		*self = *self * rhs;
	}
}

impl<const N: usize, T> std::ops::Div<T> for Vector<N, T>
	where T: Copy + std::ops::Div<Output = T>,
{
	type Output = Self;

	fn div(mut self, rhs: T) -> Self::Output {
		for val in self.data.iter_mut() {
			*val = *val / rhs;
		}

		self
	}
}

impl<const N: usize, T> std::ops::DivAssign<T> for Vector<N, T>
	where T: Copy + std::ops::Div<Output = T>,
{
	fn div_assign(&mut self, rhs: T) {
		*self = *self / rhs;
	}
}

impl<const N: usize, T> std::ops::Neg for Vector<N, T>
	where T: Copy + std::ops::Neg<Output = T>,
{
	type Output = Self;

	fn neg(mut self) -> Self::Output {
		for v in self.iter_mut() {
			*v = v.neg();
		}

		self
	}
}


impl<const N: usize, T> Vector<N, T>
	where T: Copy + std::ops::Mul<Output = T> + std::iter::Sum
{
	pub fn dot(vec_a: &Self, vec_b: &Self) -> T {
		vec_a.data.iter().zip(vec_b.data.iter())
			.map(|(a, b)| *a * *b)
			.sum()
	}

	pub fn magnitude(&self) -> T 
		where T: num::Float,
	{
		self.length_squared().sqrt()
	}

	pub fn ength(&self) -> T 
		where T: num::Float,
	{
		self.magnitude()
	}

	pub fn length_squared(&self) -> T {
		self.data.iter()
			.map(|val| *val * *val)
			.sum()
	}

	pub fn normalize(&mut self) 	
		where T: num::Float,
	{
		*self /= self.magnitude();
	}

	pub fn unit(&self) -> Self
		where T: num::Float,
	{
		*self / self.magnitude()
	}
}


impl<T> Vector<2, T> 
	where T: Copy,	
{
	#[inline]
	pub fn x(&self) -> T {
		self[0]
	}

	#[inline]
	pub fn set_x(&mut self, value: T) {		
		self[0] = value;
	}

	#[inline]
	pub fn y(&self) -> T {
		self[1]
	}

	#[inline]
	pub fn set_y(&mut self, value: T) {	
		self[1] = value;
	}
}

impl<T> Vector<3, T> 
	where T: Copy,	
{
	#[inline]
	pub fn x(&self) -> T {
		self[0]
	}

	#[inline]
	pub fn set_x(&mut self, value: T) {		
		self[0] = value;
	}

	#[inline]
	pub fn y(&self) -> T {
		self[1]
	}

	#[inline]
	pub fn set_y(&mut self, value: T) {	
		self[1] = value;
	}

	#[inline]
	pub fn z(&self) -> T {
		self[2]
	}

	#[inline]
	pub fn set_z(&mut self, value: T) {	
		self[2] = value;
	}	

	pub fn cross(u: &Self, v: &Self) -> Self 
		where T: std::ops::Mul<Output = T> + std::ops::Sub<Output = T>
	{
		Self {
			data: [
				u[1] * v[2] - u[3] * v[1],
				u[3] * v[0] - u[0] * v[3],
				u[0] * v[2] - u[2] * v[0],
			]
		}
		
	}
}
