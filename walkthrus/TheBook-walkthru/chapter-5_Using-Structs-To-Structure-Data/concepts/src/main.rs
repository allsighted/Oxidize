struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
// note: &str can be used to, but requires use lifetimes

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual; // can define field-less structs called 'unit-like structs'

fn main() {
    // It's not possible to just have certain field mutable. The whole variable is either mut or immut
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user2 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user3: User = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@mail.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user4 = User {
        username: String::from("user4name"),
        ..user3 // `..` specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
        // note that since we moved the email which is a string we can not use user3 anymore since we moved a string, if we also set the email, and only copied the values of active and sign_in_count then user3 would not be moved- just copied because of how Strings are handled. 
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Point(x, y, z) = origin;

    let subject = AlwaysEqual; // No need for curly brackets or parentheses!

    // Rectangle area calc implementations:
    // 1.
    // it’s not clear anywhere in our program that the parameters are related
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );

    // 2.
    // tuples don’t name their elements, so we have to index into the parts of the tuple, making our calculation less obvious.
    let rect = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect)
    );

    // 3.
    // Our function signature for area now says exactly what we mean: calculate the area of Rectangle, using its width and height fields. This conveys that the width and height are related to each other, and it gives descriptive names to the values rather than using the tuple index values of 0 and 1. This is a win for clarity.
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect)
    );

    dbg!(&rect);
    println!("rect = {rect:#?}");
    
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // doubles the width scale and returns it to width inside the Rectangle, therefore making width = 60
        height: 50,
    };
    dbg!(&rect1);
    let rect1 = dbg!(rect1); // dbg! consumes rect1 but it natively returns it back into the re-declared `let rect1` variable

    // println!("{rect1:?}");

    // when you call a method with object.something(), Rust automatically adds in &, &mut, or * so object matches the signature of the method. In other words, the following are the same: p1.distance(&p2); == (&p1).distance(&p2);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );


    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(4);
    println!("square={:#?}", &square);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// Each struct is allowed to have multiple impl blocks.
impl Rectangle {
    // Method
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
impl Rectangle {
    // Method
    fn width(&self) -> bool {
        self.width > 0
    }
    // Method
    fn can_hold(&self, foreign: &Rectangle) -> bool {
        self.width > foreign.width && self.height > foreign.height
    }
    // Associated function
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// As mentioned in Chapter 4, we want to borrow the struct rather than take ownership of it. This way, main retains its ownership and can continue using rect
fn area3(rectanlge: &Rectangle) -> u32 {
    rectanlge.height * rectanlge.width
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user_shorthand(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}
