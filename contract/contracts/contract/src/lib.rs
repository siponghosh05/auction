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