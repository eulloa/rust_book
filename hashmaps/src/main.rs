// HashMap https://doc.rust-lang.org/std/collections/struct.HashMap.html
// Stored on the Heap
//
use std::collections::HashMap;

fn main() {
    let mut data = HashMap::new();

    let blue_team_name = String::from("Blue");
    let blue_team_value = 10;

    let yellow_team_name = String::from("Yellow");
    let yellow_team_value = 20;

    data.insert(&blue_team_name, blue_team_value);
    data.insert(&yellow_team_name, yellow_team_value);

    let score = data.get(&yellow_team_name);
    match score {
        None => println!("Could not find the score with the provided key of {blue_team_name}"),
        Some(s) => println!("Team score: {s}"),
    }

    // alternatively, you could do...
    // copied makes it Option<i32> and not Option<&i32>
    // unwrap_or sets score to 0 if no key is found
    let score = data.get(&String::from("Orange")).copied().unwrap_or(-1);
    println!("score: {score}");

    // iterating through each key/value pair in the HashMap 
    for (key, value) in &data {
        println!("{key}: {value}");
    }

    // Managing ownership in Hash Maps
    // For types that implement the Copy trait like i32 will be copied into the hash map.
    // Owned types, like String, will be moved and the hash map will be the owner of those values
    // Demonstration below
    let field_name = String::from("my cool field");
    let field_value = String::from("my cool value");

    let mut values = HashMap::new();
    values.insert(field_name, field_value);
    // after line #43, field_name and field_value are invalid! referencing them creates an error
    
    // Updating a hash map
    //
    // Overwriting a value; the key is the same so the value gets overidden
    let mut my_map = HashMap::new();
    my_map.insert(String::from("key"), String::from("value"));
    my_map.insert(String::from("key"), String::from("value_two"));
    println!("{my_map:?}");

    // Adding a key and value only if the key is not present
    let mut players = HashMap::new();
    players.insert(String::from("Jordan"), 23);
    players.entry(String::from("Jordan")).or_insert(-1);
    players.entry(String::from("Pippen")).or_insert(33);
    println!("{players:?}");

    // Updating a value based on the old value
    let sentence = "there is something going on here today that is extremely strange, that is one thing that never gets old, old brethren are fantastic and only fantastic";
    let mut map = HashMap::new();

    for word in sentence.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

    let stats_result = stats(vec![100, 1, 3, 29, 3, 2, 2, 3, 3, 3, 1]);
    println!("{stats_result:?}");
}

fn stats(mut list: Vec<i32>) -> Vec<i32> {

    let mut values:HashMap<i32, i32> = HashMap::new();

    list.sort();
    let middle = (list.len() - 1) / 2;

    // insert values into the map, tracking number of times a number
    // appears in the list parameter
    for n in &list {
        let count = values.entry(*n).or_insert(0);
        *count += 1;
    }

    println!("{values:?}");

    // find largest value in the values hashmap
    let mut mode = 0;
    for (key, value) in values {
        if value > mode {
            mode = key;
        }
    }

    let middle_value = &list[middle];

    return vec![*middle_value, mode]; 
}

