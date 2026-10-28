fn main() {
    // first appraoch is solving with regular variables
    let width1: u32 = 30;
    let height1: u32= 50;

    println!("The area of the rectangle is {} - (regular variables)", area(width1, height1));

    // since the area function first takes 2 params and not one value with both, it's unclear if
    // the values are related and it's not as clear as it could be
    //

    // refactoring with tuples
    println!("The area of the rectangle is {} - (tuples)", tuple_area((width1, height1)));

    // in one way, with the tuple the program is better in that it groups width and height
    // together; But since tuples don't name their elements, we have to index into the parts of the
    // tuple which make the calculation less obvious. Essentially, this implementation does not
    // convey the meaning of the data in code.

    // refactoring with structs
    // structs help us label the data so it's easier to reason about 
    //
    let scale = 3;

    let rectangle = Rectangle {
        // width: dbg!(30 * scale),
        width: 30,
        height: 50
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    dbg!(&rectangle);

    println!("The area of the rectangle is {} - (structs)", struct_area(&rectangle));

    println!("{rectangle:#?}");

    // Methods
    println!("The area of the rectangle is {} - (methods); non zero width: {}", rectangle.area(), rectangle.width());
    println!("");
    println!("Can rect1 hold rect2? {}", rectangle.can_hold(&rect2));
    println!("{0}", rect2.width);
    println!("Can rect1 hold rect3? {}", rectangle.can_hold(&rect3));


    dbg!(Rectangle::square(10));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, second_rectangle: &Rectangle) -> bool {
        self.width * self.height > second_rectangle.width * second_rectangle.height
    }

    fn square(value: u32) -> Self {
        Self {
            width: value,
            height: value
        }
    }
}

fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

