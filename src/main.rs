mod lib;
fn main() {
    let sum:i32= lib::calculator::simple::add(18,32);
    println!("Sum of 18 and 32 is {}",sum);
}
