/*
    * Generics allow us to replace specific types with a placeholder that represents multiple types to remove code duplication.
*/

fn main() {
    // * Generic Data Types
    // * In Function Definitions
    // For example, say we had two functions: one that finds the largest item in a slice of i32 values and one that finds the largest item in a slice of char values. How would we eliminate that duplication? Let’s find out!
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {result}");
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // * Note that because we’ve used only one generic type to define Point<T>, this definition says that the Point<T> struct is generic over some type T, and the fields x and y are both that same type, whatever that type may be.
    // The following line would not work because the x and y values are of different types:
    // Listing 10-7: The fields x and y must be the same type because both have the same generic data type T.
    // let wont_work = Point { x: 5, y: 4.0 };
// !                                    ^^^ expected integer, found floating-point number

    // Listing 10-8: A Point<T, U> generic over two types so that x and y can be values of different types
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };
}

// * In Struct Definitions
// Listing 10-8: A Point<T, U> generic over two types so that x and y can be values of different types
struct Point2<T, U> {
    x: T,
    y: U,
}

// Listing 10-6: A Point<T> struct that holds x and y values of type T
struct Point<T> {
    x: T,
    y: T,
}

// * In Function Definitions
// Listing 10-5: The largest function using generic type parameters
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T { // Note that `T` can be any word, but it’s common to use `T` for type
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Listing 10-4: Two functions that differ only in their names and in the types in their signatures
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
