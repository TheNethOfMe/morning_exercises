use std::env;

pub fn run() {
  let args: Vec<String> = env::args().collect();
  let command = args[1].clone();

  // println!("Command: {:?}", command);

  if command == "hello" {
    println!("Hello, how are you today?");
  } else if command == "status" {
    println!("Oh, I am doing fine. Thank you for asking.");
  } else {
    println!("Available commands are hello and status.");
  }
}