use std::{any::TypeId, collections::HashSet};

use crate::ecs::{query::{Query, QueryParamList}, World};

enum ScheduleType {
	PreStartup,
	Startup,
	PostStartup,

	PreUpdate,
	Update,
	PostUpdate,
}

struct SystemNode {
	run: Box<dyn Fn(&World) + Send + Sync>,
	reads: HashSet<TypeId>,	
	writes: HashSet<TypeId>
}

pub struct Schedule<'a> {
	system_nodes: Vec<SystemNode>,
	world: &'a World,
}

impl<'a> Schedule<'a> {
	pub fn new(world: &'a World) -> Self {
		Self {
			system_nodes: Vec::new(),
			world,
		}
	}

	pub fn add_system<F, Q>(&mut self, mut system: F)
	where
		F: Fn(Query<Q>) + Send + Sync + 'static,
		Q: QueryParamList,
	{
		let (reads, writes) = Q::query_types();

		let node = SystemNode {
			reads,
			writes,
			run: Box::new(move |world: &World| {
				let query = Query::new(world);

				system(query);
			})
		};

	}
}