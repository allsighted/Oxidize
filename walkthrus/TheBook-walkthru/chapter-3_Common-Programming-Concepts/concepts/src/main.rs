// Rust is a statically typed language, which means that it must know the types of all variables at compile time

fn main() {
    let x = 5;

    // Shadow variable (over-shadows the previous `x`)
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // we can change the type of the value but reuse the same name
    let spaces = "   ";
    let _spaces = spaces.len();

    /* Error: expected `&str`, found `usize`
    let mut spaces = "   ";
    spaces = spaces.len(); 
    */ 

    // Conversion using `parse` 
    let guess: u32 = "42".parse().expect("Not a number!");
    assert_eq!(42, guess);
    let turbofish = "42".parse::<u32>().unwrap();
    assert_eq!(42, turbofish);

    /*
    Table 3-1: Integer Types in Rust:
        Length	    Signed	    Unsigned
        8-bit	    i8	        u8
        16-bit	    i16	        u16
        32-bit	    i32	        u32
        64-bit	    i64	        u64
        128-bit	    i128	    u128
        arch	    isize	    usize
     */

    /*
    Table 3-2: Integer Literals in Rust
        Number literals	    Example
        Decimal	            98_222
        Hex	                0xff
        Octal	            0o77
        Binary	            0b1111_0000
        Byte (u8 only)	    b'A'
     */

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // Operators and Symbols: https://doc.rust-lang.org/stable/book/appendix-02-operators.html
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    // remainder
    let remainder = 43 % 5;

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // Unlike a tuple, every element of an array must have the same type
    // arrays in Rust have a fixed length.
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // initialize an array to contain the same value for each element 
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    // When accessing a non-existing index Rust protects you against this kind of error by immediately exiting instead of allowing the memory access and continuing

    another_function(4);
    print_labeled_measurement(5, 'h');

    // C and Ruby, where the assignment returns the value of the assignment. In those languages, you can write x = y = 6 and have both x and y have the value 6; 
    // that is not the case in Rust.
    // let x = (let y = 6);

    // Calling a macro is an expression. 
    // A new scope block created with curly brackets is an expression, 
    // for example:
    let y = {
        let x = 5;
        let x = five();
        let x = xp_up_1(five());
        // Expressions do not include ending semicolons.
        x + 1
    };

    println!("The value of y is: {y}");

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // This won’t work because variables must have a single type, 
    // and Rust needs to know at compile time what type the number variable is, definitively
    // let condition = true;
    // let number = if condition { 5 } else { "six" };
    // println!("The value of number is: {number}");

    //  You can optionally specify a loop label on a loop that you can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn another_function(x: i32) {
    println!("The value of another_function's x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// We don’t name return values, 
// but we must declare their type after an arrow (->)
fn five() -> i32 {
    5
}

fn xp_up_1(x: i32) -> i32 {
    // x + 1
    return x + 1;
}
