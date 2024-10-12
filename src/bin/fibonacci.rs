use std::u32;

fn fibonacci_recursive(f1: u32) -> u32 {
    match f1 {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(f1 - 1) + fibonacci_recursive(f1 - 2),
    }
}
fn main() {
    println!("Enter nth number for fibonacci");
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Invalid input");
    let n: u32 = n.trim().parse().expect("Invalid integer");
    let fibonacci = fibonacci_recursive(n);
    println!(
        "fibonnaci of {}th number is {} using recursion",
        n, fibonacci
    );
    //*Iterative method */
    let fibonacci = fibonacci_iterative(n);
    println!(
        "fibonnaci of {}th number is {} using iterative",
        n, fibonacci
    );
}

fn fibonacci_iterative(f1: u32) -> u32 {
    match f1 {
        0 => 0,
        1 => 1,
        _ => {
            let mut n1: u32 = 0;
            let mut n2: u32 = 1;
            for _ in 2..=f1 {
                let iterative: u32 = n1 + n2;
                n1 = n2;
                n2 = iterative;
            }
            n2
        }
    }
}
