#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
#[warn(unused_doc_comments)]
use codec::{Decode, Encode};
use frame_support::{
    pallet_prelude::*,
    traits::{Currency, ReservableCurrency},
    Parameter,
};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{
    traits::{MaybeSerializeDeserialize, Member},
    RuntimeDebug,
};

use scale_info::StaticTypeInfo;
use sp_std::vec::Vec;
mod mock;
mod tests;
use frame_support::scale_info::{Type, TypeInfo};
use frame_support::sp_runtime::traits::{
    AccountIdConversion, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, One, UniqueSaturatedFrom,
    UniqueSaturatedInto, Zero,
};
use sp_arithmetic::traits::CheckedRem;

macro_rules! s {
    ($e: expr) => {
        sp_runtime::SaturatedConversion::saturated_into($e)
    };
}
macro_rules! bid_penguin {
    ($e:expr,$tt:ident,$class_id:expr,$token_id:expr,$caller:expr) => {{
        let mut new_penguin = $e.clone();
        let $tt {
            pre_eat_at,
            eat_count,
	        owner,
            ..
        } = $e;
		ensure!($caller == owner.clone(), Error::<T>::NoPermission);
        new_penguin.status = PenguinStatus::Bid;
        ensure!(eat_count == 0u32, Error::<T>::NeedClaimPenguinEgg);
        if new_penguin.pre_eat_at < T::PenguinProducePeriod::get() {
            new_penguin.pre_eat_at =
                frame_system::Pallet::<T>::current_block_number() - T::PenguinProducePeriod::get();
        }
        Penguins::<T>::mutate($class_id, $token_id, |penguin| {
            sp_std::mem::swap(penguin, &mut Some(PenguinFarmOf::<T>::$tt(new_penguin)));
        });
    }};
}

macro_rules! unbid_penguin {
    ($e:expr,$tt:ident,$class_id:expr,$token_id:expr,$caller:expr,$flag:expr) => {{
					let mut new_penguin = $e.clone();
					let $tt{
						pre_eat_at,
						eat_count,
						status,
						owner,
						..
				} = $e;
				ensure!($caller == owner.clone(), Error::<T>::NoPermission);
				match ($flag, pre_eat_at > T::PenguinProducePeriod::get()) {
					(false, true) => {
						if status != PenguinStatus::Hunger {
							new_penguin.status = PenguinStatus::Hunger;
							Penguins::<T>::mutate($class_id, $token_id, |penguin| {
								sp_std::mem::swap(penguin, &mut Some(PenguinFarmOf::<T>::$tt(new_penguin)));
							});
						};
					}
					(_, false) => {
					if status != PenguinStatus::Active {
					new_penguin.status = PenguinStatus::Active;
					Penguins::<T>::mutate($class_id, $token_id, |penguin| {
					sp_std::mem::swap(penguin, &mut Some(PenguinFarmOf::<T>::$tt(new_penguin)));
					});
					};
					}
					(true, true) => {
						if pre_eat_at > T::YellowPenguinDeadPeriod::get() {
							Penguins::<T>::remove($class_id, $token_id);
							let _:Result<_,usize>=OwnerYellowPenguin::<T>::try_mutate($caller, |ids| {
								let index = ids.binary_search(&$token_id)?;
								ids.remove(index);
								Ok(())
							});
						};
					}
				}
    }};
}

mod weights;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum PenguinFarm<BlockNumber, AccountId, Balance, ClassId, AssetId> {
    RedPenguin(RedPenguin<BlockNumber, AccountId, Balance, ClassId, AssetId>),
    YellowPenguin(YellowPenguin<BlockNumber, AccountId, Balance, ClassId, AssetId>),
    SmallYellowPenguin(SmallYellowPenguin<BlockNumber, AccountId, ClassId, AssetId>),
    MalePenguin(MalePenguin<BlockNumber, AccountId, Balance, ClassId, AssetId>),
}

