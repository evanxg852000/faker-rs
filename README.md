## Faker-RS


impl FakerValue {
    pub fn as_str(&self) -> &str{
        match self {
            FakerValue::Null(ref s) => s.as_str(),
            FakerValue::Bool(b) => (*b as bool).to_string().as_str(),
            FakerValue::Int(ref n) => n.to_string().as_str(),
            FakerValue::Float(ref n) => n.to_string().as_str(),
            FakerValue::Str(ref s) => s.as_str(),
            FakerValue::Error(ref s) => s.as_str(),
        }
    }
}