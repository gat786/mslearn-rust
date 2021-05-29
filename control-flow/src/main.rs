mod loop_mod;

fn main() {
    if 1 == 2 {
        println!("whoops, mathematics broke");
    } else {
        println!("everything's fine!");
    }

    // Unlike in most languages, if blocks can also act as expressions. 
    // Remember that all branches must return the same type for our code to 
    // compile.
    // This feature reminds me of working of Kotlin

    let formal = true;
    let greeting = if formal {
        "Good evening."
    } else {
        "Hello, friend!"
    };
    println!("{}", greeting); // prints "Good evening."


    // using else and else if with if blocks
    let number = 6;
    
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    loop_mod::forLoop();
}
