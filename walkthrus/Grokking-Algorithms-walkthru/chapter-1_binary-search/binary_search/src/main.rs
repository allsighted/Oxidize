/* Binary Search */
/*
    * Advantages:
        - O(log n) time complexity = 1 million items would take at most 20 comparisons
        - Well suited for static datasets where data doesn't change frequently, making the sorting cost amortized over many searches
    
    ! Disadvantages:
        - O(n log n) cost of sorting = Data must be sorted before binary search which is an upfront cost
        - Not suitrable for linked lists
        - Inefficient on small datasets
        - Data must support quick random access making it unsuitable for some types
        - Maintaining sort order or re-sorting entire dataset
*/
fn main() {
    let mut unique_names: Vec<&'static str> = vec![
        "Emma",
        "Liam",
        "Olivia",
        "Noah",
        "Ava",
        "William",
        "Sophia",
        "James",
        "Isabella",
        "Benjamin"
    ];

    unique_names.sort();

    println!("Hello, world!");

    let name = String::from("Noah");

    println!("Enum Found: {}", loop_search(&unique_names, &name).unwrap());
    let (index, tries) = binary_search_names(&unique_names, &name).unwrap();
    println!("Bin  Found: {} | tries: {}", index, tries);
}

fn binary_search_names<'a>(names: &'a Vec<&'static str>, name: &String) -> Option<(usize, i32)> {
    let mut low = 0;
    let mut high = names.len()-1;

    let mut tries = 0;

    while low <= high {
        tries+=1;

        let mid = (low+high)/2; // * could be improved if even numbers didn't cut below half
        let guess = names[mid];
        if guess == name.as_str() {
            return Some((mid, tries))
        }
        if guess > name {
            high = mid-1;
        } else {
            low = mid+1;
        }
    }
    None
}

fn loop_search<'a>(names: &'a Vec<&'static str>, name: &String) -> Option<usize> {
    for (i, n) in names.iter().enumerate() {
        // println!("{}: {}", i, n);
        if n == name {
            return Some(i)
        }
    }
    None
}


#[cfg(test)]
mod tests {
    use crate::{binary_search_names, loop_search};

    #[test]
    fn assert_binary_to_loop_search() {
        let mut unique_names: Vec<&'static str> = vec![
            "Emma",
            "Liam",
            "Olivia",
            "Noah",
            "Ava",
            "William",
            "Sophia",
            "James",
            "Isabella",
            "Benjamin"
        ];

        unique_names.sort();

        for name in &unique_names {
            let loop_index = loop_search(&unique_names, &name.to_string()).unwrap();
            let (binary_index, _) = binary_search_names(&unique_names, &name.to_string()).unwrap_or((0, 0));
            println!("Checking name: {}", name);
            println!("loop_index: {} | bin_index: {}", loop_index, binary_index);
            assert_eq!(loop_index, binary_index);
        }
    }
}
