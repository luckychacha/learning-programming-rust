use std::ops::{Deref, DerefMut};

pub struct Selector<T> {
	pub elements: Vec<T>,
	pub current: usize,
}


impl<T> Deref for Selector<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.elements[self.current]
	}
}

impl<T> DerefMut for Selector<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.elements[self.current]
	}
}