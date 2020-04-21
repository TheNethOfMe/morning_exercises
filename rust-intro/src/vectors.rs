// Vectors - resizable arrays

use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

  println!("{:?}", numbers);

  // Re-assign value
  numbers[2] = 20;

  // Add onto Vector
  numbers.push(5);
  numbers.push(6);

  // Pop off last number
  numbers.pop();

  // Get single value
  println!("{}", numbers[2]);

  // Get Vector length
  println!("Vector Length: {}", numbers.len());

  // Vectors are stack allocated
  println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

  // Get Slice
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);

  // Loop through vector values
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // Loop and mutate values
  for y in numbers.iter_mut() {
    *y *= 2;
  }
  println!("Numbers Vec: {:?}", numbers);
}