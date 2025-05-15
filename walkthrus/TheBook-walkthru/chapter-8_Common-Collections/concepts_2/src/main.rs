/*
    The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function, which determines how it places these keys and values into memory.

    Resources:
     - https://en.wikipedia.org/wiki/SipHash
*/

use std::collections::HashMap;
fn main() {
    // Listing 8-20: Creating a new hash map and inserting some keys and values
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{scores:?}");

    // * hash maps are homogeneous: all of the keys must have the same type, and all of the values must have the same type.

    // Listing 8-21: Accessing the score for the Blue team stored in the hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("scores: {scores:?} | score: {score}");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Listing 8-22: Showing that keys and values are owned by the hash map once they’re inserted
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // value borrowed here after move
    // println!("{}, {}", field_name, field_value);
    //                    ^^^^^^^^^^  ^^^^^^^^^^^ value borrowed here after move

    // Listing 8-23: Replacing a value stored with a particular key
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{scores:?}");

    // Adding a Key and Value Only If a Key Isn’t Present
    // Listing 8-24: Using the entry method to only insert if the key does not already have a value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{scores:?}");

    // Updating a Value Based on the Old Value
    // Listing 8-25: Counting occurrences of words using a hash map that stores words and counts
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // / count is &mut i32
        // Here, we store that mutable reference in the count variable, so in order to assign to that value, we must first dereference count using the asterisk (*)
        *count += 1; // explicitly modify the i32 value
        /*
            count is like an arrow pointing to a box containing a number
            \*count is the actual number inside the box
            \*count += 1 means "go to the box that count points to, and add 1 to the number inside"
        */
    }
    // * recall from the “Accessing Values in a Hash Map” section that iterating over a hash map happens in an arbitrary order.
    println!("{map:?}");

}
