use std::{thread, time};

fn main() {
    //* Ownership rules */
    // Each value in Rust has an owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.

    //* String literal */ -> stack memory
    let str1: &str = "Fight Club"; // String literal (stack-allocated)
    println!("String literal: {str1}");

    //* String type */ -> heap memory
    let mut str1 = String::from("1. Social\n"); // Shadowed str1 or dropped
    str1.push_str("2. Network");
    println!("After modification, str1: {str1}");

    let mut str2 = str1; // Ownership of str1 is transferred to str2
    str2.push_str(" - Owner transferred to str2, and data is appended.");
    // str1 is now inaccessible
    println!("Note: Attempting to print str1 will fail because it has been moved to str2.");
    // println!("{str1}"); // This line would cause a compile error if uncommented
    println!("Current value of str2: {}", str2);

    //* Clone data */
    let str1 = String::from("Girl with Dragon Tattoo"); // Shadowing str1 again
    println!("Cloning in progress... for str1");
    thread::sleep(time::Duration::from_millis(500));
    let str2 = str1.clone(); // Cloning str1
    println!("Cloning completed! This operation can be expensive.");
    println!("str1 = {str1}\nstr2 = {str2}"); // Both values are now accessible
    println!("Both values are stored separately in memory.");

    //* Stack data: Copy */
    let x: i8 = 10; // i8 is a Copy type
    let y: i8 = x; // Data is copied, similar to cloning but for Copy types
    println!("x = {x} and y = {y}"); // Both x and y are independent

    //* Ownership functions */
    let str1 = String::from("Hello");
    println!("Original str1: {str1}");
    println!("Ownership transfer in progress... for str1");
    thread::sleep(time::Duration::from_millis(500));
    change(str1); // Ownership of str1 is transferred to the change function

    // The change function
}

fn change(str: String) {
    println!("Changed string: {str}");
}
