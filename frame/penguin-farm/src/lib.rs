


#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use codec::{Decode, Encode};
use frame_support::{ensure, pallet_prelude::*, Parameter,traits::{Currency,ReservableCurrency}};
use sp_runtime::{
    traits::{
        AtLeast32BitUnsigned, CheckedAdd, CheckedSub, MaybeSerializeDeserialize, Member, One, Zero,
    },
    ArithmeticError, DispatchError, DispatchResult, RuntimeDebug,
};
use sp_std::vec::Vec;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

mod mock;
mod tests;
use frame_support::scale_info::TypeInfo;


#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug,TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum PenguinFarm {
	RedPenguin,
	YellowPenguin,
	SmallYellowPenguin,
    MalePenguin
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug,TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum PenguinStatus{
	 Active=0,
	Death=1,
	Hunger=2
}


#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq,TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct RedPenguin<BlockNumber,AccountId,Balance,AssetId,ClassId>{
	owner:AccountId,
	start: BlockNumber,
	status: PenguinStatus,
	eggs: Balance,
	asset_id:AssetId,
	class_id: ClassId
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq,TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct PenguinConfig<BlockNumber,AccountId,Balance>{
	owner:AccountId,
	start: BlockNumber,
	status: PenguinStatus,
	eggs: Balance,
}


#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq,TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct YellowPenguin<BlockNumber,AccountId,Balance,AssetId,ClassId>{

}
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq,TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct SmallYellowPenguin<BlockNumber,AccountId,Balance,AssetId,ClassId>{

}
/// Class info
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq,TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct MalePenguin<BlockNumber,AccountId,Balance,AssetId,ClassId>{

}

pub type BalanceOf<T> =
<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub use module::*;


