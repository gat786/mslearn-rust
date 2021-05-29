mod pattern_matching;

fn main() {
    // panic!("Farewell");
    // We can throw a panic when there is absolutely no way of recovering in 
    // the program.

    // let v = vec![0, 1, 2, 3];
    // println!("{}", v[6]); // this will cause a panic!

    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    // pick the first item:
    let first = fruits.get(0);
    println!("{:?}", first);

    // pick the third item:
    let third = fruits.get(2);
    println!("{:?}", third);

    // pick the 99th item, which is non-existent:
    let non_existent = fruits.get(99);
    println!("{:#?}", non_existent);

    pattern_matching::match_usage();
    pattern_matching::match_with_value();
    pattern_matching::if_let_expression();
}
