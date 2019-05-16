use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  println!("Guessing game, guess a number");

  let secret_number = rand::thread_rng().gen_range(1, 101);
  println!("Secret number is {}", secret_number);

  loop {
    println!("Your guess: ");

    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("Error reading input");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Greater => println!("Too big"),
      Ordering::Less => println!("Too small"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}
