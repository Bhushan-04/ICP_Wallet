use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::{query, update};
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
struct TokenBalance {
    token: String,
    amount: u64,
}

#[derive(Default)]
struct Wallet {
    balances: HashMap<String, u64>,
}

static mut WALLET: Option<Wallet> = None;

#[update] 
fn init_wallet() {
    unsafe {
        WALLET = Some(Wallet {
            balances: HashMap::new(),
        });
        ic_cdk::println!("Wallet initialized");
    }
}

#[query]
fn get_wallet_status() -> String {
    unsafe {
        match &WALLET {
            Some(_wallet) => format!("Wallet initialized"),
            None => "Wallet not initialized".to_string(),
        }
    }
}

#[query]
fn get_balance(token: String) -> u64 {
    unsafe {
        WALLET
            .as_ref()
            .and_then(|wallet| wallet.balances.get(&token))
            .cloned()
            .unwrap_or(0)  // Default to 0 if not found
    }
}

#[update]
fn receive_tokens(token: String, amount: u64) -> String {
    unsafe {
        if let Some(wallet) = WALLET.as_mut() {
            let entry = wallet.balances.entry(token.clone()).or_insert(0);
            *entry += amount;
            return format!("Received {} tokens of {}", amount, token);
        }
    }
    "Wallet not initialized".to_string()
}


#[update]
fn send_tokens(token: String, amount: u64) -> String {
    unsafe {
        if let Some(wallet) = WALLET.as_mut() {
            let entry = wallet.balances.entry(token.clone()).or_insert(0);
            if *entry >= amount {
                *entry -= amount;
                return format!("Sent {} tokens of {}", amount, token);
            } else {
                return "Insufficient balance".to_string();
            }
        }
    }
    "Wallet not initialized".to_string()
}
