// Find all our documentation at https://docs.near.org
use near_sdk::json_types::U64;
use near_sdk::{env, near, AccountId, NearToken, PanicOnDefault, Promise};

#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct Bid {
    pub bidder: AccountId,
    pub bid: NearToken,
}

// Define the contract structure
#[derive(PanicOnDefault)]
#[near(contract_state)]
pub struct Contract {
    highest_bid: Bid,
    auction_end_time: U64,
}

// Implement the contract structure
#[near]
impl Contract {
    #[init]
    #[private] // only callable by the contract's account
    pub fn init(end_time: U64) -> Self {
        Self {
            highest_bid: Bid {
                bidder: env::current_account_id(),
                bid: NearToken::from_yoctonear(1),
            },
            auction_end_time: end_time,
        }
    }

    #[payable]
    pub fn bid(&mut self) -> Promise {
        // Assert the auction is still ongoing
        assert!(
            env::block_timestamp() < self.auction_end_time.into(),
            "Auction has ended"
        );

        // current bid
        let bidder = env::predecessor_account_id();
        let bid = env::attached_deposit();

        // last bid
        let Bid {
            bidder: last_bidder,
            bid: last_bid,
        } = self.highest_bid.clone();

        // Assert the new bid is higher than the last bid
        assert!(bid > last_bid, "Bid must be higher than the last bid");

        // update the highest bid
        self.highest_bid = Bid { bidder, bid };

        // refund the last bidder
        Promise::new(last_bidder).transfer(last_bid)
    }

    pub fn get_highest_bid(&self) -> Bid {
        self.highest_bid.clone()
    }

    pub fn get_auction_end_time(&self) -> U64 {
        self.auction_end_time
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_contract() {
        let contract = Contract::init(U64::from(1000));

        let default_bid = contract.get_highest_bid();
        assert_eq!(default_bid.bidder, env::current_account_id());
        assert_eq!(default_bid.bid, NearToken::from_yoctonear(1));

        let end_time = contract.get_auction_end_time();
        assert_eq!(end_time, U64::from(1000));
    }
}
