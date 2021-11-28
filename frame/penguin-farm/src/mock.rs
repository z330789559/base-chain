//! Mocks for the gradually-update module.

#![cfg(test)]

use super::*;
use frame_support::{construct_runtime, parameter_types};
use primitive::*;
use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup};

use crate as nft;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

pub type AccountId = u128;
pub type BlockNumber = u64;

impl frame_system::Config for Runtime {
    type Origin = Origin;
    type Index = u64;
    type BlockNumber = BlockNumber;
    type Call = Call;
    type Hash = H256;
    type Hashing = ::sp_runtime::traits::BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type BlockWeights = ();
    type BlockLength = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = ();
    type BaseCallFilter = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
}
parameter_types! {
    pub const InitTotalSupply: Balance= 10000000000 * DOLLARS;
    pub const InitSupplyPeriod: BlockNumber= 30 * 12 * 5 * DAYS;
    pub const MalePenguinEggLimit: Balance = 5000 * DOLLARS;
    pub const SmallYellowPenguinLockPeriod: BlockNumber = 14 * DAYS;
    pub const SmallYellowPenguinGrowPeriod: BlockNumber=30 * DAYS;
    pub const RedPenguinEggCountForIncubation:  Balance = 20 * DOLLARS;
    pub const YellowPenguinEggCountForIncubation: Balance =20 * DOLLARS;
    pub const MalePenguinLifeSpan: BlockNumber= 7 * DAYS;
    pub const ColdStroage: PalletId = PalletId(*b"par/cold");
    pub const ClassOwnerId:  PalletId = PalletId(*b"par/cwid");
    pub const IncubationId:  PalletId = PalletId(*b"par/incu");
    pub const TechnicalStashId:  PalletId = PalletId(*b"par/tech");
    pub const OperationStashId:  PalletId = PalletId(*b"par/oper");
    pub const MalePenguinEggRate: Permill = Permill::from_percent(2);
    pub const RedPenguinEggRate: Permill = Permill::from_percent(40);
    pub const YellowPenguinEggRate: Permill = Permill::from_percent(48);
    pub const TechnicalPenguinEggRate: Permill = Permill::from_percent(5);
    pub const OperationPenguinEggRate: Permill = Permill::from_percent(5);
    pub const BidCommissionRate:Permill = Permill::from_percent(3);
    pub const MalePenguinRate: Permill = Permill::from_percent(1);
    pub const PenguinProducePeriod: BlockNumber =20  ;
    pub const YellowPenguinDeadPeriod: BlockNumber =20  ;
    pub const IncubationLivePeriod: BlockNumber = 5 * DAYS;
    pub const BidMaxPeriod: BlockNumber = 3 * MINUTES;
}

impl penguin_farm::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type AssetId = u32;
    type ClassId = u32;
    type TokenId = u64;
    type InitTotalSupply = InitTotalSupply;
    type InitSupplyPeriod = InitSupplyPeriod;
    type ClassOwnerId = ClassOwnerId;
    type ColdStorageId = ColdStroage;
    type IncubationId = IncubationId;
    type TechnicalStashId = TechnicalStashId;
    type OperationStashId = OperationStashId;
    type TimeStamp = Timestamp;
    type MalePenguinEggLimit = MalePenguinEggLimit;
    type MalePenguinLifeSpan = MalePenguinLifeSpan;
    type MalePenguinEggRate = MalePenguinEggRate;
    type RedPenguinEggRate = RedPenguinEggRate;
    type YellowPenguinEggRate = YellowPenguinEggRate;
    type TechnicalPenguinEggRate = TechnicalPenguinEggRate;
    type OperationPenguinEggRate = OperationPenguinEggRate;
    type MalePenguinRate = MalePenguinRate;
    type PenguinProducePeriod = PenguinProducePeriod;
    type YellowPenguinDeadPeriod = YellowPenguinDeadPeriod;
    type SmallYellowPenguinLockPeriod = SmallYellowPenguinLockPeriod;
    type SmallYellowPenguinGrowPeriod = SmallYellowPenguinGrowPeriod;
    type IncubationLivePeriod = IncubationLivePeriod;
    type RedPenguinEggCountForIncubation = RedPenguinEggCountForIncubation;
    type YellowPenguinEggCountForIncubation = YellowPenguinEggCountForIncubation;
    type BidCommissionRate = BidCommissionRate;
    type BidMaxPeriod = BidMaxPeriod;
    type WeightInfo = ();
    type Randomness = RandomnessCollectiveFlip;
}

impl pallet_randomness_collective_flip::Config for Runtime {}

parameter_types! {
    pub const ExistentialDeposit: u128 = 500 * MILLICENTS;
    // For weight estimation, we assume that the most locks on an individual account will be 50.
    // This number may need to be adjusted in the future if this assumption no longer holds true.
    pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Runtime {
    type MaxLocks = MaxLocks;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
    #[cfg(feature = "aura")]
    type OnTimestampSet = Aura;
    #[cfg(feature = "manual-seal")]
    type OnTimestampSet = ();
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
         RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage} =2 ,
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent} =3,
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>} =4 ,
        Farm: penguin_farm::{Pallet, Call, Storage, Event<T>,Config<T>}=20,
    }
);

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CLASS_ID: <Runtime as Config>::ClassId = 0;
pub const CLASS_ID_NOT_EXIST: <Runtime as Config>::ClassId = 100;
pub const TOKEN_ID: <Runtime as Config>::TokenId = 0;
pub const TOKEN_ID_NOT_EXIST: <Runtime as Config>::TokenId = 100;

pub struct ExtBuilder;

impl Default for ExtBuilder {
    fn default() -> Self {
        ExtBuilder
    }
}

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let t = frame_system::GenesisConfig::default()
            .build_storage::<Runtime>()
            .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}
