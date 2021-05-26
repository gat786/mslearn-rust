fn main() {
    // number and string(&str)
    let number: u32 = "42".parse().expect("Not a number!");
    let string = "string data";


    let x = 2.0;      // f64, default type
    let y: f32 = 3.0; // f32, via type annotation

     // Addition
     println!("1 + 2 = {}", 1u32 + 2);

     // Subtraction
     println!("1 - 2 = {}", 1i32 - 2);
     // ^ Try changing `1i32` to `1u32` to see why the type is important

     // Integer Division
     println!("9 / 2 = {}", 9u32 / 2);
 
     // Float Division
     println!("9 / 2 = {}", 9.0 / 2.0);
 
     // Multiplication
     println!("3 * 6 = {}", 3 * 6);

    // Think of String data as string data that can change as your program 
    // runs, while &str are immutable views into string data that don't change
    // as your program runs.
    // Create a String from a string literal
    let mut hello = String::from("Hello, ");  

    // Push a character into our String
    hello.push('w');
    
    // Push a string literal into our String       
    hello.push_str("orld!");
             
    println!("{}", hello);

    /*
        A tuple is a grouping of values of different types collected into one
        compound. They have fixed length, meaning that after they're declared,
        they can't grow or shrink in size. The type of a tuple is defined by 
        the sequence of each member's type.
    */
    let tuple = ("hello", 5i32, 'c');
    /*
        &'static str is the type of the first element.
        i32 is the type of the second element.
        char is the type of the third element.
    */
}
