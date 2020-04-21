// Conditionals are used to check the condition of something and act accordingly

pub fn run() {
 let age: u8 = 19;
 let check_id: bool = true;
 let knows_person_of_age: bool = false;

 // If/Else
 if age >= 21 && check_id || knows_person_of_age {
   println!("Bartender: What would you like to drink?");
 } else if age < 21 && check_id {
   println!("Bartender: Sorry, I can't serve you.");
 } else {
   println!("Bartender: I need to see some ID.");
 }

 // Shorthand If
 let is_of_age = if age >= 21 { true } else { false };
 println!("Is of Age: {}", is_of_age);
}