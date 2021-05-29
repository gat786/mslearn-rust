use std::collections::HashMap;

fn main() {

    // ARRAYS

    // Comma-separated list inside brackets
    let weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    // Print first weekday
    println!("First weekday is {}", weekdays[0]); 
    
    // Initialize an array of 512 elements where every element is a zero
    let byte_buffer = [0_u8; 512];
    // Print the 100th element in the buffer
    println!("Buffer element 100 is {}", byte_buffer[510]);


    // VECTORS

    let three_numbers = vec![1, 2, 3];
    // Print [1, 2, 3]
    println!("Initial vector: {:?}", three_numbers);  

    // The vec! macro also accepts the same syntax as the array constructor
    let ten_zeroes = vec![0; 10];
    // Print [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    println!("Ten zeroes: {:?}", ten_zeroes);
    // {:?} format parameter inside the println! used whenever we want to 
    // print something for debugging reasons. 
    // https://doc.rust-lang.org/std/fmt/ - for more detailed information

    // println!("Ten zeroes: {}", ten_zeroes[11]);
    // uncommenting ^^ line will cause panic and program to not run 
    // successfully
    // To prevent panics when using vector we can use vec::get which never
    // panics

    /*
        Vectors can also be created by using the Vec::new() method. 
        You can push values onto the end of a vector, which will grow the 
        vector as needed:
    */
    // Create an empty vector
    let mut new_v = Vec::new();

    // Push the number five into the vector
    new_v.push(5);
    // Push the number six into the vector       
    new_v.push(6);

    // And so on...
    new_v.push(7);
    new_v.push(8);

    // Print [5, 6, 7, 8]
    println!("Current vector is {:?}", new_v); 

    new_v.pop();

    println!("Current popped vector is {:?}", new_v); 


    // HASHMAPS
    let mut book_reviews: HashMap<String, String> = HashMap::new();
    
    // Add book reviews
    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );

    // Searching for an existing key returns the value associated to it
    println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);
    println!("Review for Tom: {}", book_reviews["Adventures of Huckleberry Finn"]);    
    println!("Review for Arthur: {}", book_reviews["The Adventures of Sherlock Holmes"]);

    // Remove an entry from the review list
    let sherlock = "The Adventures of Sherlock Holmes";
    assert_eq!(book_reviews.contains_key(sherlock), true);
    
    book_reviews.remove(sherlock);
    assert_eq!(book_reviews.contains_key(sherlock), false);
    
    // Verify review was removed
    if !book_reviews.contains_key("The Adventures of Sherlock Holmes") {
        println!("{} reviews found. No reviews found for The Adventures of Sherlock Holmes.", book_reviews.len());
    }
}
