fn main() {
    // rust can infer data types so no need to explicitly write type 
    // declarations unless it is necessary to do so.
    
    let a_number = 10;
    let a_boolean = true;
    // By Default every variable is const in rust. 

    println!("The number is {}.",a_number);
    println!("The boolean is {}.",a_boolean);

    let mut a_mut_number = 10;
    // To create mutable variables use mut keyword

    println!("The mutable number is {}.",a_mut_number);
    a_mut_number = 20;
    println!("The updated mutable number is {}.",a_mut_number);

    /* Below shadowing technique is shown to overwrite variables */

    // The first binding is created with the name "number"
    let number = 5;

    // A different binding shadows the name "number" 
    let number = number + 5; 

    // Again, another new binding is created
    let number = number * 2; 
    println!("The number is {}.", number);
}
