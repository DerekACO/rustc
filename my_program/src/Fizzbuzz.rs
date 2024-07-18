fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    
    let numbers = [82, 6 , 3, 8, 22, 99, 72, 42, 15, 67];

   
    for &number in numbers.iter() {
        if number % 3 == 0 && number % 5 == 0 {
            println!("{}: FizzBuzz", number);
        } else if number % 3 == 0 {
            println!("{}: Fizz", number);
        } else if number % 5 == 0 {
            println!("{}: Buzz", number);
        } else {
            if is_even(number) {
                println!("{}: Even", number);
            } else {
                println!("{}: Odd", number);
            }
        }
    }

    
    let mut sum = 0;
    let mut index = 0;
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("Sum of all numbers: {}", sum);


    let mut largest = numbers[0];
    for &number in numbers.iter() {
        if number > largest {
            largest = number;
        }
    }
    println!("Largest number: {}", largest);
}
