//! # b58ify
//!
//! `b58ify` is used to perform an extremely simple conversion to convert 
//! alphanumeric strings into a b58 counterpart. This is useful for user 
//! provided seeds for Solana PDAs.

use once_cell::sync::OnceCell;
use std::collections::HashSet;
use std::iter::FromIterator;

static ALPHANUMSET: OnceCell<HashSet<char>> = OnceCell::new();

/// Try to b58ify a &str. This will only return an Ok on inputs that 
/// exclusively contain an alphanumeric characterset.
pub fn b58ify(input: &str) -> Result<String, ()> {
    ALPHANUMSET
        .get_or_init(|| { 
            HashSet::<char>::from_iter(
                "0123456789abcdefghijkmnpqrstuvwxyzABCDEFGHIJKMNPQRSTUVWXYZ".chars()
            ) 
        });
    let input_chars = input.chars();
    let mut mapped = String::with_capacity(input.len()); 
    unsafe {
    for c in input_chars {
        // .get_unchecked is unsafe, but we have guaranteed its initialization
        // earlier in the function by called get_or_init.
        if !ALPHANUMSET.get_unchecked().contains(&c){
            return Err(()) 
        }
        if c == 'O' {
            mapped.push('o')
        } else if c == 'l' {
            mapped.push('L')
        } else {
            mapped.push(c)
        }
    }
    };
    Ok(mapped)
}
