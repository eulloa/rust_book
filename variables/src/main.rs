fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 3;
        println!("Inner scope value of x: {x}");
    }

    println!("Outer scope value of x: {x}");
}
