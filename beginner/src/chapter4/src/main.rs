fn main(){
    let mut s = String::from("Hello");

    //Passing on the ownership to a function
    takes_ownership(s);

    //Ownership has been now lost, this line would cause a compilation error
    // println!("{}", s);

    let mut s = String::from("hello");

    //borrowing a references to the value
    let len = calculate_length(&s);

    println!("The length of '{}' is {}.",s,len);

    //mutable borrowing
    change(&mut s);

    println!("The modified string is: {}", s);

    //------------------------Reference----------------------
    let x = 5; // Define a variable x

    // Create a reference to x using the & operator
    let reference_to_x = x;

    //Print the value of x and the reference to x
    println!("Value of x: {}", x);
    println!("Reference to x: {}", reference_to_x);


    //------------------------Dereference--------------------
    let x = 5;

    //Creating a reference to x using the & operator
    let reference_to_x = &x;

    //Dereference the reference and assign the value to y
    let y = *reference_to_x;

    //print the value of x and y
    println!("Value of x: {}",x);
    println!("Value of y (dereferenced) : {}",y);

 let x = 10;

 let reference_to_x = &x;

 let pointer_to_x = *reference_to_x;

 print!("Value of x: {}", x);
 println!("Value pointed to by pointer_to_x : {}", pointer_to_x);

    println!("{}",x);
    println!("{}",pointer_to_x);

    //------------------------Shallow Copy v/s Lightweight pointer--------------------
    let original = 10;

    let shallow_copy = original;

    let reference_to_original = &original;


    println!("Original: {}", original);
    println!("Shallow Copy: {}", shallow_copy);
    println!("Data via reference: {}", reference_to_original);

    //-------------------------Slice-------------------------
    let numbers = [1,2,3,4,5];

    //Creating a slice of the entire array
    let slice = &numbers[..];
    println!("Whole array: {:?}", slice);

    //Creating a slice of a portion of the array
    let slice = &numbers[1..4];
    println!("Slice: {:?}", slice);

    //Passing a slice to a function
    print_slice(&slice);
    
    //Modifying elements in  a mutable slice
    let mut numbers = [1,2,3,4,5];
    let mut_slice = &mut numbers[1..4];
    for num in mut_slice.iter_mut(){
        *num *= 2;
    }
    println!("Modified array: {:?}", numbers);
}


fn print_slice(slice: &[i32]){
    println!("Printing slice: {:?}", slice);
}


fn takes_ownership(some_string: String){
    println!("Ownership taken, string: {}", some_string);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String){
    some_string.push_str(", world");
}