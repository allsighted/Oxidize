use std::fs::{self, File};
use std::io::Read;
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
}
