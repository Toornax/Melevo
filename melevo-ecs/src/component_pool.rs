use std::any::Any;
use std::cell::{Ref, RefMut, RefCell};

use crate::utils::SparseSet;
use crate::ecs::{
	Component,
	Entity
};

pub trait ComponentPool {
	fn add_component(&mut self, entity: Entity, component: Box<dyn Any>);
	fn entities(&self) -> &Vec<Entity>;
	fn components(&self) -> Vec<&dyn Any>;
	fn get(&self, entity: &Entity) -> Option<Ref<dyn Any>>;
	fn get_mut(&self, entity: &Entity) -> Option<RefMut<dyn Any>>;
	fn iter_mut(&self) -> Box<dyn Iterator<Item = RefMut<dyn Any>> + '_>;
	fn len(&self) -> usize;
}

impl<C: Component> ComponentPool for SparseSet<RefCell<C>, Entity> {	
	fn add_component(&mut self, entity: Entity, component: Box<dyn Any>) {
		if let Ok(component) = component.downcast::<C>() {
			self.insert(entity, RefCell::new(*component)).unwrap();
		}
	}

	fn components(&self) -> Vec<&dyn Any> {
		self.dense().iter().map(|c| c as &dyn Any).collect()
	}

	fn entities(&self) -> &Vec<Entity> {
		self.keys()
	}
	
	fn get(&self, entity: &Entity) -> Option<Ref<dyn Any>> {
		if let Ok(Some(component)) = self.get(entity.clone()) {
			Some(Ref::map(component.borrow(), |comp| comp as &dyn Any))
		} else {
			None
		}
	}

	fn get_mut(&self, entity: &Entity) -> Option<RefMut<dyn Any>> {
		if let Ok(Some(component)) = self.get(entity.clone()) {
			Some(RefMut::map(component.borrow_mut(), |comp| comp as &mut dyn Any))
		} else {
			None
		}
	}

	fn iter_mut(&self) -> Box<dyn Iterator<Item = RefMut<dyn Any>> + '_> {
		Box::new(self.dense().iter().map(|c| RefMut::map(c.borrow_mut(), |v| v as &mut dyn Any)))
	}

	fn len(&self) -> usize {
		self.len()
	}
}