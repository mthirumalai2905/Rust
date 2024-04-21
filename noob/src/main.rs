use std::io;


fn main() {
    //variables
    let  x   = 4;
    println!("x is : {}", x);
    //Variable shadowing
    {
        let x = x - 1;
        println!("x is: {}", x);
    }
    let x =  5;
    println!("x is: {}", x);

    //Constants
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{}", SECONDS_IN_MINUTE);

    //Integer
    let x : i32 = 2;
    println!("{}", x);

    //Float
    let floating_point = 10.95;
    println!("{}",floating_point);
    
    //Boolean
    let true_or_false: bool = true;

    println!("{}", true_or_false);

    //Tuples
    let mut tup: (i32, bool, char) = (1,true,'s');
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);

    tup.0 = 20;
    println!("{}", tup.0);

    //Arrays
    let mut arr = [1,2,3,4,5];
    arr[4] = 50;

    println!("{}", arr[4]);
    println!("{}, {}, {}, {}", arr[4], arr[3], arr[2], arr[1]);


    //Inputs in Rust
    print!("Hello, World");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("{}", input);
}
