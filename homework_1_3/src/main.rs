fn check_guess(guess: i32, secret: i32) -> i32{
     if guess == secret {
        0      
    } else if guess > secret {
        1
    } else {
        -1
    }
}


fn main() {
    let secret = 35;

    let guess_list = [50, 30, 40, 45, 35, 60];

    let mut index = 0;

    let mut attempts = 0;

    loop {
        if index >= guess_list.len() {
            println!("No more guesses! The secret number was {}!", secret);
            break;
        }

        let mut guess = guess_list[index];
        attempts += 1;

        let result = check_guess(guess, secret);

        if result == 0{
            println!("Congrats you guessed the right number it is {}!", secret);
            break;
        }
        else if result == 1{
            println!("{} is too high try again...", guess)
        }
        else {
            println!("{} is too low try again...", guess)
        }
        index += 1;
    }

    println!("It took {} guesses.", attempts);

}
