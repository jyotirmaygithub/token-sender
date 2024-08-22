use candid::{Nat, Principal};
use ic_cdk::{export_candid, query, update};
use std::collections::HashMap;

// static -->> global variable
const OWNER_ID: &'static str = "3rott-asn2i-gpewt-g3av6-sg2w4-z5q4f-ex4gs-ybgbn-2blcx-b46lg-5ae";
const TOTAL_TOKENS: u64 = 10000;
const SYMBOL: &str = "JYOTI";
// HashMap to store the principal ID of users and the tokens they own
static mut LEDGER: Option<HashMap<Principal, Nat>> = None;

fn main() {
    // from text method of the principal return a object where object either consist a okk or err. 
    // then unwrap extract the actal pricipal from the object if result or object is okk otherwise it return an error. 
    let owner = Principal::from_text(OWNER_ID).unwrap();

    // Initialize the ledger
    unsafe {
        if LEDGER.is_none() {
            let mut ledger : HashMap<Principal,Nat>= HashMap::new();
            // Insert the owner as the first entry with all tokens
            ledger.insert(owner, Nat::from(TOTAL_TOKENS));
            LEDGER = Some(ledger);
        }
    }
}

// Function to get the number of tokens owned by a user
#[query]
fn get_user_balance(user_principal: Principal) -> Nat {
    main();
    unsafe {
        ic_cdk::println!("ledger hashmap = {:?}", LEDGER);
        match &LEDGER {
            Some(ledger) => {
                // Return the balance if the user exists in the ledger, otherwise return 0
                ledger
                    .get(&user_principal)
                    .cloned()
                    .unwrap_or(Nat::from(0u64))
            }
            None => Nat::from(0u64), // Return 0 if the ledger is not initialized
        }
    }
}

#[query]
fn get_token_name() -> String {
    SYMBOL.to_string()
}

export_candid!();
