fn is_even(n: i32) -> bool{
    n % 2 == 0
}


fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 15];

    println!("--Number list--");

    for &n in numbers.iter(){ 
        if n % 3 == 0 && n % 5 == 0 {
            println!("{} -> FizzBuzz", n);
        }
        else if n % 3 == 0 {
            println!("{} -> Fizz", n);
        }
        else if n % 5 == 0{
            println!("{} -> Buzz", n);
        }
        else{
            let even_odd = if is_even(n) { "even" } else { "odd" };
            println!("{} → {} ({})", n, even_odd, n);
        }
    }
    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("\nSum of all numbers (using while loop): {}", sum);

    let mut largest = numbers[0];
    for &num in &numbers {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {}", largest);
}
