//! # Validator Set Pallet
//!
//! The Validator Set Pallet allows addition and removal of authorities/validators via extrinsics (transaction calls), in Substrate-based
//! PoA networks.
//!
//! The pallet uses the Session pallet and implements related traits for session
//! management.

#![cfg_attr(not(feature = "std"), no_std)]

use pallet_session::Pallet as Session;
use sp_runtime::traits::{Convert, Zero};
use sp_std::{collections::btree_set::BTreeSet, prelude::*};

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_session::Config {
        /// The Event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Origin for adding or removing a validator.
        type AddRemoveOrigin: EnsureOrigin<Self::Origin>;

        ///最小的 authority
        type MinAuthorities: Get<u32>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // The pallet's storage items.
    #[pallet::storage]
    #[pallet::getter(fn validators)]
    pub type Validators<T: Config> = StorageValue<_, Vec<T::AccountId>,ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn flag)]
    pub type Flag<T: Config> = StorageValue<_, bool>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // New validator added.
        ValidatorAdded(T::AccountId),

        // Validator removed.
        ValidatorRemoved(T::AccountId),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        ///no validators
        NoValidators,
        /// Duplicated validators
        Duplicated,
        ///min Validators must leave in the set.
        MinValidators,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub validators: Vec<T::AccountId>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                validators: Vec::new(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            Pallet::<T>::initialize_validators(&self.validators);
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Add a new validator using elevated privileges.
        ///
        /// New validator's session keys should be set in session module before calling this.
        ///
        /// The origin can be configured using the `AddRemoveOrigin` type in the host runtime.
        /// Can also be set to sudo/root.
        #[pallet::weight(0)]
        pub fn add_validator(origin: OriginFor<T>, validator_id: T::AccountId) -> DispatchResult {
            T::AddRemoveOrigin::ensure_origin(origin)?;

            let validator_set: BTreeSet<_> = <Validators<T>>::get().into_iter().collect();
            ensure!(!validator_set.contains(&validator_id), Error::<T>::Duplicated);
            <Validators<T>>::mutate(|v| v.push(validator_id.clone()));

            // Calling rotate_session to queue the new session keys.
            Session::<T>::rotate_session();

            // Triggering rotate session again for the queued keys to take effect.
            Flag::<T>::put(true);

            Self::deposit_event(Event::ValidatorAdded(validator_id));
            Ok(())
        }

        /// Remove a validator using elevated privileges.
        ///
        /// The origin can be configured using the `AddRemoveOrigin` type in the host runtime.
        /// Can also be set to sudo/root.
        #[pallet::weight(0)]
        pub fn remove_validator(
            origin: OriginFor<T>,
            validator_id: T::AccountId,
        ) -> DispatchResult {
            T::AddRemoveOrigin::ensure_origin(origin)?;

            let mut validators = <Validators<T>>::get();

            ensure!(validators.len() as u32 >T::MinAuthorities::get(), Error::<T>::MinValidators);

            validators.retain(|v| *v !=validator_id);
            <Validators<T>>::put(validators);

            // Calling rotate_session to queue the new session keys.
            <pallet_session::Module<T>>::rotate_session();

            // Triggering rotate session again for the queued keys to take effect.
            Flag::<T>::put(true);

            log::info!("remaining validator:{:?}", <Validators<T>>::get());

            Self::deposit_event(Event::ValidatorRemoved(validator_id));
            Ok(())
        }

        /// Force rotate session using elevated privileges.
        #[pallet::weight(0)]
        pub fn force_rotate_session(origin: OriginFor<T>) -> DispatchResult {
            T::AddRemoveOrigin::ensure_origin(origin)?;

            <pallet_session::Module<T>>::rotate_session();

            // Triggering rotate session again for any queued keys to take effect.
            // Not sure if double rotate is needed in this scenario. **TODO**
            Flag::<T>::put(true);
            Ok(())
        }
    }
}

impl<T: Config> Pallet<T> {
    fn initialize_validators(validators: &[T::AccountId]) {
        if !validators.is_empty() {
            assert!(
                <Validators<T>>::get().len()<=0,
                "Validators are already initialized!"
            );
            <Validators<T>>::put(validators);
            log::info!("validator-set.initialize_validators");
        }
    }
}

/// Indicates to the session module if the session should be rotated.
/// We set this flag to true when we add/remove a validator.
impl<T: Config> pallet_session::ShouldEndSession<T::BlockNumber> for Pallet<T> {
    fn should_end_session(_now: T::BlockNumber) -> bool {
        let end = Self::flag().unwrap();
        log::info!("validator-set.should_end_session:{}", end);
        end
    }
}

/// Provides the new set of validators to the session module when session is being rotated.
impl<T: Config> pallet_session::SessionManager<T::AccountId> for Pallet<T> {
    fn new_session(_new_index: u32) -> Option<Vec<T::AccountId>> {
        // Flag is set to false so that the session doesn't keep rotating.
        Flag::<T>::put(false);

        log::info!("validator-set.SessionManager.new_session");

        Some(Self::validators())
    }

    fn end_session(_end_index: u32) {
        log::info!(
            "validator-set.SessionManager.end_session: index:{}",
            _end_index
        );
    }

    fn start_session(_start_index: u32) {
        log::info!(
            "validator-set.SessionManager.start_session: index:{}",
            _start_index
        );
    }
}

impl<T: Config> frame_support::traits::EstimateNextSessionRotation<T::BlockNumber> for Pallet<T> {
    fn average_session_length() -> T::BlockNumber {
        Zero::zero()
    }

    fn estimate_current_session_progress(
        _now: T::BlockNumber,
    ) -> (Option<sp_runtime::Permill>, frame_support::dispatch::Weight) {
        log::info!("estimate_current_session_progress");
        (None, Zero::zero())
    }

    fn estimate_next_session_rotation(
        _now: T::BlockNumber,
    ) -> (Option<T::BlockNumber>, frame_support::dispatch::Weight) {
        log::info!("estimate_next_session_rotation");
        (None, Zero::zero())
    }
}

/// Implementation of Convert trait for mapping ValidatorId with AccountId.
/// This is mainly used to map stash and controller keys.
/// In this module, for simplicity, we just return the same AccountId.
pub struct ValidatorOf<T>(sp_std::marker::PhantomData<T>);

impl<T: Config> Convert<T::AccountId, Option<T::AccountId>> for ValidatorOf<T> {
    fn convert(account: T::AccountId) -> Option<T::AccountId> {
        Some(account)
    }
}
