// Attributes deriving traits for this type.
#[derive(Copy, Clone, Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    login_count: usize,
    favourite_color: Color,
}

impl Person {
    // Method
    fn clone(&self) -> Person {
        return Person {
            name: format!("{} (clone)", self.name),
            login_count: 1,
            ..*self
        };
    }

    // Associated function
    fn create(name: &str, age: u32, favourite_color: Color) -> Person {
        assert_eq!(name.is_empty(), false);
        return Person {
            name: name.to_string(),
            age,
            login_count: 1,
            favourite_color,
        };
    }
}

fn log_person(person: &mut Person) {
    person.login_count += 1;
}

fn set_favourite_color(person: &mut Person, color: Color) {
    person.favourite_color = color;
}

fn print_person(person: &Person) {
    println!(
        "Person '{}', {}, has been seen {} times. Their favourite color is {:?}.",
        person.name, person.age, person.login_count, person.favourite_color
    );
}

fn main() {
    let mut person_jan = Person::create("Jan Vorisek", 24, Color(255, 0, 0));
    print_person(&person_jan);
    log_person(&mut person_jan);
    print_person(&person_jan);
    log_person(&mut person_jan);
    log_person(&mut person_jan);
    print_person(&person_jan);

    let mut person_jan_clone = person_jan.clone();
    set_favourite_color(&mut person_jan_clone, Color(128, 128, 255));
    print_person(&person_jan_clone);
    print_person(&person_jan);

    // Debug output to stderr
    dbg!(&person_jan);
}
