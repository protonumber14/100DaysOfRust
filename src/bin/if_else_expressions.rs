
fn main (){
    //*If expression using let staement*/
    let x:i8=10;
    let x= if x==0{5} else {10};
    println!("{x}");
    //* If expression */
    if x%2==0{
        println!("true");
    }
    //*Else if */
    let x=7;
    if x<4{
        println!("Small");
    }else if x>9 {
        println!("Big");
    }else if x==0 {
        println!("zero");
    } else {
        println!(" 🦀");
    }

}