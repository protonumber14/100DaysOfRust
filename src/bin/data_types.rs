pub fn main() {
    //*STatic */
    //*int */
    let x: i8 = 64;
    //Length:*i8,u8,i16,u16,i32,u32,i64,u64,i128,u128,isize,usize */
    println!("x={x}");
    //* flaot*/
    let x: f32 = 27.89;
    //*length: f32,f64 */
    println!("x={x}");
    //*Boolean */
    let x: bool = true;
    println!("x={x}");
    let x = false;
    println!("x={x}");
    //*char */
    let x: char = 'c';
    println!("x={x}");
    let x = '🦀';
    println!("x={x}");

    // compund
    let tup: (i8, i64, f32) = (2, -20, 0.9); //*Initialze 1 */
    println!("Tuple={:?}", tup);
    let first_element = tup.0; //*acess */
    println!(
        "first element = {}\n Second element ={}\n  Third element ={}",
        first_element, tup.1, tup.2
    );
    // deconstruct tup
    let tup = (0.09, 250, -9); //*Initialize 2 */
    let (x, y, z) = tup;
    println!("{}\n{}\n{}", x, y, z);

    //array
    let arr = [5, 10, 20, 25]; //*initialse 1 */
    println!("array 1={:?}", arr);
    let arr = [10, 9]; //*Initialze 2 */
    println!("array 2={:#?}", arr); //*print prettier */
    let arr: [f64; 5] = [0.01, 0.02, 0.03, 0.04, 0.05]; //* INitialze 3 */
    print!("array 3={:?}", arr); //*pretty print */
}
