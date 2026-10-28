// 6.3 Concise control flow with if let and let...else
//
// using if let means less typing, less indentation, and less boilerplate code
// but you use the exhaustive checking match enforces that ensures you don't forget
// to handle all cases

fn main() {
    // the following are identical
    let max_config = Some(3u8);

    match max_config {
        Some(max) => {
            println!("You entered a max config of {max}");
        }
        None => ()
    }

    if let Some(max) = max_config {
        println!("You entered a max config of {max}");
    }

    // the following are also identical (else)
    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    match coin {
        Coin::Quarter(us_state) => {
            println!("Your quarter\'s state is {us_state:?}");
        }
        _ => count += 1
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}
