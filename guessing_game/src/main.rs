use rand::Rng; //rand library import.
use std::cmp::Ordering;
use std::io; //IO library. //Ordering compare library.

fn main() {
    println!("Guess the number!"); //console output.

    let secret_number = rand::thread_rng().gen_range(1..=100); //start is already inclusive and the end is inclusive just if used the equal(=) sign.
    println!("The secret number is: {secret_number}");//console output.//{} are palceholders for variables values.

    loop {
        println!("Please input your guess.");//console output.

        let mut guess = String::new();//created the guess mutable variable with the type being inferred by the value.
        // let mut abc: i32 = 10;//this is an example of the type being explicit declared using :i32 (32 byte integer).

        io::stdin()
            .read_line(&mut guess)//read the keyboard input and assign to the guess variable.
            .expect("Failed to read line");//in case of error(Err) programm exits with this message.

        let guess: u32 = match guess.trim().parse() { //shadows the guess variable, remove white spaces and parse to the defined(:u32) type.
            Ok(num) => num,//if the operation succed assigns the value to the variable.
            Err(_) => { //if it fails... 
                println!("Value must be a number.");//console output.
                continue;//jump to the next loop iteration.
            }
        };

        println!("You guessed: {guess}"); //console output.//{} are palceholders for variables values.

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),//console output.
            Ordering::Greater => println!("Too big!"),//console output.
            Ordering::Equal => {
                println!("You win!");//console output.
                break;//exits the loop.
            }
        }
    }
}
