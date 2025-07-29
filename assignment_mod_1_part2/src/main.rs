fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers = [10, 3, 15, 8, 22, 30, 7, 5, 9, 13];

    for &num in numbers.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{} FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{} Fizz", num);
        } else if num % 5 == 0 {
            println!("{} Buzz", num);
        } else if is_even(num) {
            println!("{} is Even", num);
        } else {
            println!("{} is Odd", num);
        }
    }
    let mut index = 0;
    let mut sum = 0;
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("Sum of all numbers: {}", sum);

    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number in the array: {}", largest);
}
