// // use std::io;

// use std::string;


// fn main() {
//     // //variables
//     // let  x   = 4;
//     // println!("x is : {}", x);
//     // //Variable shadowing
//     // {
//     //     let x = x - 1;
//     //     println!("x is: {}", x);
//     // }
//     // let x =  5;
//     // println!("x is: {}", x);

//     // //Constants
//     // const SECONDS_IN_MINUTE: u32 = 60;
//     // println!("{}", SECONDS_IN_MINUTE);

//     // //Integer
//     // let x : i32 = 2;
//     // println!("{}", x);

//     // //Float
//     // let floating_point = 10.95;
//     // println!("{}",floating_point);
    
//     // //Boolean
//     // let true_or_false: bool = true;

//     // println!("{}", true_or_false);

//     // //Tuples
//     // let mut tup: (i32, bool, char) = (1,true,'s');
//     // println!("{}", tup.0);
//     // println!("{}", tup.1);
//     // println!("{}", tup.2);

//     // tup.0 = 20;
//     // println!("{}", tup.0);

//     // //Arrays
//     // let mut arr = [1,2,3,4,5];
//     // arr[4] = 50;

//     // println!("{}", arr[4]);
//     // println!("{}, {}, {}, {}", arr[4], arr[3], arr[2], arr[1]);


//     // //Inputs in Rust
//     // // print!("Hello, World");
//     // // let mut input = String::new();
//     // // io::stdin().read_line(&mut input).expect("failed to read line");
//     // // println!("{}", input);


//     // //Arithmetic and type casting

//     // // let x: u8 = 9; // 0 - 255
//     // // let y: i8 = 10;// -128 - 127

//     // // let z = x + y;
//     // // println!("{}", z);

//     // let x = 15i32 as i64;//Type casting
//     // let y = 15i32 as i64;

//     // let z =  x / y;
//     // println!("{}", z);

//     // //Conditional Statements
//     // let cond = (2 as  f32) < 3.2;
//     // println!("{}", cond);

//     // let food = "vadapav";

//     // if food == "cookie"{
//     //     println!("I like cookies too !");
//     // } else {
//     //     println!("Great I Like vadapav too");
//     // }
//     // test_one();
//     // test_one();
//     // test_one();
//     // test_one();
//     // test_one();
//     // test_one();
//     // add_numbers(10, 90);

//     // let number = {
//     //     let x = 3;
//     //     x + 1
//     // };
//     // println!("{}", number);

//     // let result = add_numbers(2, 3);
//     // println!("{}", result);


//     let  arr = [1,2,3,4,5];
//     for element in arr  {
//         println!("Element {}", element);
//     }


//     // let x = 2;
//     let string = String::from("hello");

//     println!("The String is {}", string);
// }

// // fn test_one(){
// //     println!("Test has been called...");
// // }

// // fn add_numbers(x: i32, y : i32){
// //     println!("The sum is {}", x + y);
// // }

// // fn add_numbers(x: i32, y: i32) -> i32{
// //     return x + y;
// // }

// // Something which returns value in rust is called an expression

fn main(){
    for i in 0..5{
        println!("Iterations {}", i);
    }
}