// String implements the Display trait

fn main() {
    // creating a new, empty string
    let mut s = String::new();
    
    // creating a String from a string literal
    let data = "this... is the end";
    let data_stringified = data.to_string();

    // also works on a literal directly
    let more_data = "this is the begininning".to_string();

    // using String::from to create a String from a string literal
    let more_string_data = String::from("this is the middle");

    // Strings are UTF-8 encoded so we can include any properly encoded data in them
    // all of the strings below are valid
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Updating a String
    // just like the contents of a Vec<T>, strings can grow in size and their contents can change
    
    // appending with push_str or push
    // note that push_str takes a string slice because we don't want to necessarily take ownership
    // of the parameter
    let mut s = String::from("empty");
    s.push_str("thoughts");

    let mut my_cool_string = String::from("The pub");
    let my_cool_string_slice = "rocks";
    my_cool_string.push_str(my_cool_string_slice);
    println!("my_cool_string_slice: {my_cool_string_slice}");

    println!("Hello, world!");

    // Slicing Strings
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{s}");

    // Iterating over strings
    for c in hello.chars() {
        println!("{c}");
    }

    for b in hello.bytes() {
        println!("{b}");
    }
}
