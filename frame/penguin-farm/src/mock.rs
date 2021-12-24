//! Mocks for the gradually-update module.

#![cfg(test)]

use super::*;
use frame_support::{construct_runtime, parameter_types, PalletId, ord_parameter_types};
use primitive::*;
use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup};
use frame_system::{self as system, EnsureOneOf, EnsureRoot,EnsureSignedBy};
use sp_runtime::{ Permill};

use crate as penguin_farm;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

pub type AccountId = u64;
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
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = ();
    type BaseCallFilter = frame_support::traits::Everything;
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

impl penguin_farm::Config  for Runtime {
    type Event = Event;
    type Call = Call;
    type PalletsOrigin = OriginCaller;
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
    // type MalePenguinRate = MalePenguinRate;
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
    type Schedule =Scheduler;
}

impl pallet_randomness_collective_flip::Config for Runtime {}

parameter_types! {
    pub const ExistentialDeposit: u128 = 500 * MILLICENTS;
    // For weight estimation, we assume that the most locks on an individual account will be 50.
    // This number may need to be adjusted in the future if this assumption no longer holds true.
    pub const MaxLocks: u32 = 50;

      // 佣金标识
	pub const CommissionStorage: PalletId = PalletId(*b"ccm/cosm");
	// 佣金率
	pub const CommissionRate: Permill =Permill::from_percent(10);
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
    type CommissionStorage = CommissionStorage;
    type CommissionRate = CommissionRate;
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

    //自己 加个
    type OnTimestampSet = ();
}

parameter_types! {
		pub MaximumSchedulerWeight: Weight = 100; //Perbill::from_percent(80) * BlockWeights::get().max_block;
		pub const MaxScheduledPerBlock: u32 = 10;
	}
ord_parameter_types! {
		pub const One: u64 = 1;
}

impl pallet_scheduler::Config for Runtime {
    type Event = Event;
    type Origin = Origin;
    type PalletsOrigin = OriginCaller;
    type Call = Call;
    type MaximumWeight = MaximumSchedulerWeight;
    type ScheduleOrigin = EnsureRoot<AccountId>;//EnsureOneOf<AccountId, EnsureRoot<AccountId>, EnsureSignedBy<One, AccountId>>; //EnsureRoot<AccountId>;
    type MaxScheduledPerBlock = MaxScheduledPerBlock;
    type WeightInfo = (); //pallet_scheduler::weights::SubstrateWeight<Runtime>;
}


parameter_types! {
	pub const AssetDeposit: u64 = 1;
	pub const ApprovalDeposit: u64 = 1;
	pub const StringLimit: u32 = 50;
	pub const MetadataDepositBase: u64 = 1;
	pub const MetadataDepositPerByte: u64 = 1;
}

impl pallet_assets::Config for Runtime {
    type Event = Event;
    type Balance = u128;
    type AssetId = u32;
    type Currency = Balances;
    type ForceOrigin = EnsureRoot<AccountId>;
    type AssetDeposit = AssetDeposit;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type ApprovalDeposit = ApprovalDeposit;
    type StringLimit = StringLimit;
    type Freezer = ();
    type Extra = ();
    type WeightInfo = ();//pallet_assets::weights::SubstrateWeight<Runtime>;
}

impl pallet_sudo::Config for Runtime {
    type Event = Event;
    type Call = Call;
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
        Scheduler: pallet_scheduler::{Pallet, Call, Storage, Event<T>}=13,
        Sudo: pallet_sudo::{Pallet, Call, Config<T>, Storage, Event<T>}=17,
        Farm: penguin_farm::{Pallet, Call, Storage, Event<T>,Config}=20,
        Assets: pallet_assets::{Pallet, Call, Storage,Event<T>}=21,
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
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Runtime>()
            .unwrap();

        /*pallet_balances::GenesisConfig::<Runtime> {
            balances: vec![(1, 10000), (2, 20000), (3, 30000)],
        }.assimilate_storage(&mut t)
            .unwrap();*/

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }

    // Build test environment by setting the root `key` for the Genesis.
    pub fn new_test_ext(self, root_key: u64) -> sp_io::TestExternalities {
        let mut stro = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
        pallet_sudo::GenesisConfig::<Runtime> { key: root_key }
            .assimilate_storage(&mut stro)
            .unwrap();
        stro.into()
    }
}
