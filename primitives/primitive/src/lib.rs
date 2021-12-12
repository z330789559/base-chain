#![cfg_attr(not(feature = "std"), no_std)]

pub type BlockNumber = u64;
pub type Balance = u128;
pub mod currency {
    use super::*;
    pub const DOLLARS: Balance = 1_000_000_000_000_000_000;
    pub const CENTS: Balance = DOLLARS / 100; // 10_000_000_000_000_000
    pub const MILLICENTS: Balance = CENTS / 1000; // 10_000_000_000_000
    pub const MICROCENTS: Balance = MILLICENTS / 1000; // 10_000_000_000
      pub const fn deposit(items: u32, bytes: u32) -> Balance {
	items as Balance * 15 * CENTS + (bytes as Balance) * 6 * CENTS
}
}

pub use currency::*;

pub const MILLISECS_PER_BLOCK: u64 = 6000;

pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;

#[cfg(ccmtest)]
pub const DAYS: BlockNumber = 3 * MINUTES;

#[cfg(not(ccmtest))]
pub const DAYS: BlockNumber = 24 * HOURS;


