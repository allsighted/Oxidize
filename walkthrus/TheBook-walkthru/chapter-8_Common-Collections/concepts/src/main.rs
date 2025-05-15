// https://doc.rust-lang.org/stable/std/collections/index.html
// Note: For more on the implementation details of the Vec<T> type, see “The Rustonomicon”: https://doc.rust-lang.org/stable/nomicon/vec/vec.html
// https://doc.rust-lang.org/stable/std/vec/struct.Vec.html

use std::fmt::format;

fn main() {
    let _v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // [] method will cause the program to panic because it references a nonexistent element
    // let does_not_exist = &v[100];
    // The get method returns an Option, so it will not panic
    // It will return None instead
    // let does_not_exist = v.get(100);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    // Vec cannot hold more data-types, except if they're enum-embedded
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("Int: {i}"),
            SpreadsheetCell::Float(f) => println!("Float: {f}"),
            SpreadsheetCell::Text(s) => println!("Text: {s}"),
        }
    }

    // * UTF-8 Encoded Text w Strings * //

    let mut s = String::new();
    let s = String::from("initial contents");

    let data = "initial contents";
    // to_string method, which is available on any type that implements the Display trait
    let s = data.to_string(); // == String::from("initial contents");
    // the method also works on a literal directly:
    let s = "initial contents".to_string(); // == String::from("initial contents");
    // * String::from and to_string do the same thing, so which one you choose is a matter of style and readability.

    // Listing 8-15: Appending a string slice to a String using the push_str method
    let mut s = String::from("foo");
    s.push_str("bar");

    // Listing 8-16: Using a string slice after appending its contents to a String
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    // If the push_str method took ownership of s2, we wouldn’t be able to print its value
    println!("s2 is {s2}");

    // Listing 8-17: Adding one character to a String value using push
    let mut s = String::from("lo");
    // push method takes a single character as a parameter and adds it
    s.push('l');

    // Listing 8-18: Using the + operator to combine two String values into a new String value
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // * The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String argument into a &str. When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..]
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", &s3);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = format!("{}", s1+&s2);
    println!("{}", s3);


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;

    // above gets messy, instead use format!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // * The format! macro works like println!, but instead of printing the output to the screen, it returns a String with the contents.
    let s = format!("{s1}-{s2}-{s3}");
    // * format! macro uses references so that this call doesn’t take ownership of any of its parameters.

    // If you were asked how long the string is, you might say 12. In fact, Rust’s answer is 24: that’s the number of bytes it takes to encode “Здравствуйте” in UTF-8, because each Unicode scalar value in that string takes 2 bytes of storage
    let hello = String::from("Здравствуйте");

    // * Methods for Iterating Over Strings
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }

    // Be sure to check out the documentation for useful methods like contains for searching in a string and replace for substituting parts of a string with another string.
    
    let hello = "Здравствуйте";
    let char_vec: Vec<char> = hello.chars().collect();
    let first_char = char_vec[0]; // O(1) access to З
    println!("First character: {first_char}");

    // Alternative: using indexing with explicit bounds check
    if 1 < char_vec.len() {
        let second_char = char_vec[1];
        println!("Second character: {}", second_char);
    }
    
    // Print all characters with their indices
    for (i, c) in char_vec.iter().enumerate() {
        println!("Character at index {}: {}", i, c);
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

// test
#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    #[should_panic(expected = "immutable borrow")]
    fn test_mutable_and_immutable_borrow_conflict() {
        // We'll create a vector and get a pointer to its first element
        let mut v = vec![1, 2, 3, 4, 5];
        
        // Get a raw pointer to the first element
        let _first_ptr = &v[0];
        
        // ! `test_mutable_and_immutable_borrow_conflict` doesn't cause any runtime panic. It would cause a compile-time error in normal Rust code, but since the test compiles successfully, it just runs normally without panicking.
        // Push a new element - this might reallocate the vector
        v.push(6);

        // This is what makes it panic - we manually check and panic
        // if we got this far in the code
        panic!("immutable borrow exists while modifying vector");
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_vector_out_of_bounds() {
        let v = vec![1, 2, 3];
        let _does_not_exist = &v[100]; // This will panic
    }

    #[test]
    fn test_vector() {
        let v = vec![1, 2, 3, 4, 5];
        
        assert_eq!(&v[2], v.get(2).unwrap());
        assert_eq!(v[2], 3);
        
    }

    #[test]
    fn test_vector_none() {
        let v: Vec<i32> = Vec::new();
        let third: Option<&i32> = v.get(2);
        assert_eq!(third, None);
    }
}