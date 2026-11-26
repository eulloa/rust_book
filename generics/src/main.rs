fn main() {
    // the following code to find the largest number in a list
    // is duplicated; we can fix that with generics; 
    let nums = vec![1, 30, 103, 193, -1, 2];
    let mut largest = &nums[0];


    for n in &nums {
        if n > largest {
            largest = n
        }
    }

    println!("largest: {largest:?}");

    let nums = vec![1034, 10, 3, 1, 3300, 10, 1003, 99, 999];
    let mut largest = &nums[0];

    for n in &nums {
        if n > largest {
            largest = n;
        }
    }

    println!("largest: {largest:?}");

    let the_largest = find_largest_in_list(&nums);
    println!("the_largets: {the_largest:?}");

    let largest_val = largest_generics(&nums);
    println!("largest_val: {largest_val:?}");

    // examples of Point<T>
    let integer = Point {
        x: 5, y: 20
    };

    // calling a method on that struct
    let x_value = integer.x();
    println!("integer.x(): {x_value}");

    let float = Point {
        x: 5.6, y: 3.3
    };

    // example of PointTwo<T, U>
    let mix = PointTwo {
        x: 1,
        y: true 
    };
}

// ...but first, let's create a function that does this for us
fn find_largest_in_list(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for n in list {
        if n > largest {
            largest = n;
        }
    }

    largest
}

//... then we'll solve it using generics
//
//We use Generics to create definitions for items like function signatures or structs,
//which we can then use with many different conceret types 

// Function Definitions - we use the PartialOrd trait to constrain the generic type to 
// only those that impelement PartialOrd (their values can be ordered); the standard library
// implements PartialOrd on int32 and char
//
fn largest_generics<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];

    for n in list {
        if n > max {
            max = n;
        }
    }

    max
}

// Struct Definitions
// the following says that this struct is generic over some type T,
// and that x and y are both that same type
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

#[derive(Debug)]
struct PointTwo<T, U> {
    x: T,
    y: U
}

// Enum Definitions
/*
enum Option<T> {
    Some(T),
    None
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}
*/

// Method Definitions
// here, we define a method called x on the Point struct
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
