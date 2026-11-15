use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::path::Path;

fn main() {
    // explicit call to panic to end program execution
    // in this case, our program calls the macro
    // panic!("Oh no!");

   // intentionally causing a panic in other people's code 
   let nums = vec![1, 2, 3];
   // nums[99]; causes an error of course since it's an index out of bounds

   // match is a good utility for handling the error and ok case that File::Open returns
   // Result<T, E>
   let file_name = String::from("/Users/efrenulloa/Documents/rust/error_handling/src/todo.txt");
   let path = Path::new(&file_name);
   let file = File::open(path);

   let file_result = match file {
       Err(error) => match error.kind() {
           ErrorKind::NotFound => match File::create("hello.txt") {
               Ok(fc) => fc,
               Err(e) =>  panic!("Problem creating the file: {e:?}")
           },
           _ => {
               panic!("Problem reading the file");
           }
       },
       Ok(f) => f
   };

   // alternative, cleaner way to write the above using unwrap_or_else and closures
   let another_file_result = File::open(&file_name).unwrap_or_else(|error| {
       if error.kind() == ErrorKind::NotFound {
           File::create("hello.txt").unwrap_or_else(|error| {
               panic!("Problem creating the file: {error:?}");
           })
       } else {
           panic!("Problem reading this file: {error:?}");
       }
   });

   println!("file result: {file_result:?}");
   println!("another file result: {another_file_result:?}");

   // Shortcuts for Panic on Error
   // let file_not_found = File::open("does_not_exist.pdf")
   //     .unwrap();
   //
   //  let file_not_foudn = File::open("does_not_exist.pdf")
   //      .expect("Don\'t forget to include this! 🤷‍♂️");

   let username_from_file = read_username_from_file();
   println!("username from file: {username_from_file:?}");

   let username_from_file_alt = read_username_from_file_alt();
   println!("username from field alt: {username_from_file_alt:?}");

   let my_file = read_username_from_file_shortest(file_name);
}

// The ? Operator
// when placed after a Result<T, E> type, it allows you to propagate errors
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("username.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// alternative to function above
fn read_username_from_file_alt() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// aaaaand an even shorter, more ergonomic way of writing it
fn read_username_from_file_shortest(file: String) -> Result<String, io::Error> {
   fs::read_to_string(&file)
}
