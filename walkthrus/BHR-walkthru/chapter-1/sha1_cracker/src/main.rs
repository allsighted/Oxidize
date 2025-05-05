use std::{
    env,
    fs::File,
    path::Path,
    error::Error
};
use std::io::{Read, self, BufRead, BufReader};
use sha1::Digest;

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect(); 
    // - https://doc.rust-lang.org/stable/std/env/fn.args.html
    // - https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.collect
    dbg!(&args); 
    // - https://doc.rust-lang.org/stable/std/macro.dbg.html

    if args.len() != 3 {
        println!("Usage:");
        // - https://doc.rust-lang.org/book/ch20-05-macros.html
        // - macros prevent [format string vulnerabilities](https://owasp.org/www-community/attacks/Format_string_attack)
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        return Ok(());
    }

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }

    let wordlist_loc = Path::new(args[1].trim());
    let mut wordlist_file = match File::open(wordlist_loc) {
        Err(_) => return Err("couldn't open wordlist".into()),
        Ok(wordlist_file) => wordlist_file,
    };

    // let mut wordlist = String::new();
    // match wordlist_file.read_to_string(&mut wordlist) {
    //     Err(_) => return Err("couldn't read wordlist".into()),
    //     Ok(bytes_read) => {
    //         let line_count = wordlist.lines().count();
    //         println!("Loaded {} passwords ({} bytes)", line_count, bytes_read);
    //     }    
    // }

    let reader = BufReader::new(wordlist_file);
    for line in reader.lines() {
        let line = line?.trim().to_string();
        if !line.is_empty() {
            println!("{}", line);
            // Your password processing code here
            if hash_to_crack == &hex::encode(sha1::Sha1::digest(line.as_bytes())) {
                println!("Password found: {}", &line);
                return Ok(());
            }
        }
    };

    println!("password not found in wordlist");

    Ok(())
}
