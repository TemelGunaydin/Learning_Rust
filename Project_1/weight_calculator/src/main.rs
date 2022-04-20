//Enter your weight and learn your weight on Planet Mars.
//Date : 20.04.2022
//Author : Temel Gunaydin
use std::io; //Used for read input

fn main() { //famous main function

    println!("Enter your weight()kg");
    let mut input = String::new();//get input as string. This will be allocated in heap.input is a pointer to the String on the heap.

    io::stdin().read_line(&mut input).unwrap();//unwrap used for handling errors.
    let weight : f32 = input.trim().parse().unwrap();//if user enters whitespace, trim will remove them,parse will convert the string into f32.
    let planet_weight = weight_calculator(weight);
    println!("Weight on Planet X is {}kg",planet_weight);
}
fn weight_calculator(weight:f32) -> f32{
    (weight / 9.81) * 3.711
}
