use std::io; //Library with useful features

fn main() {

    println!("Guess the number!");
    let secret_number = rand::random_range(1..=100);
    println!("The secret number is: {secret_number}");


    loop {
        println!("Please input your guess FN 🥷 ");
        let mut guess_string = String::new();
        io::stdin().read_line(&mut guess_string).expect("Failed to read line");
         let guess:u32 = guess_string.trim().parse().expect("Please type a number!");
    // trim: strips out hidden newline space
    // parse: convert clean string slice into number
    // u32: tells Rust what 32-bit integer to turn it into
        if guess<secret_number{
            println!("The number you guessed is lower than the secret number");
        } else if guess>secret_number{
            println!("The number you guessed is higher than the secret number");

        } else{
            println!("You guessed the secret number which is {secret_number}");
            break;
        }
 
    }
    
    //Initialize an empty string and populate it with user input from the terminal
}
