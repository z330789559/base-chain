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
use sp_std::vec::Vec;
use frame_support::sp_runtime::traits::{AccountIdConversion, CheckedAdd, CheckedDiv, CheckedSub, One, Zero};
use sp_arithmetic::traits::CheckedRem;
use sp_runtime::traits::BlockNumberProvider;

use primitive::DAYS;
use primitive::MALE_PENGUIN_RATE;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

macro_rules! s {
    ($e: expr) => {
        sp_runtime::SaturatedConversion::saturated_into($e)
    };
}

macro_rules! purchase_penguin {
    ($penguin:expr,$penguin_type:ident,$owner_penguin:ident,$caller:expr,$class_id:expr,$token_id:expr,$penguin_form:expr) => {{
        let mut new_penguin = $penguin.clone();
        let $penguin_type { owner, status,.. } = $penguin;
        ensure!(&$caller != owner, Error::<T>::CanNotBuySelf);
        ensure!(
            status == &PenguinStatus::Bid,
            Error::<T>::PenguinStatusError
        );
        let price: BalanceOf<T> = BidPenguin::<T>::get(($class_id, $token_id));
        let amount = <T as Config>::Currency::total_balance(&$caller);
        ensure!(amount > price, Error::<T>::BidBalanceNoEnough);
        let commission = T::BidCommissionRate::get() * price;
        <T as Config>::Currency::transfer(
            &$caller,
            owner,
            price - commission,
            ExistenceRequirement::KeepAlive,
        )?;
        <T as Config>::Currency::withdraw(
            &$caller,
            commission,
            WithdrawReasons::FEE,
            ExistenceRequirement::KeepAlive,
        )?;
		//挂拍卖蛋企鹅状态都是hunger
        BidPenguin::<T>::remove(($class_id, $token_id));
		if status != &PenguinStatus::Hunger {
							new_penguin.status = PenguinStatus::Hunger;
						};
        new_penguin.owner = $caller.clone();
           $owner_penguin::<T>::try_mutate(owner, |ids| {
            let index = ids.binary_search(&$token_id)?;
            ids.remove(index);
            Ok(())
        }).map_err(|_:usize|Error::<T>::RemoveTokenIdError)?;
		       $owner_penguin::<T>::try_mutate($caller.clone(), |ids| {
                    ids.push($token_id);
            Ok(())
        }).map_err(|_:usize|Error::<T>::AddTokenIdError)?;
			update_penguin!(@ $class_id, $token_id,$penguin_type,new_penguin);
        Self::deposit_event(Event::<T>::Purchase(
            $caller,
            ($class_id, $token_id),
            owner.clone(),
            price,
        ));
    }};
}
macro_rules! bid_penguin {
    // 红 和 小黄的逻辑
    ($e:expr,$tt:ident,$class_id:expr,$token_id:expr,$caller:expr,$flag:expr) => {{
        let current_block = frame_system::Pallet::<T>::current_block_number();
        let mut new_penguin = $e.clone();
        let $tt {
            eat_count,
            owner,
            start,
			status,
            ..
        } = $e;
        ensure!($caller == owner.clone(), Error::<T>::NoPermission);
		//小黄 确保大于14天 ，红企鹅确保产的蛋已经领取
        if ($flag) {
            ensure!(
                current_block - start >= T::SmallYellowPenguinGrowPeriod::get(),
                Error::<T>::SmallYellowLiveTooShort
            );
        }else{
		 ensure!(!(eat_count ==1u32 && new_penguin.pre_eat_at > T::PenguinProducePeriod::get()),Error::<T>::NeedClaimPenguinEgg)
		}
		ensure!(PenguinStatus::Active ==status ||PenguinStatus::Hunger ==status,  Error::<T>::PenguinStatusError );
        new_penguin.status = PenguinStatus::Bid;

        if new_penguin.pre_eat_at < T::PenguinProducePeriod::get()   {
			if current_block  >   T::PenguinProducePeriod::get(){
				     new_penguin.pre_eat_at = current_block - T::PenguinProducePeriod::get();
			}else{
				new_penguin.pre_eat_at = Zero::zero();
			}

        }
	update_penguin!(@ $class_id, $token_id,$tt,new_penguin);
		 let _: Result<
                                Vec<(
                                    _,
                                    _,
                                )>,
                                DispatchError,
                            > = PendingTaskPenguin::<T>::try_mutate(
                                current_block + T::BidMaxPeriod::get(),
                                |value| {
                                    let old_value = value.clone();
                                    value.push(($class_id, $token_id));
                                    Ok(old_value)
                                },
                            );
    }};
    // 黄的处理逻辑
	($e:expr,$tt:ident,$class_id:expr,$token_id:expr,$caller:expr,$death_period:expr,$penguin_owner:ident) => {{
        let current_block = frame_system::Pallet::<T>::current_block_number();
        let mut new_penguin = $e.clone();
        let $tt {
            pre_eat_at,
            eat_count,
            owner,
			status,
            ..
        } = $e;
        ensure!($caller == owner.clone(), Error::<T>::NoPermission);
        let bid_perd=T::BidMaxPeriod::get();
        log::info!("current:{:?}, yellow pre_eat_at {:?}, death_period:{:?},bid_period:{:?}, PenguinProducePeriod:{:?}",
            current_block,pre_eat_at,$death_period,bid_perd,T::PenguinProducePeriod::get());
		ensure!(current_block - pre_eat_at < $death_period - bid_perd,Error::<T>::PenguinNeedFeed);
        /*if current_block > pre_eat_at + $death_period {
              Self::yellow_penguin_death($class_id, $token_id, owner.clone())?;
              return Err(Error::<T>::PenguinHadDeath)?;
        }*/

		ensure!(PenguinStatus::Active ==status ||PenguinStatus::Hunger ==status,  Error::<T>::PenguinStatusError );
		ensure!(!(eat_count ==1u32 && new_penguin.pre_eat_at > T::PenguinProducePeriod::get()),Error::<T>::NeedClaimPenguinEgg);
        new_penguin.status = PenguinStatus::Bid;
        if new_penguin.pre_eat_at < T::PenguinProducePeriod::get()   {
			if current_block  >   T::PenguinProducePeriod::get(){
				     new_penguin.pre_eat_at = current_block - T::PenguinProducePeriod::get();
				     new_penguin.eat_count=0;
			}else{
				new_penguin.pre_eat_at = Zero::zero();
			}

        }
			update_penguin!(@ $class_id, $token_id,$tt,new_penguin);
		 let _: Result<
                                Vec<(
                                    _,
                                    _,
                                )>,
                                DispatchError,
                            > = PendingTaskPenguin::<T>::try_mutate(
                                current_block + T::BidMaxPeriod::get(),
                                |value| {
                                    let old_value = value.clone();
                                    value.push(($class_id, $token_id));
                                    Ok(old_value)
                                },
                            );

    }};
}

macro_rules! unbid_penguin {
    ($e:expr,$tt:ident,$class_id:expr,$token_id:expr,$caller:expr,$flag:expr) => {{
					let mut new_penguin = $e.clone();
					let $tt{
						status,
						owner,
						..
				} = $e;
					ensure!(PenguinStatus::Bid == status,Error::<T>::MustNeedIsBid);
				ensure!($caller == owner.clone(), Error::<T>::NoPermission);
		           if status != PenguinStatus::Hunger {
							new_penguin.status = PenguinStatus::Hunger;
							update_penguin!(@ $class_id, $token_id,$tt,new_penguin);
						};
    }};
}