#[frame_support::pallet]
pub mod module {
	use frame_support::PalletId;
	use frame_support::traits::UnixTime;
	use frame_system::Account;
	use orml_nft::AssetData;
	use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config  + orml_nft::Config + pallet_assets::Config{
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
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

	pub type AssetOf<T> =<T as pallet_assets::Config>::AssetId;
	pub type BlockNumberOf<T>=<T as frame_system::Config>::BlockNumber;
	pub type AccountIdOf<T>=<T as frame_system::Config>::AccountId;
	pub type ClassIdOf<T> = <T as orml_nft::Config>::ClassId;
	pub type TokenIdOf<T> =<T as orml_nft::Config>::TokenId;
	pub type RedPenguinOf<T>=RedPenguin<BlockNumberOf<T>,AccountIdOf<T>,BalanceOf<T>,ClassIdOf<T>,TokenIdOf<T>>;
	pub type YellowPenguinOf<T>=RedPenguin<BlockNumberOf<T>,AccountIdOf<T>,BalanceOf<T>,ClassIdOf<T>,TokenIdOf<T>>;
	pub type SmallYellowPenguinOf<T>=RedPenguin<BlockNumberOf<T>,AccountIdOf<T>,BalanceOf<T>,ClassIdOf<T>,TokenIdOf<T>>;
	pub type MalePenguinOf<T>=RedPenguin<BlockNumberOf<T>,AccountIdOf<T>,BalanceOf<T>,ClassIdOf<T>,TokenIdOf<T>>;
	pub type PenguinConfigOf<T>=PenguinConfig<BlockNumberOf<T>,AccountIdOf<T>,BalanceOf<T>>;
    /// red penguin owner
	#[pallet::storage]
    #[pallet::getter(fn owner_red_penguin)]
    pub type OwnerRedPenguin<T: Config> =  StorageMap<_, Twox64Concat, T::AccountId, Vec<RedPenguinOf<T>>, ValueQuery>;

	/// yellow penguin owner
	#[pallet::storage]
	#[pallet::getter(fn owner_yellow_penguin)]
	pub type OwnerYellowPenguin<T: Config> =  StorageMap<_, Twox64Concat, T::AccountId, Vec<YellowPenguinOf<T>>, ValueQuery>;

	/// small yellow penguin owner
	#[pallet::storage]
	#[pallet::getter(fn owner_small_yellow_penguin)]
	pub type OwnerSmallYellowPenguin<T: Config> =  StorageMap<_, Twox64Concat, T::AccountId, Vec<SmallYellowPenguinOf<T>>, ValueQuery>;

	/// male penguin owner
	#[pallet::storage]
	#[pallet::getter(fn owner_male_penguin)]
	pub type OwnerMalePenguin<T: Config> =  StorageMap<_, Twox64Concat, T::AccountId, Vec<MalePenguinOf<T>>, ValueQuery>;


	/// current epoch
	#[pallet::storage]
	#[pallet::getter(fn current_epoch)]
	pub type CurrentEpoch<T: Config> =  StorageValue<_, u32, ValueQuery>;

	/// current
	#[pallet::storage]
	#[pallet::getter(fn red_peng_class_id)]
	pub type RedPenguinClassId<T: Config> =  StorageValue<_, u32, ValueQuery>;


	/// 当前epoch
	#[pallet::storage]
	#[pallet::getter(fn yellow_penguin_class_id)]
	pub type YellowPenguinClassId<T: Config> =  StorageValue<_, u32, ValueQuery>;


	/// 当前epoch
	#[pallet::storage]
	#[pallet::getter(fn small_yellow_penguin_class_id)]
	pub type SmallYellowPenguinClassId<T: Config> =  StorageValue<_, u32, ValueQuery>;

	/// 当前孵化眷
	#[pallet::storage]
	#[pallet::getter(fn incubation_coupon_asset_id)]
	pub type IncubationCouponAssetId<T: Config> =  StorageValue<_, u32, ValueQuery>;


	/// 当前每天的供应量
	#[pallet::storage]
	#[pallet::getter(fn supply_per_day)]
	pub type SupplyPerDay<T: Config> =  StorageValue<_, u32, ValueQuery>;

	/// 当前epoch
	#[pallet::storage]
	#[pallet::getter(fn Incubation_coupon)]
	pub type IncubationCoupon<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, BalanceOf<T>, ValueQuery>;



    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub red_penguin: Vec<(AccountIdOf<T>,Vec<PenguinConfigOf<T>>)>,
		pub yellow_penguin: Vec<(AccountIdOf<T>,Vec<yellowPenguinOf<T>>)>,
		pub red_penguin_class_id: ClassIdOf<T>,
		pub yellow_penguin_class_id: ClassIdOf<T>,
		pub small_yellow_class_id:  ClassIdOf<T>,
		pub male_penguin_class_id:  ClassIdOf<T>,
		pub incubation_coupon_asset_id: AssetOf<T>,
		pub init_per_day_supply:BalanceOf<T>,
		pub init_had_supply: BalanceOf<T>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            GenesisConfig {
				red_penguin: vec![],
				yellow_penguin: vec![],
				red_penguin_class_id:Default::default(),
				yellow_penguin_class_id: Default::default(),
				small_yellow_class_id: Default::default(),
				male_penguin_class_id: Default::default(),
				incubation_coupon_asset_id: Default::default(),
				init_per_day_supply:Default::default(),
				init_had_supply:Default::default(),
			}
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
			///初始化红企鹅
			let red_class_id=orml_nft::Pallet::<T>::create_native_class(<T as Config>::ClassOwnerId,vec![],20000u128.into()).expect("Create class cannot fail while building genesis");
            self.red_penguin.iter().for_each(|(account_id,red_penguin_of)| {
				let mut penguins:Vec<RedPenguinOf<T>>= vec![];
				for (index,red_penguin) in  red_penguin_of.enumerate(){
					let token_id=orml_nft::Pallet::<T>::mint(account_id,red_class_id,vec![],AssetData{
						deposit: Default::default(),
						name: (account_id,red_class_id,index).encode(),
						description: red_penguin.encode()
					}).expect("mint red_penguin error ");
					penguins.push(RedPenguinOf{
						owner: account_id.clone(),
						start: red_penguin.start,
						status: red_penguin.status,
						eggs: red_penguin.eggs,
						asset_id: token_id,
						class_id: red_class_id
					});
				}
				OwnerRedPenguin::<T>::insert(account_id,penguins);
            });
          ///yellow penguin
			let red_class_id=orml_nft::Pallet::<T>::create_native_class(<T as Config>::ClassOwnerId,vec![],20000u128.into()).expect("Create class cannot fail while building genesis");
			self.red_penguin.iter().for_each(|(account_id,red_penguin_of)| {
				let mut penguins:Vec<RedPenguinOf<T>>= vec![];
				for (index,red_penguin) in  red_penguin_of.enumerate(){
					let token_id=orml_nft::Pallet::<T>::mint(account_id,red_class_id,vec![],AssetData{
						deposit: Default::default(),
						name: (account_id,red_class_id,index).encode(),
						description: red_penguin.encode()
					}).expect("mint red_penguin error ");
					penguins.push(RedPenguinOf{
						owner: account_id.clone(),
						start: red_penguin.start,
						status: red_penguin.status,
						eggs: red_penguin.eggs,
						asset_id: token_id,
						class_id: red_class_id
					});
				}
				OwnerRedPenguin::<T>::insert(account_id,penguins);
			});
			///small yellow penguin
			let red_class_id=orml_nft::Pallet::<T>::create_native_class(<T as Config>::ClassOwnerId,vec![],20000u128.into()).expect("Create class cannot fail while building genesis");
			self.red_penguin.iter().for_each(|(account_id,red_penguin_of)| {
				let mut penguins:Vec<RedPenguinOf<T>>= vec![];
				for (index,red_penguin) in  red_penguin_of.enumerate(){
					let token_id=orml_nft::Pallet::<T>::mint(account_id,red_class_id,vec![],AssetData{
						deposit: Default::default(),
						name: (account_id,red_class_id,index).encode(),
						description: red_penguin.encode()
					}).expect("mint red_penguin error ");
					penguins.push(RedPenguinOf{
						owner: account_id.clone(),
						start: red_penguin.start,
						status: red_penguin.status,
						eggs: red_penguin.eggs,
						asset_id: token_id,
						class_id: red_class_id
					});
				}
				OwnerRedPenguin::<T>::insert(account_id,penguins);
			});
			///male penguin
			let red_class_id=orml_nft::Pallet::<T>::create_native_class(<T as Config>::ClassOwnerId,vec![],20000u128.into()).expect("Create class cannot fail while building genesis");
			self.red_penguin.iter().for_each(|(account_id,red_penguin_of)| {
				let mut penguins:Vec<RedPenguinOf<T>>= vec![];
				for (index,red_penguin) in  red_penguin_of.enumerate(){
					let token_id=orml_nft::Pallet::<T>::mint(account_id,red_class_id,vec![],AssetData{
						deposit: Default::default(),
						name: (account_id,red_class_id,index).encode(),
						description: red_penguin.encode()
					}).expect("mint red_penguin error ");
					penguins.push(RedPenguinOf{
						owner: account_id.clone(),
						start: red_penguin.start,
						status: red_penguin.status,
						eggs: red_penguin.eggs,
						asset_id: token_id,
						class_id: red_class_id
					});
				}
				OwnerRedPenguin::<T>::insert(account_id,penguins);
			});
			///


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
}
