//! # Non Fungible Token
//! The module provides implementations for non-fungible-token.
//!
//! - [`Config`](./trait.Config.html)
//! - [`Call`](./enum.Call.html)
//! - [`Module`](./struct.Module.html)
//!
//! ## Overview
//!
//! This module provides basic functions to create and manager
//! NFT(non fungible token) such as `create_class`, `transfer`, `mint`, `burn`.

//! ### Module Functions
//!
//! - `create_class` - Create NFT(non fungible token) class
//! - `transfer` - Transfer NFT(non fungible token) to another account.
//! - `mint` - Mint NFT(non fungible token)
//! - `burn` - Burn NFT(non fungible token)
//! - `destroy_class` - Destroy NFT(non fungible token) class

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use codec::{Decode, Encode};
use frame_support::{
    ensure,
    pallet_prelude::*,
    traits::{Currency, ReservableCurrency},
    Parameter,
};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{
    traits::{
        AtLeast32BitUnsigned, CheckedAdd, CheckedSub, MaybeSerializeDeserialize, Member, One, Zero,
    },
    ArithmeticError, DispatchError, DispatchResult, RuntimeDebug,
};
use sp_std::vec::Vec;

mod mock;
mod tests;
use frame_support::scale_info::TypeInfo;

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CollectionType {
    Collectable,
    Executable,
}

impl CollectionType {
    pub fn is_collectable(&self) -> bool {
        match *self {
            CollectionType::Collectable => true,
            _ => false,
        }
    }

    pub fn is_executable(&self) -> bool {
        match *self {
            CollectionType::Executable => true,
            _ => false,
        }
    }
}

