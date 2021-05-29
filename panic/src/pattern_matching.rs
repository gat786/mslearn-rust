/*
    Rust has an extremely powerful control flow operator called match, which
    you can use to compare a value against a series of patterns and then
    execute code based on which pattern matches.
*/

pub fn match_usage(){
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }

    /*
        In the preceding code, we iterate through the same indexes from our
        previous example (0, 2, and 99) and then use each one to retrieve a
        value from the fruits vector by using the fruits.get(index)
        expression.

        Because the fruits vector contains &str elements, we know that the
        result of this expression is of type Option<&str>. You then use a
        match expression against the Option value and define a course of
        action for each of its variants. Rust refers to those branches as
        match arms, and each arm can handle one possible outcome for the
        matched value.
    */
}

/*
    You can refine your match expression even further to act differently,
    depending on the values inside a Some variant. For example, you could
    stress the fact that coconuts are awesome by running the following:
*/

/**
 * Function which uses match keyword to match with type and a value as well
 */
pub fn match_with_value(){
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(&"coconut") => println!("Coconuts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }
}

/*
    Whenever you use the match expression, keep the following rules in mind:

    match arms are evaluated from top to bottom. Specific cases must be
    defined earlier than generic cases or they'll never be matched and
    evaluated.

    match arms must cover every possible value that the input type could have.
    You'll get a compiler error if you try to match against a non-exhaustive
    pattern list.
*/

pub fn if_let_expression(){
    let some_number: Option<u8> = Some(7);
    match some_number {
        Some(7) => println!("That's my lucky number!"),
        _ => {},
    }
    if let Some(7) = some_number {
        println!("That's my lucky number!");
    }
}