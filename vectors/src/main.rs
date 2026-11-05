// Vectors, a collection type data structure, allow you to store more than one value (of the same type)
// in a single data structure that puts all the values next to each other in memory.
// https://doc.rust-lang.org/std/vec/struct.Vec.html

fn main() {
    // type annotation required when Rust cannot infer the type of data to store
    let v: Vec<i32> = Vec::new();

    // A vector with initial values; does not require type annotation because
    // Rust infers the type
    let _v = vec![1, 2, 3];

    let mut numbers = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    for i in &numbers {
        println!("i: {i}");
    }

    // read elements from a vector
    // in this example, we don't need the borrowed read because
    // primitives are stored on the stack; so passing to foo works
    // even without them
    let my_cool_list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let my_cool_number: &i32 = &my_cool_list[2];

    println!("my_cool_number: {my_cool_number}");
    let sum = foo(my_cool_number + 100);


    let default_value = 100;
    let another_number: Option<&i32> = my_cool_list.get(100);
    match another_number {
        Some(n) => {
            println!("n: {n}");
            n
        },
        None => {
            println!("No value found, returning a default value");
            &default_value
        } 
    };

    println!("another_number: {0:?}", another_number);

    // indexing without matching on the Option can canuse panics
    // like the next few lines that index a non existant vector index
    let example_elements = [1, 2, 3, 4, 5];
    // let does_not_exist = &example_elements[100];
    let does_not_exist = example_elements.get(100);

    // you can also iterate over mutable references to each element in the vector
    // if you want to make changes to all elements
    // note that the use of the dereference operator (*) is necessary to get
    // the underlying value
    let mut another_vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for i in &mut another_vector {
        *i += 50
    }

    // iterating over a vector, mutably or immutably, is safe because of the borrow checker's rules
    // If you attempt to insert or remove items in the body of the for loops above the program
    // won't compile
    //
    //
    // Vectors can only store values of the same type but you can define things in an enum to get
    // around this
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(121),
        SpreadsheetCell::Float(1.23),
        SpreadsheetCell::Text(String::from("enums in vectors, yay!"))
    ];
}

fn foo(x: i32) {
    println!("my_cool_number (from foo): {x}");
}
