use std::rc::Rc;
use std::cell::RefCell;

use crate::engine::{ Engine, FakerValue };


#[derive(Debug)]
pub struct Faker {
    engine: Rc<RefCell<Engine>>,
}

impl Faker {
    pub fn new(path: Option<String>, locale: Option<String>) -> Self {
        let path = path.unwrap_or(String::from("./.faker.bundle/"));
        let locale = locale.unwrap_or(String::from("en"));
        Faker {
            engine: Engine::new(path, locale, String::from("en")),
        }
    }

    pub fn get(&self, expr: &str) -> FakerValue {
        self.engine.borrow().get(expr)
    }

}



#[cfg(test)]
mod tests {
    use super::Faker;
    use crate::engine::FakerValue;

    #[test]
    fn test_arrays() {
        let f = Faker::new(None, None);
        // assert_eq!(f.get(""), FakerValue::Bool(true));
    }
}
