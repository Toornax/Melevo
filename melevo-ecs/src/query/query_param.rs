use std::{any::TypeId, collections::HashSet};

use crate::{Component, ComponentPool, Entity, World};
use super::QueryIter;

pub trait QueryParam: Sized { 
	type Item: Component;
	type ReturnType<'b>;
	const POOL_COUNT: usize;

	fn fetch<'a>(entity: &Entity, pools: &[&'a Box<dyn ComponentPool>]) -> Option<Self::ReturnType<'a>>;
	fn query_types() -> (HashSet<TypeId>, HashSet<TypeId>);
}

impl<C: Component> QueryParam for &C {
	type Item = C;
	type ReturnType<'b> = &'b C;
	const POOL_COUNT: usize = 1;

	fn fetch<'a>(entity: &Entity, pools: &[&'a Box<dyn ComponentPool>]) -> Option<Self::ReturnType<'a>> {
		pools.first().and_then(|pool| {
			if let Some(comp) = pool.get(entity) {
				comp.downcast_ref::<C>().map(|r| unsafe { std::mem::transmute::<&C, &'a C>(r) })
			} else {
				None
			}
		})
	}
			
	fn query_types() -> (HashSet<TypeId>, HashSet<TypeId>) {
		let mut set = HashSet::new();
		set.insert(TypeId::of::<C>());

		(set, HashSet::new())
	}
}

impl<C: Component> QueryParam for &mut C {
	type Item = C;
	type ReturnType<'b> = &'b mut C;
	const POOL_COUNT: usize = 1;

	fn fetch<'a>(entity: &Entity, pools: &[&'a Box<dyn ComponentPool>]) -> Option<Self::ReturnType<'a>> {
		pools.first().and_then(|pool| {
			if let Some(mut comp) = pool.get_mut(entity) {
				comp.downcast_mut::<C>().map(|r| unsafe { std::mem::transmute::<&mut C, &'a mut C>(r) })
			} else {
				None
			}
		})
	}
			
	fn query_types() -> (HashSet<TypeId>, HashSet<TypeId>) {
		let mut set = HashSet::new();
		set.insert(TypeId::of::<C>());

		(HashSet::new(), set)
	}
}

pub trait QueryParamList: Sized {
	type Output<'a>;
	type Iter<'a>: Iterator<Item = Self::Output<'a>>;
	const POOL_COUNT: usize;

	fn iter<'w>(world: &'w World) -> Self::Iter<'w>;
	fn fetch<'a>(entity: &Entity, pools: &[&'a Box<dyn ComponentPool>]) -> Option<Self::Output<'a>>;
	fn query_types() -> (HashSet<TypeId>, HashSet<TypeId>);
}

macro_rules! implement_query_param_list {
	($($comp:ident),+) => {
		impl<$($comp: QueryParam),+> QueryParamList for ($($comp),+) {
			type Output<'a> = ($($comp::ReturnType<'a>),+);
			type Iter<'a> = QueryIter<'a, Self>;			
			const POOL_COUNT: usize = 0 $(+ $comp::POOL_COUNT)+;

			fn iter<'w>(world: &'w World) -> Self::Iter<'w> {
				let mut pools: Vec<&Box<dyn ComponentPool>> = Vec::new();

				$(
					if let Some(pool) = world.component_pool::<$comp::Item>() {
						pools.push(pool);
					}
				)+

				QueryIter::new(pools)
			}

			fn fetch<'a>(entity: &Entity, pools: &[&'a Box<dyn ComponentPool>]) -> Option<Self::Output<'a>> {
				if pools.len() < Self::POOL_COUNT {
					return None;
				}
				
				let mut slice = pools;

				Some(($(
					{
						let (head, tail) = slice.split_at($comp::POOL_COUNT);
						slice = tail;
						$comp::fetch(entity, head)?
					}
				),+))
			}
			
			fn query_types() -> (HashSet<TypeId>, HashSet<TypeId>) {
				let mut set_read = HashSet::new();
				let mut set_write = HashSet::new();
				
				$({
					let (read, write) = $comp::query_types();
					set_read.extend(read);
					set_write.extend(write);
				})+

				(set_read, set_write)
			}
		}
	};
}


implement_query_param_list!(A);
implement_query_param_list!(A, B);
implement_query_param_list!(A, B, C);
implement_query_param_list!(A, B, C, D);
implement_query_param_list!(A, B, C, D, E);
implement_query_param_list!(A, B, C, D, E, F);
implement_query_param_list!(A, B, C, D, E, F, G);
implement_query_param_list!(A, B, C, D, E, F, G, H);