macro_rules! update_penguin {
    ($class_id:expr,$token_id:expr,$tt:ident,$new_penguin:expr) => {{
        Penguins::<T>::mutate($class_id, $token_id, |penguin| {
            sp_std::mem::swap(penguin, &mut Some(PenguinFarmOf::<T>::$tt($new_penguin)));
        });
    }};
    (@ $class_id:tt,$token_id:tt,$tt:tt,$new_penguin:expr) => {{
        log::info!("update_penguin@");
        Penguins::<T>::mutate($class_id, $token_id, |penguin| {
            sp_std::mem::swap(penguin, &mut Some(PenguinFarmOf::<T>::$tt($new_penguin)));
        });
    }};
}

macro_rules! unbid_penguin_system {
    ($e:expr,$tt:ident,$class_id:expr,$token_id:expr,$flag:expr) => {{
        let mut new_penguin = $e.clone();
        let $tt { status, .. } = $e;
        if PenguinStatus::Bid != status {
            return;
        }
        new_penguin.status = PenguinStatus::Hunger;
        Penguins::<T>::mutate($class_id, $token_id, |penguin| {
            sp_std::mem::swap(penguin, &mut Some(PenguinFarmOf::<T>::$tt(new_penguin)));
        });
    }};
}

mod weights;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
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

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum PenguinStatus {
    ///正常
    Active,
    ///饥饿
    Hunger,
    ///挂单
    Bid,
    ///死亡
    Death,
}

impl Default for PenguinStatus {
    fn default() -> Self { PenguinStatus::Death }
}


#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CouponStatus {
    //未激活
    UnActive,
    //流通
    Liquid,
    //收回
    Retire,
    //孵化
    Hatch,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
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


