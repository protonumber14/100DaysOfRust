fn temp_conversion(temp: f64) -> f64 {
    println!("Enter 'f' for farenheit\nEnter 'c' for celcius ");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line ");
    let input = input.trim().to_lowercase();
    //*pattern matching */
    match input.as_str() {
        "f" => {
            let fahrenheit = ((temp * 9.0 / 5.0) + 32.0).round();
            println!("{temp}°C is {fahrenheit}°F");
            fahrenheit
        }
        "c" => {
            let celsius = ((temp - 32.0) * 5.0 / 9.0).round();
            println!("{temp}°F is {celsius}°C");
            celsius
        }
        _=>{
            println!("Invalid input");
                temp
        }

    }
    //*If else method */
    // if input == 'f' || input == 'F' {
    //     let fahrenheit = ((temp * 9.0 / 5.0) + 32.0).round();
    //     println!("{temp}°C is {fahrenheit}°F");
    //     fahrenheit
    // } else if input == 'c' || input == 'C' {
    //     let celsius = ((temp - 32.0) * 5.0 / 9.0).round();
    //     println!("{temp}°F is {celsius}°C");
    //     celsius
    // } else {
    //     println!("Invalid input");
    //     temp
    // 
}

fn main() {
    println!("Temperature conversion\nEnter a number:");
    let mut temp = String::new();
    std::io::stdin()
        .read_line(&mut temp)
        .expect("Invalid input");
    let temp: f64 = temp.trim().parse().expect("Unable to convert");
    temp_conversion(temp);
}
