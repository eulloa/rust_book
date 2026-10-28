fn main() {
    println!("Hello, world!");
    another_function(5, 'F');

    let x = five();
    println!("x = {x}");
}

// functions that return expressions must not end in a semicolon
// those which do end in a semicolon turn them into statements
fn five() -> i32 {
    5
}

fn another_function(x: i8, y: char) {
    println!("another function! {x} - {y}");
}
