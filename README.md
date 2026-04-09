# 🔧 Soroban Smart Contract — Auction Platform
<img width="1902" height="903" alt="image" src="https://github.com/user-attachments/assets/4f1d3e8d-bec5-4ba5-8b25-fbb9a6749f28" />

https://stellar.expert/explorer/testnet/contract/CCAIZLXS5JZ2VDHPVDMO7B3WNZPYWTB4KFMPJEO6HBZM2NJGPXAWUKSA
### Core Logic:

* Seller initializes auction
* Users place bids
* Highest bid tracked
* Auction can be finalized

```rust
#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, Symbol, symbol_short,
};

#[contract]
pub struct AuctionContract;

#[contracttype]
#[derive(Clone)]
pub struct Auction {
    pub seller: Address,
    pub highest_bidder: Address,
    pub highest_bid: i128,
    pub ended: bool,
}

const AUCTION_KEY: Symbol = symbol_short!("AUCTION");

#[contractimpl]
impl AuctionContract {

    // Initialize auction
    pub fn init(env: Env, seller: Address) {
        seller.require_auth();

        if env.storage().instance().has(&AUCTION_KEY) {
            panic!("Auction already exists");
        }

        let auction = Auction {
            seller: seller.clone(),
            highest_bidder: seller.clone(),
            highest_bid: 0,
            ended: false,
        };

        env.storage().instance().set(&AUCTION_KEY, &auction);
    }

    // Place a bid
    pub fn bid(env: Env, bidder: Address, amount: i128) {
        bidder.require_auth();

        let mut auction: Auction = env
            .storage()
            .instance()
            .get(&AUCTION_KEY)
            .unwrap();

        if auction.ended {
            panic!("Auction already ended");
        }

        if amount <= auction.highest_bid {
            panic!("Bid too low");
        }

        auction.highest_bid = amount;
        auction.highest_bidder = bidder;

        env.storage().instance().set(&AUCTION_KEY, &auction);
    }

    // Get auction details
    pub fn get_auction(env: Env) -> Auction {
        env.storage().instance().get(&AUCTION_KEY).unwrap()
    }

    // End auction
    pub fn end(env: Env, seller: Address) {
        seller.require_auth();

        let mut auction: Auction = env
            .storage()
            .instance()
            .get(&AUCTION_KEY)
            .unwrap();

        if seller != auction.seller {
            panic!("Only seller can end auction");
        }

        if auction.ended {
            panic!("Auction already ended");
        }

        auction.ended = true;

        env.storage().instance().set(&AUCTION_KEY, &auction);
    }
}
```

---

# 📄 README.md (Production-Ready)

---

## 🚀 Auction Platform Smart Contract (Soroban)

---

## 📌 Project Description

This project implements a decentralized **Auction Platform** using **Soroban**, the smart contract framework on the Stellar network.

The contract enables a trustless auction mechanism where users can place bids, and the highest bidder wins once the auction is finalized. All logic is executed on-chain, ensuring transparency, security, and fairness.

---

## ⚙️ What It Does

The smart contract allows:

* A seller to create an auction
* Participants to place bids
* Automatic tracking of the highest bid
* Secure enforcement of bidding rules
* Auction finalization by the seller

All auction data is stored on-chain, removing the need for intermediaries.

---

## ✨ Features

* 🔐 **Authentication enforced** using `require_auth()`
* 🏆 **Highest bid tracking** in real time
* 📦 **On-chain state management**
* ⛔ **Prevents invalid bids** (must exceed current highest)
* 🧾 **Transparent auction lifecycle**
* 🧩 **Extendable architecture** (multi-auction, tokens, time limits)

---

## 🔗 Deployed Smart Contract

**Contract Name:**
Auction Platform

**Contract Address:**
`Auction Platform`

*(Replace with actual deployed contract ID after deployment)*

---

## 🛠️ Tech Stack

* **Rust** (Smart contract language)
* **Soroban SDK**
* **Stellar Blockchain**

---

## ▶️ Contract Functions

### 1. Initialize Auction

```
init(seller: Address)
```

### 2. Place Bid

```
bid(bidder: Address, amount: i128)
```

### 3. Get Auction Details

```
get_auction()
```

### 4. End Auction

```
end(seller: Address)
```

---

## 📈 Future Improvements

This is a **baseline architecture**. To make it production-grade:

* ⏱️ Add time-based auction ending
* 💰 Integrate Stellar token transfers (real bidding funds)
* 🔁 Refund previous bidders automatically
* 📊 Support multiple auctions
* 📢 Emit events for frontend tracking
* 🛡️ Add error enums instead of `panic!`

---

## 👨‍💻 Author

Sipon Ghosh

---

# ⚠️ Critical Insight

Right now:

* No real money transfer → bids are just numbers
* No refund logic → unsafe for real use
* No time constraint → auction depends on manual ending

If you're serious about turning this into a **startup-grade product**, the next step is:
👉 token integration + escrow + time-lock logic

---
