use std::rc::Rc;
use std::cell::RefCell;

use crate::engine::Engine;

pub trait Generator {
    fn set_engine(&mut self, e: &Rc<RefCell<Engine>>);
}