// Structs are used to create custom data types

// Traditional Struct
struct Color {
  red: u8,
  green: u8,
  blue: u8
}

// Tuple Struct
struct Hue(u8, u8, u8);

struct Person {
  first_name: String,
  last_name: String
}

impl Person {
  // Construct a person
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string()
    }
  }
  // Get full name
  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }
  // Set last name
  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();
  }
  // Name to tuple
  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}

pub fn run() {
  let mut c = Color {
    red: 255,
    green: 0,
    blue: 0
  };

  c.red = 200;

  println!("Color: {} {} {}", c.red, c.green, c.blue);

  let mut h = Hue(255, 0, 0);

  h.1 = 200;

  println!("Hue: {} {} {}", h.0, h.1, h.2);

  let mut p = Person::new("John", "Doe");
  println!("Person: {}", p.full_name() );
  p.set_last_name("Williams");
  println!("Person(Tuple): {:?}", p.to_tuple());
}