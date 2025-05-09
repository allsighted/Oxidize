/*
    Collect:
    - Total Value of all coins
    - Number and Value of Each Coin Type
    - List States of Collected Quarters
    - List States of Un-Collected Quarters
*/

use std::collections::HashMap;
use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    
    // Generate a vector of 10 random coins
    let random_coins: Vec<Coin> = (0..10)
        .map(|_| random_coin(&mut rng))
        .collect();
    
    // Print the random coins
    // println!("Random coins:");
    // for (i, coin) in random_coins.iter().enumerate() {
    //     println!("{}: {:?}", i + 1, coin);
    // }

    let coin_tracker = sort_coins(&random_coins);

    println!("{}", coin_tracker);
}

fn sort_coins(coins: &Vec<Coin>) -> CoinTracker {
    let mut tracker = CoinTracker {
        total_value: 0,
        total_coins: coins.len() as u32,
        coin_type: HashMap::new(),
        collected_quarters_states: Vec::new(),
        uncollected_quarters_states: Vec::new(),
    };

    tracker.coin_type.insert(Coin::Penny, CoinType { amount: 0, value: 0 });
    tracker.coin_type.insert(Coin::Nickel, CoinType { amount: 0, value: 0 });
    tracker.coin_type.insert(Coin::Dime, CoinType { amount: 0, value: 0 });
    // Alabama is here just for easier tracking, it's not correct and just there for sanity sake
    let quarter_key = Coin::Quarter(UsState::Alabama); // Using Alabama as representative
    tracker.coin_type.insert(quarter_key, CoinType { amount: 0, value: 0 });

    for coin in coins {
        match coin {
            Coin::Penny => {
            if let Some(coin_data) = tracker.coin_type.get_mut(&Coin::Penny) {
                coin_data.amount += 1;
                coin_data.value += 1;
            }
            tracker.total_value += 1;            
            },
            Coin::Nickel => {
                if let Some(coin_data) = tracker.coin_type.get_mut(&Coin::Nickel) {
                    coin_data.amount += 1;
                    coin_data.value += 5;
                }
                tracker.total_value += 5; 
            },
            Coin::Dime => {
                if let Some(coin_data) = tracker.coin_type.get_mut(&Coin::Dime) {
                    coin_data.amount += 1;
                    coin_data.value += 10;
                }
                tracker.total_value += 10; 
            },
            Coin::Quarter(state) => {
                    // Update quarter stats (using our representative key)
                    if let Some(coin_data) = tracker.coin_type.get_mut(&Coin::Quarter(UsState::Alabama)) {
                        coin_data.amount += 1;
                        coin_data.value += 25;
                    }
                    tracker.total_value += 25;
                    
                    // Add the state to collected states
                    tracker.collected_quarters_states.push(*state);
            },
        }
    }

    // Determine uncollected states
    let all_states = [
        UsState::Alabama, UsState::Alaska, UsState::Arizona, UsState::Arkansas,
        UsState::California, UsState::Colorado, UsState::Connecticut, UsState::Delaware,
    ];
    
    // Find states that aren't in collected_quarters_states
    for state in all_states.iter() {
        if !tracker.collected_quarters_states.contains(state) {
            tracker.uncollected_quarters_states.push(*state);
        }
    }
    
    tracker
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CoinTracker {
    total_value: u32,
    total_coins: u32,
    coin_type: HashMap<Coin,CoinType>,
    collected_quarters_states: Vec<UsState>,
    uncollected_quarters_states: Vec<UsState>,
}

use std::fmt;

impl fmt::Display for CoinTracker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Total value: {} cents", self.total_value)?;
        writeln!(f, "Total coins: {}", self.total_coins)?;
        
        writeln!(f, "\nCoin Types:")?;
        for (coin, data) in &self.coin_type {
            writeln!(f, "{:?}: {} coins = {} cents", coin, data.amount, data.value)?;
        }
        
        writeln!(f, "\nCollected Quarter States:")?;
        if self.collected_quarters_states.is_empty() {
            writeln!(f, "No quarter states collected")?;
        } else {
            for state in &self.collected_quarters_states {
                writeln!(f, "{:?}", state)?;
            }
        }
        
        writeln!(f, "\nUncollected Quarter States:")?;
        if self.uncollected_quarters_states.is_empty() {
            writeln!(f, "All states collected!")?;
        } else {
            for state in &self.uncollected_quarters_states {
                writeln!(f, "{:?}", state)?;
            }
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CoinType {
    amount: u32,
    value: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    // You can add more as needed
}

// Function to generate a random coin
fn random_coin(rng: &mut impl Rng) -> Coin {
    // Generate a random number between 0 and 3 for the coin type
    match rng.random_range(0..4) {
        0 => Coin::Penny,
        1 => Coin::Nickel,
        2 => Coin::Dime,
        3 => {
            // For Quarter, also generate a random state
            let states = [
                UsState::Alabama,
                UsState::Alaska,
                UsState::Arizona,
                UsState::Arkansas,
                UsState::California,
                UsState::Colorado,
                UsState::Connecticut,
                UsState::Delaware,
                // Add more states as needed
            ];
            
            // Pick a random state
            let state = states[rng.random_range(0..states.len())];
            Coin::Quarter(state)
        },
        _ => unreachable!(), // This won't happen due to the range 0..4
    }
}