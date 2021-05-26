fn main() {
    /*
        A struct is a type that's composed of other types. Like tuples, the
        pieces of a struct can be different types. But in a struct, you can
        name each piece of data so it's clear what the values mean.

        Structs in Rust come in three flavors: classic structs, tuple structs,
        and unit structs.

        Classic C structs are the most commonly used. Each field defined within
        them has a name and a type. After they're defined, they can be accessed
        by using example_struct.field syntax.
        Tuple structs are similar to classic structs, but their fields have no
        names. For accessing individual variables, the same syntax is used as
        with regular tuples, namely, foo.0, foo.1, and so on, starting at zero.
        Unit structs are most commonly used as markers. We'll learn more about
        why these structs can be useful when we learn about Rust's traits
        feature.
    */
    // A struct with named fields
    struct Person {
        name: String,
        age: u8,
        likes_oranges: bool,
    }

    // A tuple struct
    struct Point2D(u32, u32);

    // A unit struct
    struct Unit;

    // Instantiate a classic struct, with named fields. Order does not matter.
    let person = Person {
        name: String::from("Adam"),
        likes_oranges: true,
        age: 25,
    };

    // Instantiate a tuple struct by passing the values in the same order as defined.
    let origin = Point2D(0, 0);

    // Instantiate a unit struct.
    let unit = Unit;

    // Display the details of the person
    if person.likes_oranges {
        println!("{:?} is {:?} and likes oranges.", person.name, person.age);
    } else {
        println!(
            "{:?} is {:?} and doesn't like oranges.",
            person.name, person.age
        );
    }

    /*
        Enums are types that can be any one of several variants.

        What Rust calls enums are more commonly known as algebraic data types.
        The important detail is that each enum variant can have data to go
        along with it.

        The enum keyword allows the creation of a type, which might be one of
        a few different variants. Enum variants, just like structs, can have
        fields with names, fields without names, or no fields at all.

        In the following example, we define an enum to classify a web event.
        Each variant is independent and stores different amounts and types of
        values.
    */
    enum WebEvent {
        // An enum can be unit-like
        PageLoad,
        PageUnload,
    
        // Or it can include characters and strings
        KeyPress(char),
        Paste(String),

        // Custom structs
        KeyPressEvent(KeyPressEvent),

        // Or it can include tuple structs
        Click{x: i64, y: i64}
    }

    struct KeyPressEvent(char);
}
