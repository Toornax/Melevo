use std::{marker::PhantomData, slice::Iter};

use crate::{ComponentPool, Entity};

use super::QueryParamList;


pub struct QueryIter<'a, P: QueryParamList> {
	iter: Option<Iter<'a, Entity>>,
	pools: Vec<&'a Box<dyn ComponentPool>>,
	_marker: PhantomData<P>
}

impl<'a, P> QueryIter<'a, P>
	where P: QueryParamList
{
	pub fn new(pools: Vec<&'a Box<dyn ComponentPool>>) -> Self {
		let iter = pools.iter().min_by(|x, y| x.len().cmp(&y.len())).map(|v| v.entities().iter());

		Self { 
			iter,
			pools, 
			_marker: PhantomData 
		}
	}
}

impl<'a, P: QueryParamList> Iterator for QueryIter<'a, P> {
	type Item = P::Output<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		self.iter.as_mut().and_then(|entity_it| {
			entity_it.next().and_then(|entity| {
				P::fetch(entity, &self.pools)
			})
		})
	}
}