impl Default for CollectionType {
    fn default() -> Self {
        CollectionType::Collectable
    }
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum TokenType {
    Transferable,
    BoundToAddress,
}

impl TokenType {
    pub fn is_transferable(&self) -> bool {
        match *self {
            TokenType::Transferable => true,
            _ => false,
        }
    }
}

impl Default for TokenType {
    fn default() -> Self {
        TokenType::BoundToAddress
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ClassData<Balance> {
    pub deposit: Balance,
    pub metadata: Vec<u8>,
    pub token_type: TokenType,
    pub collection_type: CollectionType,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AssetData<Balance> {
    pub deposit: Balance,
    pub name: Vec<u8>,
    pub description: Vec<u8>,
}
/// Class info
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct ClassInfo<Balance, AccountId, Data> {
    /// Class metadata
    pub metadata: Vec<u8>,
    /// Total issuance for the class
    pub total_issuance: Balance,
    /// Class owner
    pub owner: AccountId,
    /// Class Properties
    pub data: Data,
}

/// Token info
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct TokenInfo<AccountId, Data> {
    /// Token metadata
    pub metadata: Vec<u8>,
    /// Token owner
    pub owner: AccountId,
    /// Token Properties
    pub data: Data,
}
pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
pub use module::*;

#[frame_support::pallet]
pub mod module {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
        /// The class ID type
        type ClassId: Parameter
            + Member
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + MaybeSerializeDeserialize;
        /// The token ID type
        type TokenId: Parameter
            + Member
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + MaybeSerializeDeserialize;
    }

    pub type ClassInfoOf<T> =
        ClassInfo<BalanceOf<T>, <T as frame_system::Config>::AccountId, ClassData<BalanceOf<T>>>;
    pub type TokenInfoOf<T> =
        TokenInfo<<T as frame_system::Config>::AccountId, AssetData<BalanceOf<T>>>;

    pub type GenesisTokenData<T> = (
        <T as frame_system::Config>::AccountId, // Token owner
        Vec<u8>,                                // Token metadata
        AssetData<BalanceOf<T>>,
    );
    pub type GenesisTokens<T> = (
        <T as frame_system::Config>::AccountId, // Token class owner
        Vec<u8>,                                // Token class metadata
        ClassData<BalanceOf<T>>,
        Vec<GenesisTokenData<T>>, // Vector of tokens belonging to this class
    );

    /// Error for non-fungible-token module.
    #[pallet::error]
    pub enum Error<T> {
        ///No available Issuancenumber
        NoAvailableClassIssuance,
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

    /// Next available class ID.
    #[pallet::storage]
    #[pallet::getter(fn next_class_id)]
    pub type NextClassId<T: Config> = StorageValue<_, T::ClassId, ValueQuery>;

    /// Next available token ID.
    #[pallet::storage]
    #[pallet::getter(fn next_token_id)]
    pub type NextTokenId<T: Config> =
        StorageMap<_, Twox64Concat, T::ClassId, T::TokenId, ValueQuery>;

    /// Store class info.
    ///
    /// Returns `None` if class info not set or removed.
    #[pallet::storage]
    #[pallet::getter(fn classes)]
    pub type Classes<T: Config> = StorageMap<_, Twox64Concat, T::ClassId, ClassInfoOf<T>>;

    /// Store token info.
    ///
    /// Returns `None` if token info not set or removed.
    #[pallet::storage]
    #[pallet::getter(fn tokens)]
    pub type Tokens<T: Config> =
        StorageDoubleMap<_, Twox64Concat, T::ClassId, Twox64Concat, T::TokenId, TokenInfoOf<T>>;

    /// Token existence check by owner and class ID.
    #[pallet::storage]
    #[pallet::getter(fn tokens_by_owner)]
    pub type TokensByOwner<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        T::AccountId,
        Twox64Concat,
        (T::ClassId, T::TokenId),
        (),
        ValueQuery,
    >;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub tokens: Vec<GenesisTokens<T>>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            GenesisConfig { tokens: vec![] }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            self.tokens.iter().for_each(|token_class| {
                let class_id = Pallet::<T>::create_class(
                    &token_class.0,
                    token_class.1.to_vec(),
                    token_class.2.clone(),
                )
                .expect("Create class cannot fail while building genesis");
                for (account_id, token_metadata, token_data) in &token_class.3 {
                    Pallet::<T>::mint(
                        &account_id,
                        class_id,
                        token_metadata.to_vec(),
                        token_data.clone(),
                    )
                    .expect("Token mint cannot fail during genesis");
                }
            })
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
    /// Create NFT(non fungible token) class
    pub fn create_class(
        owner: &T::AccountId,
        metadata: Vec<u8>,
        data: ClassData<BalanceOf<T>>,
    ) -> Result<T::ClassId, DispatchError> {
        let class_id = NextClassId::<T>::try_mutate(|id| -> Result<T::ClassId, DispatchError> {
            let current_id = *id;
            *id = id
                .checked_add(&One::one())
                .ok_or(Error::<T>::NoAvailableClassId)?;
            Ok(current_id)
        })?;

        let info = ClassInfo {
            metadata,
            total_issuance: Default::default(),
            owner: owner.clone(),
            data,
        };
        Classes::<T>::insert(class_id, info);

        Ok(class_id)
    }

    pub fn create_native_class(
        owner: &T::AccountId,
        metadata: Vec<u8>,
        total_issuance: BalanceOf<T>,
        token_type: TokenType,
    ) -> Result<T::ClassId, DispatchError> {
        // ensure!(total_issuance > 0.into(),Error::<T>::NoAvailableClassId);
        let class_id = NextClassId::<T>::try_mutate(|id| -> Result<T::ClassId, DispatchError> {
            let current_id = *id;
            *id = id
                .checked_add(&One::one())
                .ok_or(Error::<T>::NoAvailableClassId)?;
            Ok(current_id)
        })?;

        let info = ClassInfo {
            metadata,
            total_issuance: total_issuance,
            owner: owner.clone(),
            data: ClassData {
                deposit: Default::default(),
                metadata: Default::default(),
                token_type,
                collection_type: CollectionType::Collectable,
            },
        };
        Classes::<T>::insert(class_id, info);

        Ok(class_id)
    }

    /// Transfer NFT(non fungible token) from `from` account to `to` account
    pub fn transfer(
        from: &T::AccountId,
        to: &T::AccountId,
        token: (T::ClassId, T::TokenId),
    ) -> DispatchResult {
        Tokens::<T>::try_mutate(token.0, token.1, |token_info| -> DispatchResult {
            let mut info = token_info.as_mut().ok_or(Error::<T>::TokenNotFound)?;
            ensure!(info.owner == *from, Error::<T>::NoPermission);
            if from == to {
                // no change needed
                return Ok(());
            }

            info.owner = to.clone();

            TokensByOwner::<T>::remove(from, token);
            TokensByOwner::<T>::insert(to, token, ());

            Ok(())
        })
    }

    /// Mint NFT(non fungible token) to `owner`
    pub fn mint(
        owner: &T::AccountId,
        class_id: T::ClassId,
        metadata: Vec<u8>,
        data: AssetData<BalanceOf<T>>,
    ) -> Result<T::TokenId, DispatchError> {
        NextTokenId::<T>::try_mutate(class_id, |id| -> Result<T::TokenId, DispatchError> {
            let token_id = *id;
            *id = id
                .checked_add(&One::one())
                .ok_or(Error::<T>::NoAvailableTokenId)?;

            Classes::<T>::try_mutate(class_id, |class_info| -> DispatchResult {
                let info = class_info.as_mut().ok_or(Error::<T>::ClassNotFound)?;
                info.total_issuance = info
                    .total_issuance
                    .checked_add(&One::one())
                    .ok_or(ArithmeticError::Overflow)?;
                Ok(())
            })?;

            let token_info = TokenInfo {
                metadata,
                owner: owner.clone(),
                data,
            };
            Tokens::<T>::insert(class_id, token_id, token_info);
            TokensByOwner::<T>::insert(owner, (class_id, token_id), ());

            Ok(token_id)
        })
    }

    /// Burn NFT(non fungible token) from `owner`
    pub fn burn(owner: &T::AccountId, token: (T::ClassId, T::TokenId)) -> DispatchResult {
        Tokens::<T>::try_mutate_exists(token.0, token.1, |token_info| -> DispatchResult {
            let t = token_info.take().ok_or(Error::<T>::TokenNotFound)?;
            ensure!(t.owner == *owner, Error::<T>::NoPermission);

            Classes::<T>::try_mutate(token.0, |class_info| -> DispatchResult {
                let info = class_info.as_mut().ok_or(Error::<T>::ClassNotFound)?;
                info.total_issuance = info
                    .total_issuance
                    .checked_sub(&One::one())
                    .ok_or(ArithmeticError::Overflow)?;
                Ok(())
            })?;

            TokensByOwner::<T>::remove(owner, token);

            Ok(())
        })
    }

    /// Destroy NFT(non fungible token) class
    pub fn destroy_class(owner: &T::AccountId, class_id: T::ClassId) -> DispatchResult {
        Classes::<T>::try_mutate_exists(class_id, |class_info| -> DispatchResult {
            let info = class_info.take().ok_or(Error::<T>::ClassNotFound)?;
            ensure!(info.owner == *owner, Error::<T>::NoPermission);
            ensure!(
                info.total_issuance == Zero::zero(),
                Error::<T>::CannotDestroyClass
            );

            NextTokenId::<T>::remove(class_id);

            Ok(())
        })
    }

    pub fn is_owner(account: &T::AccountId, token: (T::ClassId, T::TokenId)) -> bool {
        TokensByOwner::<T>::contains_key(account, token)
    }
}
