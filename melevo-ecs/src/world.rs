use std::{
	any::{self, TypeId}, cell::RefCell, collections::HashMap
};

use utils::SparseSet;

use crate::{
	Component,
	ComponentPool, 
	Entity,
	Query,
	query::QueryParamList
};


const MAX_ENTITIES: usize = 1024;


pub struct World {
	entities: Vec<Entity>,
	component_pools: HashMap<any::TypeId, Box<dyn ComponentPool>>
}

impl World {
	pub fn new() -> Self {
		Self {
			entities: Vec::new(),
			component_pools: HashMap::new(),
		}
	}

	pub fn create_entity(&mut self) -> Entity {
		let entity_id = self.entities.len() as u64;
		let new_entity = Entity::new(entity_id, 0);

		self.entities.push(new_entity);

		new_entity
	}

	pub fn add_component<C: Component>(&mut self, entity: Entity, component: C) {
		let entry = self.component_pools
						.entry(component.type_id())
						.or_insert(Box::new(SparseSet::<RefCell<C>, Entity>::new(MAX_ENTITIES)));


		entry.add_component(entity, Box::new(component));
	}

}

impl World {
	pub(crate) fn component_pool<C: Component>(&self) -> Option<&Box<dyn ComponentPool>> {
		self.component_pools
			.get(&TypeId::of::<C>())
	}
}