impl<BlockNumber, AccountId, Balance, ClassId, AssetId>
    PenguinFarm<BlockNumber, AccountId, Balance, ClassId, AssetId>
{
    pub fn query_penguin_status(&self) -> PenguinStatus {
        match self {
            PenguinFarm::RedPenguin(penguin) => penguin.status,
            PenguinFarm::YellowPenguin(penguin) => penguin.status,
            PenguinFarm::SmallYellowPenguin(penguin) => penguin.status,
            PenguinFarm::MalePenguin(penguin) => penguin.status,
        }
    }

    pub fn query_prev_eat_at(&self) -> &BlockNumber {
        match self {
            PenguinFarm::RedPenguin(RedPenguin { pre_eat_at, .. }) => pre_eat_at,
            PenguinFarm::YellowPenguin(YellowPenguin { pre_eat_at, .. }) => pre_eat_at,
            PenguinFarm::SmallYellowPenguin(SmallYellowPenguin { pre_eat_at, .. }) => pre_eat_at,
            PenguinFarm::MalePenguin(MalePenguin { pre_eat_at, .. }) => pre_eat_at,
        }
    }
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum PenguinStatus {
    ///正常
    Active = 0,
    ///饥饿
    Hunger = 1,
    ///挂单
    Bid = 2,
    ///死亡
    death = 3,
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CouponStatus {
    //未激活
    UnActive = 0,
    //流通
    Liquid = 1,
    //收回
    Retire = 2,
    //孵化
    Hatch = 3,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct RedPenguin<BlockNumber, AccountId, Balance, ClassId, AssetId> {
    pub owner: AccountId,
    pub start: BlockNumber,
    pub pre_eat_at: BlockNumber,
    pub eat_count: u32,
    pub status: PenguinStatus,
    pub eggs: Balance,
    pub asset_id: AssetId,
    pub class_id: ClassId,
    pub incubation_remain: Balance,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct IncubationCoupon<BlockNumber, AccountId, ClassId, AssetId> {
    pub owner: AccountId,
    pub start: BlockNumber,
    pub status: CouponStatus,
    pub asset_id: AssetId,
    pub class_id: ClassId,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct PenguinConfig<BlockNumber, AccountId, Balance> {
    pub owner: AccountId,
    pub start: BlockNumber,
    pub pre_eat_at: BlockNumber,
    pub status: PenguinStatus,
    pub eggs: Balance,
}
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct YellowPenguin<BlockNumber, AccountId, Balance, ClassId, AssetId> {
    pub owner: AccountId,
    pub start: BlockNumber,
    pub eat_count: u32,
    pub status: PenguinStatus,
    pub pre_eat_at: BlockNumber,
    pub eggs: Balance,
    pub asset_id: AssetId,
    pub class_id: ClassId,
    pub incubation_remain: Balance,
}
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct SmallYellowPenguin<BlockNumber, AccountId, ClassId, AssetId> {
    pub owner: AccountId,
    pub start: BlockNumber,
    pub pre_eat_at: BlockNumber,
    pub eat_count: u32,
    pub status: PenguinStatus,
    pub asset_id: AssetId,
    pub class_id: ClassId,
    pub grow_value: BlockNumber,
}
/// Class info
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct MalePenguin<BlockNumber, AccountId, Balance, ClassId, AssetId> {
    pub owner: AccountId,
    pub start: BlockNumber,
    pub pre_eat_at: BlockNumber,
    pub eat_count: u32,
    pub eggs: Balance,
    pub status: PenguinStatus,
    pub asset_id: AssetId,
    pub class_id: ClassId,
    pub incubation_remain: Balance,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Logs<T: Config> {
    penguin: Vec<PenguinFarmOf<T>>,
    eggs: Vec<(AccountIdOf<T>, BalanceOf<T>)>,
    incubation: Vec<IncubationCouponOf<T>>,
}

pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use crate::weights::WeightInfo;
    use frame_support::sp_runtime::sp_std::ops::Sub;
    use frame_support::sp_runtime::traits::{
        AtLeast32BitUnsigned, BlockNumberProvider, Bounded, UniqueSaturatedInto,
    };
    use frame_support::sp_runtime::{Permill, SaturatedConversion};
    use frame_support::sp_std::time::Duration;
    use frame_support::traits::fungibles::Mutate;
    use frame_support::traits::{Time, UnixTime};
    use frame_support::{Never, PalletId};
    use frame_system::ensure_signed;
    use frame_system::pallet_prelude::OriginFor;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_assets::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        type AssetId: IsType<<Self as pallet_assets::Config>::AssetId>
            + MaybeSerializeDeserialize
            + Encode
            + Decode
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + Default
            + Member
            + Parameter
            + From<u32>
            + Into<u32>
            + Copy;

        type ClassId: Parameter
            + Member
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + From<u32>
            + Into<u32>
            + MaybeSerializeDeserialize;

        type TokenId: Parameter
            + Member
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + From<u64>
            + Into<u64>
            + MaybeSerializeDeserialize;

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
        #[pallet::constant]
        type ColdStorageId: Get<PalletId>;
        ///劵资产地址id
        #[pallet::constant]
        type IncubationId: Get<PalletId>;

        ///技术团队分配资金地址
        #[pallet::constant]
        type TechnicalStashId: Get<PalletId>;

        ///运营团队分配资金地址
        #[pallet::constant]
        type OperationStashId: Get<PalletId>;
        /// unix时间
        type TimeStamp: UnixTime;

        ///公企鹅产蛋每天最大限制
        #[pallet::constant]
        type MalePenguinEggLimit: Get<BalanceOf<Self>>;

        ///公企鹅生命时间
        #[pallet::constant]
        type MalePenguinLifeSpan: Get<<Self as frame_system::Config>::BlockNumber>;

        ///公企鹅当前分配比率
        #[pallet::constant]
        type MalePenguinEggRate: Get<Permill>;

        ///红企鹅当前分配比率
        #[pallet::constant]
        type RedPenguinEggRate: Get<Permill>;

        ///黄企鹅当前分配比率
        #[pallet::constant]
        type YellowPenguinEggRate: Get<Permill>;

        ///技术团队当前分配比率
        #[pallet::constant]
        type TechnicalPenguinEggRate: Get<Permill>;

        ///运营团队当前分配比率
        #[pallet::constant]
        type OperationPenguinEggRate: Get<Permill>;

        ///运营团队当前分配比率
        #[pallet::constant]
        type MalePenguinRate: Get<Permill>;

        ///企鹅产蛋间隔时间
        #[pallet::constant]
        type PenguinProducePeriod: Get<<Self as frame_system::Config>::BlockNumber>;

        ///黄企鹅死亡时间间隔
        #[pallet::constant]
        type YellowPenguinDeadPeriod: Get<<Self as frame_system::Config>::BlockNumber>;

        ///小黄企业锁定交易时间
        #[pallet::constant]
        type SmallYellowPenguinLockPeriod: Get<<Self as frame_system::Config>::BlockNumber>;

        ///小黄企鹅生长时间
        #[pallet::constant]
        type SmallYellowPenguinGrowPeriod: Get<<Self as frame_system::Config>::BlockNumber>;

        ///小黄企鹅生长时间
        #[pallet::constant]
        type IncubationLivePeriod: Get<<Self as frame_system::Config>::BlockNumber>;

        ///红企鹅产生孵化劵需要蛋数
        #[pallet::constant]
        type RedPenguinEggCountForIncubation: Get<BalanceOf<Self>>;

        ///黄企鹅产生孵化劵需要蛋数
        #[pallet::constant]
        type YellowPenguinEggCountForIncubation: Get<BalanceOf<Self>>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        Claim(PenguinFarmOf<T>, BalanceOf<T>),
        Incubation(IncubationCouponOf<T>),
        Incubations(Vec<TokenIdOf<T>>, BlockNumberOf<T>),
        LowIncubation(IncubationCouponOf<T>),
        Bid(ClassIdOf<T>, TokenIdOf<T>),
        UnBid(ClassIdOf<T>, TokenIdOf<T>),
    }

    /// Error for non-fungible-token module.
    #[pallet::error]
    pub enum Error<T> {
        ///必须先收取企鹅蛋
        NeedClaimPenguinEgg,
        ///求下次剩余发卷次数报错
        IncubationRemainError,
        ///小黄企鹅不产蛋
        PenguinIsSmallYellow,
        ///企鹅处于非active状态
        PenguinStatusError,
        ///企鹅不存在
        PenguinNoExist,
        ///公企鹅禁止拍卖
        MalePenguinBanBid,
        ///获取孵化劵id错误
        IncubationTokenIdError,

        PenguinProduceCalError,
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
    pub type ClassIdOf<T> = <T as Config>::ClassId;
    pub type TokenIdOf<T> = <T as Config>::TokenId;
    pub type RedPenguinOf<T> =
        RedPenguin<BlockNumberOf<T>, AccountIdOf<T>, BalanceOf<T>, ClassIdOf<T>, TokenIdOf<T>>;
    pub type YellowPenguinOf<T> =
        YellowPenguin<BlockNumberOf<T>, AccountIdOf<T>, BalanceOf<T>, ClassIdOf<T>, TokenIdOf<T>>;
    pub type SmallYellowPenguinOf<T> =
        SmallYellowPenguin<BlockNumberOf<T>, AccountIdOf<T>, ClassIdOf<T>, TokenIdOf<T>>;
    pub type MalePenguinOf<T> =
        MalePenguin<BlockNumberOf<T>, AccountIdOf<T>, BalanceOf<T>, ClassIdOf<T>, TokenIdOf<T>>;
    pub type PenguinConfigOf<T> = PenguinConfig<BlockNumberOf<T>, AccountIdOf<T>, BalanceOf<T>>;

    pub type IncubationCouponOf<T> =
        IncubationCoupon<BlockNumberOf<T>, AccountIdOf<T>, ClassIdOf<T>, TokenIdOf<T>>;
    pub type PenguinFarmOf<T> =
        PenguinFarm<BlockNumberOf<T>, AccountIdOf<T>, BalanceOf<T>, ClassIdOf<T>, TokenIdOf<T>>;
    /// red penguin owner
    #[pallet::storage]
    #[pallet::getter(fn owner_red_penguin)]
    pub type OwnerRedPenguin<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Vec<TokenIdOf<T>>, ValueQuery>;

    /// yellow penguin owner
    #[pallet::storage]
    #[pallet::getter(fn owner_yellow_penguin)]
    pub type OwnerYellowPenguin<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Vec<TokenIdOf<T>>, ValueQuery>;

    /// small yellow penguin owner
    #[pallet::storage]
    #[pallet::getter(fn owner_small_yellow_penguin)]
    pub type OwnerSmallYellowPenguin<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Vec<TokenIdOf<T>>, ValueQuery>;

    /// male penguin owner
    #[pallet::storage]
    #[pallet::getter(fn owner_male_penguin)]
    pub type OwnerMalePenguin<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Vec<TokenIdOf<T>>, ValueQuery>;

    #[pallet::type_value]
    pub fn default_for_epoch<T: Config>() -> u32 {
        1u32
    }

    /// current epoch
    #[pallet::storage]
    #[pallet::getter(fn current_epoch)]
    pub type CurrentEpoch<T: Config> = StorageValue<_, u32, ValueQuery, default_for_epoch<T>>;
    ///current epoch amount
    #[pallet::storage]
    #[pallet::getter(fn current_epoch_balance)]
    pub type CurrentEpochBalance<T: Config> =
        StorageMap<_, Twox64Concat, u32, BalanceOf<T>, ValueQuery>;
    ///current epoch period
    #[pallet::storage]
    #[pallet::getter(fn current_epoch_period)]
    pub type CurrentEpochPeriod<T: Config> =
        StorageMap<_, Twox64Concat, u32, BlockNumberOf<T>, ValueQuery>;

    ///红企鹅每天产蛋率
    #[pallet::storage]
    #[pallet::getter(fn red_penguin_produce_rate)]
    pub type RedPenguinProduceRate<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    ///黄企鹅每天产蛋率
    #[pallet::storage]
    #[pallet::getter(fn yellow_penguin_produce_rate)]
    pub type YellowPenguinProduceRate<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    ///公企鹅每天产蛋率
    #[pallet::storage]
    #[pallet::getter(fn male_penguin_produce_rate)]
    pub type MalePenguinProduceRate<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;
    /// current
    #[pallet::storage]
    #[pallet::getter(fn red_peng_class_id)]
    pub type RedPenguinClassId<T: Config> = StorageValue<_, ClassIdOf<T>, ValueQuery>;

    /// 当前epoch
    #[pallet::storage]
    #[pallet::getter(fn yellow_penguin_class_id)]
    pub type YellowPenguinClassId<T: Config> = StorageValue<_, ClassIdOf<T>, ValueQuery>;

    /// 当前epoch
    #[pallet::storage]
    #[pallet::getter(fn small_yellow_penguin_class_id)]
    pub type SmallYellowPenguinClassId<T: Config> = StorageValue<_, ClassIdOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn next_class_id)]
    pub type ClassId<T: Config> = StorageValue<_, ClassIdOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn next_red_token_id)]
    pub type RedTokenId<T: Config> = StorageValue<_, TokenIdOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn next_yellow_token_id)]
    pub type YellowTokenId<T: Config> = StorageValue<_, TokenIdOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn next_small_yellow_token_id)]
    pub type SmallYellowTokenId<T: Config> = StorageValue<_, TokenIdOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn next_male_token_id)]
    pub type MakeTokenId<T: Config> = StorageValue<_, TokenIdOf<T>, ValueQuery>;

    ///高级孵化劵当前id
    #[pallet::storage]
    #[pallet::getter(fn next_incubation_token_id)]
    pub type IncubationTokenId<T: Config> = StorageValue<_, TokenIdOf<T>, ValueQuery>;

    ///公企鹅类别id
    #[pallet::storage]
    #[pallet::getter(fn male_penguin_class_id)]
    pub type MalePenguinClassId<T: Config> = StorageValue<_, ClassIdOf<T>, ValueQuery>;

    /// 高级孵化眷id
    #[pallet::storage]
    #[pallet::getter(fn incubation_coupon_asset_id)]
    pub type IncubationCouponClassId<T: Config> = StorageValue<_, ClassIdOf<T>, ValueQuery>;

    ///低级孵化劵id
    #[pallet::storage]
    #[pallet::getter(fn low_incubation_coupon_asset_id)]
    pub type LowIncubationCouponClassId<T: Config> = StorageValue<_, ClassIdOf<T>, ValueQuery>;

    ///企鹅仓库
    #[pallet::storage]
    #[pallet::getter(fn query_penguin)]
    pub type Penguins<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        ClassIdOf<T>,
        Twox64Concat,
        TokenIdOf<T>,
        PenguinFarmOf<T>,
        OptionQuery,
    >;

    /// 当前孵化眷数量
    #[pallet::storage]
    #[pallet::getter(fn incubation_coupon_asset)]
    pub type OwnerIncubationCouponAsset<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Vec<TokenIdOf<T>>, ValueQuery>;

    ///高级孵化劵仓库
    #[pallet::storage]
    #[pallet::getter(fn query_incubation)]
    pub type IncubationCoupons<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        ClassIdOf<T>,
        Twox64Concat,
        TokenIdOf<T>,
        IncubationCouponOf<T>,
        OptionQuery,
    >;

    ///低级孵化劵仓库
    #[pallet::storage]
    #[pallet::getter(fn query_low_incubation)]
    pub type LowIncubationCoupons<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        ClassIdOf<T>,
        Twox64Concat,
        TokenIdOf<T>,
        IncubationCouponOf<T>,
        OptionQuery,
    >;

    ///待发放企鹅蛋数量明细
    #[pallet::storage]
    #[pallet::getter(fn pending_balance)]
    pub type PendingBalance<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, BalanceOf<T>, ValueQuery>;

    ///待发放高级孵化劵数量明细
    #[pallet::storage]
    #[pallet::getter(fn pending_incubation)]
    pub type PendingIncubationCoupons<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, BalanceOf<T>, ValueQuery>;

    ///上一次发蛋时间
    #[pallet::storage]
    #[pallet::getter(fn prev_preduce_egg_time)]
    pub type PrevProduceEggTime<T: Config> = StorageValue<_, u64, ValueQuery>;

    ///系统地址，类拥有着地址，冷库地址，劵地址
    #[pallet::storage]
    #[pallet::getter(fn system_address)]
    pub type SystemAddress<T: Config> =
        StorageValue<_, (AccountIdOf<T>, AccountIdOf<T>, AccountIdOf<T>), ValueQuery>;

    ///公企鹅蛋池子
    #[pallet::storage]
    #[pallet::getter(fn male_penguin_egg_pool)]
    pub type MalePenguinEggPool<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    ///待处理的企鹅任务
    #[pallet::storage]
    #[pallet::getter(fn pending_task_penguin)]
    pub type PendingTaskPenguin<T: Config> =
        StorageMap<_, Twox64Concat, BlockNumberOf<T>, PenguinFarmOf<T>, OptionQuery>;

    ///待处理的孵化劵任务
    #[pallet::storage]
    #[pallet::getter(fn pending_task_incubation)]
    pub type PendingTaskIncubation<T: Config> = StorageMap<
        _,
        Twox64Concat,
        BlockNumberOf<T>,
        Vec<(ClassIdOf<T>, TokenIdOf<T>)>,
        ValueQuery,
    >;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub red_penguin: Vec<(AccountIdOf<T>, PenguinConfigOf<T>, usize)>,
        pub yellow_penguin: Vec<(AccountIdOf<T>, PenguinConfigOf<T>, usize)>,
        pub small_yellow_penguin: Vec<(AccountIdOf<T>, PenguinConfigOf<T>, usize)>,
        pub male_penguin: Vec<(AccountIdOf<T>, PenguinConfigOf<T>, usize)>,
        pub init_per_day_supply: BalanceOf<T>,
        pub incubation_coupon_asset: Vec<(AccountIdOf<T>, IncubationCouponOf<T>, usize)>,
        pub low_incubation_coupon_asset: Vec<(AccountIdOf<T>, IncubationCouponOf<T>, usize)>,
        pub prev_produce_time: u64,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            GenesisConfig {
                red_penguin: Default::default(),
                yellow_penguin: Default::default(),
                small_yellow_penguin: Default::default(),
                male_penguin: Default::default(),
                incubation_coupon_asset: Default::default(),
                init_per_day_supply: Default::default(),
                low_incubation_coupon_asset: Default::default(),
                prev_produce_time: 0u64,
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T>
    where
        <<T as pallet::Config>::Currency as frame_support::traits::Currency<
            <T as frame_system::Config>::AccountId,
        >>::Balance: From<u128>,
    {
        fn build(&self) {
            RedTokenId::<T>::set(1000000u32.into());
            YellowTokenId::<T>::set(2000000u32.into());
            SmallYellowTokenId::<T>::set(5000000u32.into());
            MakeTokenId::<T>::set(7000000u32.into());

            IncubationTokenId::<T>::set(9000000u32.into());
            SystemAddress::<T>::set((
                <Pallet<T>>::class_owner_id(),
                <Pallet<T>>::cold_storage_owner_id(),
                <Pallet<T>>::incubation_owner_id(),
            ));
            RedPenguinClassId::<T>::set(
                <Pallet<T>>::get_next_class_id().expect("class id get error"),
            );
            YellowPenguinClassId::<T>::set(
                <Pallet<T>>::get_next_class_id().expect("class id get error"),
            );
            SmallYellowPenguinClassId::<T>::set(
                <Pallet<T>>::get_next_class_id().expect("class id get error"),
            );
            MalePenguinClassId::<T>::set(
                <Pallet<T>>::get_next_class_id().expect("class id get error"),
            );
            IncubationCouponClassId::<T>::set(
                <Pallet<T>>::get_next_class_id().expect("class id get error"),
            );
            LowIncubationCouponClassId::<T>::set(
                <Pallet<T>>::get_next_class_id().expect("class id get error"),
            );

            let red_penguin_class_id = RedPenguinClassId::<T>::get();
            let yellow_penguin_class_id = YellowPenguinClassId::<T>::get();
            let small_yellow_penguin_class_id = SmallYellowPenguinClassId::<T>::get();
            let male_penguin_class_id = MalePenguinClassId::<T>::get();
            let incubation_coupon_class_id = IncubationCouponClassId::<T>::get();
            let low_incubation_coupon_class_id = LowIncubationCouponClassId::<T>::get();

            let block: u128 = s!(T::InitSupplyPeriod::get());
            let eggs_per_day = T::InitTotalSupply::get()
                .checked_div(&block.into())
                .expect("eggs_per_day error");
            log::info!("red penguin egg rate {:?}", T::RedPenguinEggRate::get());
            log::info!(
                "red penguin egg result {:?}",
                T::RedPenguinEggRate::get() * eggs_per_day
            );

            RedPenguinProduceRate::<T>::set(T::RedPenguinEggRate::get() * eggs_per_day);
            YellowPenguinProduceRate::<T>::set(T::YellowPenguinEggRate::get() * eggs_per_day);
            MalePenguinProduceRate::<T>::set(T::MalePenguinEggRate::get() * eggs_per_day);
            PrevProduceEggTime::<T>::set(self.prev_produce_time);
            let mut remain_supply: BalanceOf<T> = Default::default();
            let current_block = frame_system::Pallet::<T>::current_block_number();
            self.red_penguin
                .clone()
                .into_iter()
                .for_each(|(account_id, penguin_of, count)| {
                    let mut penguin_ids: Vec<TokenIdOf<T>> = vec![];

                    for _ in 0..count {
                        let id = <Pallet<T>>::get_next_red_id().expect("token id get error");
                        Penguins::<T>::insert(
                            red_penguin_class_id,
                            id,
                            PenguinFarmOf::<T>::RedPenguin(RedPenguin {
                                owner: account_id.clone(),
                                start: penguin_of.start,
                                pre_eat_at: current_block,
                                status: penguin_of.status,
                                eggs: penguin_of.eggs,
                                asset_id: id,
                                eat_count: 0u32,
                                class_id: red_penguin_class_id,
                                incubation_remain: Default::default(),
                            }),
                        );
                        remain_supply = remain_supply + penguin_of.eggs;
                        penguin_ids.push(id);
                    }

                    let _: Result<_, DispatchError> =
                        OwnerRedPenguin::<T>::try_mutate(account_id, |red_penguin| {
                            red_penguin.extend(penguin_ids.into_iter());
                            Ok(())
                        });
                });

            self.yellow_penguin
                .clone()
                .into_iter()
                .for_each(|(account_id, penguin_of, count)| {
                    let mut penguin_ids: Vec<TokenIdOf<T>> = vec![];

                    for _ in 0..count {
                        let id = <Pallet<T>>::get_next_yellow_id().expect("token id get error");
                        Penguins::<T>::insert(
                            yellow_penguin_class_id,
                            id,
                            PenguinFarmOf::<T>::YellowPenguin(YellowPenguin {
                                owner: account_id.clone(),
                                start: penguin_of.start,
                                pre_eat_at: current_block,
                                status: penguin_of.status,
                                eggs: penguin_of.eggs,
                                eat_count: 0u32,
                                asset_id: id,
                                class_id: yellow_penguin_class_id,
                                incubation_remain: Default::default(),
                            }),
                        );
                        remain_supply = remain_supply + penguin_of.eggs;
                        penguin_ids.push(id);
                    }
                    let _: Result<_, DispatchError> =
                        OwnerYellowPenguin::<T>::try_mutate(account_id, |yellow_penguin| {
                            yellow_penguin.extend(penguin_ids.into_iter());
                            Ok(())
                        });
                });

            self.small_yellow_penguin.clone().into_iter().for_each(
                |(account_id, penguin_of, count)| {
                    let mut penguin_ids: Vec<TokenIdOf<T>> = vec![];

                    for _ in 0..count {
                        let id = <Pallet<T>>::get_next_yellow_id().expect("token id get error");
                        Penguins::<T>::insert(
                            small_yellow_penguin_class_id,
                            id,
                            PenguinFarmOf::<T>::SmallYellowPenguin(SmallYellowPenguin {
                                owner: account_id.clone(),
                                start: penguin_of.start,
                                pre_eat_at: current_block,
                                status: penguin_of.status,
                                asset_id: id,
                                eat_count: 0u32,
                                class_id: small_yellow_penguin_class_id,
                                grow_value: Default::default(),
                            }),
                        );
                        penguin_ids.push(id);
                    }
                    let _: Result<_, DispatchError> = OwnerSmallYellowPenguin::<T>::try_mutate(
                        account_id,
                        |small_yellow_penguin| {
                            small_yellow_penguin.extend(penguin_ids.into_iter());
                            Ok(())
                        },
                    );
                },
            );

            self.male_penguin
                .clone()
                .into_iter()
                .for_each(|(account_id, penguin_of, count)| {
                    let mut penguin_ids: Vec<TokenIdOf<T>> = vec![];

                    for _ in 0..count {
                        let id = <Pallet<T>>::get_next_yellow_id().expect("token id get error");
                        Penguins::<T>::insert(
                            male_penguin_class_id,
                            id,
                            PenguinFarmOf::<T>::MalePenguin(MalePenguin {
                                owner: account_id.clone(),
                                start: penguin_of.start,
                                pre_eat_at: current_block,
                                eggs: penguin_of.eggs,
                                status: penguin_of.status,
                                eat_count: 0u32,
                                asset_id: id,
                                class_id: male_penguin_class_id,
                                incubation_remain: Default::default(),
                            }),
                        );
                        remain_supply = remain_supply + penguin_of.eggs;
                        penguin_ids.push(id);
                    }
                    let _: Result<_, DispatchError> =
                        OwnerMalePenguin::<T>::try_mutate(account_id, |male_penguin| {
                            male_penguin.extend(penguin_ids.into_iter());
                            Ok(())
                        });
                });

            self.incubation_coupon_asset.clone().into_iter().for_each(
                |(account_id, incubation_coupon_of, count)| {
                    let mut incubation_ids: Vec<TokenIdOf<T>> = vec![];
                    for _ in 0..count {
                        let id = <Pallet<T>>::get_next_incubation_id().expect("token id get error");
                        IncubationCoupons::<T>::insert(
                            male_penguin_class_id,
                            id,
                            IncubationCouponOf::<T> {
                                owner: account_id.clone(),
                                start: incubation_coupon_of.start,
                                status: incubation_coupon_of.status,
                                asset_id: id,
                                class_id: incubation_coupon_class_id,
                            },
                        );
                        incubation_ids.push(id);
                    }
                    let _: Result<_, DispatchError> =
                        OwnerIncubationCouponAsset::<T>::try_mutate(account_id, |incubation| {
                            incubation.extend(incubation_ids.into_iter());
                            Ok(())
                        });
                },
            );

            self.low_incubation_coupon_asset
                .clone()
                .into_iter()
                .for_each(|(account_id, incubation_coupon_of, count)| {
                    let mut incubation_ids: Vec<TokenIdOf<T>> = vec![];

                    for _ in 0..count {
                        let id = <Pallet<T>>::get_next_incubation_id().expect("token id get error");
                        IncubationCoupons::<T>::insert(
                            male_penguin_class_id,
                            id,
                            IncubationCouponOf::<T> {
                                owner: account_id.clone(),
                                start: incubation_coupon_of.start,
                                status: incubation_coupon_of.status,
                                asset_id: id,
                                class_id: low_incubation_coupon_class_id,
                            },
                        );
                        incubation_ids.push(id);
                    }
                    let _: Result<_, DispatchError> =
                        OwnerIncubationCouponAsset::<T>::try_mutate(account_id, |incubation| {
                            incubation.extend(incubation_ids.into_iter());
                            Ok(())
                        });
                });
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberOf<T>> for Pallet<T>
    where
        <<T as pallet::Config>::Currency as frame_support::traits::Currency<
            <T as frame_system::Config>::AccountId,
        >>::Balance: From<u128>,
    {
        fn on_finalize(now: T::BlockNumber) {
            let expire_incoubation =
                PendingTaskIncubation::<T>::get(now - T::IncubationLivePeriod::get());
            expire_incoubation
                .into_iter()
                .for_each(|(class_id, token_id)| {
                    if class_id == IncubationCouponClassId::<T>::get() {
                        IncubationCoupons::<T>::remove(class_id, token_id);
                    } else {
                        LowIncubationCoupons::<T>::remove(class_id, token_id);
                    }
                });
            //修改企鹅状态(喂养时间，喂养状态)
        }

        fn on_initialize(now: T::BlockNumber) -> Weight {
            T::DbWeight::get().reads_writes(2, 1)
        }
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T>
    where
        <<T as pallet::Config>::Currency as frame_support::traits::Currency<
            <T as frame_system::Config>::AccountId,
        >>::Balance: From<u128>,
    {
        //!
        //! 收获企鹅蛋，会根据喂养次数计算收成，
        //! 每20枚企鹅蛋产生一张孵化劵，孵化劵有一定概率孵化出公企鹅
        //!
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn claim_penguin_egg(
            origin: OriginFor<T>,
            #[pallet::compact] class_id: ClassIdOf<T>,
            #[pallet::compact] token_id: TokenIdOf<T>,
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;

            let penguin =
                Penguins::<T>::get(class_id, token_id).ok_or(Error::<T>::PenguinNoExist)?;
            ensure!(
                class_id == RedPenguinClassId::<T>::get()
                    || class_id == YellowPenguinClassId::<T>::get()
                    || class_id == MalePenguinClassId::<T>::get(),
                Error::<T>::PenguinIsSmallYellow
            );
            ensure!(
                PenguinStatus::Active == penguin.query_penguin_status(),
                Error::<T>::PenguinStatusError
            );
            let penguin_produce_egg_rate: BalanceOf<T> =
                <Pallet<T>>::compute_produce_egg_rate(&penguin);
            let mut produce_egg: BalanceOf<T> = Default::default();
            let block_number = frame_system::Pallet::<T>::current_block_number();
            match &penguin {
                PenguinFarm::RedPenguin(redPenguin) => {
                    let mut new_penguin = redPenguin.clone();
                    let RedPenguin {
                        pre_eat_at,
                        owner,
                        eat_count,
                        incubation_remain,
                        eggs,
                        ..
                    } = redPenguin;
                    ensure!(owner == &caller, Error::<T>::NoPermission);
                    //企鹅产蛋率 * 喂养次数
                    let eat_count =
                        if block_number - *pre_eat_at < s!(T::PenguinProducePeriod::get()) {
                            new_penguin.eat_count = 1u32;
                            eat_count - 1
                        } else {
                            new_penguin.eat_count = 0u32;
                            *eat_count
                        };
                    produce_egg = penguin_produce_egg_rate
                        * UniqueSaturatedFrom::<u32>::unique_saturated_from(eat_count);
                    new_penguin.eggs = new_penguin.eggs + produce_egg;
                    //处理孵化劵张数，发放孵化劵
                    let incubation_count = UniqueSaturatedInto::<usize>::unique_saturated_into(
                        *incubation_remain / T::RedPenguinEggCountForIncubation::get(),
                    );
                    new_penguin.incubation_remain = incubation_remain
                        .checked_rem(&T::RedPenguinEggCountForIncubation::get())
                        .ok_or(Error::<T>::IncubationRemainError)?;
                    new_penguin.eggs = *eggs + produce_egg;
                    let incubation_coupon_class_id = IncubationCouponClassId::<T>::get();
                    let mut incubation_ids = sp_std::vec![];

                    let _: Vec<TokenIdOf<T>> = (0..incubation_count)
                        .filter_map(|_| {
                            let id: TokenIdOf<T> = <Pallet<T>>::get_next_incubation_id()
                                .map_err(|_| Error::<T>::IncubationTokenIdError)
                                .ok()?;
                            incubation_ids.push(id);
                            IncubationCoupons::<T>::insert(
                                incubation_coupon_class_id,
                                id.clone(),
                                IncubationCouponOf::<T> {
                                    owner: owner.clone(),
                                    start: block_number,
                                    status: CouponStatus::Liquid,
                                    asset_id: id,
                                    class_id: incubation_coupon_class_id,
                                },
                            );
                            let _: Result<
                                Vec<(
                                    <T as pallet::Config>::ClassId,
                                    <T as pallet::Config>::TokenId,
                                )>,
                                DispatchError,
                            > = PendingTaskIncubation::<T>::try_mutate(
                                block_number + T::IncubationLivePeriod::get(),
                                |value| {
                                    let old_value = value.clone();
                                    value.push((incubation_coupon_class_id, id));
                                    Ok(old_value)
                                },
                            );
                            Some(id)
                        })
                        .collect::<Vec<TokenIdOf<T>>>();

                    Self::deposit_event(Event::<T>::Incubations(incubation_ids, block_number));
                    //发放企鹅蛋
                    let _ = <T as pallet::Config>::Currency::deposit_into_existing(
                        &caller,
                        produce_egg,
                    );
                    Self::deposit_event(Event::<T>::Claim(
                        PenguinFarmOf::<T>::RedPenguin(new_penguin),
                        produce_egg,
                    ));
                }
                PenguinFarm::YellowPenguin(YellowPenguin {
                    owner,
                    eat_count,
                    incubation_remain,
                    eggs,
                    ..
                }) => {}
                PenguinFarm::MalePenguin(MalePenguin {
                    owner,
                    eat_count,
                    incubation_remain,
                    eggs,
                    ..
                }) => {}
                PenguinFarm::SmallYellowPenguin(_) => {}
            }

            Ok(().into())
        }

        ///喂养企鹅
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn feed_penguin(
            origin: OriginFor<T>,
            #[pallet::compact] class_id: ClassIdOf<T>,
            #[pallet::compact] token_id: TokenIdOf<T>,
        ) -> DispatchResultWithPostInfo {
            Ok(().into())
        }

        ///孵化企鹅
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn incubation_penguin(
            origin: OriginFor<T>,
            #[pallet::compact] class_id: ClassIdOf<T>,
            #[pallet::compact] token_id: TokenIdOf<T>,
        ) -> DispatchResultWithPostInfo {
            Ok(().into())
        }

        ///交易企鹅
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn transfor_penguin(
            origin: OriginFor<T>,
            #[pallet::compact] class_id: ClassIdOf<T>,
            #[pallet::compact] token_id: TokenIdOf<T>,
        ) -> DispatchResultWithPostInfo {
            Ok(().into())
        }

        ///挂拍卖 当天喂养次数清除，
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn bid(
            origin: OriginFor<T>,
            #[pallet::compact] class_id: ClassIdOf<T>,
            #[pallet::compact] token_id: TokenIdOf<T>,
        ) -> DispatchResultWithPostInfo {
			let caller =ensure_signed(origin)?;
            let penguin =
                Penguins::<T>::get(class_id, token_id).ok_or(Error::<T>::PenguinNoExist)?;

            match penguin {
                PenguinFarm::MalePenguin(MalePenguin { .. }) => {
                    return Err(Error::<T>::MalePenguinBanBid)?
                }
                PenguinFarm::RedPenguin(red_penguin) => {
                    bid_penguin!(red_penguin, RedPenguin, class_id, token_id,caller)
                }
                PenguinFarm::YellowPenguin(yellow_penguin) => {
                    bid_penguin!(yellow_penguin, YellowPenguin, class_id, token_id,caller)
                }
                PenguinFarm::SmallYellowPenguin(small_yellow) => {
                    bid_penguin!(small_yellow, SmallYellowPenguin, class_id, token_id,caller)
                }
            }
            Self::deposit_event(Event::<T>::Bid(class_id, token_id));

            Ok(().into())
        }

        ///取消拍卖 ，
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn unbid(
            origin: OriginFor<T>,
            #[pallet::compact] class_id: ClassIdOf<T>,
            #[pallet::compact] token_id: TokenIdOf<T>,
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            let penguin =Penguins::<T>::get(class_id, token_id).ok_or(Error::<T>::PenguinNoExist)?;

            match penguin {
                PenguinFarm::MalePenguin(MalePenguin { .. }) => {
                    return Err(Error::<T>::MalePenguinBanBid)?
                }
                PenguinFarm::RedPenguin(red_penguin) => {
					unbid_penguin!(
						red_penguin,
						RedPenguin,
						class_id,
						token_id,
						caller,
						false
					);
                }
                PenguinFarm::YellowPenguin(yellow_penguin) => {
                    unbid_penguin!(
                        yellow_penguin,
                        YellowPenguin,
                        class_id,
                        token_id,
                        caller,
                        true
                    );
                }
                PenguinFarm::SmallYellowPenguin(small_yellow_penguin) => {
                    unbid_penguin!(
                        small_yellow_penguin,
                        SmallYellowPenguin,
                        class_id,
                        &token_id,
                        caller,
                        false
                    );
                }
            }
            Self::deposit_event(Event::<T>::UnBid(class_id, token_id));
            Ok(().into())
        }
    }
}

impl<T: Config> Pallet<T>
where
    <<T as pallet::Config>::Currency as frame_support::traits::Currency<
        <T as frame_system::Config>::AccountId,
    >>::Balance: From<u128>,
{
    pub fn compute_produce_egg_rate(penguin: &PenguinFarmOf<T>) -> BalanceOf<T> {
        match penguin {
            PenguinFarm::RedPenguin(_) => {
                let red_penguin_count = <Pallet<T>>::query_red_penguin_num();

                let red_penguin_count = if red_penguin_count == Zero::zero() {
                    One::one()
                } else {
                    red_penguin_count
                };
                RedPenguinProduceRate::<T>::get() / red_penguin_count
            }
            PenguinFarm::YellowPenguin(_) => {
                let yellow_penguin_num = <Pallet<T>>::query_yellow_penguin_num();
                let yellow_penguin_num = if yellow_penguin_num == Zero::zero() {
                    One::one()
                } else {
                    yellow_penguin_num
                };
                YellowPenguinProduceRate::<T>::get() / yellow_penguin_num
            }
            PenguinFarm::MalePenguin(_) => {
                let male_penguin_num = <Pallet<T>>::query_male_penguin_num();
                let male_penguin_num = if male_penguin_num == Zero::zero() {
                    One::one()
                } else {
                    male_penguin_num
                };
                let current_pool = MalePenguinEggPool::<T>::get();
                let male_penguin_produce_rate = current_pool / male_penguin_num;
                let male_penguin_produce_rate =
                    if male_penguin_produce_rate > T::MalePenguinEggLimit::get() {
                        s!(5000u128)
                    } else {
                        male_penguin_produce_rate
                    };
                male_penguin_produce_rate
            }
            PenguinFarm::SmallYellowPenguin(_) => Zero::zero(),
        }
    }
    pub fn class_owner_id() -> AccountIdOf<T> {
        T::ClassOwnerId::get().into_account()
    }

    pub fn incubation_owner_id() -> AccountIdOf<T> {
        T::IncubationId::get().into_account()
    }

    pub fn cold_storage_owner_id() -> AccountIdOf<T> {
        T::ColdStorageId::get().into_account()
    }

    pub fn get_next_class_id() -> Result<T::ClassId, DispatchError> {
        ClassId::<T>::try_mutate(|pre| {
            let current = *pre;
            *pre = pre
                .checked_add(&One::one())
                .ok_or(Error::<T>::NoAvailableClassId)?;
            Ok(current)
        })
    }

    pub fn get_next_red_id() -> Result<TokenIdOf<T>, DispatchError> {
        RedTokenId::<T>::try_mutate(|pre| {
            let current = *pre;
            *pre = pre
                .checked_add(&One::one())
                .ok_or(Error::<T>::NoAvailableClassId)?;
            Ok(current)
        })
    }

    pub fn get_next_yellow_id() -> Result<TokenIdOf<T>, DispatchError> {
        YellowTokenId::<T>::try_mutate(|pre| {
            let current = *pre;
            *pre = pre
                .checked_add(&One::one())
                .ok_or(Error::<T>::NoAvailableTokenId)?;
            Ok(current)
        })
    }

    pub fn get_next_small_yellow_id() -> Result<TokenIdOf<T>, DispatchError> {
        SmallYellowTokenId::<T>::try_mutate(|pre| {
            let current = *pre;
            *pre = pre
                .checked_add(&One::one())
                .ok_or(Error::<T>::NoAvailableTokenId)?;
            Ok(current)
        })
    }
    pub fn get_next_male_id() -> Result<TokenIdOf<T>, DispatchError> {
        MakeTokenId::<T>::try_mutate(|pre| {
            let current = *pre;
            *pre = pre
                .checked_add(&One::one())
                .ok_or(Error::<T>::NoAvailableTokenId)?;
            Ok(current)
        })
    }

    pub fn get_next_incubation_id() -> Result<TokenIdOf<T>, DispatchError> {
        IncubationTokenId::<T>::try_mutate(|pre| {
            let current = *pre;
            *pre = pre
                .checked_add(&One::one())
                .ok_or(Error::<T>::NoAvailableTokenId)?;
            Ok(current)
        })
    }
    pub fn query_red_penguin_num() -> BalanceOf<T> {
        let count = Penguins::<T>::iter_prefix_values(RedPenguinClassId::<T>::get())
            .map(|_| 1u32)
            .sum::<u32>();
        count.into()
    }

    pub fn query_all_penguin_num() -> BalanceOf<T> {
        let count = Penguins::<T>::iter().map(|_| 1u32).sum::<u32>();
        count.into()
    }

    pub fn query_yellow_penguin_num() -> BalanceOf<T> {
        let count = Penguins::<T>::iter_prefix_values(YellowPenguinClassId::<T>::get())
            .map(|_| 1u32)
            .sum::<u32>();
        count.into()
    }

    pub fn query_male_penguin_num() -> BalanceOf<T> {
        let count = Penguins::<T>::iter_prefix_values(MalePenguinClassId::<T>::get())
            .map(|_| 1u32)
            .sum::<u32>();
        count.into()
    }
}