#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct IncubationCoupon<BlockNumber, AccountId, ClassId, AssetId> {
    pub owner: AccountId,
    pub start: BlockNumber,
    pub status: CouponStatus,
    pub asset_id: AssetId,
    pub class_id: ClassId,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct PenguinConfig<BlockNumber, AccountId, Balance> {
    pub owner: AccountId,
    pub start: BlockNumber,
    pub pre_eat_at: BlockNumber,
    pub status: PenguinStatus,
    pub eggs: Balance,
}
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
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
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
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
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
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

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
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
    use frame_support::dispatch::{Dispatchable, HasCompact};
    use frame_support::sp_runtime::traits::{
        AtLeast32BitUnsigned, BlockNumberProvider, Saturating, UniqueSaturatedInto,
    };
    use frame_support::sp_runtime::Permill;
    use frame_support::sp_std::convert::TryFrom;
    use frame_support::sp_std::ops::Add;
    use frame_support::traits::schedule::{Anon, DispatchTime};
    use frame_support::traits::Randomness;
    use frame_support::traits::{ExistenceRequirement, UnixTime, WithdrawReasons};
    use frame_support::transactional;
    use frame_support::PalletId;
    use frame_system::pallet_prelude::OriginFor;
    use frame_system::{ensure_none, ensure_root, ensure_signed};

    use sp_runtime::traits::Hash;

    #[pallet::config]
    pub trait Config:
        frame_system::Config + pallet_assets::Config + pallet_scheduler::Config
    {
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
            + HasCompact
            + Parameter
            + From<u32>
            + Into<u32>
            + Copy;

        type Call: Parameter
            + Dispatchable<Origin = <Self as frame_system::Config>::Origin>
            + From<Call<Self>>;

        type ClassId: Parameter
            + Member
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + HasCompact
            + From<u32>
            + Into<u32>
            + MaybeSerializeDeserialize;

        type TokenId: Parameter
            + Member
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + HasCompact
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
        ///挂单佣金率
        #[pallet::constant]
        type BidCommissionRate: Get<Permill>;

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

        // ///运营团队当前分配比率
        // #[pallet::constant]
        // type MalePenguinRate: Get<Permill>;

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

        ///孵化劵存活时间
        #[pallet::constant]
        type IncubationLivePeriod: Get<<Self as frame_system::Config>::BlockNumber>;

        ///挂单最大时间
        #[pallet::constant]
        type BidMaxPeriod: Get<<Self as frame_system::Config>::BlockNumber>;

        ///红企鹅产生孵化劵需要蛋数
        #[pallet::constant]
        type RedPenguinEggCountForIncubation: Get<BalanceOf<Self>>;

        type PalletsOrigin: From<frame_system::RawOrigin<Self::AccountId>>;

        ///黄企鹅产生孵化劵需要蛋数
        #[pallet::constant]
        type YellowPenguinEggCountForIncubation: Get<BalanceOf<Self>>;

        type Schedule: frame_support::traits::schedule::Anon<
            <Self as frame_system::Config>::BlockNumber,
            <Self as Config>::Call,
            <Self as Config>::PalletsOrigin,
        >;

        type Randomness: Randomness<
            <Self as frame_system::Config>::Hash,
            <Self as frame_system::Config>::BlockNumber,
        >;

        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        Claim(ClassIdOf<T>, TokenIdOf<T>, BalanceOf<T>),
        Incubation(IncubationCouponOf<T>),
        Incubations(Vec<TokenIdOf<T>>, BlockNumberOf<T>),
        LowIncubations(Vec<TokenIdOf<T>>, BlockNumberOf<T>),
        LowIncubation(IncubationCouponOf<T>),
        Bid(ClassIdOf<T>, TokenIdOf<T>),
        UnBid(ClassIdOf<T>, TokenIdOf<T>),
        Purchase(
            AccountIdOf<T>,
            (ClassIdOf<T>, TokenIdOf<T>),
            AccountIdOf<T>,
            BalanceOf<T>,
        ),
        AdminChanged(AccountIdOf<T>),
        HightCouponExpire(ClassIdOf<T>, TokenIdOf<T>),
        LowCouponExpire(ClassIdOf<T>, TokenIdOf<T>),
        PenguinDead(ClassIdOf<T>, TokenIdOf<T>),
        PenguinUpgrade(ClassIdOf<T>, TokenIdOf<T>, ClassIdOf<T>, TokenIdOf<T>),
        FeedPenguinSuccess(ClassIdOf<T>, TokenIdOf<T>),
        ///公企鹅🐧被移除，Male Penguin removed
        MalePenguinRemoved(ClassIdOf<T>, TokenIdOf<T>,AccountIdOf<T>),
        ///手续费收取
        FeeCollected(AccountIdOf<T>, BalanceOf<T>),
    }

    /// Error for non-fungible-token module.
    #[pallet::error]
    pub enum Error<T> {
        ///增加token错误
        AddTokenIdError,
        ///企鹅已经死亡
        PenguinHadDeath,
        ///超过最大数量限制
        MaxNumberLimit,
        ///不支持迁入
        NoSupportMoveIn,
        ///企鹅需要喂养
        PenguinNeedFeed,
        ///移除TokenId错误
        RemoveTokenIdError,
        ///移除公企鹅id错误
        RemoveMalePenguinError,
        ///删除孵化券错误
        RemoveCouponError,
        ///企鹅蛋已经领取
        PenguinEggHadClaim,
        ///企鹅产蛋间隔时间太短
        PenguinProduceTooShort,
        ///孵化劵已经过期
        IncubationCouponExpire,
        ///孵化劵不存在
        IncubationCouponNoExist,
        ///孵化劵类别不对
        IncubationCouponClassError,
        ///孵化蛋不足
        IncubationBalanceNoEnough,
        ///企鹅处于吃饱状态
        PenguinNoHunger,
        ///管理员不存在
        AdminNoExist,
        ///管理员已经存在
        AdminHadExist,
        ///要求管理员
        RequireAdmin,
        ///购买金额不足
        BidBalanceNoEnough,
        ///拍卖价格不匹配
        BidPriceUnmatch,
        ///必须是bdi状态
        MustNeedIsBid,
        ///不能到达的地方
        UnBidUnReach,
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
        SmallYellowLiveTooShort,

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
        ///不能买自己的企鹅
        CanNotBuySelf,
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

    /// 红企鹅拥有数量
    #[pallet::storage]
    #[pallet::getter(fn owner_red_penguin)]
    pub type OwnerRedPenguin<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Vec<TokenIdOf<T>>, ValueQuery>;

    /// 黄企鹅拥有数量
    #[pallet::storage]
    #[pallet::getter(fn owner_yellow_penguin)]
    pub type OwnerYellowPenguin<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Vec<TokenIdOf<T>>, ValueQuery>;

    /// 小黄企鹅拥有数量
    #[pallet::storage]
    #[pallet::getter(fn owner_small_yellow_penguin)]
    pub type OwnerSmallYellowPenguin<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Vec<TokenIdOf<T>>, ValueQuery>;

    /// 公企鹅拥有数量
    #[pallet::storage]
    #[pallet::getter(fn owner_male_penguin)]
    pub type OwnerMalePenguin<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Vec<TokenIdOf<T>>, ValueQuery>;

    #[pallet::type_value]
    pub fn DefaultForEpoch<T: Config>() -> u32 {
        1u32
    }

    /// 当前纪元索引
    #[pallet::storage]
    #[pallet::getter(fn current_epoch)]
    pub type CurrentEpoch<T: Config> = StorageValue<_, u32, ValueQuery, DefaultForEpoch<T>>;

    ///黄企鹅数量
    #[pallet::storage]
    #[pallet::getter(fn yellow_penguin_count)]
    pub type YellowPenguinCount<T: Config> = StorageValue<_, u32, ValueQuery>;

    ///小黄企鹅数量
    #[pallet::storage]
    #[pallet::getter(fn small_yellow_penguin_count)]
    pub type SmallYellowPenguinCount<T: Config> = StorageValue<_, u32, ValueQuery>;

    ///公企鹅数量
    #[pallet::storage]
    #[pallet::getter(fn male_penguin_count)]
    pub type MalePenguinCount<T: Config> = StorageValue<_, u32, ValueQuery>;
    ///当前纪元总金额
    #[pallet::storage]
    #[pallet::getter(fn current_epoch_balance)]
    pub type CurrentEpochBalance<T: Config> =
        StorageMap<_, Twox64Concat, u32, BalanceOf<T>, ValueQuery>;
    ///当前纪元时长
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
    /// 红企鹅的类别id
    #[pallet::storage]
    #[pallet::getter(fn red_peng_class_id)]
    pub type RedPenguinClassId<T: Config> = StorageValue<_, ClassIdOf<T>, ValueQuery>;

    /// 黄企鹅的类别id
    #[pallet::storage]
    #[pallet::getter(fn yellow_penguin_class_id)]
    pub type YellowPenguinClassId<T: Config> = StorageValue<_, ClassIdOf<T>, ValueQuery>;

    /// 小黄企鹅的类别id
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

    ///低级孵化劵当前id
    #[pallet::storage]
    #[pallet::getter(fn next_low_ncubation_token_id)]
    pub type LowIncubationTokenId<T: Config> = StorageValue<_, TokenIdOf<T>, ValueQuery>;

    ///公企鹅类别id
    #[pallet::storage]
    #[pallet::getter(fn male_penguin_class_id)]
    pub type MalePenguinClassId<T: Config> = StorageValue<_, ClassIdOf<T>, ValueQuery>;

    /// 高级孵化眷id
    #[pallet::storage]
    #[pallet::getter(fn incubation_coupon_asset_id)]
    pub type IncubationCouponClassId<T: Config> = StorageValue<_, ClassIdOf<T>, ValueQuery>;

    ///低级孵化劵类别id
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
    pub type PendingTaskPenguin<T: Config> = StorageMap<
        _,
        Twox64Concat,
        BlockNumberOf<T>,
        Vec<(ClassIdOf<T>, TokenIdOf<T>)>,
        ValueQuery,
    >;

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

    ///拍卖中的企鹅
    #[pallet::storage]
    #[pallet::getter(fn bid_penguin)]
    pub type BidPenguin<T: Config> =
        StorageMap<_, Twox64Concat, (ClassIdOf<T>, TokenIdOf<T>), BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn admin)]
    pub(super) type Admin<T: Config> = StorageValue<_, Vec<AccountIdOf<T>>, ValueQuery>;

    #[pallet::genesis_config]
    #[derive(Default)]
    pub struct GenesisConfig {}

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig
    where
        <<T as pallet::Config>::Currency as frame_support::traits::Currency<
            <T as frame_system::Config>::AccountId,
        >>::Balance: From<u128>,
    {
        fn build(&self) {
            <Pallet<T>>::print_init_variable();

            RedTokenId::<T>::set(1000000u32.into());
            YellowTokenId::<T>::set(2000000u32.into());
            SmallYellowTokenId::<T>::set(5000000u32.into());
            MakeTokenId::<T>::set(7000000u32.into());

            IncubationTokenId::<T>::set(9000000u32.into());
            LowIncubationTokenId::<T>::set(10000000u32.into());
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

            // let red_penguin_class_id = RedPenguinClassId::<T>::get();
            // let yellow_penguin_class_id = YellowPenguinClassId::<T>::get();
            // let small_yellow_penguin_class_id = SmallYellowPenguinClassId::<T>::get();
            // let male_penguin_class_id = MalePenguinClassId::<T>::get();
            // let incubation_coupon_class_id = IncubationCouponClassId::<T>::get();
            // let low_incubation_coupon_class_id = LowIncubationCouponClassId::<T>::get();

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

            // 初始化第一个纪元的60个月
            let firstEpoch: T::BlockNumber=  T::BlockNumber::from((DAYS * 60 * 30) as u32) ;
            CurrentEpochPeriod::<T>::insert(1u32,firstEpoch);

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
            //纪元切换逻辑
            //
            let current_epoch = CurrentEpoch::<T>::get();
            let end_point = CurrentEpochPeriod::<T>::get(current_epoch);
            // 1 60月 2  35个月 3 18月 4。。 12隔越
            if end_point == now {
                CurrentEpoch::<T>::set(current_epoch + 1);
                let remain_balance = <T as Config>::InitTotalSupply::get()
                    .checked_sub(&<T as Config>::Currency::total_issuance()).unwrap_or(Default::default());
                CurrentEpochBalance::<T>::insert(current_epoch + 1, remain_balance);
                match current_epoch {
                    1u32 => {
                        let time = now + s!(DAYS * 35 * 30);
                        CurrentEpochPeriod::<T>::insert(current_epoch + 1, time);
                    }
                    2u32 => {
                        let time = now + s!(DAYS * 18 * 30);
                        CurrentEpochPeriod::<T>::insert(current_epoch + 1, time);
                    }
                    _ => {
                        let time = now + s!(DAYS * 12 * 30);
                        CurrentEpochPeriod::<T>::insert(current_epoch + 1, time);
                    }
                }
                let next_end_point = CurrentEpochPeriod::<T>::get(current_epoch + 1);
                let block: u128 = s!(next_end_point - now);

                let eggs_per_day = remain_balance
                    .checked_div(&block.into())
                    .expect("cal eggs_per_day error");
                RedPenguinProduceRate::<T>::set(T::RedPenguinEggRate::get() * eggs_per_day);
                YellowPenguinProduceRate::<T>::set(T::YellowPenguinEggRate::get() * eggs_per_day);
                MalePenguinProduceRate::<T>::set(T::MalePenguinEggRate::get() * eggs_per_day);
            }
        }

        fn on_initialize(now: T::BlockNumber) -> Weight {
            let mut consumed_weight: Weight = 0;

            let mut add_db_reads_writes = |reads, writes| {
                consumed_weight += T::DbWeight::get().reads_writes(reads, writes);
            };
            let expire_incubation = PendingTaskIncubation::<T>::take(
                //now.saturating_sub(T::IncubationLivePeriod::get()),
                now
            );
            expire_incubation
                .into_iter()
                .for_each(|(class_id, token_id)| {
                    if class_id == IncubationCouponClassId::<T>::get() {
                       let _= IncubationCoupons::<T>::take(class_id, token_id).map(|coupon| {
                            OwnerIncubationCouponAsset::<T>::try_mutate(&coupon.owner, |ids| {
                                ids.binary_search(&token_id).map(|index| ids.remove(index))
                            });
                        });

                        Self::deposit_event(Event::<T>::HightCouponExpire(class_id, token_id));
                    } else {
						let _=IncubationCoupons::<T>::take(class_id, token_id).map(|coupon| {
                            OwnerIncubationCouponAsset::<T>::try_mutate(&coupon.owner, |ids| {
                                ids.binary_search(&token_id).map(|index| ids.remove(index))
                            });
                        });
                        Self::deposit_event(Event::<T>::LowCouponExpire(class_id, token_id));
                    }

                    add_db_reads_writes(2, 1);
                });

            if now >= T::BidMaxPeriod::get() {
                let expire_bid =
                    PendingTaskPenguin::<T>::take(
                        //now.saturating_sub(T::BidMaxPeriod::get())
                        now
                    );
                expire_bid.into_iter().for_each(|(class_id, token_id)| {
                    BidPenguin::<T>::remove((class_id, token_id));

                    let penguin = Penguins::<T>::get(class_id, token_id);
                    match penguin {
                        Some(inner) => match inner {
                            PenguinFarm::RedPenguin(red_penguin) => {
                                unbid_penguin_system!(
                                    red_penguin,
                                    RedPenguin,
                                    class_id,
                                    token_id,
                                    false
                                )
                            }
                            PenguinFarm::YellowPenguin(yellow_penguin) => {
                                unbid_penguin_system!(
                                    yellow_penguin,
                                    YellowPenguin,
                                    class_id,
                                    token_id,
                                    true
                                )
                            }
                            PenguinFarm::SmallYellowPenguin(small_yellow_penguin) => {
                                unbid_penguin_system!(
                                    small_yellow_penguin,
                                    SmallYellowPenguin,
                                    class_id,
                                    token_id,
                                    false
                                )
                            }
                            _ => {}
                        },
                        None => {}
                    }
                    Self::deposit_event(Event::<T>::UnBid(class_id, token_id));
                    add_db_reads_writes(4, 2);
                });
            }

            consumed_weight
        }
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        //!
        //! 收获企鹅蛋，会根据喂养次数计算收成，
        //! 每20枚企鹅蛋产生一张孵化劵，孵化劵有一定概率孵化出公企鹅
        //! 公企鹅不生产孵化劵
        //! 领取蛋之后处于饥饿状态，但是领蛋的前提并不要求一定是active状态，因为蛋不是一定要active才能领取
        //!
        #[pallet::weight(20_00 + T::DbWeight::get().writes(1))]
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
            let block_number = frame_system::Pallet::<T>::current_block_number();
            match &penguin {
                PenguinFarm::RedPenguin(red_penguin) => {
                    let mut new_penguin = red_penguin.clone();
                    let RedPenguin {
                        pre_eat_at,
                        owner,
                        eat_count,
                        incubation_remain,
                        ..
                    } = red_penguin;
                    ensure!(owner == &caller, Error::<T>::NoPermission);
                    //企鹅产蛋率 * 喂养次数
                    ensure!(
                        block_number >= *pre_eat_at + s!(T::PenguinProducePeriod::get()),
                        Error::<T>::PenguinProduceTooShort
                    );
                    ensure!(*eat_count == 1, Error::<T>::PenguinEggHadClaim);
                    new_penguin.eggs = new_penguin.eggs + penguin_produce_egg_rate;
                    let remain = penguin_produce_egg_rate + *incubation_remain;
                    //处理孵化劵张数，发放孵化劵
                    let incubation_count = UniqueSaturatedInto::<usize>::unique_saturated_into(
                        remain / T::RedPenguinEggCountForIncubation::get(),
                    );
                    new_penguin.eat_count = 0;
                    new_penguin.status = PenguinStatus::Hunger;
                    new_penguin.incubation_remain = remain
                        .checked_rem(&T::RedPenguinEggCountForIncubation::get())
                        .ok_or(Error::<T>::IncubationRemainError)?;
                    // new_penguin.eggs = *eggs + penguin_produce_egg_rate;
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
                            let _: Result<_, ()> =
                                OwnerIncubationCouponAsset::<T>::try_mutate(owner.clone(), |ids| {
                                    ids.push(id);
                                    Ok(())
                                });
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
                            log::info!("孵化劵id  {:?}", id);
                            Some(id)
                        })
                        .collect::<Vec<TokenIdOf<T>>>();
                    if incubation_ids.len() > 0 {
                        Self::deposit_event(Event::<T>::Incubations(incubation_ids, block_number));
                    }
                    //发放企鹅蛋
                    let _ = <T as pallet::Config>::Currency::deposit_into_existing(
                        &caller,
                        penguin_produce_egg_rate,
                    );
                    update_penguin!(class_id, token_id, RedPenguin, new_penguin);
                    Self::deposit_event(Event::<T>::Claim(
                        class_id,
                        token_id,
                        penguin_produce_egg_rate,
                    ));
                }
                PenguinFarm::YellowPenguin(yellow_penguin) => {
                    let YellowPenguin {
                        owner,
                        eat_count,
                        incubation_remain,
                        eggs,
                        pre_eat_at,
                        ..
                    } = yellow_penguin;
                    let mut new_penguin = yellow_penguin.clone();
                    ensure!(owner == &caller, Error::<T>::NoPermission);
                    //企鹅产蛋率 * 喂养次数
                    ensure!(
                        block_number >= *pre_eat_at + s!(T::PenguinProducePeriod::get()),
                        Error::<T>::PenguinProduceTooShort
                    );

                    if block_number > *pre_eat_at + s!(T::YellowPenguinDeadPeriod::get()) {
                        Self::yellow_penguin_death(class_id, token_id, owner.clone())?;
                        return Err(Error::<T>::PenguinHadDeath)?;
                    }

                    ensure!(
                        block_number <= *pre_eat_at + s!(T::YellowPenguinDeadPeriod::get()),
                        Error::<T>::PenguinHadDeath
                    );
                    ensure!(*eat_count == 1, Error::<T>::PenguinEggHadClaim);
                    new_penguin.eat_count = 0;
                    new_penguin.status = PenguinStatus::Hunger;
                    log::info!("产蛋数量 {:?}", penguin_produce_egg_rate);
                    new_penguin.eggs = new_penguin.eggs + penguin_produce_egg_rate;
                    let remain = penguin_produce_egg_rate + *incubation_remain;
                    //处理孵化劵张数，发放孵化劵
                    let incubation_count = UniqueSaturatedInto::<usize>::unique_saturated_into(
                        remain / T::YellowPenguinEggCountForIncubation::get(),
                    );
                    log::info!("孵化劵数量 {:?}", incubation_count);
                    new_penguin.incubation_remain = remain
                        .checked_rem(&T::YellowPenguinEggCountForIncubation::get())
                        .ok_or(Error::<T>::IncubationRemainError)?;
                    new_penguin.eggs = *eggs + penguin_produce_egg_rate;
                    let low_incubation_coupon_class_id = LowIncubationCouponClassId::<T>::get();
                    let mut incubation_ids = sp_std::vec![];

                    let _: Vec<TokenIdOf<T>> = (0..incubation_count)
                        .filter_map(|_| {
                            let id: TokenIdOf<T> = <Pallet<T>>::get_next_low_incubation_id()
                                .map_err(|_| Error::<T>::IncubationTokenIdError)
                                .ok()?;
                            incubation_ids.push(id);
                            IncubationCoupons::<T>::insert(
                                low_incubation_coupon_class_id,
                                id.clone(),
                                IncubationCouponOf::<T> {
                                    owner: owner.clone(),
                                    start: block_number,
                                    status: CouponStatus::Liquid,
                                    asset_id: id,
                                    class_id: low_incubation_coupon_class_id,
                                },
                            );
                            let _: Result<_, ()> =
                                OwnerIncubationCouponAsset::<T>::try_mutate(owner.clone(), |ids| {
                                    ids.push(id);
                                    Ok(())
                                });
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
                                    value.push((low_incubation_coupon_class_id, id));
                                    Ok(old_value)
                                },
                            );
                            Some(id)
                        })
                        .collect::<Vec<TokenIdOf<T>>>();

                    if incubation_ids.len() > 0 {
                        Self::deposit_event(Event::<T>::Incubations(incubation_ids, block_number));
                    }
                    //发放企鹅蛋
                    let _ = <T as pallet::Config>::Currency::deposit_into_existing(
                        &caller,
                        penguin_produce_egg_rate,
                    );
                    update_penguin!(class_id, token_id, YellowPenguin, new_penguin);
                    Self::deposit_event(Event::<T>::Claim(
                        class_id,
                        token_id,
                        penguin_produce_egg_rate,
                    ));
                }
                PenguinFarm::MalePenguin(male_penguin) => {
                    let mut new_penguin = male_penguin.clone();
                    let MalePenguin {
                        pre_eat_at,
                        owner,
                        eat_count,
                        ..
                    } = male_penguin;
                    ensure!(owner == &caller, Error::<T>::NoPermission);
                    //企鹅产蛋率 * 喂养次数
                    ensure!(
                        block_number >= *pre_eat_at + s!(T::PenguinProducePeriod::get()),
                        Error::<T>::PenguinProduceTooShort
                    );
                    ensure!(
                        block_number >= *pre_eat_at + s!(T::PenguinProducePeriod::get()),
                        Error::<T>::PenguinHadDeath
                    );
                    ensure!(*eat_count == 1, Error::<T>::PenguinEggHadClaim);
                    new_penguin.eat_count = 0;
                    new_penguin.status = PenguinStatus::Hunger;
                    log::info!("产蛋数量 {:?}", penguin_produce_egg_rate);
                    new_penguin.eggs = new_penguin.eggs + penguin_produce_egg_rate;
                    //发放企鹅蛋
                    let _ = <T as pallet::Config>::Currency::deposit_into_existing(
                        &caller,
                        penguin_produce_egg_rate,
                    );
                    update_penguin!(class_id, token_id, MalePenguin, new_penguin);
                    Self::deposit_event(Event::<T>::Claim(
                        class_id,
                        token_id,
                        penguin_produce_egg_rate,
                    ));
                }
                PenguinFarm::SmallYellowPenguin(_) => {}
            }

            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn revert_admin(
            origin: OriginFor<T>,
            new: AccountIdOf<T>,
        ) -> DispatchResultWithPostInfo {
            // This is a public call, so we ensure that the origin is some signed account.
            ensure_root(origin)?;

            ensure!(
                <Admin<T>>::get().binary_search(&new).is_ok(),
                Error::<T>::AdminNoExist
            );
            <Admin<T>>::mutate(|admins| {
                admins.binary_search(&new).map(|index| {
                    admins.remove(index);
                });
            });

            Self::deposit_event(Event::AdminChanged(new));
            // Admin user does not pay a fee.
            Ok(Pays::No.into())
        }

        #[pallet::weight(0)]
        pub fn force_set_admin(
            origin: OriginFor<T>,
            new: AccountIdOf<T>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            ensure!(
                <Admin<T>>::get().binary_search(&new).is_err(),
                Error::<T>::AdminHadExist
            );

            <Admin<T>>::mutate(|admins| {
                admins.push(new.clone());
            });
            Self::deposit_event(Event::AdminChanged(new));
            Ok(Pays::No.into())
        }
        ///迁入企鹅
        #[pallet::weight(0)]
        #[transactional]
        pub fn move_in(
            origin: OriginFor<T>,
            #[pallet::compact] class_id: ClassIdOf<T>,
            to: AccountIdOf<T>,
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(
                <Admin<T>>::get().contains(&caller),
                Error::<T>::NoPermission
            );
            Self::inner_move_in(class_id, to)?;

            Ok(().into())
        }

        ///批量迁入企鹅
        #[pallet::weight(0)]
        #[transactional]
        pub fn batch_move_in(
            origin: OriginFor<T>,
            class_ids: Vec<ClassIdOf<T>>,
            to: AccountIdOf<T>,
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(
                <Admin<T>>::get().contains(&caller),
                Error::<T>::NoPermission
            );
            ensure!(class_ids.len() <= 20, Error::<T>::MaxNumberLimit);
            class_ids
                .into_iter()
                .for_each(|class_id|{
		         let _:Result<_,DispatchError>=	Self::inner_move_in(class_id, to.clone());
				});

            Ok(().into())
        }

        /// 喂养企鹅
        /// 系统喂养所以暂时不收手续费
        /// 喂养的前提是必须间隔时间必须大于产蛋时间
        /// 所有企鹅都需要喂养
        #[pallet::weight(0)]
        #[transactional]
        pub fn feed_penguin(
            origin: OriginFor<T>,
            #[pallet::compact] class_id: ClassIdOf<T>,
            #[pallet::compact] token_id: TokenIdOf<T>,
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(
                <Admin<T>>::get().contains(&caller),
                Error::<T>::NoPermission
            );
            let block_number = frame_system::Pallet::<T>::current_block_number();
            let penguin =
                Penguins::<T>::get(class_id, token_id).ok_or(Error::<T>::PenguinNoExist)?;
            match penguin {
                PenguinFarm::RedPenguin(red_penguin) => {
                    let mut new_penguin = red_penguin.clone();
                    let RedPenguin {
                        pre_eat_at,
                        eat_count,
                        status,
                        ..
                    } = red_penguin;
                    ensure!(eat_count == 0, Error::<T>::NeedClaimPenguinEgg);
                    ensure!(
                        status == PenguinStatus::Hunger,
                        Error::<T>::PenguinStatusError
                    );
                    ensure!(
                        block_number > pre_eat_at + T::PenguinProducePeriod::get(),
                        Error::<T>::PenguinNoHunger
                    );
                    new_penguin.eat_count = 1;
                    new_penguin.pre_eat_at = block_number;
                    new_penguin.status = PenguinStatus::Active;
                    Penguins::<T>::mutate(class_id, token_id, |penguin| {
                        sp_std::mem::swap(
                            penguin,
                            &mut Some(PenguinFarmOf::<T>::RedPenguin(new_penguin)),
                        );
                    });
                    Self::deposit_event(Event::<T>::FeedPenguinSuccess(class_id, token_id));
                }
                PenguinFarm::YellowPenguin(yellow_penguin) => {
                    let mut new_penguin = yellow_penguin.clone();
                    let YellowPenguin {
                        pre_eat_at,
                        eat_count,
                        status,
                        owner,
                        ..
                    } = yellow_penguin;
                    ensure!(eat_count == 0, Error::<T>::NeedClaimPenguinEgg);
                    ensure!(
                        status == PenguinStatus::Hunger,
                        Error::<T>::PenguinStatusError
                    );
                    ensure!(
                        block_number > pre_eat_at + T::PenguinProducePeriod::get(),
                        Error::<T>::PenguinNoHunger
                    );
                    //企鹅死亡
                    if block_number > pre_eat_at + T::YellowPenguinDeadPeriod::get() {
                        Penguins::<T>::remove(class_id, token_id);
                        OwnerYellowPenguin::<T>::try_mutate(&owner, |ids| {
                            ids.binary_search(&token_id).map(|index| ids.remove(index))
                        }).map_err(|_|Error::<T>::PenguinHadDeath)?;
                        YellowPenguinCount::<T>::mutate(|value| *value = *value - 1);
                        Self::deposit_event(Event::<T>::PenguinDead(class_id, token_id));
                    } else {
                        new_penguin.eat_count = 1;
                        new_penguin.pre_eat_at = block_number;
                        new_penguin.status = PenguinStatus::Active;
                        Penguins::<T>::mutate(class_id, token_id, |penguin| {
                            sp_std::mem::swap(
                                penguin,
                                &mut Some(PenguinFarmOf::<T>::YellowPenguin(new_penguin)),
                            );
                        });
                        Self::deposit_event(Event::<T>::FeedPenguinSuccess(class_id, token_id));
                    }
                }
                PenguinFarm::MalePenguin(male_penguin) => {
                    let mut new_penguin = male_penguin.clone();
                    let MalePenguin {
                        pre_eat_at,
                        eat_count,
                        status,
                        owner,
                        ..
                    } = male_penguin;
                    ensure!(eat_count == 0, Error::<T>::NeedClaimPenguinEgg);
                    ensure!(
                        status == PenguinStatus::Hunger,
                        Error::<T>::PenguinStatusError
                    );
                    ensure!(
                        block_number > pre_eat_at + T::PenguinProducePeriod::get(),
                        Error::<T>::PenguinNoHunger
                    );
                    //企鹅死亡
                    /*if block_number > pre_eat_at + T::MalePenguinLifeSpan::get() {
                        Penguins::<T>::remove(class_id, token_id);
                        OwnerMalePenguin::<T>::try_mutate(&owner, |ids| {
                            ids.binary_search(&token_id).map(|index| ids.remove(index))
                        })
                        .map_err(|_| Error::<T>::RemoveMalePenguinError)?;
                        MalePenguinCount::<T>::mutate(|value| *value = *value - 1);
                        Self::deposit_event(Event::<T>::PenguinDead(class_id, token_id));
                    } else {*/
                        new_penguin.eat_count = 1;
                        new_penguin.pre_eat_at = block_number;
                        new_penguin.status = PenguinStatus::Active;
                        Penguins::<T>::mutate(class_id, token_id, |penguin| {
                            sp_std::mem::swap(
                                penguin,
                                &mut Some(PenguinFarmOf::<T>::MalePenguin(new_penguin)),
                            );
                        });
                        Self::deposit_event(Event::<T>::FeedPenguinSuccess(class_id, token_id));
                    //}
                }
                PenguinFarm::SmallYellowPenguin(small_yellow_penguin) => {
                    let mut new_penguin = small_yellow_penguin.clone();
                    let SmallYellowPenguin {
                        pre_eat_at,
                        status,
                        grow_value,
                        owner,
                        ..
                    } = small_yellow_penguin;
                    ensure!(status != PenguinStatus::Bid, Error::<T>::PenguinStatusError);
                    ensure!(
                        block_number > pre_eat_at + T::PenguinProducePeriod::get(),
                        Error::<T>::PenguinNoHunger
                    );

                    if new_penguin.grow_value == T::SmallYellowPenguinGrowPeriod::get() {
                        let id = Self::get_next_yellow_id()?;
                        let yellow_penguin_class_id = YellowPenguinClassId::<T>::get();
                        let  new_penguin = YellowPenguin {
                            owner: new_penguin.owner,
                            start: block_number,
                            eat_count: 1u32,
                            status: PenguinStatus::Active,
                            pre_eat_at: block_number,
                            eggs: Default::default(),
                            asset_id: id,
                            class_id: yellow_penguin_class_id,
                            incubation_remain: Default::default(),
                        };
                        Penguins::<T>::remove(class_id, token_id);
                        SmallYellowPenguinCount::<T>::mutate(|value| *value = *value - 1);
                        Penguins::<T>::insert(
                            class_id,
                            id,
                            PenguinFarmOf::<T>::YellowPenguin(new_penguin),
                        );
                        OwnerYellowPenguin::<T>::mutate(owner.clone(), |ids| {
                            ids.push(id);
                        });
						OwnerSmallYellowPenguin::<T>::mutate(owner, |ids| {
							ids.binary_search(&token_id).map(|index|{
								ids.remove(index)
							})
						});
                        YellowPenguinCount::<T>::mutate(|value| *value = *value + 1);
                        Self::deposit_event(Event::<T>::PenguinUpgrade(
                            class_id,
                            token_id,
                            yellow_penguin_class_id,
                            id,
                        ));
                    } else {
                        new_penguin.eat_count = 1;
                        new_penguin.pre_eat_at = block_number;
                        new_penguin.grow_value = grow_value.add(T::PenguinProducePeriod::get());
                        Penguins::<T>::mutate(class_id, token_id, |penguin| {
                            sp_std::mem::swap(
                                penguin,
                                &mut Some(PenguinFarmOf::<T>::SmallYellowPenguin(new_penguin)),
                            );
                        });
                        Self::deposit_event(Event::<T>::FeedPenguinSuccess(class_id, token_id));
                    }
                }
            }

            Ok(Pays::No.into())
        }
        #[pallet::weight(0)]
        pub fn remove_male_penguin(
            origin: OriginFor<T>,
            #[pallet::compact] class_id: ClassIdOf<T>,
            #[pallet::compact] token_id: TokenIdOf<T>,
        ) -> DispatchResultWithPostInfo {

            log::info!("remove_male_penguin");

            ensure_none(origin)?;
            let penguin =
                Penguins::<T>::take(class_id, token_id).ok_or(Error::<T>::PenguinNoExist)?;
            match penguin {
                PenguinFarm::RedPenguin(_)
                | PenguinFarm::YellowPenguin(_)
                | PenguinFarm::SmallYellowPenguin(_) => {}
                PenguinFarm::MalePenguin(MalePenguin { owner, .. }) => {
                    let _ :Result<(),DispatchError>= OwnerMalePenguin::<T>::mutate(&owner, |ids| {
                        ids.binary_search(&token_id).map(|index| {
                            ids.remove(index);
                            log::info!("removed male owner:{:?}, token_id:{:?},class_id:{:?}",owner.clone(),token_id,class_id);
                        }).map_err(|_|Error::<T>::PenguinNoExist)?;
						Ok(())
                    });
                    MalePenguinCount::<T>::mutate(|value| *value = *value - 1);
                    Self::deposit_event(Event::<T>::MalePenguinRemoved(class_id, token_id,owner));

                }
            }
            Ok(Pays::No.into())
        }

        ///孵化企鹅
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        #[transactional]
        pub fn incubation_penguin(
            origin: OriginFor<T>,
            #[pallet::compact] class_id: ClassIdOf<T>,
            #[pallet::compact] token_id: TokenIdOf<T>,
        ) -> DispatchResultWithPostInfo
		{
            let caller = ensure_signed(origin)?;
            let amount = <T as Config>::Currency::total_balance(&caller);
            let block_number = frame_system::Pallet::<T>::current_block_number();
            ensure!(amount > One::one(), Error::<T>::IncubationBalanceNoEnough);
            let incubation_class_id = IncubationCouponClassId::<T>::get();
            let low_incubation_class_id = LowIncubationCouponClassId::<T>::get();
            match class_id {
                b @ n if b == incubation_class_id => {
                    let coupon = IncubationCoupons::<T>::get(class_id, token_id)
                        .ok_or(Error::<T>::IncubationCouponNoExist)?;
                    ensure!(
                        block_number < coupon.start + T::IncubationLivePeriod::get(),
                        Error::<T>::IncubationCouponExpire
                    );
                    //销毁一个企鹅蛋
                    <T as Config>::Currency::withdraw(
                        &caller,
                        One::one(),
                        WithdrawReasons::FEE,
                        ExistenceRequirement::KeepAlive,
                    )?;
                    //销毁孵化劵
                    IncubationCoupons::<T>::remove(class_id, token_id);
                    OwnerIncubationCouponAsset::<T>::mutate(&caller, |ids| {
                        ids.binary_search(&token_id).map(|index| ids.remove(index))
                    })
                    .map_err(|_| Error::<T>::RemoveCouponError)?;

                    //生成一个小黄企鹅
                    let id = Self::get_next_small_yellow_id()?;
                    let small_yellow_class_id = SmallYellowPenguinClassId::<T>::get();
                    Penguins::<T>::insert(
                        small_yellow_class_id,
                        id,
                        PenguinFarmOf::<T>::SmallYellowPenguin(SmallYellowPenguin {
                            owner: caller.clone(),
                            start: block_number,
                            pre_eat_at: block_number,
                            eat_count: 1,
                            status: PenguinStatus::Active,
                            asset_id: id,
                            class_id: small_yellow_class_id,
                            grow_value: Default::default(),
                        }),
                    );
                    OwnerSmallYellowPenguin::<T>::mutate(&caller, |ids| ids.push(id));
                    SmallYellowPenguinCount::<T>::mutate(|value| *value = *value + 1);
                    //计算公企鹅概率
                    let payload = (T::Randomness::random_seed(), token_id);
                    let mut h = payload.using_encoded(<T as frame_system::Config>::Hashing::hash);
                    let dna: &mut [u8] = h.as_mut();
                    let b = <[u8; 4]>::try_from(&dna[0..4])
                        .map_err(|_| Error::<T>::NoAvailableClassId)?;
                    let n = <u32>::from_le_bytes(b);
                    log::info!("random number {}", n);
                    if n % 1000u32 < MALE_PENGUIN_RATE {
                        let id = Self::get_next_male_id()?;
                        let male_class_id = MalePenguinClassId::<T>::get();
                        Penguins::<T>::insert(
                            male_class_id,
                            id,
                            PenguinFarmOf::<T>::MalePenguin(MalePenguin {
                                owner: caller.clone(),
                                start: block_number,
                                pre_eat_at: block_number,
                                eat_count: 1u32,
                                eggs: Default::default(),
                                status: PenguinStatus::Active,
                                asset_id: id,
                                class_id: male_class_id,
                                incubation_remain: Zero::zero(),
                            }),
                        );
                        OwnerMalePenguin::<T>::mutate(&caller, |ids| ids.push(id));

                        MalePenguinCount::<T>::mutate(|value| *value = *value + 1);

                        // let call = IsSubType::<<T as pallet::Config>::Call>::is_sub_type(&Call::<T>::remove_male_penguin{class_id:male_class_id,token_id:id}).ok_or(Error::<T>::NoAvailableClassId)?;
                      T::Schedule::schedule(
                            DispatchTime::After(T::MalePenguinLifeSpan::get()),
                            None,
                            MalePenguinCount::<T>::get() as u8,
                            frame_system::RawOrigin::None.into(),
                            Call::<T>::remove_male_penguin(male_class_id,token_id).into(),
                        )?;
                    }
                }
                b @ n if b == low_incubation_class_id => {
                    let coupon = IncubationCoupons::<T>::get(class_id, token_id)
                        .ok_or(Error::<T>::IncubationCouponNoExist)?;
                    ensure!(
                        block_number < coupon.start + T::IncubationLivePeriod::get(),
                        Error::<T>::IncubationCouponExpire
                    );
                    //销毁一个企鹅蛋
                    <T as Config>::Currency::withdraw(
                        &caller,
                        One::one(),
                        WithdrawReasons::FEE,
                        ExistenceRequirement::KeepAlive,
                    )?;
                    //销毁孵化劵
                    LowIncubationCoupons::<T>::remove(class_id, token_id);
                    OwnerIncubationCouponAsset::<T>::mutate(&caller, |ids| {
                        ids.binary_search(&token_id).map(|index| ids.remove(index))
                    }).map_err(|_|Error::<T>::PenguinNoExist)?;

                    //生成一个小黄企鹅
                    let id = Self::get_next_small_yellow_id()?;
                    let small_yellow_class_id = SmallYellowPenguinClassId::<T>::get();
                    Penguins::<T>::insert(
                        small_yellow_class_id,
                        id,
                        PenguinFarmOf::<T>::SmallYellowPenguin(SmallYellowPenguin {
                            owner: caller.clone(),
                            start: block_number,
                            pre_eat_at: block_number,
                            eat_count: 1,
                            status: PenguinStatus::Active,
                            asset_id: id,
                            class_id: small_yellow_class_id,
                            grow_value: Default::default(),
                        }),
                    );
                    OwnerSmallYellowPenguin::<T>::mutate(&caller, |ids| ids.push(id));
                }
                _ => Err(Error::<T>::IncubationCouponClassError)?,
            };

            Ok(().into())
        }

        ///购买企鹅
        /// 企鹅必须在挂单
        /// 企鹅购买金额必须大于手续费加企鹅价格
        /// 购买成功之后必须unbid
        #[pallet::weight(10_00 + T::DbWeight::get().writes(1))]
        #[transactional]
        pub fn purchase_penguin(
            origin: OriginFor<T>,
            #[pallet::compact] class_id: ClassIdOf<T>,
            #[pallet::compact] token_id: TokenIdOf<T>,
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            let penguin =
                Penguins::<T>::get(class_id, token_id).ok_or(Error::<T>::PenguinNoExist)?;
            match &penguin {
                PenguinFarm::RedPenguin(red_penguin) => {
                    purchase_penguin!(
                        red_penguin,
                        RedPenguin,
                        OwnerRedPenguin,
                        caller,
                        class_id,
                        token_id,
                        penguin
                    )
                }
                PenguinFarm::YellowPenguin(yellow_penguin) => {
                    purchase_penguin!(
                        yellow_penguin,
                        YellowPenguin,
                        OwnerYellowPenguin,
                        caller,
                        class_id,
                        token_id,
                        penguin
                    )
                }
                PenguinFarm::SmallYellowPenguin(small_yellow_penguin) => {
                    purchase_penguin!(
                        small_yellow_penguin,
                        SmallYellowPenguin,
                        OwnerSmallYellowPenguin,
                        caller,
                        class_id,
                        token_id,
                        penguin
                    )
                }
                PenguinFarm::MalePenguin(_) => {}
            }

            Ok(().into())
        }

        ///挂拍卖 当天喂养次数清除，
        #[pallet::weight(10_00 + T::DbWeight::get().writes(1))]
        #[transactional]
        pub fn bid(
            origin: OriginFor<T>,
            #[pallet::compact] class_id: ClassIdOf<T>,
            #[pallet::compact] token_id: TokenIdOf<T>,
            #[pallet::compact] price: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            let penguin =
                Penguins::<T>::get(class_id, token_id).ok_or(Error::<T>::PenguinNoExist)?;

            match penguin {
                PenguinFarm::MalePenguin(MalePenguin { .. }) => {
                    return Err(Error::<T>::MalePenguinBanBid)?
                }
                PenguinFarm::RedPenguin(red_penguin) => {
                    bid_penguin!(red_penguin, RedPenguin, class_id, token_id, caller, false)
                }
                PenguinFarm::YellowPenguin(yellow_penguin) => {
                    bid_penguin!(
                        yellow_penguin,
                        YellowPenguin,
                        class_id,
                        token_id,
                        caller,
                        T::YellowPenguinDeadPeriod::get(),
                        OwnerYellowPenguin
                    )
                }
                PenguinFarm::SmallYellowPenguin(small_yellow) => {
                    bid_penguin!(
                        small_yellow,
                        SmallYellowPenguin,
                        class_id,
                        token_id,
                        caller,
                        true
                    );
                }
            }

            BidPenguin::<T>::insert((class_id, token_id), price);
            Self::deposit_event(Event::<T>::Bid(class_id, token_id));

            Ok(().into())
        }

        ///取消拍卖 ，
        #[pallet::weight(10_00 + T::DbWeight::get().writes(1))]
        pub fn unbid(
            origin: OriginFor<T>,
            #[pallet::compact] class_id: ClassIdOf<T>,
            #[pallet::compact] token_id: TokenIdOf<T>,
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            let penguin =
                Penguins::<T>::get(class_id, token_id).ok_or(Error::<T>::PenguinNoExist)?;
            Self::inner_unbid(penguin, caller, class_id, token_id)?;
            Ok(().into())
        }
    }
}

