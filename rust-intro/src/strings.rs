// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when yo uneed to modify or own string data

pub fn run() {
  // Primitive String
  let prim_hello = "Hello";
  // Growable String
  let mut hello = String::from("Hello ");

  // Get Length
  println!("Length: {}", hello.len());
  // Push single char to string
  hello.push('W');
  // Push string onto string
  hello.push_str("orl");

  // Capacity in bytes
  println!("Capacity: {}", hello.capacity());

  // Check if string is empty
  println!("Is Empty: {}", hello.is_empty());

  // Contains
  println!("Contains World: {}", hello.contains("World"));

  // Replace
  println!("Replace: {}", hello.replace("Worl", "Planet"));

  // Loop through string by whitespace
  for word in hello.split_whitespace() {
    println!("{}", word);
  }

  // Create string with capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');

  // Assertion testing
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());

  println!("{:?}", (prim_hello, hello, s));
}