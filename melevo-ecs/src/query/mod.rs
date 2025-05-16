pub use query_param::QueryParamList;
pub use query_iter::QueryIter;

mod query_iter;
mod query_param;


use std::marker::PhantomData;
use super::World;

pub struct Query<'a, P: QueryParamList> { 
	world: &'a World,
	_marker: PhantomData<P>	
}

impl<'a, P: QueryParamList> Query<'a, P> {
	pub fn new(world: &'a World) -> Self {
		Self {
			world,
			_marker: PhantomData, 
		}
	}

	pub fn iter(&self) -> P::Iter<'a> {
		P::iter(self.world)
	}
}
