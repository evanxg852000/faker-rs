use faker::{ Faker, FakerValue };

fn main() {
    let f = Faker::new(None, None);
    let v = f.get("random.integer");

    println!("Faker cli");
}
