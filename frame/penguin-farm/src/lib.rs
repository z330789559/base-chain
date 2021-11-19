#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use codec::{Decode, Encode};
use frame_support::{
    pallet_prelude::*,
    traits::{Currency, ReservableCurrency},
    Parameter,
};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{
    traits::{
         MaybeSerializeDeserialize, Member
    },
     RuntimeDebug,
};
use sp_std::vec::Vec;

mod mock;
mod tests;
use frame_support::scale_info::TypeInfo;
use frame_support::sp_runtime::traits::AccountIdConversion;

macro_rules! s {
    ($e: expr) => {
        sp_runtime::SaturatedConversion::saturated_into($e)
    };
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum PenguinFarm {
    RedPenguin,
    YellowPenguin,
    SmallYellowPenguin,
    MalePenguin,
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum PenguinStatus {
    Active = 0,
    Death = 1,
    Hunger = 2,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct RedPenguin<BlockNumber, AccountId, Balance, ClassId, AssetId> {
    pub owner: AccountId,
    pub start: BlockNumber,
    pub status: PenguinStatus,
    pub eggs: Balance,
    pub asset_id: AssetId,
    pub class_id: ClassId,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct PenguinConfig<BlockNumber, AccountId, Balance> {
    pub owner: AccountId,
    pub start: BlockNumber,
    pub status: PenguinStatus,
    pub eggs: Balance,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct YellowPenguin<BlockNumber, AccountId, Balance, ClassId, AssetId> {
    pub owner: AccountId,
    pub start: BlockNumber,
    pub status: PenguinStatus,
    pub eggs: Balance,
    pub asset_id: AssetId,
    pub class_id: ClassId,
}
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct SmallYellowPenguin<BlockNumber, AccountId, ClassId, AssetId> {
    pub owner: AccountId,
    pub start: BlockNumber,
    pub status: PenguinStatus,
    pub asset_id: AssetId,
    pub class_id: ClassId,
}
/// Class info
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct MalePenguin<BlockNumber, AccountId, Balance, ClassId, AssetId> {
    pub owner: AccountId,
    pub start: BlockNumber,
    pub eggs: Balance,
    pub status: PenguinStatus,
    pub asset_id: AssetId,
    pub class_id: ClassId,
}

pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use codec::WrapperTypeEncode;
    use frame_support::traits::UnixTime;
    use frame_support::PalletId;
    use orml_nft::{AssetData, TokenType};

    #[pallet::config]
    pub trait Config: frame_system::Config + orml_nft::Config + pallet_assets::Config {
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        type AssetId: IsType<<Self as pallet_assets::Config>::AssetId>
            + MaybeSerializeDeserialize
            + WrapperTypeEncode
            + Default
            + Member
            + Parameter
            + From<u32>
            + Into<u32>
            + Copy;

        // type Balance: IsType<<<T as orml_nft::Config>::Currency as frame_support::traits::Currency<<T as frame_system::Config>::AccountId>>::Balance> + From<u32>+ From<u64>;
        ///初始供应总量
        #[pallet::constant]
        type InitTotalSupply: Get<BalanceOf<Self>>;
        ///初始供应总时间
        #[pallet::constant]
        type InitSupplyPeriod: Get<<Self as frame_system::Config>::BlockNumber>;
        ///拥有公共资产的ClassOwnerId
        #[pallet::constant]
        type ClassOwnerId: Get<PalletId>;
        ///冷库资产id
        type ColdStorageId: Get<PalletId>;
        /// unix时间
        type TimeStamp: UnixTime;

        ///公企鹅产蛋每天最大限制
        #[pallet::constant]
        type MalePenguinEggLimit: Get<BalanceOf<Self>>;

        ///小黄企业锁定交易时间
        #[pallet::constant]
        type SmallYellowPenguinLockPeriod: Get<<Self as frame_system::Config>::BlockNumber>;

        ///小黄企鹅生长时间
        #[pallet::constant]
        type SmallYellowPenguinGrowPeriod: Get<<Self as frame_system::Config>::BlockNumber>;
    }

    // pub type ClassInfoOf<T> = ClassInfo<
    //     <T as Config>::TokenId,
    //     <T as frame_system::Config>::AccountId,
    // 	ClassData<BalanceOf<T>>
    // >;
    // pub type TokenInfoOf<T> =
    //     TokenInfo<<T as frame_system::Config>::AccountId,AssetData<BalanceOf<T>>>;
    //
    // pub type GenesisTokenData<T> = (
    //     <T as frame_system::Config>::AccountId, // Token owner
    //     Vec<u8>,                                // Token metadata
    // 	AssetData<BalanceOf<T>>,
    // );
    // pub type GenesisTokens<T> = (
    //     <T as frame_system::Config>::AccountId, // Token class owner
    //     Vec<u8>,                                // Token class metadata
    // 	ClassData<BalanceOf<T>>,
    //     Vec<GenesisTokenData<T>>, // Vector of tokens belonging to this class
    // );

    /// Error for non-fungible-token module.
    #[pallet::error]
    pub enum Error<T> {
        /// No available class ID
        NoAvailableClassId,
        /// No available token ID
        NoAvailableTokenId,
        /// Token(ClassId, TokenId) not found
        TokenNotFound,
        /// Class not found
        ClassNotFound,
        /// The operator is not the owner of the token and has no permission
        NoPermission,
        /// Can not destroy class
        /// Total issuance is not 0
        CannotDestroyClass,
    }

    pub type AssetOf<T> = <T as Config>::AssetId;
    pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    pub type ClassIdOf<T> = <T as orml_nft::Config>::ClassId;
    pub type TokenIdOf<T> = <T as orml_nft::Config>::TokenId;
    pub type RedPenguinOf<T> =
        RedPenguin<BlockNumberOf<T>, AccountIdOf<T>, BalanceOf<T>, ClassIdOf<T>, TokenIdOf<T>>;
    pub type YellowPenguinOf<T> =
        YellowPenguin<BlockNumberOf<T>, AccountIdOf<T>, BalanceOf<T>, ClassIdOf<T>, TokenIdOf<T>>;
    pub type SmallYellowPenguinOf<T> =
        SmallYellowPenguin<BlockNumberOf<T>, AccountIdOf<T>, ClassIdOf<T>, TokenIdOf<T>>;
    pub type MalePenguinOf<T> =
        MalePenguin<BlockNumberOf<T>, AccountIdOf<T>, BalanceOf<T>, ClassIdOf<T>, TokenIdOf<T>>;
    pub type PenguinConfigOf<T> = PenguinConfig<BlockNumberOf<T>, AccountIdOf<T>, BalanceOf<T>>;
    /// red penguin owner
    #[pallet::storage]
    #[pallet::getter(fn owner_red_penguin)]
    pub type OwnerRedPenguin<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Vec<RedPenguinOf<T>>, ValueQuery>;

    /// yellow penguin owner
    #[pallet::storage]
    #[pallet::getter(fn owner_yellow_penguin)]
    pub type OwnerYellowPenguin<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Vec<YellowPenguinOf<T>>, ValueQuery>;

    /// small yellow penguin owner
    #[pallet::storage]
    #[pallet::getter(fn owner_small_yellow_penguin)]
    pub type OwnerSmallYellowPenguin<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Vec<SmallYellowPenguinOf<T>>, ValueQuery>;

    /// male penguin owner
    #[pallet::storage]
    #[pallet::getter(fn owner_male_penguin)]
    pub type OwnerMalePenguin<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Vec<MalePenguinOf<T>>, ValueQuery>;

    /// current epoch
    #[pallet::storage]
    #[pallet::getter(fn current_epoch)]
    pub type CurrentEpoch<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// current
    #[pallet::storage]
    #[pallet::getter(fn red_peng_class_id)]
    pub type RedPenguinClassId<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// 当前epoch
    #[pallet::storage]
    #[pallet::getter(fn yellow_penguin_class_id)]
    pub type YellowPenguinClassId<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// 当前epoch
    #[pallet::storage]
    #[pallet::getter(fn small_yellow_penguin_class_id)]
    pub type SmallYellowPenguinClassId<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// 当前孵化眷
    #[pallet::storage]
    #[pallet::getter(fn incubation_coupon_asset_id)]
    pub type IncubationCouponAssetId<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// 当前每天的供应量
    #[pallet::storage]
    #[pallet::getter(fn supply_per_day)]
    pub type SupplyPerDay<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// 当前epoch
    #[pallet::storage]
    #[pallet::getter(fn incubation_coupon)]
    pub type IncubationCoupon<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, BalanceOf<T>, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub red_penguin: Vec<(AccountIdOf<T>, Vec<PenguinConfigOf<T>>)>,
        pub yellow_penguin: Vec<(AccountIdOf<T>, Vec<PenguinConfigOf<T>>)>,
        pub small_yellow_penguin: Vec<(AccountIdOf<T>, Vec<PenguinConfigOf<T>>)>,
        pub male_penguin: Vec<(AccountIdOf<T>, Vec<PenguinConfigOf<T>>)>,
        pub red_penguin_class_id: ClassIdOf<T>,
        pub yellow_penguin_class_id: ClassIdOf<T>,
        pub small_yellow_class_id: ClassIdOf<T>,
        pub male_penguin_class_id: ClassIdOf<T>,
        pub incubation_coupon_asset_id: AssetOf<T>,
        pub init_per_day_supply: BalanceOf<T>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            GenesisConfig {
                red_penguin: vec![],
                yellow_penguin: vec![],
                small_yellow_penguin: vec![],
                male_penguin: vec![],
                red_penguin_class_id: Default::default(),
                yellow_penguin_class_id: Default::default(),
                small_yellow_class_id: Default::default(),
                male_penguin_class_id: Default::default(),
                incubation_coupon_asset_id: Default::default(),
                init_per_day_supply: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            let class_owner_account_id = <Pallet<T>>::class_owner_id();
            //初始化红企鹅
            let red_class_id = orml_nft::Pallet::<T>::create_native_class(
                &class_owner_account_id,
                vec![],
                s!(20000u32),
                TokenType::Transferable,
            )
            .expect("Create class cannot fail while building genesis");
            self.red_penguin
                .iter()
                .for_each(|(account_id, red_penguin_of)| {
                    let mut penguins: Vec<RedPenguinOf<T>> = vec![];
                    for red_penguin in red_penguin_of {
                        let token_id = <orml_nft::Pallet<T>>::mint(
                            account_id,
                            red_class_id,
                            vec![],
                            AssetData {
                                deposit: Default::default(),
                                name: (account_id, red_class_id).encode(),
                                description: red_penguin.encode(),
                            },
                        )
                        .expect("mint red_penguin error ");
                        penguins.push(RedPenguinOf::<T> {
                            owner: account_id.clone(),
                            start: red_penguin.start,
                            status: red_penguin.status,
                            eggs: red_penguin.eggs,
                            asset_id: token_id,
                            class_id: red_class_id,
                        });
                    }
                    OwnerRedPenguin::<T>::insert(account_id, penguins);
                });
            //yellow penguin
            let yellow_class_id = orml_nft::Pallet::<T>::create_native_class(
                &class_owner_account_id,
                vec![],
                s!(self.yellow_penguin.len()),
                TokenType::Transferable,
            )
            .expect("Create class cannot fail while building genesis");
            self.yellow_penguin
                .iter()
                .for_each(|(account_id, yellow_penguin_of)| {
                    let mut penguins: Vec<YellowPenguinOf<T>> = vec![];
                    for yellow_penguin in yellow_penguin_of {
                        let token_id = orml_nft::Pallet::<T>::mint(
                            account_id,
                            yellow_class_id,
                            vec![],
                            AssetData {
                                deposit: Default::default(),
                                name: (account_id, yellow_class_id).encode(),
                                description: yellow_penguin.encode(),
                            },
                        )
                        .expect("mint yellow_penguin error ");
                        penguins.push(YellowPenguinOf::<T> {
                            owner: account_id.clone(),
                            start: yellow_penguin.start,
                            status: yellow_penguin.status,
                            eggs: yellow_penguin.eggs,
                            asset_id: token_id,
                            class_id: yellow_class_id,
                        });
                    }
                    OwnerYellowPenguin::<T>::insert(account_id, penguins);
                });
            //small yellow penguin
            let small_yellow_class_id = orml_nft::Pallet::<T>::create_native_class(
                &class_owner_account_id,
                vec![],
                s!(self.small_yellow_penguin.len()),
                TokenType::Transferable,
            )
            .expect("Create class cannot fail while building genesis");
            self.small_yellow_penguin
                .iter()
                .for_each(|(account_id, small_yellow_penguin_of)| {
                    let mut penguins: Vec<SmallYellowPenguinOf<T>> = vec![];
                    for small_yellow_penguin in small_yellow_penguin_of {
                        let token_id = orml_nft::Pallet::<T>::mint(
                            account_id,
                            small_yellow_class_id,
                            vec![],
                            AssetData {
                                deposit: Default::default(),
                                name: (account_id, small_yellow_class_id).encode(),
                                description: small_yellow_penguin.encode(),
                            },
                        )
                        .expect("mint red_penguin error ");

                        penguins.push(SmallYellowPenguinOf::<T> {
                            owner: account_id.clone(),
                            start: small_yellow_penguin.start,
                            status: small_yellow_penguin.status,
                            asset_id: token_id,
                            class_id: small_yellow_class_id,
                        });
                    }
                    OwnerSmallYellowPenguin::<T>::insert(account_id, penguins);
                });
            //male penguin
            let male_class_id = orml_nft::Pallet::<T>::create_native_class(
                &class_owner_account_id,
                vec![],
                s!(self.male_penguin.len()),
                TokenType::BoundToAddress,
            )
            .expect("Create class cannot fail while building genesis");
            self.male_penguin
                .iter()
                .for_each(|(account_id, male_penguin_of)| {
                    let mut penguins: Vec<MalePenguinOf<T>> = vec![];
                    for male_penguin in male_penguin_of {
                        let token_id = orml_nft::Pallet::<T>::mint(
                            account_id,
                            male_class_id,
                            vec![],
                            AssetData {
                                deposit: Default::default(),
                                name: (account_id, male_class_id).encode(),
                                description: male_penguin.encode(),
                            },
                        )
                        .expect("mint red_penguin error ");
                        penguins.push(MalePenguinOf::<T> {
                            owner: account_id.clone(),
                            start: male_penguin.start,
                            status: male_penguin.status,
                            eggs: male_penguin.eggs,
                            asset_id: token_id,
                            class_id: male_class_id,
                        });
                    }
                    OwnerMalePenguin::<T>::insert(account_id, penguins);
                });
            //
            IncubationCouponAssetId::<T>::set(s!(self.incubation_coupon_asset_id));

            SupplyPerDay::<T>::set(s!(self.init_per_day_supply));
        }
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {}
}

impl<T: Config> Pallet<T> {
    pub fn class_owner_id() -> AccountIdOf<T> {
        T::ClassOwnerId::get().into_account()
    }
}