impl<T: Config> Pallet<T> {
    pub fn yellow_penguin_death(
        class_id: ClassIdOf<T>,
        token_id: TokenIdOf<T>,
        owner: AccountIdOf<T>,
    ) ->Result<(),DispatchError>{
        Penguins::<T>::remove(class_id, token_id);
        OwnerYellowPenguin::<T>::mutate(owner, |ids| {
            ids.binary_search(&token_id).map(|index| ids.remove(index))
        }).map_err(|_|Error::<T>::PenguinNoExist)?;

        YellowPenguinCount::<T>::mutate(|value| *value = *value - 1);
        Self::deposit_event(Event::<T>::PenguinDead(class_id, token_id));
		Ok(())
    }

    pub fn inner_move_in(class_id: ClassIdOf<T>, to: AccountIdOf<T>) -> Result<(), DispatchError> {
        let red_class_id = RedPenguinClassId::<T>::get();
        let yellow_class_id = YellowPenguinClassId::<T>::get();
        let small_yellow_class_id = SmallYellowPenguinClassId::<T>::get();

        ensure!(
            class_id == red_class_id
                || class_id == yellow_class_id
                || class_id == small_yellow_class_id,
            Error::<T>::NoSupportMoveIn
        );
        let block_number = frame_system::Pallet::<T>::current_block_number();
        if class_id == red_class_id {
            let id = Self::get_next_red_id()?;
            Penguins::<T>::insert(
                class_id,
                id,
                PenguinFarmOf::<T>::RedPenguin(RedPenguin {
                    owner: to.clone(),
                    start: block_number,
                    pre_eat_at: block_number,
                    eat_count: 1,
                    status: PenguinStatus::Active,
                    eggs: Zero::zero(),
                    asset_id: id,
                    class_id: class_id,
                    incubation_remain: Zero::zero(),
                }),
            );
            OwnerRedPenguin::<T>::mutate(to, |ids| ids.push(id));
        } else if class_id == yellow_class_id {
            let id = Self::get_next_yellow_id()?;
            Penguins::<T>::insert(
                class_id,
                id,
                PenguinFarmOf::<T>::YellowPenguin(YellowPenguin {
                    owner: to.clone(),
                    start: block_number,
                    pre_eat_at: block_number,
                    eat_count: 1,
                    status: PenguinStatus::Active,
                    eggs: Zero::zero(),
                    asset_id: id,
                    class_id: yellow_class_id,
                    incubation_remain: Zero::zero(),
                }),
            );
            OwnerYellowPenguin::<T>::mutate(to, |ids| ids.push(id));
            YellowPenguinCount::<T>::mutate(|value| *value = *value + 1);
        } else {
            let id = Self::get_next_small_yellow_id()?;
            Penguins::<T>::insert(
                class_id,
                id,
                PenguinFarmOf::<T>::SmallYellowPenguin(SmallYellowPenguin {
                    owner: to.clone(),
                    start: block_number,
                    pre_eat_at: block_number,
                    eat_count: 1,
                    status: PenguinStatus::Active,
                    asset_id: id,
                    class_id: class_id,
                    grow_value: Default::default(),
                }),
            );
            OwnerSmallYellowPenguin::<T>::mutate(to, |ids| ids.push(id));
            SmallYellowPenguinCount::<T>::mutate(|value| *value = *value + 1);
        }
        Ok(())
    }
    pub fn inner_unbid(
        penguin: PenguinFarmOf<T>,
        caller: AccountIdOf<T>,
        class_id: ClassIdOf<T>,
        token_id: TokenIdOf<T>,
    ) -> Result<(), DispatchError> {
        match penguin {
            PenguinFarm::MalePenguin(MalePenguin { .. }) => {
                return Err(Error::<T>::MalePenguinBanBid)?
            }
            PenguinFarm::RedPenguin(red_penguin) => {
                unbid_penguin!(red_penguin, RedPenguin, class_id, token_id, caller, false);
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
        BidPenguin::<T>::remove((class_id, token_id));
        Self::deposit_event(Event::<T>::UnBid(class_id, token_id));
        Ok(())
    }
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

    pub fn get_next_low_incubation_id() -> Result<TokenIdOf<T>, DispatchError> {
        LowIncubationTokenId::<T>::try_mutate(|pre| {
            let current = *pre;
            *pre = pre
                .checked_add(&One::one())
                .ok_or(Error::<T>::NoAvailableTokenId)?;
            Ok(current)
        })
    }
    pub fn query_red_penguin_num() -> BalanceOf<T> {
        s!(20000u128)
    }

    pub fn query_yellow_penguin_num() -> BalanceOf<T> {
        s!(YellowPenguinCount::<T>::get())
    }

    pub fn query_male_penguin_num() -> BalanceOf<T> {
        s!(MalePenguinCount::<T>::get())
    }


    pub fn print_init_variable() {
        log::info!("DAYS:{}, MALE_PENGUIN_RATE:{}",DAYS,MALE_PENGUIN_RATE);
    }


}
