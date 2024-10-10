pub fn main(){
// compund
let tup:(i8,i64,f32)=(2,-20,0.9); //*Initialze 1 */
//acess
println!("Tuple={:?}",tup);
let first_element=tup.0;
println!("first element = {}\n Second element ={}\n  Third element ={}",first_element, tup.1,tup.2 );
// deconstruct tup
let tup=(0.09,250,-9); //*Initialize 2 */
let (x,y,z)=tup;
println!("{}\n{}\n{}",x,y,z);

//array
let arr=[5,10,20,25]; //*initialse 1 */
println!("array 1={:?}",arr);
let arr=[10,9]; //*Initialze 2 */
println!("array 2={:#?}",arr); //*print prettier */
let arr:[f64;5]=[0.01,0.02,0.03,0.04,0.05]; //* INitialze 3 */
println!("array 3={:?}",arr);

}