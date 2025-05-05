use std::{env, error::Error, path::Path, sync::Arc};
use tokio::{fs::File, io::{AsyncBufReadExt, BufReader}, sync::{mpsc, Mutex}};
use sha1::{Sha1, Digest};

const SHA1_HEX_STRING_LENGTH: usize = 40;
const CONCURRENT_TASKS: usize = 100;  // Adjust based on your system

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        return Ok(());
    }

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }
    
    // Create a shared flag for when password is found
    let password_found = Arc::new(Mutex::new(None::<String>));
    
    // Create channels for distributing work
    let (tx, rx) = mpsc::channel::<String>(1000);
    let rx = Arc::new(Mutex::new(rx));
    
    // Spawn worker tasks
    let mut handles = Vec::with_capacity(CONCURRENT_TASKS);
    for _ in 0..CONCURRENT_TASKS {
        let password_found = Arc::clone(&password_found);
        let rx = Arc::clone(&rx);
        let hash_to_check = hash_to_crack.to_string();
        
        let handle = tokio::spawn(async move {
            loop {
                // Check if password already found by another worker
                if password_found.lock().await.is_some() {
                    break;
                }
                
                // Get next password to check
                let line = {
                    let mut rx_guard = rx.lock().await;
                    match rx_guard.try_recv() {
                        Ok(line) => line,
                        Err(_) => break, // Channel empty or closed
                    }
                };
                
                // Skip empty lines
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                
                // Check the hash
                let mut hasher = Sha1::new();
                hasher.update(trimmed.as_bytes());
                let result = format!("{:x}", hasher.finalize());
                
                if result == hash_to_check {
                    // Found the password!
                    let mut found = password_found.lock().await;
                    *found = Some(trimmed.to_string());
                    break;
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Read file and send lines to workers
    let file = File::open(Path::new(args[1].trim())).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    
    let mut count = 0;
    while let Some(line) = lines.next_line().await? {
        count += 1;
        
        // Check if password already found
        if password_found.lock().await.is_some() {
            break;
        }
        
        // Send line to a worker
        if let Err(_) = tx.send(line).await {
            break; // All workers have exited
        }
        
        // Periodically report progress
        if count % 100 == 0 {
            println!("Checked {} passwords so far...", count);
        }
    }
    
    // Drop the sender to signal no more passwords
    drop(tx);
    
    // Wait for all workers to finish
    for handle in handles {
        let _ = handle.await;
    }
    
    // Check the result
    let result = password_found.lock().await.clone();
    if let Some(password) = result {
        println!("Password found: {}", password);
    } else {
        println!("Password not found in wordlist. Checked {} passwords.", count);
    }
    
    Ok(())
}