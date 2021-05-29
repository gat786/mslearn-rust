pub fn forevorLoop(){
    loop {
        println!("I loop forever");
    }
}

pub fn breakLoop(){
    let mut i = 1;
    let something = loop {
        i *= 2;
        if i > 100 {
            break i;
        }
    };
    assert_eq!(something, 128);
    // Every break in a loop must have the same type. When it's not explicitly
    // giving something, break; returns () (an empty tuple).
}

pub fn whileLoop(){
    /*
        A while expression loops until a predicate is false.

        A while loop begins by evaluating the boolean loop conditional 
        expression. If the loop conditional expression evaluates to true, the
        loop body block executes. Control then returns to the loop conditional
        expression. If the loop conditional expression evaluates to false, the
        while expression completes.

        The following code loops until the predicate evaluates to true:
    */

    let mut counter = 0;
    
    while counter < 10 {
        println!("hello");
        counter = counter + 1;
    }
}

pub fn forLoop(){
    // For Loops can be used to iterate through a list
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    // You can also use ranges to work with for loop

    let multiple_of = 7;
    for a in 1..11{
        println!("7 x {} \t= {}",a, a * multiple_of);
    }
}