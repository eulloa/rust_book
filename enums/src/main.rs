fn main() {
    let v4 = IpAddrKind::V4(127, 0, 0, 1);
    let v6 = IpAddrKind::V6(String::from("::1"));


    let greeting = String::from("a friendly greeting!");
    let message = Message::Write(String::from("another greeting!"));
    message.call(&greeting);

    let result = coin_to_dollars(Coin::Dollar);
    println!("result is {0}", result);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{0:?} {1:?} {2:?}", five, six, none);


    let result = operate_on_ten(10, Operand::Multiply);
    println!("result: {result}");

    let secret = 1;
    match secret {
        1 => {
            println!("You entered {0}", 1)
        }
        58 => {
            println!("You entered {0}", 58)
        },
        _ => println!("Unrecognized value entered")
    }
}

enum Operand {
    Add,
    Subtract,
    Multiply,
    Divide
}


fn operate_on_ten(x: i32, operand: Operand) -> i32 {
    match operand {
        Operand::Add => x + 10,
        Operand::Subtract => x - 10,
        Operand::Multiply => x * 10,
        Operand::Divide => x / 10
    }

    // usage of other
    // match operand {
    //     Operand::Add => x + 10,
    //     other => {
    //         println!("No match was found!");
    //         x
    //     }
    // }
}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

impl Message {
    fn call(&self, value: &String) {
        println!("{value}");
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

#[derive(Debug)]
enum UsState {
    Alabma,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    Half,
    Dollar
}

fn coin_to_dollars(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {0:?}!", state);
            25
        },
        Coin::Half => 50,
        Coin::Dollar => 100
    }
} 


// Option type
// used to encode the concept of a value being present or absent
//
// enum Option<T> {
//     None,
//     Some(T)
// }

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

