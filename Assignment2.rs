// Function to check if a number is even
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // Array of 10 integers
    let numbers: [i32; 10] = [12, 7, 15, 9, 20, 3, 8, 30, 11, 5];

    // Analyze numbers using a for loop
    for num in numbers.iter() {
        // Check even or odd
        if is_even(*num) {
            print!("{} is Even -> ", num);
        } else {
            print!("{} is Odd -> ", num);
        }

        // Fizz / Buzz / FizzBuzz logic
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        } else {
            println!("No Fizz/Buzz");
        }
    }

    // Calculate sum using a while loop
    let mut index = 0;
    let mut sum = 0;

    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }

    println!("\nSum of all numbers: {}", sum);

    // Find the largest number using a loop
    let mut largest = numbers[0];

    for num in numbers.iter() {
        if *num > largest {
            largest = *num;
        }
    }

    println!("Largest number in the array: {}", largest);
}
