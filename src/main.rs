struct Address {
    city: String,
    zip: String,
}

struct Person {
    name: String,
    address: Address, // Composing an Address struct inside Person
}

fn main() {
    

    let person = Person {
        name: String::from("Alice"),
     address:    Address {
        city: String::from("New York"),
        zip: String::from("10001"),
    },
    };

    println!("{} lives in {}", person.name, person.address.city);
}