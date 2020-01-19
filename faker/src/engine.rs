use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use regex::Regex;

use crate::random::Random;
use crate::traits::Generator;
use crate::fake::Fake;
use crate::fixed::Fixed;


#[derive(Debug)]
pub struct Engine {
    locale: String,
    fallback: String,
    pub random: Random,
    fixed: Fixed,
    fake: Fake,
}

impl Engine {

    pub fn new(path: String, locale: String, fallback: String) -> Rc<RefCell<Self>> {
        let engine = Rc::new(RefCell::new(
            Engine {
                locale,
                fallback,
                //definitions: Definitions::new(path),
                random: Random::new(),
                fixed: Fixed::new(),
                fake: Fake::new(),
            }
        ));

        // register all generator by setting engine property
        engine.borrow_mut().random.set_engine(&engine);
        engine.borrow_mut().fake.set_engine(&engine);
        engine.borrow_mut().fixed.set_engine(&engine);

        engine
    }

    pub fn get(&self, expr: &str) -> FakerValue {
        // "{fake.int:12,34} {fake.bool}"
        match expr.contains("{") {
            true => {
                let re = Regex::new(r"{(\w+)}").unwrap();
                let format = re.replace_all(expr, "").to_string(); // "{} {}"
                let caps = re.find_iter(expr)
                    .map(|m| m.as_str())
                    .collect::<Vec<&str>>();
                self.get_combine(format, caps)
            },
            false => {
                self.get_single(expr)
            }
        }
    }

    pub fn one_of<T: Clone>(&mut self, vals: &Vec<T>) -> T {
        self.random.one_of(&vals)
    }

    pub fn some_of<T: Clone>(&mut self, vals: &Vec<T>, n: usize) -> Vec<T> {
        self.random.some_of(&vals, n)
    }

    fn get_single(&self, expr: &str) -> FakerValue {
        //format -> fake.int:12,34
        let parts: Vec<&str> = expr.split(":").collect();
        let (name, params) = match parts.len() {
            1 => (Some(parts[0]), None),
            2 => (Some(parts[0]), Some(parts[1].split(",").collect::<Vec<&str>>())),
            _ => (None, None),
        };

        if name == None && params == None {
            return FakerValue::Error(String::from("Err: syntaxe error."));
        }

        let name = name.unwrap_or("");
        let params = params.unwrap_or(vec![]);
        match name {
            "fake.int" => self.fake.int(params),
            "fake.float" => self.fake.float(params),
            "fake.bool" => self.fake.boolean(),
            "fixed.null" => self.fixed.null(params),
            "fixed.int" => self.fixed.int(params),
            "fixed.float" => self.fixed.float(params),
            "fixed.bool" => self.fixed.boolean(params),
            "fixed.str" => self.fixed.string(params),

            //TODO load locale files, names bundle, name generator, tests
            //TODO server[core, json, csv]
            //TODO docs, video, feedback


            // fixed [int:val, bool:val, null:val, float:val, str:val]
            // fake [int, float, bool]
            // address
            // commerce
            // company
            // database
            // date
            // finance
            // image
            // internet
            // lorem
            // name
            // phone
            // system
            _ => FakerValue::Error(String::from("Err: generator not found.")),
        }
    }

    fn get_combine(&self, format: String, exprs: Vec<&str>) -> FakerValue {
        let mut vals = vec![];
        for expr in exprs.iter() {
            match self.get_single(expr) {
                FakerValue::Error(e) => return FakerValue::Error(e),
                v => vals.push(v), 
            };
        }

        let s = vals
            .iter()
            .fold(format, |acc, v| acc.replacen("{}", v.to_string().as_str(), 1));
        FakerValue::Str(s)
    }

}

#[derive(Debug, Clone, PartialEq)]
pub enum FakerValue {
    Null(String),
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    //Array(Vec<FakerValue>),
    Error(String),
}


impl fmt::Display for FakerValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FakerValue::Null(s) => write!(f, "{}", s),
            FakerValue::Bool(b) => write!(f, "{}", b),
            FakerValue::Int(n) => write!(f, "{}", n),
            FakerValue::Float(n) => write!(f, "{}", n),
            FakerValue::Str(s) => write!(f, "{}", s),
            FakerValue::Error(s) => write!(f, "{}", s),
        }
    }
}

