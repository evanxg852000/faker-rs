

struct Definitions {

}

impl Definitions {



    pub fn load(&mut self) {
        match self.locale.as_str() {
            "all" => self.load_locale(),
            _ => self.load_locales(),
        }
    }

    fn load_locales(&mut self) {}

    fn load_locale(&mut self) {}

    
}