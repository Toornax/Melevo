use std::{error::Error, fmt, marker::PhantomData};

#[derive(Debug)]
pub struct SparseSet<T, K=u64> { 
	capacity: usize,
	sparse: Vec<Option<usize>>,
	dense: Vec<T>,
	keys: Vec<K>,
	_marker: PhantomData<K>
}

impl<T, K: Key> SparseSet<T, K> {
	pub fn new(capacity: usize) -> Self {
		let sparse = vec![None; capacity];

		Self {
			capacity,
			sparse,
			dense: Vec::new(),
			keys: Vec::new(),
			_marker: PhantomData
		}
	}

	pub fn insert(&mut self, key: K, value: T) -> Result<Option<T>, InvalidKeyError<K>> {
		let valid_key = self.validate_key(key)?;

		if let Some(index) = self.sparse[valid_key] {
			let last_value = std::mem::replace(&mut self.dense[index], value);

			Ok(Some(last_value))
		} else {
			let index = self.dense.len();
			self.sparse[valid_key] = Some(index);
			self.dense.push(value);
			self.keys.push(key);

			Ok(None)
		}
	}

	pub fn get(&self, key: K) -> Result<Option<&T>, InvalidKeyError<K>> {
		let valid_key = self.validate_key(key)?;

		if let Some(index) = self.sparse[valid_key] {
			Ok(Some(&self.dense[index]))
		} else {
			Ok(None)
		}
	}

	pub fn get_mut(&mut self, key: K) -> Result<Option<&mut T>, InvalidKeyError<K>> {
		let valid_key = self.validate_key(key)?;
		
		if let Some(index) = self.sparse[valid_key] {
			Ok(Some(&mut self.dense[index]))
		} else {
			Ok(None)
		}
	}

	pub fn contains(&self, key: K) -> Result<bool, InvalidKeyError<K>> {
		let valid_key = self.validate_key(key)?;

		Ok(self.sparse[valid_key].is_some())
	}

	pub fn remove(&mut self, key: K) -> Result<Option<T>, InvalidKeyError<K>> {
		let valid_key = self.validate_key(key)?;

		if let Some(index) = self.sparse[valid_key] {
			let last_index = self.dense.len() - 1;
			self.dense.swap(index, last_index);
			self.keys.swap(index, last_index);
			
			self.sparse[valid_key] = None;
			let _ = self.dense.pop();
			Ok(self.dense.pop())
		} else {
			Ok(None)
		}
	}

	pub fn len(&self) -> usize {
		self.dense.len()
	}

	pub fn dense(&self) -> &Vec<T> {
		&self.dense
	}

	pub fn keys(&self) -> &Vec<K> {
		&self.keys
	}
}

impl<T, K: Key> SparseSet<T, K> {
	fn validate_key(&self, key: K) -> Result<usize, InvalidKeyError<K>> {
		let key_usize = key.into_usize()?;

		if key_usize >= self.capacity {
			Err(InvalidKeyError {
				kind: InvalidKeyErrorKind::KeyBiggerThanCapacity { key: key, capacity: self.capacity }
			})
		} else {			
			Ok(key_usize)
		}
	}
}


#[derive(Debug)]
pub struct InvalidKeyError<K: Key = u64> {
	pub kind: InvalidKeyErrorKind<K>
}

impl<K: Key> InvalidKeyError<K> {
	pub fn kind(&self) -> InvalidKeyErrorKind<K> {
		self.kind.clone()
	}
}

#[derive(Clone, Debug)]
pub enum InvalidKeyErrorKind<K> {
	KeyBiggerThanCapacity {
		key: K,
		capacity: usize
	},
	KeyCantBeCastToUsize,
}

impl<K: Key> fmt::Display for InvalidKeyError<K> {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.kind() {
			InvalidKeyErrorKind::KeyBiggerThanCapacity { key, capacity } => {
				write!(fmt, "invalid key, the key `{key:?}` is bigger than capacity `{capacity}`.")
			}
			InvalidKeyErrorKind::KeyCantBeCastToUsize => {
				write!(fmt, "")
			}
		}
	}
}

impl<K: Key> Error for InvalidKeyError<K> {
	#[allow(deprecated)]
	fn description(&self) -> &str {
		"invalid key, the key is bigger than the capacity."
	}
}


pub trait Key: Sized + Clone + Copy + std::fmt::Debug {
	fn into_usize(self) -> Result<usize, InvalidKeyError<Self>>;
}


impl Key for u8 {
	fn into_usize(self) -> Result<usize, InvalidKeyError<Self>> {
		Ok(self.into())
	}
}

impl Key for u16 {
	fn into_usize(self) -> Result<usize, InvalidKeyError<Self>> {
		Ok(self.into())
	}
}

impl Key for u32 {
	fn into_usize(self) -> Result<usize, InvalidKeyError<Self>> {
		self.try_into().map_err(|_| InvalidKeyError { kind: InvalidKeyErrorKind::KeyCantBeCastToUsize })
	}
}

impl Key for u64 {
	fn into_usize(self) -> Result<usize, InvalidKeyError<Self>> {
		self.try_into().map_err(|_| InvalidKeyError { kind: InvalidKeyErrorKind::KeyCantBeCastToUsize })
	}
}

impl Key for u128 {
	fn into_usize(self) -> Result<usize, InvalidKeyError<Self>> {
		self.try_into().map_err(|_| InvalidKeyError { kind: InvalidKeyErrorKind::KeyCantBeCastToUsize })
	}
}

impl Key for usize {
	fn into_usize(self) -> Result<usize, InvalidKeyError<Self>> {
		Ok(self)
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_insert() -> Result<(), InvalidKeyError> {		
		let mut sparse_set = SparseSet::new(10);
		
		sparse_set.insert(1, "Alice")?;
		sparse_set.insert(2, "Bob")?;
		sparse_set.insert(3, "Charlie")?;

		// Check that elements is well inserted
		assert_eq!(sparse_set.get(1)?, Some(&"Alice"));
		assert_eq!(sparse_set.get(2)?, Some(&"Bob"));
		assert_eq!(sparse_set.get(3)?, Some(&"Charlie"));
		assert_eq!(sparse_set.get(4)?, None);

		Ok(())
	}

	#[test]
	fn test_remove() -> Result<(), InvalidKeyError> {		
		let mut sparse_set = SparseSet::new(10);
		
		sparse_set.insert(1, "Alice")?;
		sparse_set.insert(2, "Bob")?;
		sparse_set.insert(3, "Charlie")?;

		// Check if the element is well removed
		assert_eq!(sparse_set.remove(2)?, Some("Bob"));
		assert_eq!(sparse_set.get(2)?, None);

		Ok(())
	}
}
