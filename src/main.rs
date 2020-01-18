extern crate mycalculator;
fn main() {
    let sum:i32= mycalculator::calculator::simple::add(30,40);
    println!("Sum of 30 and 40 is {}",sum);
}
