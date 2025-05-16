use std::any::Any;

pub trait Component: Any { }

impl<C: Any> Component for C { }