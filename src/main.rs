pub mod calculator
{
    pub mod simple{
        pub fn add(num1:i32,num2:i32) ->i32{
            let result:i32 =num1+num2;
            return result;
        }
    }
}
fn main() {
    let sum:i32= calculator::simple::add(23,44);
    println!("Sum of 23 and 44 is {}",sum);
}
