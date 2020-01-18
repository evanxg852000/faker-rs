use std::rc::Rc;
use std::cell::RefCell;

use crate::engine::{ Engine, FakerValue };
use crate::traits::Generator;

use rand::{ Rng, rngs::ThreadRng };
use rand::distributions::{ uniform::SampleUniform };
use uuid::Uuid;

const ALPHA_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const NUMERIC_CHARSET: &[u8] = b"0123456789";
const ALPHA_NUMERIC_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const ANY_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(][*&^%$#@!~_";
const HEX_CHARSET: &[u8] = b"ABCDEF0123456789";

#[derive(Debug)]
pub struct Random {
    engine: Option<Rc<RefCell<Engine>>>,
    rng: ThreadRng,
}

impl Generator for Random {
    fn set_engine(&mut self, e: &Rc<RefCell<Engine>>) {
        self.engine = Some(Rc::clone(e));
    }
}

impl Random {
  
    pub fn new() -> Self {
        Random {
            engine: None,
            rng: rand::thread_rng(),
        }
    }

    pub fn integer(&mut self) -> i64 {
        self.rng.gen::<i64>()
    }
  
    pub fn float(&mut self) -> f64 {
        self.rng.gen::<f64>()
    }

    pub fn random<T: SampleUniform>(&mut self, min: T, max: T) -> T {
        self.rng.gen_range(min, max)
    }

    pub fn boolean(&mut self) -> bool {
        self.rng.gen::<bool>()
    }
    
    pub fn alpha_string(&mut self, n: usize) -> String {
        self.random_string(ALPHA_CHARSET, n)
    }

    pub fn numeric_string(&mut self, n: usize) -> String {
        self.random_string(NUMERIC_CHARSET, n)
    }

    pub fn alpha_numeric_string(&mut self, n: usize) -> String {
        self.random_string(ALPHA_NUMERIC_CHARSET, n)
    }

    pub fn any_string(&mut self, n: usize) -> String {
        self.random_string(ANY_CHARSET, n)
    }

    pub fn hex_string(&mut self, n: usize) -> String {
        self.random_string(HEX_CHARSET, n)
    }

    pub fn uuid(&mut self, hyphenated: bool, version: Option<&str>, namespace: Option<&str>) -> String {
        let name =  namespace.unwrap_or("faker-rust.org").as_bytes();
        let value = match version.unwrap_or("") {
            "v3" => Uuid::new_v3(&Uuid::NAMESPACE_URL, name),
            "v4" => Uuid::new_v4(),
            _ =>  Uuid::new_v5(&Uuid::NAMESPACE_URL, name),
        };

        match hyphenated {
            true => value.to_hyphenated().to_string(),
            false => value.to_string().replace("-", ""), // Simple still comes with hyphens
        }
    }

    pub fn one_of<T: Clone>(&mut self, arr: &[T]) -> T {
        let idx = self.rng.gen_range(0, arr.len()) as usize;
        arr[idx].clone()
    }

    pub fn some_of<T: Clone>(&mut self, arr: &[T], n: usize) -> Vec<T> {
        (0..n).map(|_| {
            let idx = self.rng.gen_range(0, n);
            arr[idx].clone()
        }).collect()
    }

    fn random_string(&mut self, charset: &[u8], n: usize) -> String {
        (0..n) .map(|_| {
            let idx = self.rng.gen_range(0, charset.len());
            charset[idx] as char
        }).collect()
    }

}

#[cfg(test)]
mod tests {
    use std::any::type_name;
    use super::Random;
    use regex::Regex;

    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }

    #[test]
    fn test_primitives () {
        let mut ran = Random::new();

        assert_eq!(type_of(ran.integer()), "i64");
        assert_eq!(type_of(ran.float()), "f64");
        assert_eq!(type_of(ran.random(1.2, 3.4)), "f64");
        assert_eq!(type_of(ran.boolean()), "bool");
        
        let x = ran.random(2, 5);
        assert_eq!( x >= 2 && x < 5 , true);
    }

    #[test]
    fn test_strings() {
        let mut ran = Random::new();

        let r = ran.alpha_string(10);
        assert_eq!(r.len(), 10);
        let re = Regex::new(r"[a-zA-Z]+").unwrap();
        assert_eq!(re.is_match(&r), true);

        let r = ran.numeric_string(10);
        assert_eq!(r.len(), 10);
        let re = Regex::new(r"[0-9]+").unwrap();
        assert_eq!(re.is_match(&r), true);

        let r = ran.alpha_numeric_string(10);
        assert_eq!(r.len(), 10);
        let re = Regex::new(r"[a-zA-Z0-9]+").unwrap();
        assert_eq!(re.is_match(&r), true);

        let r = ran.any_string(30);
        assert_eq!(r.len(), 30);
        let re = Regex::new(r"[a-zA-Z0-9\)\(]+").unwrap();
        assert_eq!(re.is_match(&r), true);
        
        let r = ran.hex_string(12);
        assert_eq!(r.len(), 12);
        let re = Regex::new(r"[a-fA-f0-9]+").unwrap();
        assert_eq!(re.is_match(&r), true);

        let r = ran.uuid(true, Some("v3"), None);
        assert_eq!(r.contains("-"), true);
        let re = Regex::new(r"[a-fA-f0-9]+").unwrap();
        assert_eq!(re.is_match(&r), true);

        let r = ran.uuid(false, None, None);
        assert_eq!(r.contains("-"), false);
        let re = Regex::new(r"[a-fA-f0-9]+").unwrap();
        assert_eq!(re.is_match(&r), true);
    }

    #[test]
    fn test_arrays() {
        let mut ran = Random::new();

        let arr = vec![2,3,4,5,6];
        let x = ran.one_of(&arr[0..3]);
        assert_eq!( x >= 2 && x < 5 , true);

        let x = ran.some_of(&arr, 2);
        assert_eq!(x.len(), 2);
    }
}



