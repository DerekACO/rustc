fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret = 42; 
    let guesses_array = [30, 50, 40, 45, 42]; 
    let mut guesses = 0; 

    println!("Im thinking of a number 1 through 100... can you guess what it is?");

    for &guess in guesses_array.iter() {
        guesses += 1; 

        
        let result = check_guess(guess, secret);

        
        if result == 0 {
            println!("Correct! You've guessed my number {}.", guess);
            break; 
        } else if result == 1 {
            println!("{} is too high!", guess);
        } else {
            println!("{} is too low!", guess);
        }
    }

    
    println!("It took you {} guesses.", guesses);
}
