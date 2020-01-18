use std::rc::Rc;
use std::cell::RefCell;

use crate::engine::{ Engine, FakerValue };
use crate::traits::Generator;

#[derive(Debug)]
pub struct Fixed {
    engine: Option<Rc<RefCell<Engine>>>,
}

impl Generator for Fixed {
    fn set_engine(&mut self, e: &Rc<RefCell<Engine>>) {
        self.engine = Some(Rc::clone(e));
    }
}

impl Fixed {
  
    pub fn new() -> Self {
        Fixed {
            engine: None,
        }
    }

    pub fn null(&self, params: Vec<&str>) -> FakerValue {
        match params.len() {
            0 => FakerValue::Error(String::from("Err: engine not initialised.")),
            _ => FakerValue::Null(params[0].to_string()),
        }
    }

    pub fn int(&self, params: Vec<&str>) -> FakerValue {
        match params.len() {
            0 => FakerValue::Error(String::from("Err: engine not initialised.")),
            _ => {
                match params[0].parse::<i64>() {
                    Ok(v) => FakerValue::Int(v),
                    Err(_) => FakerValue::Error(String::from("Err: wrong params.")),
                }
            }
        }
    }

    pub fn float(&self, params: Vec<&str>) -> FakerValue {
        match params.len() {
            0 => FakerValue::Error(String::from("Err: engine not initialised.")),
            _ => {
                match params[0].parse::<f64>() {
                    Ok(v) => FakerValue::Float(v),
                    Err(_) => FakerValue::Error(String::from("Err: wrong params.")),
                }
            }
        }
    }

    pub fn boolean(&self, params: Vec<&str>) -> FakerValue {
        match params.len() {
            0 => FakerValue::Error(String::from("Err: engine not initialised.")),
            _ => {
                match params[0].parse::<bool>() {
                    Ok(v) => FakerValue::Bool(v),
                    Err(_) => FakerValue::Error(String::from("Err: wrong params.")),
                }
            }
        }
    }

    pub fn string(&self, params: Vec<&str>) -> FakerValue {
        match params.len() {
            0 => FakerValue::Error(String::from("Err: engine not initialised.")),
            _ => FakerValue::String(params[0].to_string()),
        }
    }

}