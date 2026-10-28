fn main() {
    println!("Hello, world!");
    
    let y: i8 = 7;

    if y < 10 {
        println!("y is less than 10");
    } else {
        println!("y is more than 10");
    }

    let result: i8 = thing(false);

    println!("result: {result}");
}

fn thing(arg: bool) -> i8 {
    let x = if arg { 10 } else { 20 };

    x
}
