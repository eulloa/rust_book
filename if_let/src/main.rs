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
        Coin::Quarter(ref us_state) => {
            println!("Your quarter\'s state is {us_state:?}");
        }
        _ => count += 1
    }

    if let Coin::Quarter(us_state) = coin {
        println!("Your quarter\'s state is {us_state:?}");
    } else {
        count += 1
    }
}

// Staying on the "Happy Path" with let...else
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old for America!"))
    } else {
        Some(format!("{state:?} is relatively new"))
    }
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // skipped for brevity
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}
