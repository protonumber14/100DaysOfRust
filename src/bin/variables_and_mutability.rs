//constant in global scope
const PI: f32 = 22.0 / 7.0;
pub fn main() {
    //Varriables
    let x:u8 = 5;
    let mut y = 20;
    
    
    y = y + 2;// value changed

    println!("Immutable varriable x={x}\n and Mutated varriable y={y}");
    print!("Before mutation y is {y} ");   
    println!("const pi ={PI}");
    //shadowing
    let w:i8=-2;
    println!("inital value of W ={w}");
    let w:&str="Hello"; //type, typeannotation, value changed
    println!("Final value of W ={w}");
}

