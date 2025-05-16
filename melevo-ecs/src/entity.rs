use utils::sparse_set::{InvalidKeyError, InvalidKeyErrorKind, Key};


#[derive(Debug, Clone, Copy)]
pub struct Entity {
	id: u64,
	version: u64,
}

impl Entity {
	pub(crate) fn new(id: u64, version: u64) -> Self {
		Self {
			id,
			version,
		}
	}

	pub fn id(&self) -> u64 {
		self.id
	}

	pub fn version(&self) -> u64 {
		self.version
	}
}

impl Key for Entity {
	fn into_usize(self) -> Result<usize, utils::sparse_set::InvalidKeyError<Self>> {
		self.id().try_into().map_err(|_| InvalidKeyError { kind: InvalidKeyErrorKind::KeyCantBeCastToUsize })
	}
}