// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
  let name = "Tommy";
  let mut color = "Green";

  println!("{} was the {} Ranger.", name, color);

  color = "White";

  println!("{} is now the {} Ranger.", name, color);

  // Define constant
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assign multiple vars
  let ( fruit, vegatable, mineral ) = ( "Banana", "Potato", "Feldspar" );
  println!("Fruit: {}, Vegatable: {}, Mineral: {}", fruit, vegatable, mineral );
}