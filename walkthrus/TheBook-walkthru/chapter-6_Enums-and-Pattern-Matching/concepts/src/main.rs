fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // representing the same concept using just an enum is more concise: rather than an enum inside a struct, we can put data directly into each enum variant
    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));

    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));


    let m = Message::Write(String::from("hello"));
    m.call();

    // https://doc.rust-lang.org/stable/std/option/enum.Option.html
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    // ! the following would error due to Rust not knowing how to add types `i8` and `Option<i8>`
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;
    // * you have to convert an Option<T> to a T before you can perform T operations with it. Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is, you are required to explicitly handle the case when the value is null.
    // in order to use an Option<T> value, you want to have code that will handle each variant. 
    // * The match expression is a control flow construct that does just this when used with enums: it will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value.
    match absent_number {
        Some(num) => println!("Got a number: {}", num),
        None => println!("No number available"),
    }
    // Define a coin inside main
    let coin = Coin::Nickel;
    // Match on the coin inside main
    let value = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25,
    };
    // ! Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid. Especially in the case of Option<T>
    println!("The value of the coin is: {} cents", value);

    let coin = Coin::Quarter(UsState::Alabama);
    value_in_cents(coin);


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other),
        _ => reroll(),
    }

    let dice_roll = 11;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        1..=2 | 4..=6 | 8..=10 => {
            println!("Moving player in special range");
            move_player(dice_roll)
        },
        _ => reroll(),
    }

    let dice_roll = 10;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        n @ (1..=2 | 4..=6 | 8..=10) => {
            println!("Moving player {} spaces in special range", n);
            move_player(n)
        },
        _ => reroll(),
    }
    // Without the @ operator, you would have to repeat the value in your code:
    let dice_roll = 10;
    match dice_roll {
        1..=2 | 4..=6 | 8..=10 => {
            println!("You rolled {}, which is in a special range", dice_roll);
            move_player(dice_roll)
        },
        _ => reroll(),
    }

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // nothing else happens;We can express that by using the unit value (the empty tuple type)
    }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    // Instead, we could write this in a shorter way using if let.
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(ref state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    describe_state_quarter(coin);

}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        println!("{state:?} is pretty old, for America!");
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
// fn describe_state_quarter(coin: Coin) -> Option<String> {
//     let state = if let Coin::Quarter(state) = coin {
//         state
//     } else {
//         return None;
//     };

//     if state.existed_in(1900) {
//         println!("{state:?} is pretty old, for America!");
//         Some(format!("{state:?} is pretty old, for America!"))
//     } else {
//         Some(format!("{state:?} is relatively new."))
//     }
// }
// fn describe_state_quarter(coin: Coin) -> Option<String> {
//     if let Coin::Quarter(state) = coin {
//         if state.existed_in(1900) {
//             Some(format!("{state:?} is pretty old, for America!"))
//         } else {
//             Some(format!("{state:?} is relatively new."))
//         }
//     } else {
//         None
//     }
// }


fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {
    println!("too high... rerolling");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message { // we’re also able to define methods on enums
    fn call(&self) {
        println!("{:?}", &self);
    }
}

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr4 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
