fn main() {
    //*Loop with iteration continous */
    // loop {
    //     println!("I am crabby  🦀");
    // }
    //*Loop with if, break */
    let mut x: u8 = 0;
    loop {
        println!("current number = {x}");
        if x > 7 {
            println!("Loop terminated at {x}");

            break;
        }
        x += 1;
    }
    print!("🦀");

    // * Loop lables*/
    let mut num: i8 = 20;
    println!("Initial number is {num}");
    'count_loop: loop {
        println!("number after decrementing = {num}");
        let mut terminator: i8 = 0;
        loop {
            println!("terminator value = {terminator}");
            if terminator == 5 {
                println!("terminator ended value is {terminator}");
                break;
            }
            if num < 15 {
                break 'count_loop;
            }
            terminator += 1;
        }
        num -= 2
    }
    println!("number ended value is {num}");

    //*while loop */
    let mut num: i8 = 0;
    while num < 5 {
        num += 1;
        print!("curent number is {num}");
    }

    println!("🦀");
    //* For loop */
    let arr: [i8; 3] = [54, 68, -32];
    for element in arr {
        print!("{element}\t");
    }
    //*For loop with range */
    for numbers in (1..5).rev() {
        println!("{numbers}");
    }
}
