use std::rc::Rc;
use std::cell::RefCell;

use crate::engine::{ Engine, FakerValue };
use crate::traits::Generator;

#[derive(Debug)]
pub struct Fake {
    engine: Option<Rc<RefCell<Engine>>>,
}

impl Generator for Fake {
    fn set_engine(&mut self, e: &Rc<RefCell<Engine>>) {
        self.engine = Some(Rc::clone(e));
    }
}

impl Fake {
  
    pub fn new() -> Self {
        Fake {
            engine: None,
        }
    }

    pub fn int(&self, params: Vec<&str>) -> FakerValue {
        match self.engine {
            Some(ref engine) => {
                //TODO: use fn params -> [min, max]
                let v = engine.borrow_mut().random.integer();
                FakerValue::Int(v)
            },
            None => FakerValue::Error(String::from("Err: engine not initialised."))
        }
    }

    pub fn float(&self, param: Vec<&str>) -> FakerValue {
        match self.engine {
            Some(ref engine) => {
                //TODO: use fn params -> [min, max]
                let v = engine.borrow_mut().random.float();
                FakerValue::Float(v)
            },
            None => FakerValue::Error(String::from("Err: engine not initialised."))
        }
    }

    pub fn boolean(&self) -> FakerValue {
        match self.engine {
            Some(ref engine) => {
                let v = engine.borrow_mut().random.boolean();
                FakerValue::Bool(v)
            },
            None => FakerValue::Error(String::from("Err: engine not initialised."))
        }
        
    }
    
}