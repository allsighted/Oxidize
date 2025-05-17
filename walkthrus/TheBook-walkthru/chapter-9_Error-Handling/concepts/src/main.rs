use std::fs::{self, File};
use std::io::{self, Read};
use std::io::ErrorKind;
fn main() {
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    //       > RUST_BACKTRACE=1 cargo run
    // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    //       > RUST_BACKTRACE=full cargo run
    // panic!("crash and burn");

    // * Recoverable Errors with Result
    // let greeting_file_result = File::open("hello.txt");
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening file: {error:?}"),
    // };

    // * Matching on Different Errors
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(mut file) => {
            // File exists, check its contents
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => {
                    if !contents.contains("Hello world!") {
                        // File doesn't contain expected content, overwrite it
                        match fs::write("hello.txt", "Hello world!") {
                            Ok(_) => match File::open("hello.txt") {
                                Ok(new_file) => new_file,
                                Err(e) => panic!("Couldn't reopen file after writing: {e:?}"),
                            },
                            Err(e) => panic!("Problem writing to file: {e:?}"),
                        }
                    } else {
                        file // Return the file as is
                    }
                },
                Err(e) => panic!("Problem reading the file: {e:?}"),
            }
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                // Create the file
                match File::create("hello.txt") {
                    Ok(_) => {
                        // Write to the newly created file
                        match fs::write("hello.txt", "Hello world!") {
                            Ok(_) => match File::open("hello.txt") {
                                Ok(file) => file,
                                Err(e) => panic!("Couldn't reopen file after writing: {e:?}"),
                            },
                            Err(e) => panic!("Problem writing the file: {e:?}"),
                        }
                    },
                    Err(e) => {
                        // Match on the error kind for the creation error
                        match e.kind() {
                            ErrorKind::PermissionDenied | ErrorKind::AlreadyExists => {
                                panic!("Permission denied or file already exists: {e:?}")
                            }
                            _ => panic!("Problem creating the file: {e:?}"),
                        }
                    }
                }
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };


    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
    // * Alternatives to Using match with Result<T, E>
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    // * Shortcuts for Panic on Error: unwrap and expect
    let greeting_file = File::open("hello.txt").unwrap();
    // * expect in the same way as unwrap: to return the file handle or call the panic! macro. The error message used by expect in its call to panic! will be the parameter that we pass to expect, rather than the default panic! message that unwrap uses
    // let greeting_file = File::open("hello2.txt").expect("hello2.txt should be included in this project");

    // * Propagating Errors
    let username = match read_username_from_file() {
        Ok(username) => username,
        Err(e) => {
            println!("Problem reading the username: {}", e);
            // return; // Or provide a default value, or handle it differently
            String::from("User") // Return an empty string as a fallback
        }
    };
    println!("Default username: {}", username);
    // * This pattern of propagating errors is so common in Rust that Rust provides the question mark operator ? to make this easier.

    // * A Shortcut for Propagating Errors: the ? Operator
    let username = match read_username_from_file2() {
        Ok(username) => username,
        Err(e) => {
            println!("Problem reading the username: {}", e);
            // return; // Or provide a default value, or handle it differently
            String::from("User") // Return an empty string as a fallback
        }
    };
    println!("Default username: {}", username);

    // * Chaining Method Calls After the ? Operator
    let username = match read_username_from_file3() {
        Ok(username) => username,
        Err(e) => {
            println!("Problem reading the username: {}", e);
            // return; // Or provide a default value, or handle it differently
            String::from("User") // Return an empty string as a fallback
        }
    };
    println!("Default username: {}", username);

    // * Using fs::read_to_string instead of opening and then reading the file
    let username = match read_username_from_file4() {
        Ok(username) => username,
        Err(e) => {
            println!("Problem reading the username: {}", e);
            // return; // Or provide a default value, or handle it differently
            String::from("User") // Return an empty string as a fallback
        }
    };
    println!("Default username: {}", username);

    // * Where The ? Operator Can Be Used
    // Listing 9-10: Attempting to use the ? in the main function that returns () won’t compile.
    // let greeting_file = File::open("hello.txt")?;
    // * To fix the error, you have two choices. One choice is to change the return type of your function to be compatible with the value you’re using the ? operator on as long as you have no restrictions preventing that. The other choice is to use a match or one of the Result<T, E> methods to handle the Result<T, E> in whatever way is appropriate.


}

// Listing 9-13: A Guess type that will only continue with values between 1 and 100
pub struct Guess {
    value: i32, // Private value
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 { // `value` getter function
        self.value
    }
}

// Listing 9-12: Changing main to return Result<(), E> allows the use of the ? operator on Result values.
// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;

//     Ok(())
// }

// Listing 9-11: Using the ? operator on an Option<T> value
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
// The error message also mentioned that ? can be used with Option<T> values as well. As with using ? on Result, you can only use ? on Option in a function that returns an Option. The behavior of the ? operator when called on an Option<T> is similar to its behavior when called on a Result<T, E>: if the value is None, the None will be returned early from the function at that point. If the value is Some, the value inside the Some is the resultant value of the expression, and the function continues

// Listing 9-9: Using fs::read_to_string instead of opening and then reading the file
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")
}

// Listing 9-8: Chaining method calls after the ? operator
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// * A Shortcut for Propagating Errors: the ? Operator
// Listing 9-7: A function that returns errors to the calling code using the ? operator
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("username.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
// Listing 9-7 shows an implementation of read_username_from_file2 that has the same functionality as in Listing 9-6, but this implementation uses the ? operator.
// or example, we could change the read_username_from_file2 function in Listing 9-7 to return a custom error type named OurError that we define. If we also define impl From<io::Error> for OurError to construct an instance of OurError from an io::Error, then the ? operator calls in the body of read_username_from_file2 will call from and convert the error types without needing to add any more code to the function.

// * Propagating Errors    
// * instead of handling the error within the function itself you can return the error to the calling code so that it can decide what to do. This is known as propagating the error and gives more control to the calling code
// Listing 9-6: A function that returns errors to the calling code using match
fn read_username_from_file() -> Result<String, io::Error> {
    // let username_file_result = File::open("username.txt");

    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    let mut username_file = match File::open("username.txt") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
