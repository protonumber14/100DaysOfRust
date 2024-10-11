//*parameters and return type */

fn add(x:i32,y:f64)->f64{ //*x,y are parameters */
    x as f64+y
}

//*Return type */
fn str_print() -> &'static str { //* -> static str is return type  */
    "Return String in str_print function" //expression 
}


fn main() {
    let x = str_print(); //* Storing return value of another function into x */
    println!("{x}");
    let x=add(20,57.9); //*Arguements of x,y */
    println!("add function ={x}");
}
