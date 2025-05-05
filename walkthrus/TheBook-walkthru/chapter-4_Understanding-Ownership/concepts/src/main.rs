fn main() {
    let s: String = String::from("Hello all world!");

    println!("{}", first_word(&s));

    println!("{}", first_word_slices(&s));

    println!("{}", x_word(&s, 1));
    println!("{}", x_word(&s, 2));
    println!("{}", x_word(&s, 3));
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, item) in bytes.iter().enumerate() {
        if *item == b' ' {
            return i-1; // index of final letter in the first word
        }
    }

    s.len()-1
}

fn first_word_slices(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn x_word(s: &String, x: u32) -> &str {
    let bytes = s.as_bytes();

    let mut word_count: u32 = 0;
    let mut previous_word_index: usize = 0;

    for (i, item) in bytes.iter().enumerate() {
        // println!("word_count={}", word_count);

        if (*item == b' ' || i == bytes.len() - 1) && word_count+1 == x {
            // return &s[previous_word_index..=i];
            if *item == b' ' {
                // If the current character is a space, exclude it
                return &s[previous_word_index..i];
            } else {
                // If we're at the end of the string, include the last character
                return &s[previous_word_index..=i];
            }
        } else if *item == b' ' {
            previous_word_index = i+1;
            word_count+=1;
        } else {
            continue
        }
    }

    &s[..]
}
