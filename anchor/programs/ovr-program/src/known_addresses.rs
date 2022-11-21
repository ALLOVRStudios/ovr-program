// Known Addresses - mainnet
// pub const ALLOVR_PROGRAM_ID: &'static str = "GN2p6yaiKZGvxBYFTHG9Zb3KrT3h4irM9hz6u2VbK4uD";
// pub const ALLOVR_MINT_ID: &'static str = "FPc9PiJcHUYRvoLSTdnEEGYWqABykcM1GP2NQZ5MTC5u";
// pub const ALLOVR_STATE_ID: &'static str = "2QEFXkpyYqkAzGQWwyugo6yu5xASPgPRqqvYHv6S7jXb";
// pub const ALLOVR_AOVR_TREASURY_ID: &'static str = "H8LxsnEnP3FNJJniouAv1TADmT4MPDRssv4oX2vku3Mp";

use anchor_lang::prelude::Pubkey;
use std::str::FromStr;

// Known Addresses - integration tests
// pub const ALLOVR_PROGRAM_ID: &'static str = "GN2p6yaiKZGvxBYFTHG9Zb3KrT3h4irM9hz6u2VbK4uD";
pub const ALLOVR_MINT_ID: &'static str = "CnZvzJDv69bCFaEes5rnxG3dpsiKtJyYxqL2PpDyawze";
pub const ALLOVR_STATE_ID: &'static str = "GUSMgYvBw1Lm2aQQijbwDqbXaoYbdvQ7Zs6aDDMZszHF";
pub const ALLOVR_AOVR_TREASURY_ID: &'static str = "FYCLzQeuDmBFvU8uXfHs5FLuvnBkwYHUrnbFr5XEbLpN";

pub struct KnownAddress {}

impl KnownAddress {
    pub fn allovr_state() -> Pubkey {
        Pubkey::from_str(ALLOVR_STATE_ID).unwrap()
    }

    pub fn allovr_mint() -> Pubkey {
        Pubkey::from_str(ALLOVR_MINT_ID).unwrap()
    }

    pub fn allovr_dao_aovr_treasury() -> Pubkey {
        Pubkey::from_str(ALLOVR_AOVR_TREASURY_ID).unwrap()
    }
}
