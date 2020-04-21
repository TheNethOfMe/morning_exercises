// Loops are used to iterate until a condition is met

pub fn run() {
  let run_loop = false;
  let run_fizzbuzz = false;
  let mut count = 0;

  if run_loop {
    // Inifinte Loop
    loop {
      count += 1;
      println!("Numger: {}", count);
  
      if count == 20 {
        break;
      }
    }
  }

  if run_fizzbuzz {
    // While Loop
    while count <= 100 {
      if count % 15 == 0 {
        println!("fizzbuzz");
      } else if count % 3 == 0 {
        println!("fizz");
      } else if count % 5 == 0 {
        println!("buzz");
      } else {
        println!("{}", count);
      }

      count += 1;
    }
  }

  // For Range
  for x in 0..100 {
    if x % 15 == 0 {
      println!("fizzbuzz");
    } else if x % 3 == 0 {
      println!("fizz");
    } else if x % 5 == 0 {
      println!("buzz");
    } else {
      println!("{}", x);
    }
  }

}