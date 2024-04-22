fn main() {
//    let  x: i32 = 5;
//    println!("The value of x is: {}", x);
//    let  x = 6; (Variable Shadowing)
//    println!("The value of x is: {}", x);

//    const SUBSCRIBER_COUNT: u32 = 1000000;

// Data-Types

//Integer types
let x: i32 = 5;
println!("The value of x is: {}",x);

let y: u64 = 100;
println!("The value of y is: {}", y);

let f: f32 = 3.14;
println!("The value of f is: {}", f);

let pi: f64 = 3.14519256326;
println!("The value of pi is: {}", pi);

//Boolean Type
let is_rust_fun: bool = true;
println!("Is Rust fun? {}", is_rust_fun);

//Character Type
let c: char = 'A';
println!("the character is: {}", c);

//String type
let s: &str = "Hello Rust";
println!("The string is: {}", s);

//Array type
let arr: [i32; 5] = [1,2,3,4,5];
println!("Array: {:?}", arr);

//Tuple type
let tuple: (i32, f64, char) = (42, 3.14, 'A');// tuple containing an integer float and a character
let (channel,subcount, newer) = tuple;
println!("Tuple: {:?}", tuple);

//Slice type
let slice: &[i32] = &arr[1..3]; //Slice of the array
println!("Slice: {:?}", slice);

my_function();

let mut counter = 0;

let result = loop{
    counter += 1;
    println!("Again!");
    if counter == 10 {
        break counter;
    }
};

print!("The result is {}", result);

}


fn my_function(){
  println!("Another functions...");
}