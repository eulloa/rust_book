// 6.3 Concise control flow with if let and let...else

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
}
