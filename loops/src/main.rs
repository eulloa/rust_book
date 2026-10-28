fn main() {
    println!("Hello, world!");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result: {result}");

    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let slice = &a[1..3];

    for n in slice {
        println!("{n}");
    }

    let user = build_user(String::from("example"), String::from("foo@bar.com"));
    // println!("Hello {0}", {user.username});

    let user_two = User {
        email: String::from("this_is_my_email@gmail.com"),
        ..user
    };

    println!("User two email: {0}", {user_two.email});
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}


