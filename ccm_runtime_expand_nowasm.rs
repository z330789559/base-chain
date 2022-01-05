#![feature(prelude_import)]
//! The Substrate Node Template runtime. This can be compiled with `#[no_std]`, ready for Wasm.
#![recursion_limit = "256"]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
pub const WASM_BINARY : Option < & [u8] > = Some (b"R\xbcSvF\xdb\x8e\x05rustc%1.58.0-nightly (dd549dcab 2021-11-25)") ;
mod ether_singer {
    use codec::{Decode, Encode};
    use frame_support::log;
    use sha3::{Digest, Keccak256};
    use sp_core::{ecdsa, H160, H256};
    #[cfg(feature = "std")]
    pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
    pub struct EthereumSignature(ecdsa::Signature);
    impl ::core::marker::StructuralEq for EthereumSignature {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for EthereumSignature {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<ecdsa::Signature>;
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for EthereumSignature {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for EthereumSignature {
        #[inline]
        fn eq(&self, other: &EthereumSignature) -> bool {
            match *other {
                EthereumSignature(ref __self_1_0) => match *self {
                    EthereumSignature(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &EthereumSignature) -> bool {
            match *other {
                EthereumSignature(ref __self_1_0) => match *self {
                    EthereumSignature(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for EthereumSignature {
        #[inline]
        fn clone(&self) -> EthereumSignature {
            match *self {
                EthereumSignature(ref __self_0_0) => {
                    EthereumSignature(::core::clone::Clone::clone(&(*__self_0_0)))
                }
            }
        }
    }
    const _: () = {
        impl ::codec::Encode for EthereumSignature {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                ::codec::Encode::encode_to(&&self.0, __codec_dest_edqy)
            }
            fn encode(&self) -> ::codec::alloc::vec::Vec<::core::primitive::u8> {
                ::codec::Encode::encode(&&self.0)
            }
            fn using_encoded<R, F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R>(
                &self,
                f: F,
            ) -> R {
                ::codec::Encode::using_encoded(&&self.0, f)
            }
        }
        impl ::codec::EncodeLike for EthereumSignature {}
    };
    const _: () = {
        impl ::codec::Decode for EthereumSignature {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                ::core::result::Result::Ok(EthereumSignature({
                    let __codec_res_edqy =
                        <ecdsa::Signature as ::codec::Decode>::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        ::core::result::Result::Err(e) => {
                            return ::core::result::Result::Err(
                                e.chain("Could not decode `EthereumSignature.0`"),
                            )
                        }
                        ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                }))
            }
        }
    };
    impl core::fmt::Debug for EthereumSignature {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            fmt.debug_tuple("EthereumSignature").field(&self.0).finish()
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for EthereumSignature {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(
                    __serializer,
                    "EthereumSignature",
                    &self.0,
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for EthereumSignature {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<EthereumSignature>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = EthereumSignature;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct EthereumSignature",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: ecdsa::Signature =
                            match <ecdsa::Signature as _serde::Deserialize>::deserialize(__e) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                        _serde::__private::Ok(EthereumSignature(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            ecdsa::Signature,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"tuple struct EthereumSignature with 1 element",
                                ));
                            }
                        };
                        _serde::__private::Ok(EthereumSignature(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "EthereumSignature",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<EthereumSignature>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl From<ecdsa::Signature> for EthereumSignature {
        fn from(x: ecdsa::Signature) -> Self {
            EthereumSignature(x)
        }
    }
    impl sp_runtime::traits::Verify for EthereumSignature {
        type Signer = EthereumSigner;
        fn verify<L: sp_runtime::traits::Lazy<[u8]>>(&self, mut msg: L, signer: &H160) -> bool {
            let mut m = [0u8; 32];
            m.copy_from_slice(Keccak256::digest(msg.get()).as_slice());
            match sp_io::crypto::secp256k1_ecdsa_recover(self.0.as_ref(), &m) {
                Ok(pubkey) => {
                    H160::from(H256::from_slice(Keccak256::digest(&pubkey).as_slice())) == *signer
                }
                Err(sp_io::EcdsaVerifyError::BadRS) => {
                    {
                        let lvl = ::log::Level::Error;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                ::core::fmt::Arguments::new_v1(
                                    &["Error recovering: Incorrect value of R or S"],
                                    &match () {
                                        _args => [],
                                    },
                                ),
                                lvl,
                                &(
                                    "evm",
                                    "ccm_runtime::ether_singer",
                                    "ccm/runtime/src/ether_singer.rs",
                                    30u32,
                                ),
                            );
                        }
                    };
                    false
                }
                Err(sp_io::EcdsaVerifyError::BadV) => {
                    {
                        let lvl = ::log::Level::Error;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                ::core::fmt::Arguments::new_v1(
                                    &["Error recovering: Incorrect value of V"],
                                    &match () {
                                        _args => [],
                                    },
                                ),
                                lvl,
                                &(
                                    "evm",
                                    "ccm_runtime::ether_singer",
                                    "ccm/runtime/src/ether_singer.rs",
                                    34u32,
                                ),
                            );
                        }
                    };
                    false
                }
                Err(sp_io::EcdsaVerifyError::BadSignature) => {
                    {
                        let lvl = ::log::Level::Error;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                ::core::fmt::Arguments::new_v1(
                                    &["Error recovering: Invalid signature"],
                                    &match () {
                                        _args => [],
                                    },
                                ),
                                lvl,
                                &(
                                    "evm",
                                    "ccm_runtime::ether_singer",
                                    "ccm/runtime/src/ether_singer.rs",
                                    38u32,
                                ),
                            );
                        }
                    };
                    false
                }
            }
        }
    }
    /// Public key for an Ethereum / H160 compatible account
    pub struct EthereumSigner([u8; 20]);
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for EthereumSigner {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(
                    __serializer,
                    "EthereumSigner",
                    &self.0,
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for EthereumSigner {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<EthereumSigner>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = EthereumSigner;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct EthereumSigner",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: [u8; 20] =
                            match <[u8; 20] as _serde::Deserialize>::deserialize(__e) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                        _serde::__private::Ok(EthereumSigner(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<[u8; 20]>(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"tuple struct EthereumSigner with 1 element",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(EthereumSigner(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "EthereumSigner",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<EthereumSigner>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl ::core::marker::StructuralEq for EthereumSigner {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for EthereumSigner {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<[u8; 20]>;
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for EthereumSigner {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for EthereumSigner {
        #[inline]
        fn eq(&self, other: &EthereumSigner) -> bool {
            match *other {
                EthereumSigner(ref __self_1_0) => match *self {
                    EthereumSigner(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &EthereumSigner) -> bool {
            match *other {
                EthereumSigner(ref __self_1_0) => match *self {
                    EthereumSigner(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for EthereumSigner {
        #[inline]
        fn cmp(&self, other: &EthereumSigner) -> ::core::cmp::Ordering {
            match *other {
                EthereumSigner(ref __self_1_0) => match *self {
                    EthereumSigner(ref __self_0_0) => {
                        match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                            cmp => cmp,
                        }
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for EthereumSigner {
        #[inline]
        fn partial_cmp(
            &self,
            other: &EthereumSigner,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                EthereumSigner(ref __self_1_0) => match *self {
                    EthereumSigner(ref __self_0_0) => {
                        match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                            }
                            cmp => cmp,
                        }
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for EthereumSigner {
        #[inline]
        fn clone(&self) -> EthereumSigner {
            match *self {
                EthereumSigner(ref __self_0_0) => {
                    EthereumSigner(::core::clone::Clone::clone(&(*__self_0_0)))
                }
            }
        }
    }
    const _: () = {
        impl ::codec::Encode for EthereumSigner {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                ::codec::Encode::encode_to(&&self.0, __codec_dest_edqy)
            }
            fn encode(&self) -> ::codec::alloc::vec::Vec<::core::primitive::u8> {
                ::codec::Encode::encode(&&self.0)
            }
            fn using_encoded<R, F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R>(
                &self,
                f: F,
            ) -> R {
                ::codec::Encode::using_encoded(&&self.0, f)
            }
        }
        impl ::codec::EncodeLike for EthereumSigner {}
    };
    const _: () = {
        impl ::codec::Decode for EthereumSigner {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                ::core::result::Result::Ok(EthereumSigner({
                    let __codec_res_edqy =
                        <[u8; 20] as ::codec::Decode>::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        ::core::result::Result::Err(e) => {
                            return ::core::result::Result::Err(
                                e.chain("Could not decode `EthereumSigner.0`"),
                            )
                        }
                        ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                }))
            }
        }
    };
    impl core::fmt::Debug for EthereumSigner {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            fmt.debug_tuple("EthereumSigner").field(&self.0).finish()
        }
    }
    impl sp_runtime::traits::IdentifyAccount for EthereumSigner {
        type AccountId = H160;
        fn into_account(self) -> H160 {
            self.0.into()
        }
    }
    impl From<[u8; 20]> for EthereumSigner {
        fn from(x: [u8; 20]) -> Self {
            EthereumSigner(x)
        }
    }
    impl From<ecdsa::Public> for EthereumSigner {
        fn from(x: ecdsa::Public) -> Self {
            let mut m = [0u8; 20];
            m.copy_from_slice(&x.as_ref()[13..33]);
            EthereumSigner(m)
        }
    }
    #[cfg(feature = "std")]
    impl std::fmt::Display for EthereumSigner {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["ethereum signature: "],
                &match (&H160::from_slice(&self.0),) {
                    _args => [::core::fmt::ArgumentV1::new(
                        _args.0,
                        ::core::fmt::Debug::fmt,
                    )],
                },
            ))
        }
    }
}
use codec::{Codec, Decode, Encode};
pub use ether_singer::*;
use pallet_evm::{AddressMapping, EnsureAddressNever, EnsureAddressRoot, FeeCalculator};
use pallet_grandpa::{fg_primitives, AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList};
use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{
    crypto::{KeyTypeId, Public},
    OpaqueMetadata, H160, H256, U256,
};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{
        BlakeTwo256, Block as BlockT, Dispatchable, IdentifyAccount, NumberFor, PostDispatchInfoOf,
        Verify,
    },
    transaction_validity::{TransactionSource, TransactionValidity, TransactionValidityError},
    ApplyExtrinsicResult, Percent,
};
use sp_runtime::traits::AccountIdConversion;
use sp_std::{marker::PhantomData, prelude::*};
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;
use fp_rpc::TransactionStatus;
use frame_support::dispatch::fmt::Debug;
use frame_support::sp_runtime::traits::{LookupError, StaticLookup};
use frame_support::sp_runtime::MultiAddress;
use frame_support::PalletId;
pub use frame_support::{
    construct_runtime, parameter_types,
    traits::{FindAuthor, KeyOwnerProofSystem, Randomness, LockIdentifier, Imbalance},
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
        DispatchClass, IdentityFee, Weight,
    },
    ConsensusEngineId, StorageValue,
};
use frame_support::traits::{Currency, OnUnbalanced, U128CurrencyToVote};
use frame_system::limits::{BlockWeights, BlockLength};
use frame_system::{EnsureOneOf, EnsureRoot};
pub use pallet_balances::Call as BalancesCall;
use pallet_ethereum::{Call::transact, Transaction as EthereumTransaction};
use pallet_evm::{Account as EVMAccount, Runner};
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::CurrencyAdapter;
use sp_core::u32_trait::{_1, _2, _3, _4, _5};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};
use pallet_balances::Config;
/// Type of block number.
/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = ether_singer::EthereumSignature;
/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;
/// Balance of an account.
/// Index of a transaction in the chain.
pub type Index = u32;
/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;
/// Digest item type.
pub type DigestItem = generic::DigestItem<Hash>;
/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
    use super::*;
    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;
    use ::sp_runtime::serde as __opaque_keys_serde_import__SessionKeys;
    #[serde(crate = "__opaque_keys_serde_import__SessionKeys")]
    pub struct SessionKeys {
        pub aura: <Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
        pub grandpa: <Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for SessionKeys {
        #[inline]
        fn default() -> SessionKeys {
            SessionKeys {
                aura: ::core::default::Default::default(),
                grandpa: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for SessionKeys {
        #[inline]
        fn clone(&self) -> SessionKeys {
            match *self {
                SessionKeys {
                    aura: ref __self_0_0,
                    grandpa: ref __self_0_1,
                } => SessionKeys {
                    aura: ::core::clone::Clone::clone(&(*__self_0_0)),
                    grandpa: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for SessionKeys {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for SessionKeys {
        #[inline]
        fn eq(&self, other: &SessionKeys) -> bool {
            match *other {
                SessionKeys {
                    aura: ref __self_1_0,
                    grandpa: ref __self_1_1,
                } => match *self {
                    SessionKeys {
                        aura: ref __self_0_0,
                        grandpa: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &SessionKeys) -> bool {
            match *other {
                SessionKeys {
                    aura: ref __self_1_0,
                    grandpa: ref __self_1_1,
                } => match *self {
                    SessionKeys {
                        aura: ref __self_0_0,
                        grandpa: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    impl ::core::marker::StructuralEq for SessionKeys {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for SessionKeys {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<
                    <Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                >;
                let _: ::core::cmp::AssertParamIsEq<
                    <Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                >;
            }
        }
    }
    const _: () = {
        impl ::codec::Encode for SessionKeys {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                ::codec::Encode::encode_to(&self.aura, __codec_dest_edqy);
                ::codec::Encode::encode_to(&self.grandpa, __codec_dest_edqy);
            }
        }
        impl ::codec::EncodeLike for SessionKeys {}
    };
    const _: () = {
        impl ::codec::Decode for SessionKeys {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                ::core::result::Result::Ok(SessionKeys {
                    aura: {
                        let __codec_res_edqy = < < Aura as :: sp_runtime :: BoundToRuntimeAppPublic > :: Public as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `SessionKeys::aura`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                    grandpa: {
                        let __codec_res_edqy = < < Grandpa as :: sp_runtime :: BoundToRuntimeAppPublic > :: Public as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `SessionKeys::grandpa`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                })
            }
        }
    };
    impl core::fmt::Debug for SessionKeys {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            fmt.debug_struct("SessionKeys")
                .field("aura", &self.aura)
                .field("grandpa", &self.grandpa)
                .finish()
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use __opaque_keys_serde_import__SessionKeys as _serde;
        #[automatically_derived]
        impl __opaque_keys_serde_import__SessionKeys::Serialize for SessionKeys {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> __opaque_keys_serde_import__SessionKeys::__private::Result<__S::Ok, __S::Error>
            where
                __S: __opaque_keys_serde_import__SessionKeys::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "SessionKeys",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "aura",
                    &self.aura,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "grandpa",
                    &self.grandpa,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use __opaque_keys_serde_import__SessionKeys as _serde;
        #[automatically_derived]
        impl<'de> __opaque_keys_serde_import__SessionKeys::Deserialize<'de> for SessionKeys {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> __opaque_keys_serde_import__SessionKeys::__private::Result<Self, __D::Error>
            where
                __D: __opaque_keys_serde_import__SessionKeys::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "aura" => _serde::__private::Ok(__Field::__field0),
                            "grandpa" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"aura" => _serde::__private::Ok(__Field::__field0),
                            b"grandpa" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<SessionKeys>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = SessionKeys;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct SessionKeys")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            <Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SessionKeys with 2 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            <Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct SessionKeys with 2 elements",
                                ));
                            }
                        };
                        _serde::__private::Ok(SessionKeys {
                            aura: __field0,
                            grandpa: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<
                            <Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                        > = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<
                            <Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "aura",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            <Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "grandpa",
                                            ),
                                        );
                                    }
                                    __field1 = _serde :: __private :: Some (match _serde :: de :: MapAccess :: next_value :: < < Grandpa as :: sp_runtime :: BoundToRuntimeAppPublic > :: Public > (& mut __map) { _serde :: __private :: Ok (__val) => __val , _serde :: __private :: Err (__err) => { return _serde :: __private :: Err (__err) ; } }) ;
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("aura") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("grandpa") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(SessionKeys {
                            aura: __field0,
                            grandpa: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["aura", "grandpa"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "SessionKeys",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<SessionKeys>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl SessionKeys {
        /// Generate a set of keys with optionally using the given seed.
        ///
        /// The generated key pairs are stored in the keystore.
        ///
        /// Returns the concatenated SCALE encoded public keys.
        pub fn generate(
            seed: Option<::sp_runtime::sp_std::vec::Vec<u8>>,
        ) -> ::sp_runtime::sp_std::vec::Vec<u8> {
            let keys = Self { aura : < < Aura as :: sp_runtime :: BoundToRuntimeAppPublic > :: Public as :: sp_runtime :: RuntimeAppPublic > :: generate_pair (seed . clone ()) , grandpa : < < Grandpa as :: sp_runtime :: BoundToRuntimeAppPublic > :: Public as :: sp_runtime :: RuntimeAppPublic > :: generate_pair (seed . clone ()) , } ;
            ::sp_runtime::codec::Encode::encode(&keys)
        }
        /// Converts `Self` into a `Vec` of `(raw public key, KeyTypeId)`.
        pub fn into_raw_public_keys(
            self,
        ) -> ::sp_runtime::sp_std::vec::Vec<(
            ::sp_runtime::sp_std::vec::Vec<u8>,
            ::sp_runtime::KeyTypeId,
        )> {
            let mut keys = Vec::new();
            keys . push ((:: sp_runtime :: RuntimeAppPublic :: to_raw_vec (& self . aura) , < < Aura as :: sp_runtime :: BoundToRuntimeAppPublic > :: Public as :: sp_runtime :: RuntimeAppPublic > :: ID)) ;
            keys . push ((:: sp_runtime :: RuntimeAppPublic :: to_raw_vec (& self . grandpa) , < < Grandpa as :: sp_runtime :: BoundToRuntimeAppPublic > :: Public as :: sp_runtime :: RuntimeAppPublic > :: ID)) ;
            keys
        }
        /// Decode `Self` from the given `encoded` slice and convert `Self` into the raw public
        /// keys (see [`Self::into_raw_public_keys`]).
        ///
        /// Returns `None` when the decoding failed, otherwise `Some(_)`.
        pub fn decode_into_raw_public_keys(
            encoded: &[u8],
        ) -> Option<
            ::sp_runtime::sp_std::vec::Vec<(
                ::sp_runtime::sp_std::vec::Vec<u8>,
                ::sp_runtime::KeyTypeId,
            )>,
        > {
            <Self as ::sp_runtime::codec::Decode>::decode(&mut &encoded[..])
                .ok()
                .map(|s| s.into_raw_public_keys())
        }
    }
    impl ::sp_runtime::traits::OpaqueKeys for SessionKeys {
        type KeyTypeIdProviders = (Aura, Grandpa);
        fn key_ids() -> &'static [::sp_runtime::KeyTypeId] {
            & [< < Aura as :: sp_runtime :: BoundToRuntimeAppPublic > :: Public as :: sp_runtime :: RuntimeAppPublic > :: ID , < < Grandpa as :: sp_runtime :: BoundToRuntimeAppPublic > :: Public as :: sp_runtime :: RuntimeAppPublic > :: ID]
        }
        fn get_raw(&self, i: ::sp_runtime::KeyTypeId) -> &[u8] {
            match i { i if i == < < Aura as :: sp_runtime :: BoundToRuntimeAppPublic > :: Public as :: sp_runtime :: RuntimeAppPublic > :: ID => self . aura . as_ref () , i if i == < < Grandpa as :: sp_runtime :: BoundToRuntimeAppPublic > :: Public as :: sp_runtime :: RuntimeAppPublic > :: ID => self . grandpa . as_ref () , _ => & [] , }
        }
    }
}
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: { ::sp_runtime::RuntimeString::Borrowed("ccm") },
    impl_name: { ::sp_runtime::RuntimeString::Borrowed("ccm") },
    authoring_version: 1,
    spec_version: 8,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
};
pub use primitive::*;
/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}
pub const MAXIMUM_BLOCK_WEIGHT: Weight = 2 * WEIGHT_PER_SECOND;
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_perthousand(25);
pub struct Version;
impl Version {
    /// Returns the value of this parameter type.
    pub const fn get() -> RuntimeVersion {
        VERSION
    }
}
impl<I: From<RuntimeVersion>> ::frame_support::traits::Get<I> for Version {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct BlockHashCount;
impl BlockHashCount {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        2400
    }
}
impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for BlockHashCount {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct RuntimeBlockLength;
impl RuntimeBlockLength {
    /// Returns the value of this parameter type.
    pub fn get() -> BlockLength {
        BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO)
    }
}
impl<I: From<BlockLength>> ::frame_support::traits::Get<I> for RuntimeBlockLength {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct RuntimeBlockWeights;
impl RuntimeBlockWeights {
    /// Returns the value of this parameter type.
    pub fn get() -> BlockWeights {
        BlockWeights::builder()
            .base_block(BlockExecutionWeight::get())
            .for_class(DispatchClass::all(), |weights| {
                weights.base_extrinsic = ExtrinsicBaseWeight::get();
            })
            .for_class(DispatchClass::Normal, |weights| {
                weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
            })
            .for_class(DispatchClass::Operational, |weights| {
                weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
                weights.reserved =
                    Some(MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
            })
            .avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
            .build_or_panic()
    }
}
impl<I: From<BlockWeights>> ::frame_support::traits::Get<I> for RuntimeBlockWeights {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct SS58Prefix;
impl SS58Prefix {
    /// Returns the value of this parameter type.
    pub const fn get() -> u8 {
        42
    }
}
impl<I: From<u8>> ::frame_support::traits::Get<I> for SS58Prefix {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct MulitiAccountIdLookup<AccountId>(PhantomData<AccountId>);
impl<AccountId> StaticLookup for MulitiAccountIdLookup<AccountId>
where
    AccountId: Codec + Clone + PartialEq + Debug + 'static + From<H160> + Into<H160>,
{
    type Source = MultiAddress<AccountId, ()>;
    type Target = H160;
    fn lookup(s: Self::Source) -> Result<Self::Target, LookupError> {
        match s {
            MultiAddress::Id(i) => Ok(i.into()),
            MultiAddress::Raw(i) => Ok(H160::from_slice(i.as_slice())),
            MultiAddress::Address20(i) => Ok(H160::from(i)),
            MultiAddress::Address32(i) => Ok(H160::from_slice(&i[..20])),
            MultiAddress::Index(_) => Err(LookupError),
        }
    }
    fn unlookup(t: Self::Target) -> Self::Source {
        MultiAddress::Address20(t.0)
    }
}
impl frame_system::Config for Runtime {
    /// The basic call filter to use in dispatchable.
    type BaseCallFilter = frame_support::traits::Everything;
    /// Block & extrinsics weights: base values and limits.
    type BlockWeights = RuntimeBlockWeights;
    /// The maximum length of a block (in bytes).
    type BlockLength = RuntimeBlockLength;
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The aggregated dispatch type that is available for extrinsics.
    type Call = Call;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = MulitiAccountIdLookup<AccountId>;
    /// The index type for storing how many extrinsics an account has signed.
    type Index = Index;
    /// The index type for blocks.
    type BlockNumber = BlockNumber;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The header type.
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// The ubiquitous event type.
    type Event = Event;
    /// The ubiquitous origin type.
    type Origin = Origin;
    /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
    type BlockHashCount = BlockHashCount;
    /// The weight of database operations that the runtime can invoke.
    type DbWeight = RocksDbWeight;
    /// Version of the runtime.
    type Version = Version;
    /// Converts a module to the index of the module in `construct_runtime!`.
    ///
    /// This type is being generated by `construct_runtime!`.
    type PalletInfo = PalletInfo;
    /// What to do if a new account is created.
    type OnNewAccount = ();
    /// What to do if an account is fully reaped from the system.
    type OnKilledAccount = ();
    /// The data to be stored in an account.
    type AccountData = pallet_balances::AccountData<Balance>;
    /// Weight information for the extrinsics of this pallet.
    type SystemWeightInfo = ();
    /// This is used as an identifier of the chain. 42 is the generic substrate prefix.
    type SS58Prefix = SS58Prefix;
    /// The set code logic, just the default since we're not a parachain.
    type OnSetCode = ();
}
pub struct MaxAuthorities;
impl MaxAuthorities {
    /// Returns the value of this parameter type.
    pub const fn get() -> u32 {
        100
    }
}
impl<I: From<u32>> ::frame_support::traits::Get<I> for MaxAuthorities {
    fn get() -> I {
        I::from(Self::get())
    }
}
impl pallet_aura::Config for Runtime {
    type AuthorityId = AuraId;
    type DisabledValidators = ();
    type MaxAuthorities = MaxAuthorities;
}
impl pallet_grandpa::Config for Runtime {
    type Event = Event;
    type Call = Call;
    type KeyOwnerProofSystem = ();
    type KeyOwnerProof =
        <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;
    type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        GrandpaId,
    )>>::IdentificationTuple;
    type HandleEquivocation = ();
    type WeightInfo = ();
}
pub struct MinimumPeriod;
impl MinimumPeriod {
    /// Returns the value of this parameter type.
    pub const fn get() -> u64 {
        SLOT_DURATION / 2
    }
}
impl<I: From<u64>> ::frame_support::traits::Get<I> for MinimumPeriod {
    fn get() -> I {
        I::from(Self::get())
    }
}
impl pallet_timestamp::Config for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
    #[cfg(feature = "aura")]
    type OnTimestampSet = Aura;
}
pub struct ExistentialDeposit;
impl ExistentialDeposit {
    /// Returns the value of this parameter type.
    pub const fn get() -> u128 {
        5 * MILLICENTS
    }
}
impl<I: From<u128>> ::frame_support::traits::Get<I> for ExistentialDeposit {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct MaxLocks;
impl MaxLocks {
    /// Returns the value of this parameter type.
    pub const fn get() -> u32 {
        50
    }
}
impl<I: From<u32>> ::frame_support::traits::Get<I> for MaxLocks {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct CommissionStorage;
impl CommissionStorage {
    /// Returns the value of this parameter type.
    pub const fn get() -> PalletId {
        PalletId(*b"ccm/cosm")
    }
}
impl<I: From<PalletId>> ::frame_support::traits::Get<I> for CommissionStorage {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct CommissionRate;
impl CommissionRate {
    /// Returns the value of this parameter type.
    pub const fn get() -> Permill {
        Permill::from_percent(10)
    }
}
impl<I: From<Permill>> ::frame_support::traits::Get<I> for CommissionRate {
    fn get() -> I {
        I::from(Self::get())
    }
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
pub struct TransactionByteFee;
impl TransactionByteFee {
    /// Returns the value of this parameter type.
    pub const fn get() -> Balance {
        1
    }
}
impl<I: From<Balance>> ::frame_support::traits::Get<I> for TransactionByteFee {
    fn get() -> I {
        I::from(Self::get())
    }
}
impl pallet_transaction_payment::Config for Runtime {
    type OnChargeTransaction = CurrencyAdapter<Balances, DealWithFees>;
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ();
}
impl pallet_sudo::Config for Runtime {
    type Event = Event;
    type Call = Call;
}
pub struct FindAuthorTruncated<F>(PhantomData<F>);
impl<F: FindAuthor<u32>> FindAuthor<H160> for FindAuthorTruncated<F> {
    fn find_author<'a, I>(digests: I) -> Option<H160>
    where
        I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
    {
        if let Some(author_index) = F::find_author(digests) {
            let authority_id = Aura::authorities()[author_index as usize].clone();
            return Some(H160::from_slice(&authority_id.to_raw_vec()[4..24]));
        }
        None
    }
}
pub struct MaximumSchedulerWeight;
impl MaximumSchedulerWeight {
    /// Returns the value of this parameter type.
    pub fn get() -> Weight {
        Perbill::from_percent(80) * RuntimeBlockWeights::get().max_block
    }
}
impl<I: From<Weight>> ::frame_support::traits::Get<I> for MaximumSchedulerWeight {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct MaxScheduledPerBlock;
impl MaxScheduledPerBlock {
    /// Returns the value of this parameter type.
    pub const fn get() -> u32 {
        50
    }
}
impl<I: From<u32>> ::frame_support::traits::Get<I> for MaxScheduledPerBlock {
    fn get() -> I {
        I::from(Self::get())
    }
}
impl pallet_scheduler::Config for Runtime {
    type Event = Event;
    type Origin = Origin;
    type PalletsOrigin = OriginCaller;
    type Call = Call;
    type MaximumWeight = MaximumSchedulerWeight;
    type ScheduleOrigin = EnsureRoot<AccountId>;
    type MaxScheduledPerBlock = MaxScheduledPerBlock;
    type WeightInfo = pallet_scheduler::weights::SubstrateWeight<Runtime>;
}
#[cfg(not(ccmtest))]
pub const LocalChainId: u64 = 88;
pub struct ChainId;
impl ChainId {
    /// Returns the value of this parameter type.
    pub const fn get() -> u64 {
        LocalChainId
    }
}
impl<I: From<u64>> ::frame_support::traits::Get<I> for ChainId {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct BlockGasLimit;
impl BlockGasLimit {
    /// Returns the value of this parameter type.
    pub fn get() -> U256 {
        U256::from(u32::max_value())
    }
}
impl<I: From<U256>> ::frame_support::traits::Get<I> for BlockGasLimit {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct IdentityAddressMapping;
impl AddressMapping<H160> for IdentityAddressMapping {
    fn into_account_id(address: H160) -> H160 {
        address
    }
}
pub struct EthereumFindAuthor<F>(PhantomData<F>);
impl<F: FindAuthor<u32>> FindAuthor<H160> for EthereumFindAuthor<F> {
    fn find_author<'a, I>(digests: I) -> Option<H160>
    where
        I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
    {
        if let Some(author_index) = F::find_author(digests) {
            let authority_id = Aura::authorities()[author_index as usize].clone();
            return Some(H160::from_slice(&authority_id.as_slice()[4..24]));
        }
        None
    }
}
impl pallet_evm::Config for Runtime {
    type FeeCalculator = DynamicFee;
    type GasWeightMapping = ();
    type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
    type CallOrigin = EnsureAddressRoot<AccountId>;
    type WithdrawOrigin = EnsureAddressNever<AccountId>;
    type AddressMapping = IdentityAddressMapping;
    type Currency = Balances;
    type Event = Event;
    type Runner = pallet_evm::runner::stack::Runner<Self>;
    type Precompiles = (
        pallet_evm_precompile_simple::ECRecover,
        pallet_evm_precompile_simple::Sha256,
        pallet_evm_precompile_simple::Ripemd160,
        pallet_evm_precompile_simple::Identity,
        pallet_evm_precompile_modexp::Modexp,
        pallet_evm_precompile_simple::ECRecoverPublicKey,
        pallet_evm_precompile_sha3fips::Sha3FIPS256,
        pallet_evm_precompile_sha3fips::Sha3FIPS512,
    );
    type ChainId = ChainId;
    type BlockGasLimit = BlockGasLimit;
    type OnChargeTransaction = ();
    type FindAuthor = EthereumFindAuthor<Aura>;
}
impl pallet_ethereum::Config for Runtime {
    type Event = Event;
    type StateRoot = pallet_ethereum::IntermediateStateRoot;
}
pub struct BoundDivision;
impl BoundDivision {
    /// Returns the value of this parameter type.
    pub fn get() -> U256 {
        U256::from(1024)
    }
}
impl<I: From<U256>> ::frame_support::traits::Get<I> for BoundDivision {
    fn get() -> I {
        I::from(Self::get())
    }
}
impl pallet_dynamic_fee::Config for Runtime {
    type MinGasPriceBoundDivisor = BoundDivision;
}
impl pallet_randomness_collective_flip::Config for Runtime {}
pub struct ProposalBond;
impl ProposalBond {
    /// Returns the value of this parameter type.
    pub const fn get() -> Permill {
        Permill::from_percent(5)
    }
}
impl<I: From<Permill>> ::frame_support::traits::Get<I> for ProposalBond {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct ProposalBondMinimum;
impl ProposalBondMinimum {
    /// Returns the value of this parameter type.
    pub const fn get() -> Balance {
        1 * DOLLARS
    }
}
impl<I: From<Balance>> ::frame_support::traits::Get<I> for ProposalBondMinimum {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct SpendPeriod;
impl SpendPeriod {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        1 * DAYS
    }
}
impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for SpendPeriod {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct Burn;
impl Burn {
    /// Returns the value of this parameter type.
    pub const fn get() -> Permill {
        Permill::from_percent(1)
    }
}
impl<I: From<Permill>> ::frame_support::traits::Get<I> for Burn {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct TreasuryModuleId;
impl TreasuryModuleId {
    /// Returns the value of this parameter type.
    pub const fn get() -> PalletId {
        PalletId(*b"py/trsry")
    }
}
impl<I: From<PalletId>> ::frame_support::traits::Get<I> for TreasuryModuleId {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct TipCountdown;
impl TipCountdown {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        1 * DAYS
    }
}
impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for TipCountdown {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct TipFindersFee;
impl TipFindersFee {
    /// Returns the value of this parameter type.
    pub const fn get() -> Percent {
        Percent::from_percent(20)
    }
}
impl<I: From<Percent>> ::frame_support::traits::Get<I> for TipFindersFee {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct TipReportDepositBase;
impl TipReportDepositBase {
    /// Returns the value of this parameter type.
    pub const fn get() -> Balance {
        1 * DOLLARS
    }
}
impl<I: From<Balance>> ::frame_support::traits::Get<I> for TipReportDepositBase {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct DataDepositPerByte;
impl DataDepositPerByte {
    /// Returns the value of this parameter type.
    pub const fn get() -> Balance {
        10 * MILLICENTS
    }
}
impl<I: From<Balance>> ::frame_support::traits::Get<I> for DataDepositPerByte {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct MaximumReasonLength;
impl MaximumReasonLength {
    /// Returns the value of this parameter type.
    pub const fn get() -> u32 {
        16384
    }
}
impl<I: From<u32>> ::frame_support::traits::Get<I> for MaximumReasonLength {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct BountyDepositBase;
impl BountyDepositBase {
    /// Returns the value of this parameter type.
    pub const fn get() -> Balance {
        1 * DOLLARS
    }
}
impl<I: From<Balance>> ::frame_support::traits::Get<I> for BountyDepositBase {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct BountyDepositPayoutDelay;
impl BountyDepositPayoutDelay {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        1 * DAYS
    }
}
impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for BountyDepositPayoutDelay {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct BountyUpdatePeriod;
impl BountyUpdatePeriod {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        7 * DAYS
    }
}
impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for BountyUpdatePeriod {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct BountyCuratorDeposit;
impl BountyCuratorDeposit {
    /// Returns the value of this parameter type.
    pub const fn get() -> Permill {
        Permill::from_percent(50)
    }
}
impl<I: From<Permill>> ::frame_support::traits::Get<I> for BountyCuratorDeposit {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct BountyValueMinimum;
impl BountyValueMinimum {
    /// Returns the value of this parameter type.
    pub const fn get() -> Balance {
        5 * DOLLARS
    }
}
impl<I: From<Balance>> ::frame_support::traits::Get<I> for BountyValueMinimum {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct MaxApprovals;
impl MaxApprovals {
    /// Returns the value of this parameter type.
    pub const fn get() -> u32 {
        100
    }
}
impl<I: From<u32>> ::frame_support::traits::Get<I> for MaxApprovals {
    fn get() -> I {
        I::from(Self::get())
    }
}
impl pallet_treasury::Config for Runtime {
    type PalletId = TreasuryModuleId;
    type Currency = Balances;
    type ApproveOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, CouncilCollective>;
    type RejectOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _5, AccountId, CouncilCollective>;
    type Event = Event;
    type OnSlash = ();
    type ProposalBond = ProposalBond;
    type ProposalBondMinimum = ProposalBondMinimum;
    type SpendPeriod = SpendPeriod;
    type Burn = Burn;
    type SpendFunds = Bounties;
    type BurnDestination = ();
    type WeightInfo = pallet_treasury::weights::SubstrateWeight<Runtime>;
    type MaxApprovals = MaxApprovals;
}
impl pallet_bounties::Config for Runtime {
    type Event = Event;
    type BountyDepositBase = BountyDepositBase;
    type BountyDepositPayoutDelay = BountyDepositPayoutDelay;
    type BountyUpdatePeriod = BountyUpdatePeriod;
    type BountyCuratorDeposit = BountyCuratorDeposit;
    type BountyValueMinimum = BountyValueMinimum;
    type DataDepositPerByte = DataDepositPerByte;
    type MaximumReasonLength = MaximumReasonLength;
    type WeightInfo = pallet_bounties::weights::SubstrateWeight<Runtime>;
}
pub struct CouncilMotionDuration;
impl CouncilMotionDuration {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        3 * DAYS
    }
}
impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for CouncilMotionDuration {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct CouncilMaxProposals;
impl CouncilMaxProposals {
    /// Returns the value of this parameter type.
    pub const fn get() -> u32 {
        100
    }
}
impl<I: From<u32>> ::frame_support::traits::Get<I> for CouncilMaxProposals {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct GeneralCouncilMaxMembers;
impl GeneralCouncilMaxMembers {
    /// Returns the value of this parameter type.
    pub const fn get() -> u32 {
        100
    }
}
impl<I: From<u32>> ::frame_support::traits::Get<I> for GeneralCouncilMaxMembers {
    fn get() -> I {
        I::from(Self::get())
    }
}
type CouncilCollective = pallet_collective::Instance1;
impl pallet_collective::Config<CouncilCollective> for Runtime {
    type Origin = Origin;
    type Proposal = Call;
    type Event = Event;
    type MotionDuration = CouncilMotionDuration;
    type MaxProposals = CouncilMaxProposals;
    type MaxMembers = GeneralCouncilMaxMembers;
    type DefaultVote = pallet_collective::PrimeDefaultVote;
    type WeightInfo = pallet_collective::weights::SubstrateWeight<Runtime>;
}
type NegativeImbalance = <Balances as Currency<AccountId>>::NegativeImbalance;
pub struct DealWithFees;
impl OnUnbalanced<NegativeImbalance> for DealWithFees {
    fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance>) {
        let mut total = fees_then_tips
            .fold(NegativeImbalance::zero(), |sum, i| sum.merge(i))
            .peek();
        let address = CommissionStorage::get().into_account();
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(
                    ::core::fmt::Arguments::new_v1(
                        &["DealWithFees:address:", " got fees:"],
                        &match (&address, &total) {
                            _args => [
                                ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Debug::fmt),
                                ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Debug::fmt),
                            ],
                        },
                    ),
                    lvl,
                    &(
                        "ccm_runtime",
                        "ccm_runtime",
                        "ccm/runtime/src/lib.rs",
                        536u32,
                    ),
                );
            }
        };
        let min = Balances::minimum_balance();
        let free = Balances::free_balance(address);
        if (free + total) < min {
            total += (min - free);
        }
        Balances::deposit_creating(&address, total);
    }
}
pub struct AssetDeposit;
impl AssetDeposit {
    /// Returns the value of this parameter type.
    pub const fn get() -> Balance {
        100 * DOLLARS
    }
}
impl<I: From<Balance>> ::frame_support::traits::Get<I> for AssetDeposit {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct ApprovalDeposit;
impl ApprovalDeposit {
    /// Returns the value of this parameter type.
    pub const fn get() -> Balance {
        1 * DOLLARS
    }
}
impl<I: From<Balance>> ::frame_support::traits::Get<I> for ApprovalDeposit {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct StringLimit;
impl StringLimit {
    /// Returns the value of this parameter type.
    pub const fn get() -> u32 {
        50
    }
}
impl<I: From<u32>> ::frame_support::traits::Get<I> for StringLimit {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct MetadataDepositBase;
impl MetadataDepositBase {
    /// Returns the value of this parameter type.
    pub const fn get() -> Balance {
        10 * DOLLARS
    }
}
impl<I: From<Balance>> ::frame_support::traits::Get<I> for MetadataDepositBase {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct MetadataDepositPerByte;
impl MetadataDepositPerByte {
    /// Returns the value of this parameter type.
    pub const fn get() -> Balance {
        1 * DOLLARS
    }
}
impl<I: From<Balance>> ::frame_support::traits::Get<I> for MetadataDepositPerByte {
    fn get() -> I {
        I::from(Self::get())
    }
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
    type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
}
pub struct LaunchPeriod;
impl LaunchPeriod {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        2 * 24 * 60 * MINUTES
    }
}
impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for LaunchPeriod {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct VotingPeriod;
impl VotingPeriod {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        2 * 24 * 60 * MINUTES
    }
}
impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for VotingPeriod {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct FastTrackVotingPeriod;
impl FastTrackVotingPeriod {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        1 * 24 * 60 * MINUTES
    }
}
impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for FastTrackVotingPeriod {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct InstantAllowed;
impl InstantAllowed {
    /// Returns the value of this parameter type.
    pub const fn get() -> bool {
        true
    }
}
impl<I: From<bool>> ::frame_support::traits::Get<I> for InstantAllowed {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct MinimumDeposit;
impl MinimumDeposit {
    /// Returns the value of this parameter type.
    pub const fn get() -> Balance {
        10 * DOLLARS
    }
}
impl<I: From<Balance>> ::frame_support::traits::Get<I> for MinimumDeposit {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct EnactmentPeriod;
impl EnactmentPeriod {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        7 * 24 * 60 * MINUTES
    }
}
impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for EnactmentPeriod {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct CooloffPeriod;
impl CooloffPeriod {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        7 * 24 * 60 * MINUTES
    }
}
impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for CooloffPeriod {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct PreimageByteDeposit;
impl PreimageByteDeposit {
    /// Returns the value of this parameter type.
    pub const fn get() -> Balance {
        1 * CENTS
    }
}
impl<I: From<Balance>> ::frame_support::traits::Get<I> for PreimageByteDeposit {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct MaxVotes;
impl MaxVotes {
    /// Returns the value of this parameter type.
    pub const fn get() -> u32 {
        100
    }
}
impl<I: From<u32>> ::frame_support::traits::Get<I> for MaxVotes {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct MaxProposals;
impl MaxProposals {
    /// Returns the value of this parameter type.
    pub const fn get() -> u32 {
        100
    }
}
impl<I: From<u32>> ::frame_support::traits::Get<I> for MaxProposals {
    fn get() -> I {
        I::from(Self::get())
    }
}
impl pallet_democracy::Config for Runtime {
    type Proposal = Call;
    type Event = Event;
    type Currency = Balances;
    type EnactmentPeriod = EnactmentPeriod;
    type LaunchPeriod = LaunchPeriod;
    type VotingPeriod = VotingPeriod;
    type VoteLockingPeriod = EnactmentPeriod;
    type MinimumDeposit = MinimumDeposit;
    /// A straight majority of the council can decide what their next motion is.
    type ExternalOrigin =
        pallet_collective::EnsureProportionAtLeast<_1, _2, AccountId, CouncilCollective>;
    /// A super-majority can have the next scheduled referendum be a straight majority-carries vote.
    type ExternalMajorityOrigin =
        pallet_collective::EnsureProportionAtLeast<_3, _4, AccountId, CouncilCollective>;
    /// A unanimous council can have the next scheduled referendum be a straight default-carries
    /// (NTB) vote.
    type ExternalDefaultOrigin =
        pallet_collective::EnsureProportionAtLeast<_1, _1, AccountId, CouncilCollective>;
    /// Two thirds of the technical committee can have an ExternalMajority/ExternalDefault vote
    /// be tabled immediately and with a shorter voting/enactment period.
    type FastTrackOrigin =
        pallet_collective::EnsureProportionAtLeast<_2, _3, AccountId, TechnicalCollective>;
    type InstantOrigin =
        pallet_collective::EnsureProportionAtLeast<_1, _1, AccountId, TechnicalCollective>;
    type InstantAllowed = InstantAllowed;
    type FastTrackVotingPeriod = FastTrackVotingPeriod;
    type CancellationOrigin =
        pallet_collective::EnsureProportionAtLeast<_2, _3, AccountId, CouncilCollective>;
    type CancelProposalOrigin = EnsureOneOf<
        AccountId,
        EnsureRoot<AccountId>,
        pallet_collective::EnsureProportionAtLeast<_1, _1, AccountId, TechnicalCollective>,
    >;
    type BlacklistOrigin = EnsureRoot<AccountId>;
    type VetoOrigin = pallet_collective::EnsureMember<AccountId, TechnicalCollective>;
    type CooloffPeriod = CooloffPeriod;
    type PreimageByteDeposit = PreimageByteDeposit;
    type OperationalPreimageOrigin = pallet_collective::EnsureMember<AccountId, CouncilCollective>;
    type Slash = Treasury;
    type Scheduler = Scheduler;
    type PalletsOrigin = OriginCaller;
    type MaxVotes = MaxVotes;
    type WeightInfo = pallet_democracy::weights::SubstrateWeight<Runtime>;
    type MaxProposals = MaxProposals;
}
pub struct CandidacyBond;
impl CandidacyBond {
    /// Returns the value of this parameter type.
    pub const fn get() -> Balance {
        10 * DOLLARS
    }
}
impl<I: From<Balance>> ::frame_support::traits::Get<I> for CandidacyBond {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct VotingBondBase;
impl VotingBondBase {
    /// Returns the value of this parameter type.
    pub const fn get() -> Balance {
        deposit(1, 64)
    }
}
impl<I: From<Balance>> ::frame_support::traits::Get<I> for VotingBondBase {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct VotingBondFactor;
impl VotingBondFactor {
    /// Returns the value of this parameter type.
    pub const fn get() -> Balance {
        deposit(0, 32)
    }
}
impl<I: From<Balance>> ::frame_support::traits::Get<I> for VotingBondFactor {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct TermDuration;
impl TermDuration {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        7 * DAYS
    }
}
impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for TermDuration {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct DesiredMembers;
impl DesiredMembers {
    /// Returns the value of this parameter type.
    pub const fn get() -> u32 {
        13
    }
}
impl<I: From<u32>> ::frame_support::traits::Get<I> for DesiredMembers {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct DesiredRunnersUp;
impl DesiredRunnersUp {
    /// Returns the value of this parameter type.
    pub const fn get() -> u32 {
        7
    }
}
impl<I: From<u32>> ::frame_support::traits::Get<I> for DesiredRunnersUp {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct ElectionsPhragmenPalletId;
impl ElectionsPhragmenPalletId {
    /// Returns the value of this parameter type.
    pub const fn get() -> LockIdentifier {
        *b"phrelect"
    }
}
impl<I: From<LockIdentifier>> ::frame_support::traits::Get<I> for ElectionsPhragmenPalletId {
    fn get() -> I {
        I::from(Self::get())
    }
}
impl pallet_elections_phragmen::Config for Runtime {
    type Event = Event;
    type PalletId = ElectionsPhragmenPalletId;
    type Currency = Balances;
    type ChangeMembers = Council;
    type InitializeMembers = Council;
    type CurrencyToVote = U128CurrencyToVote;
    type CandidacyBond = CandidacyBond;
    type VotingBondBase = VotingBondBase;
    type VotingBondFactor = VotingBondFactor;
    type LoserCandidate = DealWithFees;
    type KickedMember = DealWithFees;
    type DesiredMembers = DesiredMembers;
    type DesiredRunnersUp = DesiredRunnersUp;
    type TermDuration = TermDuration;
    type WeightInfo = pallet_elections_phragmen::weights::SubstrateWeight<Runtime>;
}
pub struct TechnicalMotionDuration;
impl TechnicalMotionDuration {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        5 * DAYS
    }
}
impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for TechnicalMotionDuration {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct TechnicalMaxProposals;
impl TechnicalMaxProposals {
    /// Returns the value of this parameter type.
    pub const fn get() -> u32 {
        100
    }
}
impl<I: From<u32>> ::frame_support::traits::Get<I> for TechnicalMaxProposals {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct TechnicalMaxMembers;
impl TechnicalMaxMembers {
    /// Returns the value of this parameter type.
    pub const fn get() -> u32 {
        100
    }
}
impl<I: From<u32>> ::frame_support::traits::Get<I> for TechnicalMaxMembers {
    fn get() -> I {
        I::from(Self::get())
    }
}
type TechnicalCollective = pallet_collective::Instance2;
impl pallet_collective::Config<TechnicalCollective> for Runtime {
    type Origin = Origin;
    type Proposal = Call;
    type Event = Event;
    type MotionDuration = TechnicalMotionDuration;
    type MaxProposals = TechnicalMaxProposals;
    type MaxMembers = TechnicalMaxMembers;
    type DefaultVote = pallet_collective::MoreThanMajorityThenPrimeDefaultVote;
    type WeightInfo = pallet_collective::weights::SubstrateWeight<Runtime>;
}
pub struct CreateClassDeposit;
impl CreateClassDeposit {
    /// Returns the value of this parameter type.
    pub fn get() -> Balance {
        0 * CENTS
    }
}
impl<I: From<Balance>> ::frame_support::traits::Get<I> for CreateClassDeposit {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct CreateAssetDeposit;
impl CreateAssetDeposit {
    /// Returns the value of this parameter type.
    pub fn get() -> Balance {
        0 * CENTS
    }
}
impl<I: From<Balance>> ::frame_support::traits::Get<I> for CreateAssetDeposit {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct NftPalletId;
impl NftPalletId {
    /// Returns the value of this parameter type.
    pub const fn get() -> PalletId {
        PalletId(*b"par/pnft")
    }
}
impl<I: From<PalletId>> ::frame_support::traits::Get<I> for NftPalletId {
    fn get() -> I {
        I::from(Self::get())
    }
}
impl pallet_nft::Config for Runtime {
    type Event = Event;
    type CreateClassDeposit = CreateClassDeposit;
    type CreateAssetDeposit = CreateAssetDeposit;
    type PalletId = NftPalletId;
    type WeightInfo = pallet_nft::weights::SubstrateWeight<Runtime>;
}
impl orml_nft::Config for Runtime {
    type Currency = Balances;
    type ClassId = u32;
    type TokenId = u64;
}
pub struct InitTotalSupply;
impl InitTotalSupply {
    /// Returns the value of this parameter type.
    pub const fn get() -> Balance {
        10_000_000_000 * DOLLARS
    }
}
impl<I: From<Balance>> ::frame_support::traits::Get<I> for InitTotalSupply {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct InitSupplyPeriod;
impl InitSupplyPeriod {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        30 * 12 * 5 * DAYS
    }
}
impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for InitSupplyPeriod {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct MalePenguinEggLimit;
impl MalePenguinEggLimit {
    /// Returns the value of this parameter type.
    pub const fn get() -> Balance {
        5000 * DOLLARS
    }
}
impl<I: From<Balance>> ::frame_support::traits::Get<I> for MalePenguinEggLimit {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct SmallYellowPenguinLockPeriod;
impl SmallYellowPenguinLockPeriod {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        14 * DAYS
    }
}
impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for SmallYellowPenguinLockPeriod {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct SmallYellowPenguinGrowPeriod;
impl SmallYellowPenguinGrowPeriod {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        30 * DAYS
    }
}
impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for SmallYellowPenguinGrowPeriod {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct RedPenguinEggCountForIncubation;
impl RedPenguinEggCountForIncubation {
    /// Returns the value of this parameter type.
    pub const fn get() -> Balance {
        20 * DOLLARS
    }
}
impl<I: From<Balance>> ::frame_support::traits::Get<I> for RedPenguinEggCountForIncubation {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct YellowPenguinEggCountForIncubation;
impl YellowPenguinEggCountForIncubation {
    /// Returns the value of this parameter type.
    pub const fn get() -> Balance {
        20 * DOLLARS
    }
}
impl<I: From<Balance>> ::frame_support::traits::Get<I> for YellowPenguinEggCountForIncubation {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct MalePenguinLifeSpan;
impl MalePenguinLifeSpan {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        7 * DAYS
    }
}
impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for MalePenguinLifeSpan {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct ColdStroage;
impl ColdStroage {
    /// Returns the value of this parameter type.
    pub const fn get() -> PalletId {
        PalletId(*b"par/cold")
    }
}
impl<I: From<PalletId>> ::frame_support::traits::Get<I> for ColdStroage {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct ClassOwnerId;
impl ClassOwnerId {
    /// Returns the value of this parameter type.
    pub const fn get() -> PalletId {
        PalletId(*b"par/cwid")
    }
}
impl<I: From<PalletId>> ::frame_support::traits::Get<I> for ClassOwnerId {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct IncubationId;
impl IncubationId {
    /// Returns the value of this parameter type.
    pub const fn get() -> PalletId {
        PalletId(*b"par/incu")
    }
}
impl<I: From<PalletId>> ::frame_support::traits::Get<I> for IncubationId {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct TechnicalStashId;
impl TechnicalStashId {
    /// Returns the value of this parameter type.
    pub const fn get() -> PalletId {
        PalletId(*b"par/tech")
    }
}
impl<I: From<PalletId>> ::frame_support::traits::Get<I> for TechnicalStashId {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct OperationStashId;
impl OperationStashId {
    /// Returns the value of this parameter type.
    pub const fn get() -> PalletId {
        PalletId(*b"par/oper")
    }
}
impl<I: From<PalletId>> ::frame_support::traits::Get<I> for OperationStashId {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct MalePenguinEggRate;
impl MalePenguinEggRate {
    /// Returns the value of this parameter type.
    pub const fn get() -> Permill {
        Permill::from_percent(2)
    }
}
impl<I: From<Permill>> ::frame_support::traits::Get<I> for MalePenguinEggRate {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct RedPenguinEggRate;
impl RedPenguinEggRate {
    /// Returns the value of this parameter type.
    pub const fn get() -> Permill {
        Permill::from_percent(40)
    }
}
impl<I: From<Permill>> ::frame_support::traits::Get<I> for RedPenguinEggRate {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct YellowPenguinEggRate;
impl YellowPenguinEggRate {
    /// Returns the value of this parameter type.
    pub const fn get() -> Permill {
        Permill::from_percent(48)
    }
}
impl<I: From<Permill>> ::frame_support::traits::Get<I> for YellowPenguinEggRate {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct TechnicalPenguinEggRate;
impl TechnicalPenguinEggRate {
    /// Returns the value of this parameter type.
    pub const fn get() -> Permill {
        Permill::from_percent(5)
    }
}
impl<I: From<Permill>> ::frame_support::traits::Get<I> for TechnicalPenguinEggRate {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct OperationPenguinEggRate;
impl OperationPenguinEggRate {
    /// Returns the value of this parameter type.
    pub const fn get() -> Permill {
        Permill::from_percent(5)
    }
}
impl<I: From<Permill>> ::frame_support::traits::Get<I> for OperationPenguinEggRate {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct BidCommissionRate;
impl BidCommissionRate {
    /// Returns the value of this parameter type.
    pub const fn get() -> Permill {
        Permill::from_percent(3)
    }
}
impl<I: From<Permill>> ::frame_support::traits::Get<I> for BidCommissionRate {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct MalePenguinRate;
impl MalePenguinRate {
    /// Returns the value of this parameter type.
    pub const fn get() -> Permill {
        Permill::from_percent(1)
    }
}
impl<I: From<Permill>> ::frame_support::traits::Get<I> for MalePenguinRate {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct PenguinProducePeriod;
impl PenguinProducePeriod {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        1 * DAYS
    }
}
impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for PenguinProducePeriod {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct YellowPenguinDeadPeriod;
impl YellowPenguinDeadPeriod {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        14 * DAYS
    }
}
impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for YellowPenguinDeadPeriod {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct IncubationLivePeriod;
impl IncubationLivePeriod {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        5 * DAYS
    }
}
impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for IncubationLivePeriod {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct BidMaxPeriod;
impl BidMaxPeriod {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        1 * DAYS
    }
}
impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for BidMaxPeriod {
    fn get() -> I {
        I::from(Self::get())
    }
}
impl penguin_farm::Config for Runtime {
    type Event = Event;
    type Call = Call;
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
    type Schedule = Scheduler;
    type PalletsOrigin = OriginCaller;
}
#[doc(hidden)]
mod sp_api_hidden_includes_construct_runtime {
    pub extern crate frame_support as hidden_include;
}
const _: () = {
    #[allow(unused)]
    type __hidden_use_of_unchecked_extrinsic = UncheckedExtrinsic;
};
pub struct Runtime;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Runtime {
    #[inline]
    fn clone(&self) -> Runtime {
        {
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for Runtime {}
impl ::core::marker::StructuralPartialEq for Runtime {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Runtime {
    #[inline]
    fn eq(&self, other: &Runtime) -> bool {
        match *other {
            Runtime => match *self {
                Runtime => true,
            },
        }
    }
}
impl ::core::marker::StructuralEq for Runtime {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for Runtime {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {}
    }
}
impl core::fmt::Debug for Runtime {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_tuple("Runtime").finish()
    }
}
impl self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: GetNodeBlockType for Runtime { type NodeBlock = opaque :: Block ; }
impl self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: GetRuntimeBlockType for Runtime { type RuntimeBlock = Block ; }
#[allow(non_camel_case_types)]
pub enum Event {
    #[codec(index = 1u8)]
    System(frame_system::Event<Runtime>),
    #[codec(index = 4u8)]
    Balances(pallet_balances::Event<Runtime>),
    #[codec(index = 7u8)]
    Bounties(pallet_bounties::Event<Runtime>),
    #[codec(index = 8u8)]
    Grandpa(pallet_grandpa::Event),
    #[codec(index = 10u8)]
    TechnicalCommittee(pallet_collective::Event<Runtime, pallet_collective::Instance2>),
    #[codec(index = 11u8)]
    Democracy(pallet_democracy::Event<Runtime>),
    #[codec(index = 12u8)]
    Council(pallet_collective::Event<Runtime, pallet_collective::Instance1>),
    #[codec(index = 13u8)]
    Scheduler(pallet_scheduler::Event<Runtime>),
    #[codec(index = 14u8)]
    Treasury(pallet_treasury::Event<Runtime>),
    #[codec(index = 16u8)]
    Nft(pallet_nft::Event<Runtime>),
    #[codec(index = 17u8)]
    Sudo(pallet_sudo::Event<Runtime>),
    #[codec(index = 18u8)]
    Ethereum(pallet_ethereum::Event),
    #[codec(index = 19u8)]
    EVM(pallet_evm::Event<Runtime>),
    #[codec(index = 21u8)]
    Assets(pallet_assets::Event<Runtime>),
    #[codec(index = 22u8)]
    Farm(penguin_farm::Event<Runtime>),
    #[codec(index = 23u8)]
    Elections(pallet_elections_phragmen::Event<Runtime>),
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::clone::Clone for Event {
    #[inline]
    fn clone(&self) -> Event {
        match (&*self,) {
            (&Event::System(ref __self_0),) => {
                Event::System(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::Balances(ref __self_0),) => {
                Event::Balances(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::Bounties(ref __self_0),) => {
                Event::Bounties(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::Grandpa(ref __self_0),) => {
                Event::Grandpa(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::TechnicalCommittee(ref __self_0),) => {
                Event::TechnicalCommittee(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::Democracy(ref __self_0),) => {
                Event::Democracy(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::Council(ref __self_0),) => {
                Event::Council(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::Scheduler(ref __self_0),) => {
                Event::Scheduler(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::Treasury(ref __self_0),) => {
                Event::Treasury(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::Nft(ref __self_0),) => Event::Nft(::core::clone::Clone::clone(&(*__self_0))),
            (&Event::Sudo(ref __self_0),) => Event::Sudo(::core::clone::Clone::clone(&(*__self_0))),
            (&Event::Ethereum(ref __self_0),) => {
                Event::Ethereum(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::EVM(ref __self_0),) => Event::EVM(::core::clone::Clone::clone(&(*__self_0))),
            (&Event::Assets(ref __self_0),) => {
                Event::Assets(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::Farm(ref __self_0),) => Event::Farm(::core::clone::Clone::clone(&(*__self_0))),
            (&Event::Elections(ref __self_0),) => {
                Event::Elections(::core::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}
#[allow(non_camel_case_types)]
impl ::core::marker::StructuralPartialEq for Event {}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::PartialEq for Event {
    #[inline]
    fn eq(&self, other: &Event) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Event::System(ref __self_0), &Event::System(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::Balances(ref __self_0), &Event::Balances(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::Bounties(ref __self_0), &Event::Bounties(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::Grandpa(ref __self_0), &Event::Grandpa(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (
                        &Event::TechnicalCommittee(ref __self_0),
                        &Event::TechnicalCommittee(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (&Event::Democracy(ref __self_0), &Event::Democracy(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::Council(ref __self_0), &Event::Council(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::Scheduler(ref __self_0), &Event::Scheduler(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::Treasury(ref __self_0), &Event::Treasury(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::Nft(ref __self_0), &Event::Nft(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::Sudo(ref __self_0), &Event::Sudo(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::Ethereum(ref __self_0), &Event::Ethereum(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::EVM(ref __self_0), &Event::EVM(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::Assets(ref __self_0), &Event::Assets(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::Farm(ref __self_0), &Event::Farm(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::Elections(ref __self_0), &Event::Elections(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &Event) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Event::System(ref __self_0), &Event::System(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::Balances(ref __self_0), &Event::Balances(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::Bounties(ref __self_0), &Event::Bounties(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::Grandpa(ref __self_0), &Event::Grandpa(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (
                        &Event::TechnicalCommittee(ref __self_0),
                        &Event::TechnicalCommittee(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (&Event::Democracy(ref __self_0), &Event::Democracy(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::Council(ref __self_0), &Event::Council(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::Scheduler(ref __self_0), &Event::Scheduler(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::Treasury(ref __self_0), &Event::Treasury(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::Nft(ref __self_0), &Event::Nft(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::Sudo(ref __self_0), &Event::Sudo(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::Ethereum(ref __self_0), &Event::Ethereum(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::EVM(ref __self_0), &Event::EVM(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::Assets(ref __self_0), &Event::Assets(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::Farm(ref __self_0), &Event::Farm(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::Elections(ref __self_0), &Event::Elections(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}
#[allow(non_camel_case_types)]
impl ::core::marker::StructuralEq for Event {}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::Eq for Event {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<frame_system::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<pallet_balances::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<pallet_bounties::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<pallet_grandpa::Event>;
            let _: ::core::cmp::AssertParamIsEq<
                pallet_collective::Event<Runtime, pallet_collective::Instance2>,
            >;
            let _: ::core::cmp::AssertParamIsEq<pallet_democracy::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<
                pallet_collective::Event<Runtime, pallet_collective::Instance1>,
            >;
            let _: ::core::cmp::AssertParamIsEq<pallet_scheduler::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<pallet_treasury::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<pallet_nft::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<pallet_sudo::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<pallet_ethereum::Event>;
            let _: ::core::cmp::AssertParamIsEq<pallet_evm::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<pallet_assets::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<penguin_farm::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<pallet_elections_phragmen::Event<Runtime>>;
        }
    }
}
const _: () = {
    #[allow(non_camel_case_types)]
    impl ::codec::Encode for Event {
        fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                Event::System(ref aa) => {
                    __codec_dest_edqy.push_byte(1u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Event::Balances(ref aa) => {
                    __codec_dest_edqy.push_byte(4u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Event::Bounties(ref aa) => {
                    __codec_dest_edqy.push_byte(7u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Event::Grandpa(ref aa) => {
                    __codec_dest_edqy.push_byte(8u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Event::TechnicalCommittee(ref aa) => {
                    __codec_dest_edqy.push_byte(10u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Event::Democracy(ref aa) => {
                    __codec_dest_edqy.push_byte(11u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Event::Council(ref aa) => {
                    __codec_dest_edqy.push_byte(12u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Event::Scheduler(ref aa) => {
                    __codec_dest_edqy.push_byte(13u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Event::Treasury(ref aa) => {
                    __codec_dest_edqy.push_byte(14u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Event::Nft(ref aa) => {
                    __codec_dest_edqy.push_byte(16u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Event::Sudo(ref aa) => {
                    __codec_dest_edqy.push_byte(17u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Event::Ethereum(ref aa) => {
                    __codec_dest_edqy.push_byte(18u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Event::EVM(ref aa) => {
                    __codec_dest_edqy.push_byte(19u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Event::Assets(ref aa) => {
                    __codec_dest_edqy.push_byte(21u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Event::Farm(ref aa) => {
                    __codec_dest_edqy.push_byte(22u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Event::Elections(ref aa) => {
                    __codec_dest_edqy.push_byte(23u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                _ => (),
            }
        }
    }
    impl ::codec::EncodeLike for Event {}
};
const _: () = {
    #[allow(non_camel_case_types)]
    impl ::codec::Decode for Event {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| e.chain("Could not decode `Event`, failed to read variant byte"))?
            {
                __codec_x_edqy if __codec_x_edqy == 1u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::System({
                        let __codec_res_edqy =
                            <frame_system::Event<Runtime> as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::System.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 4u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::Balances({
                        let __codec_res_edqy =
                            <pallet_balances::Event<Runtime> as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::Balances.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 7u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::Bounties({
                        let __codec_res_edqy =
                            <pallet_bounties::Event<Runtime> as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::Bounties.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 8u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::Grandpa({
                        let __codec_res_edqy =
                            <pallet_grandpa::Event as ::codec::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::Grandpa.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 10u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::TechnicalCommittee({
                        let __codec_res_edqy = <pallet_collective::Event<
                            Runtime,
                            pallet_collective::Instance2,
                        > as ::codec::Decode>::decode(
                            __codec_input_edqy
                        );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::TechnicalCommittee.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 11u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::Democracy({
                        let __codec_res_edqy =
                            <pallet_democracy::Event<Runtime> as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::Democracy.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 12u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::Council({
                        let __codec_res_edqy = <pallet_collective::Event<
                            Runtime,
                            pallet_collective::Instance1,
                        > as ::codec::Decode>::decode(
                            __codec_input_edqy
                        );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::Council.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 13u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::Scheduler({
                        let __codec_res_edqy =
                            <pallet_scheduler::Event<Runtime> as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::Scheduler.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 14u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::Treasury({
                        let __codec_res_edqy =
                            <pallet_treasury::Event<Runtime> as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::Treasury.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 16u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::Nft({
                        let __codec_res_edqy =
                            <pallet_nft::Event<Runtime> as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::Nft.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 17u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::Sudo({
                        let __codec_res_edqy =
                            <pallet_sudo::Event<Runtime> as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::Sudo.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 18u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::Ethereum({
                        let __codec_res_edqy =
                            <pallet_ethereum::Event as ::codec::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::Ethereum.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 19u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::EVM({
                        let __codec_res_edqy =
                            <pallet_evm::Event<Runtime> as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::EVM.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 21u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::Assets({
                        let __codec_res_edqy =
                            <pallet_assets::Event<Runtime> as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::Assets.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 22u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::Farm({
                        let __codec_res_edqy =
                            <penguin_farm::Event<Runtime> as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::Farm.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 23u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::Elections({
                        let __codec_res_edqy =
                            <pallet_elections_phragmen::Event<Runtime> as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::Elections.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                    "Could not decode `Event`, variant doesn\'t exist",
                )),
            }
        }
    }
};
impl core::fmt::Debug for Event {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::System(ref a0) => fmt.debug_tuple("Event::System").field(a0).finish(),
            Self::Balances(ref a0) => fmt.debug_tuple("Event::Balances").field(a0).finish(),
            Self::Bounties(ref a0) => fmt.debug_tuple("Event::Bounties").field(a0).finish(),
            Self::Grandpa(ref a0) => fmt.debug_tuple("Event::Grandpa").field(a0).finish(),
            Self::TechnicalCommittee(ref a0) => fmt
                .debug_tuple("Event::TechnicalCommittee")
                .field(a0)
                .finish(),
            Self::Democracy(ref a0) => fmt.debug_tuple("Event::Democracy").field(a0).finish(),
            Self::Council(ref a0) => fmt.debug_tuple("Event::Council").field(a0).finish(),
            Self::Scheduler(ref a0) => fmt.debug_tuple("Event::Scheduler").field(a0).finish(),
            Self::Treasury(ref a0) => fmt.debug_tuple("Event::Treasury").field(a0).finish(),
            Self::Nft(ref a0) => fmt.debug_tuple("Event::Nft").field(a0).finish(),
            Self::Sudo(ref a0) => fmt.debug_tuple("Event::Sudo").field(a0).finish(),
            Self::Ethereum(ref a0) => fmt.debug_tuple("Event::Ethereum").field(a0).finish(),
            Self::EVM(ref a0) => fmt.debug_tuple("Event::EVM").field(a0).finish(),
            Self::Assets(ref a0) => fmt.debug_tuple("Event::Assets").field(a0).finish(),
            Self::Farm(ref a0) => fmt.debug_tuple("Event::Farm").field(a0).finish(),
            Self::Elections(ref a0) => fmt.debug_tuple("Event::Elections").field(a0).finish(),
            _ => Ok(()),
        }
    }
}
impl From<frame_system::Event<Runtime>> for Event {
    fn from(x: frame_system::Event<Runtime>) -> Self {
        Event::System(x)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryInto<
        frame_system::Event<Runtime>,
    > for Event
{
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        frame_system::Event<Runtime>,
        Self::Error,
    > {
        match self {
            Self::System(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_balances::Event<Runtime>> for Event {
    fn from(x: pallet_balances::Event<Runtime>) -> Self {
        Event::Balances(x)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryInto<
        pallet_balances::Event<Runtime>,
    > for Event
{
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_balances::Event<Runtime>,
        Self::Error,
    > {
        match self {
            Self::Balances(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_bounties::Event<Runtime>> for Event {
    fn from(x: pallet_bounties::Event<Runtime>) -> Self {
        Event::Bounties(x)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryInto<
        pallet_bounties::Event<Runtime>,
    > for Event
{
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_bounties::Event<Runtime>,
        Self::Error,
    > {
        match self {
            Self::Bounties(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_grandpa::Event> for Event {
    fn from(x: pallet_grandpa::Event) -> Self {
        Event::Grandpa(x)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryInto<
        pallet_grandpa::Event,
    > for Event
{
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_grandpa::Event,
        Self::Error,
    > {
        match self {
            Self::Grandpa(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_collective::Event<Runtime, pallet_collective::Instance2>> for Event {
    fn from(x: pallet_collective::Event<Runtime, pallet_collective::Instance2>) -> Self {
        Event::TechnicalCommittee(x)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryInto<
        pallet_collective::Event<Runtime, pallet_collective::Instance2>,
    > for Event
{
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_collective::Event<Runtime, pallet_collective::Instance2>,
        Self::Error,
    > {
        match self {
            Self::TechnicalCommittee(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_democracy::Event<Runtime>> for Event {
    fn from(x: pallet_democracy::Event<Runtime>) -> Self {
        Event::Democracy(x)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryInto<
        pallet_democracy::Event<Runtime>,
    > for Event
{
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_democracy::Event<Runtime>,
        Self::Error,
    > {
        match self {
            Self::Democracy(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_collective::Event<Runtime, pallet_collective::Instance1>> for Event {
    fn from(x: pallet_collective::Event<Runtime, pallet_collective::Instance1>) -> Self {
        Event::Council(x)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryInto<
        pallet_collective::Event<Runtime, pallet_collective::Instance1>,
    > for Event
{
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_collective::Event<Runtime, pallet_collective::Instance1>,
        Self::Error,
    > {
        match self {
            Self::Council(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_scheduler::Event<Runtime>> for Event {
    fn from(x: pallet_scheduler::Event<Runtime>) -> Self {
        Event::Scheduler(x)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryInto<
        pallet_scheduler::Event<Runtime>,
    > for Event
{
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_scheduler::Event<Runtime>,
        Self::Error,
    > {
        match self {
            Self::Scheduler(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_treasury::Event<Runtime>> for Event {
    fn from(x: pallet_treasury::Event<Runtime>) -> Self {
        Event::Treasury(x)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryInto<
        pallet_treasury::Event<Runtime>,
    > for Event
{
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_treasury::Event<Runtime>,
        Self::Error,
    > {
        match self {
            Self::Treasury(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_nft::Event<Runtime>> for Event {
    fn from(x: pallet_nft::Event<Runtime>) -> Self {
        Event::Nft(x)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryInto<
        pallet_nft::Event<Runtime>,
    > for Event
{
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_nft::Event<Runtime>,
        Self::Error,
    > {
        match self {
            Self::Nft(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_sudo::Event<Runtime>> for Event {
    fn from(x: pallet_sudo::Event<Runtime>) -> Self {
        Event::Sudo(x)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryInto<
        pallet_sudo::Event<Runtime>,
    > for Event
{
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_sudo::Event<Runtime>,
        Self::Error,
    > {
        match self {
            Self::Sudo(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_ethereum::Event> for Event {
    fn from(x: pallet_ethereum::Event) -> Self {
        Event::Ethereum(x)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryInto<
        pallet_ethereum::Event,
    > for Event
{
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_ethereum::Event,
        Self::Error,
    > {
        match self {
            Self::Ethereum(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_evm::Event<Runtime>> for Event {
    fn from(x: pallet_evm::Event<Runtime>) -> Self {
        Event::EVM(x)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryInto<
        pallet_evm::Event<Runtime>,
    > for Event
{
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_evm::Event<Runtime>,
        Self::Error,
    > {
        match self {
            Self::EVM(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_assets::Event<Runtime>> for Event {
    fn from(x: pallet_assets::Event<Runtime>) -> Self {
        Event::Assets(x)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryInto<
        pallet_assets::Event<Runtime>,
    > for Event
{
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_assets::Event<Runtime>,
        Self::Error,
    > {
        match self {
            Self::Assets(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<penguin_farm::Event<Runtime>> for Event {
    fn from(x: penguin_farm::Event<Runtime>) -> Self {
        Event::Farm(x)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryInto<
        penguin_farm::Event<Runtime>,
    > for Event
{
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        penguin_farm::Event<Runtime>,
        Self::Error,
    > {
        match self {
            Self::Farm(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_elections_phragmen::Event<Runtime>> for Event {
    fn from(x: pallet_elections_phragmen::Event<Runtime>) -> Self {
        Event::Elections(x)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryInto<
        pallet_elections_phragmen::Event<Runtime>,
    > for Event
{
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_elections_phragmen::Event<Runtime>,
        Self::Error,
    > {
        match self {
            Self::Elections(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
pub struct Origin {
    caller: OriginCaller,
    filter: self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc<
        Box<dyn Fn(&<Runtime as frame_system::Config>::Call) -> bool>,
    >,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Origin {
    #[inline]
    fn clone(&self) -> Origin {
        match *self {
            Origin {
                caller: ref __self_0_0,
                filter: ref __self_0_1,
            } => Origin {
                caller: ::core::clone::Clone::clone(&(*__self_0_0)),
                filter: ::core::clone::Clone::clone(&(*__self_0_1)),
            },
        }
    }
}
#[cfg(feature = "std")]
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::fmt::Debug for Origin {
    fn fmt(
        &self,
        fmt : & mut self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: fmt :: Formatter,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        (),
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::fmt::Error,
    > {
        fmt.debug_struct("Origin")
            .field("caller", &self.caller)
            .field("filter", &"[function ptr]")
            .finish()
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait
    for Origin
{
    type Call = <Runtime as frame_system::Config>::Call;
    type PalletsOrigin = OriginCaller;
    type AccountId = <Runtime as frame_system::Config>::AccountId;
    fn add_filter(&mut self, filter: impl Fn(&Self::Call) -> bool + 'static) {
        let f = self.filter.clone();
        self.filter =
            self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc::new(
                Box::new(move |call| f(call) && filter(call)),
            );
    }
    fn reset_filter(&mut self) {
        let filter = < < Runtime as frame_system :: Config > :: BaseCallFilter as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: Contains < < Runtime as frame_system :: Config > :: Call > > :: contains ;
        self.filter =
            self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc::new(
                Box::new(filter),
            );
    }
    fn set_caller_from(&mut self, other: impl Into<Self>) {
        self.caller = other.into().caller;
    }
    fn filter_call(&self, call: &Self::Call) -> bool {
        (self.filter)(call)
    }
    fn caller(&self) -> &Self::PalletsOrigin {
        &self.caller
    }
    fn try_with_caller<R>(
        mut self,
        f: impl FnOnce(Self::PalletsOrigin) -> Result<R, Self::PalletsOrigin>,
    ) -> Result<R, Self> {
        match f(self.caller) {
            Ok(r) => Ok(r),
            Err(caller) => {
                self.caller = caller;
                Err(self)
            }
        }
    }
    /// Create with system none origin and `frame-system::Config::BaseCallFilter`.
    fn none() -> Self {
        frame_system::RawOrigin::None.into()
    }
    /// Create with system root origin and no filter.
    fn root() -> Self {
        frame_system::RawOrigin::Root.into()
    }
    /// Create with system signed origin and `frame-system::Config::BaseCallFilter`.
    fn signed(by: <Runtime as frame_system::Config>::AccountId) -> Self {
        frame_system::RawOrigin::Signed(by).into()
    }
}
#[allow(non_camel_case_types)]
pub enum OriginCaller {
    #[codec(index = 1u8)]
    system(frame_system::Origin<Runtime>),
    #[codec(index = 10u8)]
    TechnicalCommittee(pallet_collective::Origin<Runtime, pallet_collective::Instance2>),
    #[codec(index = 12u8)]
    Council(pallet_collective::Origin<Runtime, pallet_collective::Instance1>),
    #[allow(dead_code)]
    Void(self::sp_api_hidden_includes_construct_runtime::hidden_include::Void),
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::clone::Clone for OriginCaller {
    #[inline]
    fn clone(&self) -> OriginCaller {
        match (&*self,) {
            (&OriginCaller::system(ref __self_0),) => {
                OriginCaller::system(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&OriginCaller::TechnicalCommittee(ref __self_0),) => {
                OriginCaller::TechnicalCommittee(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&OriginCaller::Council(ref __self_0),) => {
                OriginCaller::Council(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&OriginCaller::Void(ref __self_0),) => {
                OriginCaller::Void(::core::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}
#[allow(non_camel_case_types)]
impl ::core::marker::StructuralPartialEq for OriginCaller {}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::PartialEq for OriginCaller {
    #[inline]
    fn eq(&self, other: &OriginCaller) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&OriginCaller::system(ref __self_0), &OriginCaller::system(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (
                        &OriginCaller::TechnicalCommittee(ref __self_0),
                        &OriginCaller::TechnicalCommittee(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (
                        &OriginCaller::Council(ref __self_0),
                        &OriginCaller::Council(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (&OriginCaller::Void(ref __self_0), &OriginCaller::Void(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &OriginCaller) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&OriginCaller::system(ref __self_0), &OriginCaller::system(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (
                        &OriginCaller::TechnicalCommittee(ref __self_0),
                        &OriginCaller::TechnicalCommittee(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (
                        &OriginCaller::Council(ref __self_0),
                        &OriginCaller::Council(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (&OriginCaller::Void(ref __self_0), &OriginCaller::Void(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}
#[allow(non_camel_case_types)]
impl ::core::marker::StructuralEq for OriginCaller {}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::Eq for OriginCaller {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<frame_system::Origin<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<
                pallet_collective::Origin<Runtime, pallet_collective::Instance2>,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                pallet_collective::Origin<Runtime, pallet_collective::Instance1>,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                self::sp_api_hidden_includes_construct_runtime::hidden_include::Void,
            >;
        }
    }
}
impl core::fmt::Debug for OriginCaller {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::system(ref a0) => fmt.debug_tuple("OriginCaller::system").field(a0).finish(),
            Self::TechnicalCommittee(ref a0) => fmt
                .debug_tuple("OriginCaller::TechnicalCommittee")
                .field(a0)
                .finish(),
            Self::Council(ref a0) => fmt.debug_tuple("OriginCaller::Council").field(a0).finish(),
            Self::Void(ref a0) => fmt.debug_tuple("OriginCaller::Void").field(a0).finish(),
            _ => Ok(()),
        }
    }
}
const _: () = {
    #[allow(non_camel_case_types)]
    impl ::codec::Encode for OriginCaller {
        fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                OriginCaller::system(ref aa) => {
                    __codec_dest_edqy.push_byte(1u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                OriginCaller::TechnicalCommittee(ref aa) => {
                    __codec_dest_edqy.push_byte(10u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                OriginCaller::Council(ref aa) => {
                    __codec_dest_edqy.push_byte(12u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                OriginCaller::Void(ref aa) => {
                    __codec_dest_edqy.push_byte(3usize as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                _ => (),
            }
        }
    }
    impl ::codec::EncodeLike for OriginCaller {}
};
const _: () = {
    #[allow(non_camel_case_types)]
    impl ::codec::Decode for OriginCaller {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy.read_byte().map_err(|e| {
                e.chain("Could not decode `OriginCaller`, failed to read variant byte")
            })? {
                __codec_x_edqy if __codec_x_edqy == 1u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(OriginCaller::system({
                        let __codec_res_edqy =
                            <frame_system::Origin<Runtime> as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `OriginCaller::system.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 10u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(OriginCaller::TechnicalCommittee({
                        let __codec_res_edqy = <pallet_collective::Origin<
                            Runtime,
                            pallet_collective::Instance2,
                        > as ::codec::Decode>::decode(
                            __codec_input_edqy
                        );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(e.chain(
                                    "Could not decode `OriginCaller::TechnicalCommittee.0`",
                                ))
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 12u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(OriginCaller::Council({
                        let __codec_res_edqy = <pallet_collective::Origin<
                            Runtime,
                            pallet_collective::Instance1,
                        > as ::codec::Decode>::decode(
                            __codec_input_edqy
                        );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `OriginCaller::Council.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 3usize as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(OriginCaller::Void({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: Void as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `OriginCaller::Void.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                    "Could not decode `OriginCaller`, variant doesn\'t exist",
                )),
            }
        }
    }
};
#[allow(dead_code)]
impl Origin {
    /// Create with system none origin and `frame-system::Config::BaseCallFilter`.
    pub fn none() -> Self {
        < Origin as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait > :: none ()
    }
    /// Create with system root origin and no filter.
    pub fn root() -> Self {
        < Origin as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait > :: root ()
    }
    /// Create with system signed origin and `frame-system::Config::BaseCallFilter`.
    pub fn signed(by: <Runtime as frame_system::Config>::AccountId) -> Self {
        < Origin as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait > :: signed (by)
    }
}
impl From<frame_system::Origin<Runtime>> for OriginCaller {
    fn from(x: frame_system::Origin<Runtime>) -> Self {
        OriginCaller::system(x)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryFrom<
        OriginCaller,
    > for frame_system::Origin<Runtime>
{
    type Error = OriginCaller;
    fn try_from(
        x: OriginCaller,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        frame_system::Origin<Runtime>,
        OriginCaller,
    > {
        if let OriginCaller::system(l) = x {
            Ok(l)
        } else {
            Err(x)
        }
    }
}
impl From<frame_system::Origin<Runtime>> for Origin {
    /// Convert to runtime origin:
    /// * root origin is built with no filter
    /// * others use `frame-system::Config::BaseCallFilter`
    fn from(x: frame_system::Origin<Runtime>) -> Self {
        let o: OriginCaller = x.into();
        o.into()
    }
}
impl From<OriginCaller> for Origin {
    fn from(x: OriginCaller) -> Self {
        let mut o = Origin {
            caller: x,
            filter:
                self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc::new(
                    Box::new(|_| true),
                ),
        };
        if !match o.caller {
            OriginCaller::system(frame_system::Origin::<Runtime>::Root) => true,
            _ => false,
        } {
            self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait :: reset_filter (& mut o) ;
        }
        o
    }
}
impl From<Origin>
    for self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        frame_system::Origin<Runtime>,
        Origin,
    >
{
    /// NOTE: converting to pallet origin loses the origin filter information.
    fn from(val: Origin) -> Self {
        if let OriginCaller::system(l) = val.caller {
            Ok(l)
        } else {
            Err(val)
        }
    }
}
impl From<Option<<Runtime as frame_system::Config>::AccountId>> for Origin {
    /// Convert to runtime origin with caller being system signed or none and use filter
    /// `frame-system::Config::BaseCallFilter`.
    fn from(x: Option<<Runtime as frame_system::Config>::AccountId>) -> Self {
        <frame_system::Origin<Runtime>>::from(x).into()
    }
}
impl From<pallet_collective::Origin<Runtime, pallet_collective::Instance2>> for OriginCaller {
    fn from(x: pallet_collective::Origin<Runtime, pallet_collective::Instance2>) -> Self {
        OriginCaller::TechnicalCommittee(x)
    }
}
impl From<pallet_collective::Origin<Runtime, pallet_collective::Instance2>> for Origin {
    /// Convert to runtime origin using `frame-system::Config::BaseCallFilter`.
    fn from(x: pallet_collective::Origin<Runtime, pallet_collective::Instance2>) -> Self {
        let x: OriginCaller = x.into();
        x.into()
    }
}
impl From<Origin>
    for self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_collective::Origin<Runtime, pallet_collective::Instance2>,
        Origin,
    >
{
    /// NOTE: converting to pallet origin loses the origin filter information.
    fn from(val: Origin) -> Self {
        if let OriginCaller::TechnicalCommittee(l) = val.caller {
            Ok(l)
        } else {
            Err(val)
        }
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryFrom<
        OriginCaller,
    > for pallet_collective::Origin<Runtime, pallet_collective::Instance2>
{
    type Error = OriginCaller;
    fn try_from(
        x: OriginCaller,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_collective::Origin<Runtime, pallet_collective::Instance2>,
        OriginCaller,
    > {
        if let OriginCaller::TechnicalCommittee(l) = x {
            Ok(l)
        } else {
            Err(x)
        }
    }
}
impl From<pallet_collective::Origin<Runtime, pallet_collective::Instance1>> for OriginCaller {
    fn from(x: pallet_collective::Origin<Runtime, pallet_collective::Instance1>) -> Self {
        OriginCaller::Council(x)
    }
}
impl From<pallet_collective::Origin<Runtime, pallet_collective::Instance1>> for Origin {
    /// Convert to runtime origin using `frame-system::Config::BaseCallFilter`.
    fn from(x: pallet_collective::Origin<Runtime, pallet_collective::Instance1>) -> Self {
        let x: OriginCaller = x.into();
        x.into()
    }
}
impl From<Origin>
    for self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_collective::Origin<Runtime, pallet_collective::Instance1>,
        Origin,
    >
{
    /// NOTE: converting to pallet origin loses the origin filter information.
    fn from(val: Origin) -> Self {
        if let OriginCaller::Council(l) = val.caller {
            Ok(l)
        } else {
            Err(val)
        }
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryFrom<
        OriginCaller,
    > for pallet_collective::Origin<Runtime, pallet_collective::Instance1>
{
    type Error = OriginCaller;
    fn try_from(
        x: OriginCaller,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_collective::Origin<Runtime, pallet_collective::Instance1>,
        OriginCaller,
    > {
        if let OriginCaller::Council(l) = x {
            Ok(l)
        } else {
            Err(x)
        }
    }
}
pub type System = frame_system::Pallet<Runtime>;
pub type RandomnessCollectiveFlip = pallet_randomness_collective_flip::Pallet<Runtime>;
pub type Timestamp = pallet_timestamp::Pallet<Runtime>;
pub type Balances = pallet_balances::Pallet<Runtime>;
pub type Aura = pallet_aura::Pallet<Runtime>;
pub type Bounties = pallet_bounties::Pallet<Runtime>;
pub type Grandpa = pallet_grandpa::Pallet<Runtime>;
pub type TransactionPayment = pallet_transaction_payment::Pallet<Runtime>;
pub type TechnicalCommittee = pallet_collective::Pallet<Runtime, pallet_collective::Instance2>;
pub type Democracy = pallet_democracy::Pallet<Runtime>;
pub type Council = pallet_collective::Pallet<Runtime, pallet_collective::Instance1>;
pub type Scheduler = pallet_scheduler::Pallet<Runtime>;
pub type Treasury = pallet_treasury::Pallet<Runtime>;
pub type OrmlNft = orml_nft::Pallet<Runtime>;
pub type Nft = pallet_nft::Pallet<Runtime>;
pub type Sudo = pallet_sudo::Pallet<Runtime>;
pub type Ethereum = pallet_ethereum::Pallet<Runtime>;
pub type EVM = pallet_evm::Pallet<Runtime>;
pub type DynamicFee = pallet_dynamic_fee::Pallet<Runtime>;
pub type Assets = pallet_assets::Pallet<Runtime>;
pub type Farm = penguin_farm::Pallet<Runtime>;
pub type Elections = pallet_elections_phragmen::Pallet<Runtime>;
/// All pallets included in the runtime as a nested tuple of types.
/// Excludes the System pallet.
pub type AllPallets = ((Elections , (Farm , (Assets , (DynamicFee , (EVM , (Ethereum , (Sudo , (Nft , (OrmlNft , (Treasury , (Scheduler , (Council , (Democracy , (TechnicalCommittee , (TransactionPayment , (Grandpa , (Bounties , (Aura , (Balances , (Timestamp , (RandomnessCollectiveFlip ,)))))))))))))))))))))) ;
/// All pallets included in the runtime as a nested tuple of types.
pub type AllPalletsWithSystem = ((Elections , (Farm , (Assets , (DynamicFee , (EVM , (Ethereum , (Sudo , (Nft , (OrmlNft , (Treasury , (Scheduler , (Council , (Democracy , (TechnicalCommittee , (TransactionPayment , (Grandpa , (Bounties , (Aura , (Balances , (Timestamp , (RandomnessCollectiveFlip , (System ,))))))))))))))))))))))) ;
/// All modules included in the runtime as a nested tuple of types.
/// Excludes the System pallet.
#[deprecated(note = "use `AllPallets` instead")]
#[allow(dead_code)]
pub type AllModules = ((Elections , (Farm , (Assets , (DynamicFee , (EVM , (Ethereum , (Sudo , (Nft , (OrmlNft , (Treasury , (Scheduler , (Council , (Democracy , (TechnicalCommittee , (TransactionPayment , (Grandpa , (Bounties , (Aura , (Balances , (Timestamp , (RandomnessCollectiveFlip ,)))))))))))))))))))))) ;
/// All modules included in the runtime as a nested tuple of types.
#[deprecated(note = "use `AllPalletsWithSystem` instead")]
#[allow(dead_code)]
pub type AllModulesWithSystem = ((Elections , (Farm , (Assets , (DynamicFee , (EVM , (Ethereum , (Sudo , (Nft , (OrmlNft , (Treasury , (Scheduler , (Council , (Democracy , (TechnicalCommittee , (TransactionPayment , (Grandpa , (Bounties , (Aura , (Balances , (Timestamp , (RandomnessCollectiveFlip , (System ,))))))))))))))))))))))) ;
/// Provides an implementation of `PalletInfo` to provide information
/// about the pallet setup in the runtime.
pub struct PalletInfo;
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfo
    for PalletInfo
{
    fn index<P: 'static>() -> Option<usize> {
        let type_id =
            self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                P,
            >();
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < System > () { return Some (1usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < RandomnessCollectiveFlip > () { return Some (2usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Timestamp > () { return Some (3usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Balances > () { return Some (4usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Aura > () { return Some (5usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Bounties > () { return Some (7usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Grandpa > () { return Some (8usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < TransactionPayment > () { return Some (9usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < TechnicalCommittee > () { return Some (10usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Democracy > () { return Some (11usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Council > () { return Some (12usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Scheduler > () { return Some (13usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Treasury > () { return Some (14usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < OrmlNft > () { return Some (15usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Nft > () { return Some (16usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Sudo > () { return Some (17usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Ethereum > () { return Some (18usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < EVM > () { return Some (19usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < DynamicFee > () { return Some (20usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Assets > () { return Some (21usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Farm > () { return Some (22usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Elections > () { return Some (23usize) }
        None
    }
    fn name<P: 'static>() -> Option<&'static str> {
        let type_id =
            self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                P,
            >();
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < System > () { return Some ("System") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < RandomnessCollectiveFlip > () { return Some ("RandomnessCollectiveFlip") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Timestamp > () { return Some ("Timestamp") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Balances > () { return Some ("Balances") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Aura > () { return Some ("Aura") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Bounties > () { return Some ("Bounties") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Grandpa > () { return Some ("Grandpa") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < TransactionPayment > () { return Some ("TransactionPayment") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < TechnicalCommittee > () { return Some ("TechnicalCommittee") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Democracy > () { return Some ("Democracy") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Council > () { return Some ("Council") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Scheduler > () { return Some ("Scheduler") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Treasury > () { return Some ("Treasury") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < OrmlNft > () { return Some ("OrmlNft") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Nft > () { return Some ("Nft") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Sudo > () { return Some ("Sudo") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Ethereum > () { return Some ("Ethereum") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < EVM > () { return Some ("EVM") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < DynamicFee > () { return Some ("DynamicFee") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Assets > () { return Some ("Assets") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Farm > () { return Some ("Farm") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Elections > () { return Some ("Elections") }
        None
    }
}
pub enum Call {
    #[codec(index = 1u8)]
    System(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            System,
            Runtime,
        >,
    ),
    #[codec(index = 3u8)]
    Timestamp(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Timestamp,
            Runtime,
        >,
    ),
    #[codec(index = 4u8)]
    Balances(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Balances,
            Runtime,
        >,
    ),
    #[codec(index = 7u8)]
    Bounties(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Bounties,
            Runtime,
        >,
    ),
    #[codec(index = 8u8)]
    Grandpa(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Grandpa,
            Runtime,
        >,
    ),
    #[codec(index = 10u8)]
    TechnicalCommittee(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            TechnicalCommittee,
            Runtime,
        >,
    ),
    #[codec(index = 11u8)]
    Democracy(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Democracy,
            Runtime,
        >,
    ),
    #[codec(index = 12u8)]
    Council(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Council,
            Runtime,
        >,
    ),
    #[codec(index = 13u8)]
    Scheduler(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Scheduler,
            Runtime,
        >,
    ),
    #[codec(index = 14u8)]
    Treasury(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Treasury,
            Runtime,
        >,
    ),
    #[codec(index = 16u8)]
    Nft(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Nft,
            Runtime,
        >,
    ),
    #[codec(index = 17u8)]
    Sudo(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Sudo,
            Runtime,
        >,
    ),
    #[codec(index = 18u8)]
    Ethereum(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Ethereum,
            Runtime,
        >,
    ),
    #[codec(index = 19u8)]
    EVM(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            EVM,
            Runtime,
        >,
    ),
    #[codec(index = 20u8)]
    DynamicFee(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            DynamicFee,
            Runtime,
        >,
    ),
    #[codec(index = 21u8)]
    Assets(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Assets,
            Runtime,
        >,
    ),
    #[codec(index = 22u8)]
    Farm(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Farm,
            Runtime,
        >,
    ),
    #[codec(index = 23u8)]
    Elections(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Elections,
            Runtime,
        >,
    ),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Call {
    #[inline]
    fn clone(&self) -> Call {
        match (&*self,) {
            (&Call::System(ref __self_0),) => {
                Call::System(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Timestamp(ref __self_0),) => {
                Call::Timestamp(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Balances(ref __self_0),) => {
                Call::Balances(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Bounties(ref __self_0),) => {
                Call::Bounties(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Grandpa(ref __self_0),) => {
                Call::Grandpa(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::TechnicalCommittee(ref __self_0),) => {
                Call::TechnicalCommittee(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Democracy(ref __self_0),) => {
                Call::Democracy(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Council(ref __self_0),) => {
                Call::Council(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Scheduler(ref __self_0),) => {
                Call::Scheduler(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Treasury(ref __self_0),) => {
                Call::Treasury(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Nft(ref __self_0),) => Call::Nft(::core::clone::Clone::clone(&(*__self_0))),
            (&Call::Sudo(ref __self_0),) => Call::Sudo(::core::clone::Clone::clone(&(*__self_0))),
            (&Call::Ethereum(ref __self_0),) => {
                Call::Ethereum(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::EVM(ref __self_0),) => Call::EVM(::core::clone::Clone::clone(&(*__self_0))),
            (&Call::DynamicFee(ref __self_0),) => {
                Call::DynamicFee(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Assets(ref __self_0),) => {
                Call::Assets(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Farm(ref __self_0),) => Call::Farm(::core::clone::Clone::clone(&(*__self_0))),
            (&Call::Elections(ref __self_0),) => {
                Call::Elections(::core::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}
impl ::core::marker::StructuralPartialEq for Call {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Call {
    #[inline]
    fn eq(&self, other: &Call) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Call::System(ref __self_0), &Call::System(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Timestamp(ref __self_0), &Call::Timestamp(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Balances(ref __self_0), &Call::Balances(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Bounties(ref __self_0), &Call::Bounties(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Grandpa(ref __self_0), &Call::Grandpa(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (
                        &Call::TechnicalCommittee(ref __self_0),
                        &Call::TechnicalCommittee(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (&Call::Democracy(ref __self_0), &Call::Democracy(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Council(ref __self_0), &Call::Council(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Scheduler(ref __self_0), &Call::Scheduler(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Treasury(ref __self_0), &Call::Treasury(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Nft(ref __self_0), &Call::Nft(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Sudo(ref __self_0), &Call::Sudo(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Ethereum(ref __self_0), &Call::Ethereum(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::EVM(ref __self_0), &Call::EVM(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::DynamicFee(ref __self_0), &Call::DynamicFee(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Assets(ref __self_0), &Call::Assets(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Farm(ref __self_0), &Call::Farm(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Elections(ref __self_0), &Call::Elections(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &Call) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Call::System(ref __self_0), &Call::System(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Timestamp(ref __self_0), &Call::Timestamp(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Balances(ref __self_0), &Call::Balances(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Bounties(ref __self_0), &Call::Bounties(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Grandpa(ref __self_0), &Call::Grandpa(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (
                        &Call::TechnicalCommittee(ref __self_0),
                        &Call::TechnicalCommittee(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (&Call::Democracy(ref __self_0), &Call::Democracy(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Council(ref __self_0), &Call::Council(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Scheduler(ref __self_0), &Call::Scheduler(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Treasury(ref __self_0), &Call::Treasury(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Nft(ref __self_0), &Call::Nft(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Sudo(ref __self_0), &Call::Sudo(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Ethereum(ref __self_0), &Call::Ethereum(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::EVM(ref __self_0), &Call::EVM(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::DynamicFee(ref __self_0), &Call::DynamicFee(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Assets(ref __self_0), &Call::Assets(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Farm(ref __self_0), &Call::Farm(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Elections(ref __self_0), &Call::Elections(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}
impl ::core::marker::StructuralEq for Call {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for Call {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < System , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Timestamp , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Balances , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Bounties , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Grandpa , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < TechnicalCommittee , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Democracy , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Council , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Scheduler , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Treasury , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Nft , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Sudo , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Ethereum , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < EVM , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < DynamicFee , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Assets , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Farm , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Elections , Runtime > > ;
        }
    }
}
const _: () = {
    impl ::codec::Encode for Call {
        fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                Call::System(ref aa) => {
                    __codec_dest_edqy.push_byte(1u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::Timestamp(ref aa) => {
                    __codec_dest_edqy.push_byte(3u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::Balances(ref aa) => {
                    __codec_dest_edqy.push_byte(4u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::Bounties(ref aa) => {
                    __codec_dest_edqy.push_byte(7u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::Grandpa(ref aa) => {
                    __codec_dest_edqy.push_byte(8u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::TechnicalCommittee(ref aa) => {
                    __codec_dest_edqy.push_byte(10u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::Democracy(ref aa) => {
                    __codec_dest_edqy.push_byte(11u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::Council(ref aa) => {
                    __codec_dest_edqy.push_byte(12u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::Scheduler(ref aa) => {
                    __codec_dest_edqy.push_byte(13u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::Treasury(ref aa) => {
                    __codec_dest_edqy.push_byte(14u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::Nft(ref aa) => {
                    __codec_dest_edqy.push_byte(16u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::Sudo(ref aa) => {
                    __codec_dest_edqy.push_byte(17u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::Ethereum(ref aa) => {
                    __codec_dest_edqy.push_byte(18u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::EVM(ref aa) => {
                    __codec_dest_edqy.push_byte(19u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::DynamicFee(ref aa) => {
                    __codec_dest_edqy.push_byte(20u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::Assets(ref aa) => {
                    __codec_dest_edqy.push_byte(21u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::Farm(ref aa) => {
                    __codec_dest_edqy.push_byte(22u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::Elections(ref aa) => {
                    __codec_dest_edqy.push_byte(23u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                _ => (),
            }
        }
    }
    impl ::codec::EncodeLike for Call {}
};
const _: () = {
    impl ::codec::Decode for Call {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| e.chain("Could not decode `Call`, failed to read variant byte"))?
            {
                __codec_x_edqy if __codec_x_edqy == 1u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::System({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < System , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::System.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 3u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::Timestamp({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Timestamp , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::Timestamp.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 4u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::Balances({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Balances , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::Balances.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 7u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::Bounties({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Bounties , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::Bounties.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 8u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::Grandpa({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Grandpa , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::Grandpa.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 10u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::TechnicalCommittee({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < TechnicalCommittee , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::TechnicalCommittee.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 11u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::Democracy({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Democracy , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::Democracy.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 12u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::Council({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Council , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::Council.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 13u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::Scheduler({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Scheduler , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::Scheduler.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 14u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::Treasury({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Treasury , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::Treasury.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 16u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::Nft({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Nft , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::Nft.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 17u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::Sudo({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Sudo , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::Sudo.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 18u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::Ethereum({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Ethereum , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::Ethereum.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 19u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::EVM({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < EVM , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::EVM.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 20u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::DynamicFee({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < DynamicFee , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::DynamicFee.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 21u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::Assets({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Assets , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::Assets.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 22u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::Farm({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Farm , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::Farm.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 23u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::Elections({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Elections , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::Elections.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                    "Could not decode `Call`, variant doesn\'t exist",
                )),
            }
        }
    }
};
impl core::fmt::Debug for Call {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::System(ref a0) => fmt.debug_tuple("Call::System").field(a0).finish(),
            Self::Timestamp(ref a0) => fmt.debug_tuple("Call::Timestamp").field(a0).finish(),
            Self::Balances(ref a0) => fmt.debug_tuple("Call::Balances").field(a0).finish(),
            Self::Bounties(ref a0) => fmt.debug_tuple("Call::Bounties").field(a0).finish(),
            Self::Grandpa(ref a0) => fmt.debug_tuple("Call::Grandpa").field(a0).finish(),
            Self::TechnicalCommittee(ref a0) => fmt
                .debug_tuple("Call::TechnicalCommittee")
                .field(a0)
                .finish(),
            Self::Democracy(ref a0) => fmt.debug_tuple("Call::Democracy").field(a0).finish(),
            Self::Council(ref a0) => fmt.debug_tuple("Call::Council").field(a0).finish(),
            Self::Scheduler(ref a0) => fmt.debug_tuple("Call::Scheduler").field(a0).finish(),
            Self::Treasury(ref a0) => fmt.debug_tuple("Call::Treasury").field(a0).finish(),
            Self::Nft(ref a0) => fmt.debug_tuple("Call::Nft").field(a0).finish(),
            Self::Sudo(ref a0) => fmt.debug_tuple("Call::Sudo").field(a0).finish(),
            Self::Ethereum(ref a0) => fmt.debug_tuple("Call::Ethereum").field(a0).finish(),
            Self::EVM(ref a0) => fmt.debug_tuple("Call::EVM").field(a0).finish(),
            Self::DynamicFee(ref a0) => fmt.debug_tuple("Call::DynamicFee").field(a0).finish(),
            Self::Assets(ref a0) => fmt.debug_tuple("Call::Assets").field(a0).finish(),
            Self::Farm(ref a0) => fmt.debug_tuple("Call::Farm").field(a0).finish(),
            Self::Elections(ref a0) => fmt.debug_tuple("Call::Elections").field(a0).finish(),
            _ => Ok(()),
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::GetDispatchInfo
    for Call
{
    fn get_dispatch_info(
        &self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::DispatchInfo
    {
        match self {
            Call::System(call) => call.get_dispatch_info(),
            Call::Timestamp(call) => call.get_dispatch_info(),
            Call::Balances(call) => call.get_dispatch_info(),
            Call::Bounties(call) => call.get_dispatch_info(),
            Call::Grandpa(call) => call.get_dispatch_info(),
            Call::TechnicalCommittee(call) => call.get_dispatch_info(),
            Call::Democracy(call) => call.get_dispatch_info(),
            Call::Council(call) => call.get_dispatch_info(),
            Call::Scheduler(call) => call.get_dispatch_info(),
            Call::Treasury(call) => call.get_dispatch_info(),
            Call::Nft(call) => call.get_dispatch_info(),
            Call::Sudo(call) => call.get_dispatch_info(),
            Call::Ethereum(call) => call.get_dispatch_info(),
            Call::EVM(call) => call.get_dispatch_info(),
            Call::DynamicFee(call) => call.get_dispatch_info(),
            Call::Assets(call) => call.get_dispatch_info(),
            Call::Farm(call) => call.get_dispatch_info(),
            Call::Elections(call) => call.get_dispatch_info(),
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::GetCallMetadata
    for Call
{
    fn get_call_metadata(
        &self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata
    {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::GetCallName;
        match self {
            Call::System(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "System";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::Timestamp(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Timestamp";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::Balances(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Balances";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::Bounties(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Bounties";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::Grandpa(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Grandpa";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::TechnicalCommittee(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "TechnicalCommittee";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::Democracy(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Democracy";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::Council(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Council";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::Scheduler(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Scheduler";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::Treasury(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Treasury";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::Nft(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Nft";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::Sudo(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Sudo";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::Ethereum(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Ethereum";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::EVM(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "EVM";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::DynamicFee(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "DynamicFee";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::Assets(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Assets";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::Farm(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Farm";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::Elections(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Elections";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
        }
    }
    fn get_module_names() -> &'static [&'static str] {
        &[
            "System",
            "Timestamp",
            "Balances",
            "Bounties",
            "Grandpa",
            "TechnicalCommittee",
            "Democracy",
            "Council",
            "Scheduler",
            "Treasury",
            "Nft",
            "Sudo",
            "Ethereum",
            "EVM",
            "DynamicFee",
            "Assets",
            "Farm",
            "Elections",
        ]
    }
    fn get_call_names(module: &str) -> &'static [&'static str] {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::{
            Callable, GetCallName,
        };
        match module {
            "System" => <<System as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "Timestamp" => {
                <<Timestamp as Callable<Runtime>>::Call as GetCallName>::get_call_names()
            }
            "Balances" => <<Balances as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "Bounties" => <<Bounties as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "Grandpa" => <<Grandpa as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "TechnicalCommittee" => {
                <<TechnicalCommittee as Callable<Runtime>>::Call as GetCallName>::get_call_names()
            }
            "Democracy" => {
                <<Democracy as Callable<Runtime>>::Call as GetCallName>::get_call_names()
            }
            "Council" => <<Council as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "Scheduler" => {
                <<Scheduler as Callable<Runtime>>::Call as GetCallName>::get_call_names()
            }
            "Treasury" => <<Treasury as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "Nft" => <<Nft as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "Sudo" => <<Sudo as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "Ethereum" => <<Ethereum as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "EVM" => <<EVM as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "DynamicFee" => {
                <<DynamicFee as Callable<Runtime>>::Call as GetCallName>::get_call_names()
            }
            "Assets" => <<Assets as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "Farm" => <<Farm as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "Elections" => {
                <<Elections as Callable<Runtime>>::Call as GetCallName>::get_call_names()
            }
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::Dispatchable
    for Call
{
    type Origin = Origin;
    type Config = Call;
    type Info =
        self::sp_api_hidden_includes_construct_runtime::hidden_include::weights::DispatchInfo;
    type PostInfo =
        self::sp_api_hidden_includes_construct_runtime::hidden_include::weights::PostDispatchInfo;    fn dispatch (self , origin : Origin) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: DispatchResultWithPostInfo{
        if ! < Self :: Origin as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait > :: filter_call (& origin , & self) { return self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: result :: Result :: Err (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: DispatchError :: BadOrigin . into ()) ; }
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (self , origin)
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable
    for Call
{
    type Origin = Origin;    fn dispatch_bypass_filter (self , origin : Origin) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: DispatchResultWithPostInfo{
        match self { Call :: System (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: Timestamp (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: Balances (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: Bounties (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: Grandpa (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: TechnicalCommittee (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: Democracy (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: Council (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: Scheduler (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: Treasury (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: Nft (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: Sudo (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: Ethereum (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: EVM (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: DynamicFee (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: Assets (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: Farm (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: Elections (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , }
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            System,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            System,
            Runtime,
        >,
    > {
        match self {
            Call::System(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            System,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < System , Runtime >,
    ) -> Self {
        Call::System(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Timestamp,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Timestamp,
            Runtime,
        >,
    > {
        match self {
            Call::Timestamp(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Timestamp,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Timestamp , Runtime >,
    ) -> Self {
        Call::Timestamp(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Balances,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Balances,
            Runtime,
        >,
    > {
        match self {
            Call::Balances(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Balances,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Balances , Runtime >,
    ) -> Self {
        Call::Balances(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Bounties,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Bounties,
            Runtime,
        >,
    > {
        match self {
            Call::Bounties(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Bounties,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Bounties , Runtime >,
    ) -> Self {
        Call::Bounties(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Grandpa,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Grandpa,
            Runtime,
        >,
    > {
        match self {
            Call::Grandpa(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Grandpa,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Grandpa , Runtime >,
    ) -> Self {
        Call::Grandpa(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            TechnicalCommittee,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            TechnicalCommittee,
            Runtime,
        >,
    > {
        match self {
            Call::TechnicalCommittee(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            TechnicalCommittee,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < TechnicalCommittee , Runtime >,
    ) -> Self {
        Call::TechnicalCommittee(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Democracy,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Democracy,
            Runtime,
        >,
    > {
        match self {
            Call::Democracy(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Democracy,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Democracy , Runtime >,
    ) -> Self {
        Call::Democracy(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Council,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Council,
            Runtime,
        >,
    > {
        match self {
            Call::Council(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Council,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Council , Runtime >,
    ) -> Self {
        Call::Council(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Scheduler,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Scheduler,
            Runtime,
        >,
    > {
        match self {
            Call::Scheduler(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Scheduler,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Scheduler , Runtime >,
    ) -> Self {
        Call::Scheduler(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Treasury,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Treasury,
            Runtime,
        >,
    > {
        match self {
            Call::Treasury(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Treasury,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Treasury , Runtime >,
    ) -> Self {
        Call::Treasury(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Nft,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Nft,
            Runtime,
        >,
    > {
        match self {
            Call::Nft(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Nft,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Nft , Runtime >,
    ) -> Self {
        Call::Nft(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Sudo,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Sudo,
            Runtime,
        >,
    > {
        match self {
            Call::Sudo(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Sudo,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Sudo , Runtime >,
    ) -> Self {
        Call::Sudo(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Ethereum,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Ethereum,
            Runtime,
        >,
    > {
        match self {
            Call::Ethereum(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Ethereum,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Ethereum , Runtime >,
    ) -> Self {
        Call::Ethereum(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            EVM,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            EVM,
            Runtime,
        >,
    > {
        match self {
            Call::EVM(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            EVM,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < EVM , Runtime >,
    ) -> Self {
        Call::EVM(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            DynamicFee,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            DynamicFee,
            Runtime,
        >,
    > {
        match self {
            Call::DynamicFee(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            DynamicFee,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < DynamicFee , Runtime >,
    ) -> Self {
        Call::DynamicFee(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Assets,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Assets,
            Runtime,
        >,
    > {
        match self {
            Call::Assets(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Assets,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Assets , Runtime >,
    ) -> Self {
        Call::Assets(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Farm,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Farm,
            Runtime,
        >,
    > {
        match self {
            Call::Farm(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Farm,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Farm , Runtime >,
    ) -> Self {
        Call::Farm(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Elections,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Elections,
            Runtime,
        >,
    > {
        match self {
            Call::Elections(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Elections,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Elections , Runtime >,
    ) -> Self {
        Call::Elections(call)
    }
}
impl Runtime {
    pub fn metadata () -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: RuntimeMetadataPrefixed{
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: RuntimeMetadataLastVersion { modules : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("System") , index : 1u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (frame_system :: Pallet :: < Runtime > :: storage_metadata))) , calls : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (frame_system :: Pallet :: < Runtime > :: call_functions))) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (frame_system :: Event :: < Runtime > :: metadata))) , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (frame_system :: Pallet :: < Runtime > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< frame_system :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("RandomnessCollectiveFlip") , index : 2u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_randomness_collective_flip :: Pallet :: < Runtime > :: storage_metadata))) , calls : None , event : None , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_randomness_collective_flip :: Pallet :: < Runtime > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< pallet_randomness_collective_flip :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Timestamp") , index : 3u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_timestamp :: Pallet :: < Runtime > :: storage_metadata))) , calls : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_timestamp :: Pallet :: < Runtime > :: call_functions))) , event : None , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_timestamp :: Pallet :: < Runtime > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< pallet_timestamp :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Balances") , index : 4u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_balances :: Pallet :: < Runtime > :: storage_metadata))) , calls : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_balances :: Pallet :: < Runtime > :: call_functions))) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_balances :: Event :: < Runtime > :: metadata))) , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_balances :: Pallet :: < Runtime > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< pallet_balances :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Aura") , index : 5u8 , storage : None , calls : None , event : None , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_aura :: Pallet :: < Runtime > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< pallet_aura :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Bounties") , index : 7u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_bounties :: Pallet :: < Runtime > :: storage_metadata))) , calls : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_bounties :: Pallet :: < Runtime > :: call_functions))) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_bounties :: Event :: < Runtime > :: metadata))) , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_bounties :: Pallet :: < Runtime > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< pallet_bounties :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Grandpa") , index : 8u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_grandpa :: Pallet :: < Runtime > :: storage_metadata))) , calls : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_grandpa :: Pallet :: < Runtime > :: call_functions))) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_grandpa :: Event :: metadata))) , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_grandpa :: Pallet :: < Runtime > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< pallet_grandpa :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("TransactionPayment") , index : 9u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_transaction_payment :: Pallet :: < Runtime > :: storage_metadata))) , calls : None , event : None , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_transaction_payment :: Pallet :: < Runtime > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< pallet_transaction_payment :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("TechnicalCommittee") , index : 10u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_collective :: Pallet :: < Runtime , pallet_collective :: Instance2 > :: storage_metadata))) , calls : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_collective :: Pallet :: < Runtime , pallet_collective :: Instance2 > :: call_functions))) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_collective :: Event :: < Runtime , pallet_collective :: Instance2 > :: metadata))) , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_collective :: Pallet :: < Runtime , pallet_collective :: Instance2 > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< pallet_collective :: Pallet < Runtime , pallet_collective :: Instance2 > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Democracy") , index : 11u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_democracy :: Pallet :: < Runtime > :: storage_metadata))) , calls : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_democracy :: Pallet :: < Runtime > :: call_functions))) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_democracy :: Event :: < Runtime > :: metadata))) , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_democracy :: Pallet :: < Runtime > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< pallet_democracy :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Council") , index : 12u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_collective :: Pallet :: < Runtime , pallet_collective :: Instance1 > :: storage_metadata))) , calls : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_collective :: Pallet :: < Runtime , pallet_collective :: Instance1 > :: call_functions))) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_collective :: Event :: < Runtime , pallet_collective :: Instance1 > :: metadata))) , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_collective :: Pallet :: < Runtime , pallet_collective :: Instance1 > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< pallet_collective :: Pallet < Runtime , pallet_collective :: Instance1 > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Scheduler") , index : 13u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_scheduler :: Pallet :: < Runtime > :: storage_metadata))) , calls : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_scheduler :: Pallet :: < Runtime > :: call_functions))) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_scheduler :: Event :: < Runtime > :: metadata))) , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_scheduler :: Pallet :: < Runtime > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< pallet_scheduler :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Treasury") , index : 14u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_treasury :: Pallet :: < Runtime > :: storage_metadata))) , calls : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_treasury :: Pallet :: < Runtime > :: call_functions))) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_treasury :: Event :: < Runtime > :: metadata))) , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_treasury :: Pallet :: < Runtime > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< pallet_treasury :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("OrmlNft") , index : 15u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (orml_nft :: Pallet :: < Runtime > :: storage_metadata))) , calls : None , event : None , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (orml_nft :: Pallet :: < Runtime > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< orml_nft :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Nft") , index : 16u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_nft :: Pallet :: < Runtime > :: storage_metadata))) , calls : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_nft :: Pallet :: < Runtime > :: call_functions))) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_nft :: Event :: < Runtime > :: metadata))) , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_nft :: Pallet :: < Runtime > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< pallet_nft :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Sudo") , index : 17u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_sudo :: Pallet :: < Runtime > :: storage_metadata))) , calls : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_sudo :: Pallet :: < Runtime > :: call_functions))) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_sudo :: Event :: < Runtime > :: metadata))) , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_sudo :: Pallet :: < Runtime > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< pallet_sudo :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Ethereum") , index : 18u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_ethereum :: Pallet :: < Runtime > :: storage_metadata))) , calls : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_ethereum :: Pallet :: < Runtime > :: call_functions))) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_ethereum :: Event :: metadata))) , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_ethereum :: Pallet :: < Runtime > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< pallet_ethereum :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("EVM") , index : 19u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_evm :: Pallet :: < Runtime > :: storage_metadata))) , calls : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_evm :: Pallet :: < Runtime > :: call_functions))) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_evm :: Event :: < Runtime > :: metadata))) , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_evm :: Pallet :: < Runtime > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< pallet_evm :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("DynamicFee") , index : 20u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_dynamic_fee :: Pallet :: < Runtime > :: storage_metadata))) , calls : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_dynamic_fee :: Pallet :: < Runtime > :: call_functions))) , event : None , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_dynamic_fee :: Pallet :: < Runtime > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< pallet_dynamic_fee :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Assets") , index : 21u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_assets :: Pallet :: < Runtime > :: storage_metadata))) , calls : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_assets :: Pallet :: < Runtime > :: call_functions))) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_assets :: Event :: < Runtime > :: metadata))) , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_assets :: Pallet :: < Runtime > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< pallet_assets :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Farm") , index : 22u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (penguin_farm :: Pallet :: < Runtime > :: storage_metadata))) , calls : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (penguin_farm :: Pallet :: < Runtime > :: call_functions))) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (penguin_farm :: Event :: < Runtime > :: metadata))) , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (penguin_farm :: Pallet :: < Runtime > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< penguin_farm :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Elections") , index : 23u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_elections_phragmen :: Pallet :: < Runtime > :: storage_metadata))) , calls : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_elections_phragmen :: Pallet :: < Runtime > :: call_functions))) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_elections_phragmen :: Event :: < Runtime > :: metadata))) , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_elections_phragmen :: Pallet :: < Runtime > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< pallet_elections_phragmen :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , }]) , extrinsic : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ExtrinsicMetadata { version : < UncheckedExtrinsic as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: ExtrinsicMetadata > :: VERSION , signed_extensions : < < UncheckedExtrinsic as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: ExtrinsicMetadata > :: SignedExtensions as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: SignedExtension > :: identifier () . into_iter () . map (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode) . collect () , } , } . into ()
    }
}
#[cfg(any(feature = "std", test))]
pub type SystemConfig = frame_system::GenesisConfig;
#[cfg(any(feature = "std", test))]
pub type BalancesConfig = pallet_balances::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type AuraConfig = pallet_aura::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type GrandpaConfig = pallet_grandpa::GenesisConfig;
#[cfg(any(feature = "std", test))]
pub type TechnicalCommitteeConfig =
    pallet_collective::GenesisConfig<Runtime, pallet_collective::Instance2>;
#[cfg(any(feature = "std", test))]
pub type DemocracyConfig = pallet_democracy::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type CouncilConfig = pallet_collective::GenesisConfig<Runtime, pallet_collective::Instance1>;
#[cfg(any(feature = "std", test))]
pub type TreasuryConfig = pallet_treasury::GenesisConfig;
#[cfg(any(feature = "std", test))]
pub type SudoConfig = pallet_sudo::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type EthereumConfig = pallet_ethereum::GenesisConfig;
#[cfg(any(feature = "std", test))]
pub type EVMConfig = pallet_evm::GenesisConfig;
#[cfg(any(feature = "std", test))]
pub type DynamicFeeConfig = pallet_dynamic_fee::GenesisConfig;
#[cfg(any(feature = "std", test))]
pub type FarmConfig = penguin_farm::GenesisConfig;
#[cfg(any(feature = "std", test))]
pub type ElectionsConfig = pallet_elections_phragmen::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
use self::sp_api_hidden_includes_construct_runtime::hidden_include::serde as __genesis_config_serde_import__;
#[cfg(any(feature = "std", test))]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(crate = "__genesis_config_serde_import__")]
pub struct GenesisConfig {
    pub system: SystemConfig,
    pub balances: BalancesConfig,
    pub aura: AuraConfig,
    pub grandpa: GrandpaConfig,
    pub technical_committee: TechnicalCommitteeConfig,
    pub democracy: DemocracyConfig,
    pub council: CouncilConfig,
    pub treasury: TreasuryConfig,
    pub sudo: SudoConfig,
    pub ethereum: EthereumConfig,
    pub evm: EVMConfig,
    pub dynamic_fee: DynamicFeeConfig,
    pub farm: FarmConfig,
    pub elections: ElectionsConfig,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use __genesis_config_serde_import__ as _serde;
    #[automatically_derived]
    impl __genesis_config_serde_import__::Serialize for GenesisConfig {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> __genesis_config_serde_import__::__private::Result<__S::Ok, __S::Error>
        where
            __S: __genesis_config_serde_import__::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "GenesisConfig",
                false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "system",
                &self.system,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "balances",
                &self.balances,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "aura",
                &self.aura,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "grandpa",
                &self.grandpa,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "technicalCommittee",
                &self.technical_committee,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "democracy",
                &self.democracy,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "council",
                &self.council,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "treasury",
                &self.treasury,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "sudo",
                &self.sudo,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "ethereum",
                &self.ethereum,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "evm",
                &self.evm,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "dynamicFee",
                &self.dynamic_fee,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "farm",
                &self.farm,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "elections",
                &self.elections,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use __genesis_config_serde_import__ as _serde;
    #[automatically_derived]
    impl<'de> __genesis_config_serde_import__::Deserialize<'de> for GenesisConfig {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> __genesis_config_serde_import__::__private::Result<Self, __D::Error>
        where
            __D: __genesis_config_serde_import__::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
                __field6,
                __field7,
                __field8,
                __field9,
                __field10,
                __field11,
                __field12,
                __field13,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        4u64 => _serde::__private::Ok(__Field::__field4),
                        5u64 => _serde::__private::Ok(__Field::__field5),
                        6u64 => _serde::__private::Ok(__Field::__field6),
                        7u64 => _serde::__private::Ok(__Field::__field7),
                        8u64 => _serde::__private::Ok(__Field::__field8),
                        9u64 => _serde::__private::Ok(__Field::__field9),
                        10u64 => _serde::__private::Ok(__Field::__field10),
                        11u64 => _serde::__private::Ok(__Field::__field11),
                        12u64 => _serde::__private::Ok(__Field::__field12),
                        13u64 => _serde::__private::Ok(__Field::__field13),
                        _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"field index 0 <= i < 14",
                        )),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "system" => _serde::__private::Ok(__Field::__field0),
                        "balances" => _serde::__private::Ok(__Field::__field1),
                        "aura" => _serde::__private::Ok(__Field::__field2),
                        "grandpa" => _serde::__private::Ok(__Field::__field3),
                        "technicalCommittee" => _serde::__private::Ok(__Field::__field4),
                        "democracy" => _serde::__private::Ok(__Field::__field5),
                        "council" => _serde::__private::Ok(__Field::__field6),
                        "treasury" => _serde::__private::Ok(__Field::__field7),
                        "sudo" => _serde::__private::Ok(__Field::__field8),
                        "ethereum" => _serde::__private::Ok(__Field::__field9),
                        "evm" => _serde::__private::Ok(__Field::__field10),
                        "dynamicFee" => _serde::__private::Ok(__Field::__field11),
                        "farm" => _serde::__private::Ok(__Field::__field12),
                        "elections" => _serde::__private::Ok(__Field::__field13),
                        _ => _serde::__private::Err(_serde::de::Error::unknown_field(
                            __value, FIELDS,
                        )),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"system" => _serde::__private::Ok(__Field::__field0),
                        b"balances" => _serde::__private::Ok(__Field::__field1),
                        b"aura" => _serde::__private::Ok(__Field::__field2),
                        b"grandpa" => _serde::__private::Ok(__Field::__field3),
                        b"technicalCommittee" => _serde::__private::Ok(__Field::__field4),
                        b"democracy" => _serde::__private::Ok(__Field::__field5),
                        b"council" => _serde::__private::Ok(__Field::__field6),
                        b"treasury" => _serde::__private::Ok(__Field::__field7),
                        b"sudo" => _serde::__private::Ok(__Field::__field8),
                        b"ethereum" => _serde::__private::Ok(__Field::__field9),
                        b"evm" => _serde::__private::Ok(__Field::__field10),
                        b"dynamicFee" => _serde::__private::Ok(__Field::__field11),
                        b"farm" => _serde::__private::Ok(__Field::__field12),
                        b"elections" => _serde::__private::Ok(__Field::__field13),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(_serde::de::Error::unknown_field(
                                __value, FIELDS,
                            ))
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<GenesisConfig>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = GenesisConfig;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct GenesisConfig")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match match _serde::de::SeqAccess::next_element::<SystemConfig>(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct GenesisConfig with 14 elements",
                                ));
                            }
                        };
                    let __field1 = match match _serde::de::SeqAccess::next_element::<BalancesConfig>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                1usize,
                                &"struct GenesisConfig with 14 elements",
                            ));
                        }
                    };
                    let __field2 =
                        match match _serde::de::SeqAccess::next_element::<AuraConfig>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct GenesisConfig with 14 elements",
                                ));
                            }
                        };
                    let __field3 = match match _serde::de::SeqAccess::next_element::<GrandpaConfig>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                3usize,
                                &"struct GenesisConfig with 14 elements",
                            ));
                        }
                    };
                    let __field4 = match match _serde::de::SeqAccess::next_element::<
                        TechnicalCommitteeConfig,
                    >(&mut __seq)
                    {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                4usize,
                                &"struct GenesisConfig with 14 elements",
                            ));
                        }
                    };
                    let __field5 = match match _serde::de::SeqAccess::next_element::<DemocracyConfig>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                5usize,
                                &"struct GenesisConfig with 14 elements",
                            ));
                        }
                    };
                    let __field6 = match match _serde::de::SeqAccess::next_element::<CouncilConfig>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                6usize,
                                &"struct GenesisConfig with 14 elements",
                            ));
                        }
                    };
                    let __field7 = match match _serde::de::SeqAccess::next_element::<TreasuryConfig>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                7usize,
                                &"struct GenesisConfig with 14 elements",
                            ));
                        }
                    };
                    let __field8 =
                        match match _serde::de::SeqAccess::next_element::<SudoConfig>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    8usize,
                                    &"struct GenesisConfig with 14 elements",
                                ));
                            }
                        };
                    let __field9 = match match _serde::de::SeqAccess::next_element::<EthereumConfig>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                9usize,
                                &"struct GenesisConfig with 14 elements",
                            ));
                        }
                    };
                    let __field10 =
                        match match _serde::de::SeqAccess::next_element::<EVMConfig>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    10usize,
                                    &"struct GenesisConfig with 14 elements",
                                ));
                            }
                        };
                    let __field11 = match match _serde::de::SeqAccess::next_element::<
                        DynamicFeeConfig,
                    >(&mut __seq)
                    {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                11usize,
                                &"struct GenesisConfig with 14 elements",
                            ));
                        }
                    };
                    let __field12 =
                        match match _serde::de::SeqAccess::next_element::<FarmConfig>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    12usize,
                                    &"struct GenesisConfig with 14 elements",
                                ));
                            }
                        };
                    let __field13 = match match _serde::de::SeqAccess::next_element::<ElectionsConfig>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                13usize,
                                &"struct GenesisConfig with 14 elements",
                            ));
                        }
                    };
                    _serde::__private::Ok(GenesisConfig {
                        system: __field0,
                        balances: __field1,
                        aura: __field2,
                        grandpa: __field3,
                        technical_committee: __field4,
                        democracy: __field5,
                        council: __field6,
                        treasury: __field7,
                        sudo: __field8,
                        ethereum: __field9,
                        evm: __field10,
                        dynamic_fee: __field11,
                        farm: __field12,
                        elections: __field13,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<SystemConfig> =
                        _serde::__private::None;
                    let mut __field1: _serde::__private::Option<BalancesConfig> =
                        _serde::__private::None;
                    let mut __field2: _serde::__private::Option<AuraConfig> =
                        _serde::__private::None;
                    let mut __field3: _serde::__private::Option<GrandpaConfig> =
                        _serde::__private::None;
                    let mut __field4: _serde::__private::Option<TechnicalCommitteeConfig> =
                        _serde::__private::None;
                    let mut __field5: _serde::__private::Option<DemocracyConfig> =
                        _serde::__private::None;
                    let mut __field6: _serde::__private::Option<CouncilConfig> =
                        _serde::__private::None;
                    let mut __field7: _serde::__private::Option<TreasuryConfig> =
                        _serde::__private::None;
                    let mut __field8: _serde::__private::Option<SudoConfig> =
                        _serde::__private::None;
                    let mut __field9: _serde::__private::Option<EthereumConfig> =
                        _serde::__private::None;
                    let mut __field10: _serde::__private::Option<EVMConfig> =
                        _serde::__private::None;
                    let mut __field11: _serde::__private::Option<DynamicFeeConfig> =
                        _serde::__private::None;
                    let mut __field12: _serde::__private::Option<FarmConfig> =
                        _serde::__private::None;
                    let mut __field13: _serde::__private::Option<ElectionsConfig> =
                        _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "system",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<SystemConfig>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "balances",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<BalancesConfig>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("aura"),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<AuraConfig>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field3 => {
                                if _serde::__private::Option::is_some(&__field3) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "grandpa",
                                        ),
                                    );
                                }
                                __field3 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<GrandpaConfig>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field4 => {
                                if _serde::__private::Option::is_some(&__field4) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "technicalCommittee",
                                        ),
                                    );
                                }
                                __field4 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<
                                        TechnicalCommitteeConfig,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field5 => {
                                if _serde::__private::Option::is_some(&__field5) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "democracy",
                                        ),
                                    );
                                }
                                __field5 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<DemocracyConfig>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field6 => {
                                if _serde::__private::Option::is_some(&__field6) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "council",
                                        ),
                                    );
                                }
                                __field6 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<CouncilConfig>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field7 => {
                                if _serde::__private::Option::is_some(&__field7) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "treasury",
                                        ),
                                    );
                                }
                                __field7 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<TreasuryConfig>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field8 => {
                                if _serde::__private::Option::is_some(&__field8) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("sudo"),
                                    );
                                }
                                __field8 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<SudoConfig>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field9 => {
                                if _serde::__private::Option::is_some(&__field9) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ethereum",
                                        ),
                                    );
                                }
                                __field9 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<EthereumConfig>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field10 => {
                                if _serde::__private::Option::is_some(&__field10) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("evm"),
                                    );
                                }
                                __field10 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<EVMConfig>(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field11 => {
                                if _serde::__private::Option::is_some(&__field11) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dynamicFee",
                                        ),
                                    );
                                }
                                __field11 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<DynamicFeeConfig>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field12 => {
                                if _serde::__private::Option::is_some(&__field12) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("farm"),
                                    );
                                }
                                __field12 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<FarmConfig>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field13 => {
                                if _serde::__private::Option::is_some(&__field13) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "elections",
                                        ),
                                    );
                                }
                                __field13 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<ElectionsConfig>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("system") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("balances") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("aura") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::__private::Some(__field3) => __field3,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("grandpa") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field4 = match __field4 {
                        _serde::__private::Some(__field4) => __field4,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("technicalCommittee") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field5 = match __field5 {
                        _serde::__private::Some(__field5) => __field5,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("democracy") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field6 = match __field6 {
                        _serde::__private::Some(__field6) => __field6,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("council") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field7 = match __field7 {
                        _serde::__private::Some(__field7) => __field7,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("treasury") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field8 = match __field8 {
                        _serde::__private::Some(__field8) => __field8,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("sudo") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field9 = match __field9 {
                        _serde::__private::Some(__field9) => __field9,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("ethereum") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field10 = match __field10 {
                        _serde::__private::Some(__field10) => __field10,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("evm") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field11 = match __field11 {
                        _serde::__private::Some(__field11) => __field11,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("dynamicFee") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field12 = match __field12 {
                        _serde::__private::Some(__field12) => __field12,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("farm") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field13 = match __field13 {
                        _serde::__private::Some(__field13) => __field13,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("elections") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(GenesisConfig {
                        system: __field0,
                        balances: __field1,
                        aura: __field2,
                        grandpa: __field3,
                        technical_committee: __field4,
                        democracy: __field5,
                        council: __field6,
                        treasury: __field7,
                        sudo: __field8,
                        ethereum: __field9,
                        evm: __field10,
                        dynamic_fee: __field11,
                        farm: __field12,
                        elections: __field13,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &[
                "system",
                "balances",
                "aura",
                "grandpa",
                "technicalCommittee",
                "democracy",
                "council",
                "treasury",
                "sudo",
                "ethereum",
                "evm",
                "dynamicFee",
                "farm",
                "elections",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "GenesisConfig",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<GenesisConfig>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for GenesisConfig {
    #[inline]
    fn default() -> GenesisConfig {
        GenesisConfig {
            system: ::core::default::Default::default(),
            balances: ::core::default::Default::default(),
            aura: ::core::default::Default::default(),
            grandpa: ::core::default::Default::default(),
            technical_committee: ::core::default::Default::default(),
            democracy: ::core::default::Default::default(),
            council: ::core::default::Default::default(),
            treasury: ::core::default::Default::default(),
            sudo: ::core::default::Default::default(),
            ethereum: ::core::default::Default::default(),
            evm: ::core::default::Default::default(),
            dynamic_fee: ::core::default::Default::default(),
            farm: ::core::default::Default::default(),
            elections: ::core::default::Default::default(),
        }
    }
}
#[cfg(any(feature = "std", test))]
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildStorage
    for GenesisConfig
{
    fn assimilate_storage(
        &self,
        storage : & mut self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: Storage,
    ) -> std::result::Result<(), String> {
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Runtime , frame_system :: __InherentHiddenInstance > :: build_module_genesis_storage (& self . system , storage) ? ;
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Runtime , pallet_balances :: __InherentHiddenInstance > :: build_module_genesis_storage (& self . balances , storage) ? ;
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Runtime , pallet_aura :: __InherentHiddenInstance > :: build_module_genesis_storage (& self . aura , storage) ? ;
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Runtime , pallet_grandpa :: __InherentHiddenInstance > :: build_module_genesis_storage (& self . grandpa , storage) ? ;
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Runtime , pallet_collective :: Instance2 > :: build_module_genesis_storage (& self . technical_committee , storage) ? ;
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Runtime , pallet_democracy :: __InherentHiddenInstance > :: build_module_genesis_storage (& self . democracy , storage) ? ;
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Runtime , pallet_collective :: Instance1 > :: build_module_genesis_storage (& self . council , storage) ? ;
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Runtime , pallet_treasury :: __InherentHiddenInstance > :: build_module_genesis_storage (& self . treasury , storage) ? ;
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Runtime , pallet_sudo :: __InherentHiddenInstance > :: build_module_genesis_storage (& self . sudo , storage) ? ;
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Runtime , pallet_ethereum :: __InherentHiddenInstance > :: build_module_genesis_storage (& self . ethereum , storage) ? ;
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Runtime , pallet_evm :: __InherentHiddenInstance > :: build_module_genesis_storage (& self . evm , storage) ? ;
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Runtime , pallet_dynamic_fee :: __InherentHiddenInstance > :: build_module_genesis_storage (& self . dynamic_fee , storage) ? ;
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Runtime , penguin_farm :: __InherentHiddenInstance > :: build_module_genesis_storage (& self . farm , storage) ? ;
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Runtime , pallet_elections_phragmen :: __InherentHiddenInstance > :: build_module_genesis_storage (& self . elections , storage) ? ;
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: BasicExternalities :: execute_with_storage (storage , | | { < AllPalletsWithSystem as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OnGenesis > :: on_genesis () ; }) ;
        Ok(())
    }
}
trait InherentDataExt {
    fn create_extrinsics (& self) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: Vec < < Block as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: BlockT > :: Extrinsic > ;
    fn check_extrinsics (& self , block : & Block) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: CheckInherentsResult ;
}
impl InherentDataExt
    for self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::InherentData
{
    fn create_extrinsics (& self) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: Vec < < Block as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: BlockT > :: Extrinsic >{
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::ProvideInherent;
        let mut inherents = Vec::new();
        if let Some(inherent) = Timestamp::create_inherent(self) {
            let inherent = < UncheckedExtrinsic as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: Extrinsic > :: new (inherent . into () , None) . expect ("Runtime UncheckedExtrinsic is not Opaque, so it has to return \
							`Some`; qed") ;
            inherents.push(inherent);
        }
        if let Some(inherent) = DynamicFee::create_inherent(self) {
            let inherent = < UncheckedExtrinsic as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: Extrinsic > :: new (inherent . into () , None) . expect ("Runtime UncheckedExtrinsic is not Opaque, so it has to return \
							`Some`; qed") ;
            inherents.push(inherent);
        }
        inherents
    }    fn check_extrinsics (& self , block : & Block) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: CheckInherentsResult{
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::{
            ProvideInherent, IsFatalError,
        };
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::{
            IsSubType, ExtrinsicCall,
        };
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block as _;
        let mut result = self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: CheckInherentsResult :: new () ;
        for xt in block.extrinsics() {
            if self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: Extrinsic :: is_signed (xt) . unwrap_or (false) { break }
            let mut is_inherent = false;
            {
                let call = <UncheckedExtrinsic as ExtrinsicCall>::call(xt);
                if let Some(call) = IsSubType::<_>::is_sub_type(call) {
                    if Timestamp::is_inherent(call) {
                        is_inherent = true;
                        if let Err(e) = Timestamp::check_inherent(call, self) {
                            result
                                .put_error(Timestamp::INHERENT_IDENTIFIER, &e)
                                .expect("There is only one fatal error; qed");
                            if e.is_fatal_error() {
                                return result;
                            }
                        }
                    }
                }
            }
            {
                let call = <UncheckedExtrinsic as ExtrinsicCall>::call(xt);
                if let Some(call) = IsSubType::<_>::is_sub_type(call) {
                    if DynamicFee::is_inherent(call) {
                        is_inherent = true;
                        if let Err(e) = DynamicFee::check_inherent(call, self) {
                            result
                                .put_error(DynamicFee::INHERENT_IDENTIFIER, &e)
                                .expect("There is only one fatal error; qed");
                            if e.is_fatal_error() {
                                return result;
                            }
                        }
                    }
                }
            }
            if !is_inherent {
                break;
            }
        }
        match Timestamp::is_inherent_required(self) {
            Ok(Some(e)) => {
                let found = block . extrinsics () . iter () . any (| xt | { let is_signed = self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: Extrinsic :: is_signed (xt) . unwrap_or (false) ; if ! is_signed { let call = < UncheckedExtrinsic as ExtrinsicCall > :: call (xt) ; if let Some (call) = IsSubType :: < _ > :: is_sub_type (call) { Timestamp :: is_inherent (& call) } else { false } } else { false } }) ;
                if !found {
                    result
                        .put_error(Timestamp::INHERENT_IDENTIFIER, &e)
                        .expect("There is only one fatal error; qed");
                    if e.is_fatal_error() {
                        return result;
                    }
                }
            }
            Ok(None) => (),
            Err(e) => {
                result
                    .put_error(Timestamp::INHERENT_IDENTIFIER, &e)
                    .expect("There is only one fatal error; qed");
                if e.is_fatal_error() {
                    return result;
                }
            }
        }
        match DynamicFee::is_inherent_required(self) {
            Ok(Some(e)) => {
                let found = block . extrinsics () . iter () . any (| xt | { let is_signed = self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: Extrinsic :: is_signed (xt) . unwrap_or (false) ; if ! is_signed { let call = < UncheckedExtrinsic as ExtrinsicCall > :: call (xt) ; if let Some (call) = IsSubType :: < _ > :: is_sub_type (call) { DynamicFee :: is_inherent (& call) } else { false } } else { false } }) ;
                if !found {
                    result
                        .put_error(DynamicFee::INHERENT_IDENTIFIER, &e)
                        .expect("There is only one fatal error; qed");
                    if e.is_fatal_error() {
                        return result;
                    }
                }
            }
            Ok(None) => (),
            Err(e) => {
                result
                    .put_error(DynamicFee::INHERENT_IDENTIFIER, &e)
                    .expect("There is only one fatal error; qed");
                if e.is_fatal_error() {
                    return result;
                }
            }
        }
        result
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::EnsureInherentsAreFirst<
        Block,
    > for Runtime
{
    fn ensure_inherents_are_first(block: &Block) -> Result<(), u32> {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::ProvideInherent;
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::{
            IsSubType, ExtrinsicCall,
        };
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block as _;
        let mut first_signed_observed = false;
        for (i, xt) in block.extrinsics().iter().enumerate() {
            let is_signed = self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: Extrinsic :: is_signed (xt) . unwrap_or (false) ;
            let is_inherent = if is_signed {
                false
            } else {
                let mut is_inherent = false;
                {
                    let call = <UncheckedExtrinsic as ExtrinsicCall>::call(xt);
                    if let Some(call) = IsSubType::<_>::is_sub_type(call) {
                        if Timestamp::is_inherent(&call) {
                            is_inherent = true;
                        }
                    }
                }
                {
                    let call = <UncheckedExtrinsic as ExtrinsicCall>::call(xt);
                    if let Some(call) = IsSubType::<_>::is_sub_type(call) {
                        if DynamicFee::is_inherent(&call) {
                            is_inherent = true;
                        }
                    }
                }
                is_inherent
            };
            if !is_inherent {
                first_signed_observed = true;
            }
            if first_signed_observed && is_inherent {
                return Err(i as u32);
            }
        }
        Ok(())
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::ValidateUnsigned
    for Runtime
{
    type Call = Call;    fn pre_dispatch (call : & Self :: Call) -> Result < () , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: unsigned :: TransactionValidityError >{
        #[allow(unreachable_patterns)]
        match call {
            Call::Ethereum(inner_call) => Ethereum::pre_dispatch(inner_call),
            _ => Ok(()),
        }
    }
    fn validate_unsigned(
        # [allow (unused_variables)] source : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: unsigned :: TransactionSource,
        call: &Self::Call,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::TransactionValidity
    {
        # [allow (unreachable_patterns)] match call { Call :: Ethereum (inner_call) => Ethereum :: validate_unsigned (source , inner_call) , _ => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: unsigned :: UnknownTransaction :: NoUnsignedValidator . into () , }
    }
}
#[cfg(test)]
mod __construct_runtime_integrity_test {
    use super::*;
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const runtime_integrity_tests: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "__construct_runtime_integrity_test::runtime_integrity_tests",
            ),
            ignore: false,
            allow_fail: false,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(runtime_integrity_tests())),
    };
    pub fn runtime_integrity_tests() {
        < AllPalletsWithSystem as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: IntegrityTest > :: integrity_test () ;
    }
}
pub struct TransactionConverter;
impl fp_rpc::ConvertTransaction<UncheckedExtrinsic> for TransactionConverter {
    fn convert_transaction(&self, transaction: pallet_ethereum::Transaction) -> UncheckedExtrinsic {
        UncheckedExtrinsic::new_unsigned(
            pallet_ethereum::Call::<Runtime>::transact(transaction).into(),
        )
    }
}
impl fp_rpc::ConvertTransaction<opaque::UncheckedExtrinsic> for TransactionConverter {
    fn convert_transaction(
        &self,
        transaction: pallet_ethereum::Transaction,
    ) -> opaque::UncheckedExtrinsic {
        let extrinsic = UncheckedExtrinsic::new_unsigned(
            pallet_ethereum::Call::<Runtime>::transact(transaction).into(),
        );
        let encoded = extrinsic.encode();
        opaque::UncheckedExtrinsic::decode(&mut &encoded[..])
            .expect("Encoded extrinsic is always valid")
    }
}
/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPallets,
>;
#[doc(hidden)]
mod sp_api_hidden_includes_IMPL_RUNTIME_APIS {
    pub extern crate sp_api as sp_api;
}
pub struct RuntimeApi {}
/// Implements all runtime apis for the client side.
#[cfg(any(feature = "std", test))]
pub struct RuntimeApiImpl<
    Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT,
    C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block> + 'static,
> {
    call: &'static C,
    commit_on_success: std::cell::RefCell<bool>,
    changes: std::cell::RefCell<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::OverlayedChanges,
    >,
    storage_transaction_cache: std::cell::RefCell<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StorageTransactionCache<
            Block,
            C::StateBackend,
        >,
    >,
    recorder: Option<self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ProofRecorder<Block>>,
}
#[cfg(any(feature = "std", test))]
unsafe impl<
        Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT,
        C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block>,
    > Send for RuntimeApiImpl<Block, C>
{
}
#[cfg(any(feature = "std", test))]
unsafe impl<
        Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT,
        C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block>,
    > Sync for RuntimeApiImpl<Block, C>
{
}
#[cfg(any(feature = "std", test))]
impl<
        Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT,
        C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block>,
    > self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiExt<Block>
    for RuntimeApiImpl<Block, C>
{
    type StateBackend = C::StateBackend;
    fn execute_in_transaction<
        F: FnOnce(
            &Self,
        )
            -> self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::TransactionOutcome<R>,
        R,
    >(
        &self,
        call: F,
    ) -> R
    where
        Self: Sized,
    {
        self.changes.borrow_mut().start_transaction();
        *self.commit_on_success.borrow_mut() = false;
        let res = call(self);
        *self.commit_on_success.borrow_mut() = true;
        self.commit_or_rollback(match res {
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::TransactionOutcome::Commit(
                _,
            ) => true,
            _ => false,
        });
        res.into_inner()
    }
    fn has_api<A: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::RuntimeApiInfo + ?Sized>(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<Block>,
    ) -> std::result::Result<bool, self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError>
    where
        Self: Sized,
    {
        self.call
            .runtime_version_at(at)
            .map(|v| v.has_api_with(&A::ID, |v| v == A::VERSION))
    }
    fn has_api_with<
        A: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::RuntimeApiInfo + ?Sized,
        P: Fn(u32) -> bool,
    >(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<Block>,
        pred: P,
    ) -> std::result::Result<bool, self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError>
    where
        Self: Sized,
    {
        self.call
            .runtime_version_at(at)
            .map(|v| v.has_api_with(&A::ID, pred))
    }
    fn api_version<
        A: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::RuntimeApiInfo + ?Sized,
    >(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<Block>,
    ) -> std::result::Result<
        Option<u32>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    >
    where
        Self: Sized,
    {
        self.call
            .runtime_version_at(at)
            .map(|v| v.api_version(&A::ID))
    }
    fn record_proof(&mut self) {
        self.recorder = Some(Default::default());
    }
    fn proof_recorder(
        &self,
    ) -> Option<self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ProofRecorder<Block>> {
        self.recorder.clone()
    }
    fn extract_proof(
        &mut self,
    ) -> Option<self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StorageProof> {
        self.recorder
            .take()
            .map(|recorder| recorder.to_storage_proof())
    }
    fn into_storage_changes(
        &self,
        backend: &Self::StateBackend,
        changes_trie_state: Option<
            &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ChangesTrieState<
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<Block>,
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NumberFor<Block>,
            >,
        >,
        parent_hash: Block::Hash,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StorageChanges<
            C::StateBackend,
            Block,
        >,
        String,
    >
    where
        Self: Sized,
    {
        self.changes
            .replace(Default::default())
            .into_storage_changes(
                backend,
                changes_trie_state,
                parent_hash,
                self.storage_transaction_cache.replace(Default::default()),
            )
    }
}
#[cfg(any(feature = "std", test))]
impl<Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT, C>
    self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ConstructRuntimeApi<Block, C>
    for RuntimeApi
where
    C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block> + 'static,
{
    type RuntimeApi = RuntimeApiImpl<Block, C>;
    fn construct_runtime_api<'a>(
        call: &'a C,
    ) -> self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiRef<'a, Self::RuntimeApi> {
        RuntimeApiImpl {
            call: unsafe { std::mem::transmute(call) },
            commit_on_success: true.into(),
            changes: Default::default(),
            recorder: Default::default(),
            storage_transaction_cache: Default::default(),
        }
        .into()
    }
}
#[cfg(any(feature = "std", test))]
impl<
        Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT,
        C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block>,
    > RuntimeApiImpl<Block, C>
{
    fn call_api_at<
        R: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode
            + self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Decode
            + PartialEq,
        F: FnOnce(
            &C,
            &std::cell::RefCell<
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::OverlayedChanges,
            >,
            &std::cell::RefCell<
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StorageTransactionCache<
                    Block,
                    C::StateBackend,
                >,
            >,
            &Option<self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ProofRecorder<Block>>,
        ) -> std::result::Result<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<R>,
            E,
        >,
        E,
    >(
        &self,
        call_api_at: F,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<R>,
        E,
    > {
        if *self.commit_on_success.borrow() {
            self.changes.borrow_mut().start_transaction();
        }
        let res = call_api_at(
            &self.call,
            &self.changes,
            &self.storage_transaction_cache,
            &self.recorder,
        );
        self.commit_or_rollback(res.is_ok());
        res
    }
    fn commit_or_rollback(&self, commit: bool) {
        let proof = "\
					We only close a transaction when we opened one ourself.
					Other parts of the runtime that make use of transactions (state-machine)
					also balance their transactions. The runtime cannot close client initiated
					transactions. qed";
        if *self.commit_on_success.borrow() {
            if commit {
                self.changes.borrow_mut().commit_transaction().expect(proof);
            } else {
                self.changes
                    .borrow_mut()
                    .rollback_transaction()
                    .expect(proof);
            }
        }
    }
}
impl sp_api::runtime_decl_for_Core::Core<Block> for Runtime {
    fn version() -> RuntimeVersion {
        VERSION
    }
    fn execute_block(block: Block) {
        Executive::execute_block(block)
    }
    fn initialize_block(header: &<Block as BlockT>::Header) {
        Executive::initialize_block(header)
    }
}
impl sp_api::runtime_decl_for_Metadata::Metadata<Block> for Runtime {
    fn metadata() -> OpaqueMetadata {
        Runtime::metadata().into()
    }
}
impl sp_block_builder::runtime_decl_for_BlockBuilder::BlockBuilder<Block> for Runtime {
    fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
        Executive::apply_extrinsic(extrinsic)
    }
    fn finalize_block() -> <Block as BlockT>::Header {
        Executive::finalize_block()
    }
    fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
        data.create_extrinsics()
    }
    fn check_inherents(
        block: Block,
        data: sp_inherents::InherentData,
    ) -> sp_inherents::CheckInherentsResult {
        data.check_extrinsics(&block)
    }
}
impl sp_transaction_pool :: runtime_api :: runtime_decl_for_TaggedTransactionQueue :: TaggedTransactionQueue < Block > for Runtime { fn validate_transaction (source : TransactionSource , tx : < Block as BlockT > :: Extrinsic , block_hash : < Block as BlockT > :: Hash) -> TransactionValidity { Executive :: validate_transaction (source , tx , block_hash) } }
impl sp_offchain::runtime_decl_for_OffchainWorkerApi::OffchainWorkerApi<Block> for Runtime {
    fn offchain_worker(header: &<Block as BlockT>::Header) {
        Executive::offchain_worker(header)
    }
}
impl sp_consensus_aura::runtime_decl_for_AuraApi::AuraApi<Block, AuraId> for Runtime {
    fn slot_duration() -> sp_consensus_aura::SlotDuration {
        sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
    }
    fn authorities() -> Vec<AuraId> {
        Aura::authorities().to_vec()
    }
}
impl
    frame_system_rpc_runtime_api::runtime_decl_for_AccountNonceApi::AccountNonceApi<
        Block,
        AccountId,
        Index,
    > for Runtime
{
    fn account_nonce(account: AccountId) -> Index {
        System::account_nonce(account)
    }
}
impl fp_rpc::runtime_decl_for_EthereumRuntimeRPCApi::EthereumRuntimeRPCApi<Block> for Runtime {
    fn chain_id() -> u64 {
        <Runtime as pallet_evm::Config>::ChainId::get()
    }
    fn account_basic(address: H160) -> EVMAccount {
        EVM::account_basic(&address)
    }
    fn gas_price() -> U256 {
        <Runtime as pallet_evm::Config>::FeeCalculator::min_gas_price()
    }
    fn account_code_at(address: H160) -> Vec<u8> {
        EVM::account_codes(address)
    }
    fn author() -> H160 {
        <pallet_evm::Pallet<Runtime>>::find_author()
    }
    fn storage_at(address: H160, index: U256) -> H256 {
        let mut tmp = [0u8; 32];
        index.to_big_endian(&mut tmp);
        EVM::account_storages(address, H256::from_slice(&tmp[..]))
    }
    fn call(
        from: H160,
        to: H160,
        data: Vec<u8>,
        value: U256,
        gas_limit: U256,
        gas_price: Option<U256>,
        nonce: Option<U256>,
        estimate: bool,
    ) -> Result<pallet_evm::CallInfo, sp_runtime::DispatchError> {
        let config = if estimate {
            let mut config = <Runtime as pallet_evm::Config>::config().clone();
            config.estimate = true;
            Some(config)
        } else {
            None
        };
        <Runtime as pallet_evm::Config>::Runner::call(
            from,
            to,
            data,
            value,
            gas_limit.low_u64(),
            gas_price,
            nonce,
            config
                .as_ref()
                .unwrap_or(<Runtime as pallet_evm::Config>::config()),
        )
        .map_err(|err| err.into())
    }
    fn create(
        from: H160,
        data: Vec<u8>,
        value: U256,
        gas_limit: U256,
        gas_price: Option<U256>,
        nonce: Option<U256>,
        estimate: bool,
    ) -> Result<pallet_evm::CreateInfo, sp_runtime::DispatchError> {
        let config = if estimate {
            let mut config = <Runtime as pallet_evm::Config>::config().clone();
            config.estimate = true;
            Some(config)
        } else {
            None
        };
        <Runtime as pallet_evm::Config>::Runner::create(
            from,
            data,
            value,
            gas_limit.low_u64(),
            gas_price,
            nonce,
            config
                .as_ref()
                .unwrap_or(<Runtime as pallet_evm::Config>::config()),
        )
        .map_err(|err| err.into())
    }
    fn current_transaction_statuses() -> Option<Vec<TransactionStatus>> {
        Ethereum::current_transaction_statuses()
    }
    fn current_block() -> Option<pallet_ethereum::Block> {
        Ethereum::current_block()
    }
    fn current_receipts() -> Option<Vec<pallet_ethereum::Receipt>> {
        Ethereum::current_receipts()
    }
    fn current_all() -> (
        Option<pallet_ethereum::Block>,
        Option<Vec<pallet_ethereum::Receipt>>,
        Option<Vec<TransactionStatus>>,
    ) {
        (
            Ethereum::current_block(),
            Ethereum::current_receipts(),
            Ethereum::current_transaction_statuses(),
        )
    }
    fn extrinsic_filter(xts: Vec<<Block as BlockT>::Extrinsic>) -> Vec<EthereumTransaction> {
        xts.into_iter()
            .filter_map(|xt| match xt.function {
                Call::Ethereum(transact(t)) => Some(t),
                _ => None,
            })
            .collect::<Vec<EthereumTransaction>>()
    }
}
impl pallet_transaction_payment_rpc_runtime_api :: runtime_decl_for_TransactionPaymentApi :: TransactionPaymentApi < Block , Balance > for Runtime { fn query_info (uxt : < Block as BlockT > :: Extrinsic , len : u32) -> pallet_transaction_payment_rpc_runtime_api :: RuntimeDispatchInfo < Balance > { TransactionPayment :: query_info (uxt , len) } fn query_fee_details (uxt : < Block as BlockT > :: Extrinsic , len : u32) -> pallet_transaction_payment :: FeeDetails < Balance > { TransactionPayment :: query_fee_details (uxt , len) } }
impl sp_session::runtime_decl_for_SessionKeys::SessionKeys<Block> for Runtime {
    fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
        opaque::SessionKeys::generate(seed)
    }
    fn decode_session_keys(encoded: Vec<u8>) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
        opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
    }
}
impl fg_primitives::runtime_decl_for_GrandpaApi::GrandpaApi<Block> for Runtime {
    fn grandpa_authorities() -> GrandpaAuthorityList {
        Grandpa::grandpa_authorities()
    }
    fn current_set_id() -> fg_primitives::SetId {
        Grandpa::current_set_id()
    }
    fn submit_report_equivocation_unsigned_extrinsic(
        _equivocation_proof: fg_primitives::EquivocationProof<
            <Block as BlockT>::Hash,
            NumberFor<Block>,
        >,
        _key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
    ) -> Option<()> {
        None
    }
    fn generate_key_ownership_proof(
        _set_id: fg_primitives::SetId,
        _authority_id: GrandpaId,
    ) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
        None
    }
}
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > sp_api::Core<__SR_API_BLOCK__> for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    RuntimeVersion: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    <__SR_API_BLOCK__ as BlockT>::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn Core_version_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<RuntimeVersion>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self.call_api_at(
            |call_runtime_at, changes, storage_transaction_cache, recorder| {
                sp_api::runtime_decl_for_Core::version_call_api_at(
                    call_runtime_at,
                    at,
                    params_encoded,
                    changes,
                    storage_transaction_cache,
                    params.map(|p| {
                        sp_api::runtime_decl_for_Core::version_native_call_generator::<
                            Runtime,
                            __SR_API_BLOCK__,
                            Block,
                        >()
                    }),
                    context,
                    recorder,
                )
            },
        )
    }
    fn Core_execute_block_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(__SR_API_BLOCK__)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<()>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self.call_api_at(
            |call_runtime_at, changes, storage_transaction_cache, recorder| {
                sp_api::runtime_decl_for_Core::execute_block_call_api_at(
                    call_runtime_at,
                    at,
                    params_encoded,
                    changes,
                    storage_transaction_cache,
                    params.map(|p| {
                        sp_api::runtime_decl_for_Core::execute_block_native_call_generator::<
                            Runtime,
                            __SR_API_BLOCK__,
                            Block,
                        >(p)
                    }),
                    context,
                    recorder,
                )
            },
        )
    }
    fn Core_initialize_block_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(&<__SR_API_BLOCK__ as BlockT>::Header)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<()>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self.call_api_at(
            |call_runtime_at, changes, storage_transaction_cache, recorder| {
                sp_api::runtime_decl_for_Core::initialize_block_call_api_at(
                    call_runtime_at,
                    at,
                    params_encoded,
                    changes,
                    storage_transaction_cache,
                    params.map(|p| {
                        sp_api::runtime_decl_for_Core::initialize_block_native_call_generator::<
                            Runtime,
                            __SR_API_BLOCK__,
                            Block,
                        >(p)
                    }),
                    context,
                    recorder,
                )
            },
        )
    }
}
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > sp_api::Metadata<__SR_API_BLOCK__> for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    OpaqueMetadata: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn Metadata_metadata_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<OpaqueMetadata>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self.call_api_at(
            |call_runtime_at, changes, storage_transaction_cache, recorder| {
                sp_api::runtime_decl_for_Metadata::metadata_call_api_at(
                    call_runtime_at,
                    at,
                    params_encoded,
                    changes,
                    storage_transaction_cache,
                    params.map(|p| {
                        sp_api::runtime_decl_for_Metadata::metadata_native_call_generator::<
                            Runtime,
                            __SR_API_BLOCK__,
                            Block,
                        >()
                    }),
                    context,
                    recorder,
                )
            },
        )
    }
}
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > sp_block_builder::BlockBuilder<__SR_API_BLOCK__>
    for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    <__SR_API_BLOCK__ as BlockT>::Extrinsic: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    ApplyExtrinsicResult: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    <__SR_API_BLOCK__ as BlockT>::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    sp_inherents::InherentData: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Vec<<__SR_API_BLOCK__ as BlockT>::Extrinsic>:
        std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    sp_inherents::InherentData: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    sp_inherents::CheckInherentsResult: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn BlockBuilder_apply_extrinsic_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(<__SR_API_BLOCK__ as BlockT>::Extrinsic)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            ApplyExtrinsicResult,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_block_builder :: runtime_decl_for_BlockBuilder :: apply_extrinsic_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_block_builder :: runtime_decl_for_BlockBuilder :: apply_extrinsic_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p) }) , context , recorder) })
    }
    fn BlockBuilder_finalize_block_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            <__SR_API_BLOCK__ as BlockT>::Header,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_block_builder :: runtime_decl_for_BlockBuilder :: finalize_block_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_block_builder :: runtime_decl_for_BlockBuilder :: finalize_block_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > () }) , context , recorder) })
    }
    fn BlockBuilder_inherent_extrinsics_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(sp_inherents::InherentData)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            Vec<<__SR_API_BLOCK__ as BlockT>::Extrinsic>,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_block_builder :: runtime_decl_for_BlockBuilder :: inherent_extrinsics_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_block_builder :: runtime_decl_for_BlockBuilder :: inherent_extrinsics_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p) }) , context , recorder) })
    }
    fn BlockBuilder_check_inherents_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(__SR_API_BLOCK__, sp_inherents::InherentData)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            sp_inherents::CheckInherentsResult,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_block_builder :: runtime_decl_for_BlockBuilder :: check_inherents_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_block_builder :: runtime_decl_for_BlockBuilder :: check_inherents_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p . 0 , p . 1) }) , context , recorder) })
    }
}
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > sp_transaction_pool::runtime_api::TaggedTransactionQueue<__SR_API_BLOCK__>
    for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    TransactionSource: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    <__SR_API_BLOCK__ as BlockT>::Extrinsic: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    <__SR_API_BLOCK__ as BlockT>::Hash: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    TransactionValidity: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn TaggedTransactionQueue_validate_transaction_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(
            TransactionSource,
            <__SR_API_BLOCK__ as BlockT>::Extrinsic,
            <__SR_API_BLOCK__ as BlockT>::Hash,
        )>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            TransactionValidity,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_transaction_pool :: runtime_api :: runtime_decl_for_TaggedTransactionQueue :: validate_transaction_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_transaction_pool :: runtime_api :: runtime_decl_for_TaggedTransactionQueue :: validate_transaction_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p . 0 , p . 1 , p . 2) }) , context , recorder) })
    }
}
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > sp_offchain::OffchainWorkerApi<__SR_API_BLOCK__>
    for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    <__SR_API_BLOCK__ as BlockT>::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn OffchainWorkerApi_offchain_worker_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(&<__SR_API_BLOCK__ as BlockT>::Header)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<()>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_offchain :: runtime_decl_for_OffchainWorkerApi :: offchain_worker_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_offchain :: runtime_decl_for_OffchainWorkerApi :: offchain_worker_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p) }) , context , recorder) })
    }
}
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > sp_consensus_aura::AuraApi<__SR_API_BLOCK__, AuraId>
    for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    sp_consensus_aura::SlotDuration: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Vec<AuraId>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn AuraApi_slot_duration_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            sp_consensus_aura::SlotDuration,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_consensus_aura :: runtime_decl_for_AuraApi :: slot_duration_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_consensus_aura :: runtime_decl_for_AuraApi :: slot_duration_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block , AuraId > () }) , context , recorder) })
    }
    fn AuraApi_authorities_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<Vec<AuraId>>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_consensus_aura :: runtime_decl_for_AuraApi :: authorities_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_consensus_aura :: runtime_decl_for_AuraApi :: authorities_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block , AuraId > () }) , context , recorder) })
    }
}
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > frame_system_rpc_runtime_api::AccountNonceApi<__SR_API_BLOCK__, AccountId, Index>
    for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    AccountId: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Index: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn AccountNonceApi_account_nonce_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(AccountId)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<Index>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { frame_system_rpc_runtime_api :: runtime_decl_for_AccountNonceApi :: account_nonce_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { frame_system_rpc_runtime_api :: runtime_decl_for_AccountNonceApi :: account_nonce_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block , AccountId , Index > (p) }) , context , recorder) })
    }
}
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > fp_rpc::EthereumRuntimeRPCApi<__SR_API_BLOCK__>
    for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    u64: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    H160: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    EVMAccount: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    U256: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    H160: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Vec<u8>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    H160: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    H160: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    U256: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    H256: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    H160: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    H160: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Vec<u8>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    U256: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    U256: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Option<U256>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Option<U256>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    bool: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Result<pallet_evm::CallInfo, sp_runtime::DispatchError>:
        std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    H160: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Vec<u8>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    U256: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    U256: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Option<U256>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Option<U256>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    bool: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Result<pallet_evm::CreateInfo, sp_runtime::DispatchError>:
        std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Option<Vec<TransactionStatus>>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Option<pallet_ethereum::Block>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Option<Vec<pallet_ethereum::Receipt>>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    (
        Option<pallet_ethereum::Block>,
        Option<Vec<pallet_ethereum::Receipt>>,
        Option<Vec<TransactionStatus>>,
    ): std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Vec<<__SR_API_BLOCK__ as BlockT>::Extrinsic>:
        std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Vec<EthereumTransaction>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn EthereumRuntimeRPCApi_chain_id_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<u64>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: chain_id_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: chain_id_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > () }) , context , recorder) })
    }
    fn EthereumRuntimeRPCApi_account_basic_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(H160)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<EVMAccount>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: account_basic_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: account_basic_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p) }) , context , recorder) })
    }
    fn EthereumRuntimeRPCApi_gas_price_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<U256>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: gas_price_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: gas_price_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > () }) , context , recorder) })
    }
    fn EthereumRuntimeRPCApi_account_code_at_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(H160)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<Vec<u8>>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: account_code_at_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: account_code_at_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p) }) , context , recorder) })
    }
    fn EthereumRuntimeRPCApi_author_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<H160>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: author_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: author_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > () }) , context , recorder) })
    }
    fn EthereumRuntimeRPCApi_storage_at_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(H160, U256)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<H256>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: storage_at_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: storage_at_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p . 0 , p . 1) }) , context , recorder) })
    }
    fn EthereumRuntimeRPCApi_call_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(
            H160,
            H160,
            Vec<u8>,
            U256,
            U256,
            Option<U256>,
            Option<U256>,
            bool,
        )>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            Result<pallet_evm::CallInfo, sp_runtime::DispatchError>,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: call_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: call_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p . 0 , p . 1 , p . 2 , p . 3 , p . 4 , p . 5 , p . 6 , p . 7) }) , context , recorder) })
    }
    fn EthereumRuntimeRPCApi_create_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(H160, Vec<u8>, U256, U256, Option<U256>, Option<U256>, bool)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            Result<pallet_evm::CreateInfo, sp_runtime::DispatchError>,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: create_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: create_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p . 0 , p . 1 , p . 2 , p . 3 , p . 4 , p . 5 , p . 6) }) , context , recorder) })
    }
    fn EthereumRuntimeRPCApi_current_transaction_statuses_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            Option<Vec<TransactionStatus>>,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: current_transaction_statuses_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: current_transaction_statuses_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > () }) , context , recorder) })
    }
    fn EthereumRuntimeRPCApi_current_block_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            Option<pallet_ethereum::Block>,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: current_block_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: current_block_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > () }) , context , recorder) })
    }
    fn EthereumRuntimeRPCApi_current_receipts_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            Option<Vec<pallet_ethereum::Receipt>>,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: current_receipts_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: current_receipts_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > () }) , context , recorder) })
    }
    fn EthereumRuntimeRPCApi_current_all_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<(
            Option<pallet_ethereum::Block>,
            Option<Vec<pallet_ethereum::Receipt>>,
            Option<Vec<TransactionStatus>>,
        )>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: current_all_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: current_all_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > () }) , context , recorder) })
    }
    fn EthereumRuntimeRPCApi_extrinsic_filter_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(Vec<<__SR_API_BLOCK__ as BlockT>::Extrinsic>)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            Vec<EthereumTransaction>,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: extrinsic_filter_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: extrinsic_filter_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p) }) , context , recorder) })
    }
}
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<__SR_API_BLOCK__, Balance>
    for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    <__SR_API_BLOCK__ as BlockT>::Extrinsic: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    u32: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance>:
        std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    <__SR_API_BLOCK__ as BlockT>::Extrinsic: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    u32: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    pallet_transaction_payment::FeeDetails<Balance>:
        std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn TransactionPaymentApi_query_info_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(<__SR_API_BLOCK__ as BlockT>::Extrinsic, u32)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance>,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { pallet_transaction_payment_rpc_runtime_api :: runtime_decl_for_TransactionPaymentApi :: query_info_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { pallet_transaction_payment_rpc_runtime_api :: runtime_decl_for_TransactionPaymentApi :: query_info_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block , Balance > (p . 0 , p . 1) }) , context , recorder) })
    }
    fn TransactionPaymentApi_query_fee_details_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(<__SR_API_BLOCK__ as BlockT>::Extrinsic, u32)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            pallet_transaction_payment::FeeDetails<Balance>,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { pallet_transaction_payment_rpc_runtime_api :: runtime_decl_for_TransactionPaymentApi :: query_fee_details_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { pallet_transaction_payment_rpc_runtime_api :: runtime_decl_for_TransactionPaymentApi :: query_fee_details_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block , Balance > (p . 0 , p . 1) }) , context , recorder) })
    }
}
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > sp_session::SessionKeys<__SR_API_BLOCK__>
    for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    Option<Vec<u8>>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Vec<u8>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Vec<u8>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Option<Vec<(Vec<u8>, KeyTypeId)>>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn SessionKeys_generate_session_keys_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(Option<Vec<u8>>)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<Vec<u8>>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_session :: runtime_decl_for_SessionKeys :: generate_session_keys_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_session :: runtime_decl_for_SessionKeys :: generate_session_keys_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p) }) , context , recorder) })
    }
    fn SessionKeys_decode_session_keys_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(Vec<u8>)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            Option<Vec<(Vec<u8>, KeyTypeId)>>,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_session :: runtime_decl_for_SessionKeys :: decode_session_keys_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_session :: runtime_decl_for_SessionKeys :: decode_session_keys_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p) }) , context , recorder) })
    }
}
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > fg_primitives::GrandpaApi<__SR_API_BLOCK__>
    for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    GrandpaAuthorityList: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    fg_primitives::SetId: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    fg_primitives::EquivocationProof<
        <__SR_API_BLOCK__ as BlockT>::Hash,
        NumberFor<__SR_API_BLOCK__>,
    >: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    fg_primitives::OpaqueKeyOwnershipProof: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Option<()>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    fg_primitives::SetId: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    GrandpaId: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Option<fg_primitives::OpaqueKeyOwnershipProof>:
        std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn GrandpaApi_grandpa_authorities_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            GrandpaAuthorityList,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { fg_primitives :: runtime_decl_for_GrandpaApi :: grandpa_authorities_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { fg_primitives :: runtime_decl_for_GrandpaApi :: grandpa_authorities_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > () }) , context , recorder) })
    }
    fn GrandpaApi_current_set_id_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            fg_primitives::SetId,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { fg_primitives :: runtime_decl_for_GrandpaApi :: current_set_id_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { fg_primitives :: runtime_decl_for_GrandpaApi :: current_set_id_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > () }) , context , recorder) })
    }
    fn GrandpaApi_submit_report_equivocation_unsigned_extrinsic_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(
            fg_primitives::EquivocationProof<
                <__SR_API_BLOCK__ as BlockT>::Hash,
                NumberFor<__SR_API_BLOCK__>,
            >,
            fg_primitives::OpaqueKeyOwnershipProof,
        )>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<Option<()>>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { fg_primitives :: runtime_decl_for_GrandpaApi :: submit_report_equivocation_unsigned_extrinsic_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { fg_primitives :: runtime_decl_for_GrandpaApi :: submit_report_equivocation_unsigned_extrinsic_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p . 0 , p . 1) }) , context , recorder) })
    }
    fn GrandpaApi_generate_key_ownership_proof_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(fg_primitives::SetId, GrandpaId)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            Option<fg_primitives::OpaqueKeyOwnershipProof>,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { fg_primitives :: runtime_decl_for_GrandpaApi :: generate_key_ownership_proof_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { fg_primitives :: runtime_decl_for_GrandpaApi :: generate_key_ownership_proof_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p . 0 , p . 1) }) , context , recorder) })
    }
}
const RUNTIME_API_VERSIONS : self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: ApisVec = :: sp_version :: sp_std :: borrow :: Cow :: Borrowed (& [(sp_api :: runtime_decl_for_Core :: ID , sp_api :: runtime_decl_for_Core :: VERSION) , (sp_api :: runtime_decl_for_Metadata :: ID , sp_api :: runtime_decl_for_Metadata :: VERSION) , (sp_block_builder :: runtime_decl_for_BlockBuilder :: ID , sp_block_builder :: runtime_decl_for_BlockBuilder :: VERSION) , (sp_transaction_pool :: runtime_api :: runtime_decl_for_TaggedTransactionQueue :: ID , sp_transaction_pool :: runtime_api :: runtime_decl_for_TaggedTransactionQueue :: VERSION) , (sp_offchain :: runtime_decl_for_OffchainWorkerApi :: ID , sp_offchain :: runtime_decl_for_OffchainWorkerApi :: VERSION) , (sp_consensus_aura :: runtime_decl_for_AuraApi :: ID , sp_consensus_aura :: runtime_decl_for_AuraApi :: VERSION) , (frame_system_rpc_runtime_api :: runtime_decl_for_AccountNonceApi :: ID , frame_system_rpc_runtime_api :: runtime_decl_for_AccountNonceApi :: VERSION) , (fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: ID , fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: VERSION) , (pallet_transaction_payment_rpc_runtime_api :: runtime_decl_for_TransactionPaymentApi :: ID , pallet_transaction_payment_rpc_runtime_api :: runtime_decl_for_TransactionPaymentApi :: VERSION) , (sp_session :: runtime_decl_for_SessionKeys :: ID , sp_session :: runtime_decl_for_SessionKeys :: VERSION) , (fg_primitives :: runtime_decl_for_GrandpaApi :: ID , fg_primitives :: runtime_decl_for_GrandpaApi :: VERSION)]) ;
const _: () = {};
const _: () = {};
const _: () = {};
const _: () = {};
const _: () = {};
const _: () = {};
const _: () = {};
const _: () = {};
const _: () = {};
const _: () = {};
const _: () = {};
pub mod api {
    use super::*;
    #[cfg(feature = "std")]
    pub fn dispatch(method: &str, mut __sp_api__input_data: &[u8]) -> Option<Vec<u8>> {
        match method {
            "Core_version" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let () : () = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "version" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    #[allow(deprecated)]
                    <Runtime as sp_api::runtime_decl_for_Core::Core<Block>>::version()
                }),
            ),
            "Core_execute_block" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (block) : (Block) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "execute_block" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    #[allow(deprecated)]
                    <Runtime as sp_api::runtime_decl_for_Core::Core<Block>>::execute_block(block)
                }),
            ),
            "Core_initialize_block" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (header) : (< Block as BlockT > :: Header) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "initialize_block" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    #[allow(deprecated)]
                    <Runtime as sp_api::runtime_decl_for_Core::Core<Block>>::initialize_block(
                        &header,
                    )
                }),
            ),
            "Metadata_metadata" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let () : () = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "metadata" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    #[allow(deprecated)]
                    <Runtime as sp_api::runtime_decl_for_Metadata::Metadata<Block>>::metadata()
                }),
            ),
            "BlockBuilder_apply_extrinsic" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (extrinsic) : (< Block as BlockT > :: Extrinsic) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "apply_extrinsic" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    #[allow(deprecated)]
                    <Runtime as sp_block_builder::runtime_decl_for_BlockBuilder::BlockBuilder<
                        Block,
                    >>::apply_extrinsic(extrinsic)
                }),
            ),
            "BlockBuilder_finalize_block" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let () : () = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "finalize_block" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    #[allow(deprecated)]
                    <Runtime as sp_block_builder::runtime_decl_for_BlockBuilder::BlockBuilder<
                        Block,
                    >>::finalize_block()
                }),
            ),
            "BlockBuilder_inherent_extrinsics" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (data) : (sp_inherents :: InherentData) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "inherent_extrinsics" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    #[allow(deprecated)]
                    <Runtime as sp_block_builder::runtime_decl_for_BlockBuilder::BlockBuilder<
                        Block,
                    >>::inherent_extrinsics(data)
                }),
            ),
            "BlockBuilder_check_inherents" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (block , data) : (Block , sp_inherents :: InherentData) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "check_inherents" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    #[allow(deprecated)]
                    <Runtime as sp_block_builder::runtime_decl_for_BlockBuilder::BlockBuilder<
                        Block,
                    >>::check_inherents(block, data)
                }),
            ),
            "TaggedTransactionQueue_validate_transaction" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (source , tx , block_hash) : (TransactionSource , < Block as BlockT > :: Extrinsic , < Block as BlockT > :: Hash) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "validate_transaction" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as sp_transaction_pool :: runtime_api :: runtime_decl_for_TaggedTransactionQueue :: TaggedTransactionQueue < Block > > :: validate_transaction (source , tx , block_hash)
                }),
            ),
            "OffchainWorkerApi_offchain_worker" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (header) : (< Block as BlockT > :: Header) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "offchain_worker" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as sp_offchain :: runtime_decl_for_OffchainWorkerApi :: OffchainWorkerApi < Block > > :: offchain_worker (& header)
                }),
            ),
            "AuraApi_slot_duration" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let () : () = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "slot_duration" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    #[allow(deprecated)]
                    <Runtime as sp_consensus_aura::runtime_decl_for_AuraApi::AuraApi<
                        Block,
                        AuraId,
                    >>::slot_duration()
                }),
            ),
            "AuraApi_authorities" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let () : () = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "authorities" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    #[allow(deprecated)]
                    <Runtime as sp_consensus_aura::runtime_decl_for_AuraApi::AuraApi<
                        Block,
                        AuraId,
                    >>::authorities()
                }),
            ),
            "AccountNonceApi_account_nonce" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (account) : (AccountId) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "account_nonce" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as frame_system_rpc_runtime_api :: runtime_decl_for_AccountNonceApi :: AccountNonceApi < Block , AccountId , Index > > :: account_nonce (account)
                }),
            ),
            "EthereumRuntimeRPCApi_chain_id" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let () : () = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "chain_id" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: EthereumRuntimeRPCApi < Block > > :: chain_id ()
                }),
            ),
            "EthereumRuntimeRPCApi_account_basic" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (address) : (H160) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "account_basic" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: EthereumRuntimeRPCApi < Block > > :: account_basic (address)
                }),
            ),
            "EthereumRuntimeRPCApi_gas_price" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let () : () = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "gas_price" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: EthereumRuntimeRPCApi < Block > > :: gas_price ()
                }),
            ),
            "EthereumRuntimeRPCApi_account_code_at" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (address) : (H160) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "account_code_at" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: EthereumRuntimeRPCApi < Block > > :: account_code_at (address)
                }),
            ),
            "EthereumRuntimeRPCApi_author" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let () : () = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "author" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: EthereumRuntimeRPCApi < Block > > :: author ()
                }),
            ),
            "EthereumRuntimeRPCApi_storage_at" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (address , index) : (H160 , U256) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "storage_at" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: EthereumRuntimeRPCApi < Block > > :: storage_at (address , index)
                }),
            ),
            "EthereumRuntimeRPCApi_call" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (from , to , data , value , gas_limit , gas_price , nonce , estimate) : (H160 , H160 , Vec < u8 > , U256 , U256 , Option < U256 > , Option < U256 > , bool) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "call" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: EthereumRuntimeRPCApi < Block > > :: call (from , to , data , value , gas_limit , gas_price , nonce , estimate)
                }),
            ),
            "EthereumRuntimeRPCApi_create" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (from , data , value , gas_limit , gas_price , nonce , estimate) : (H160 , Vec < u8 > , U256 , U256 , Option < U256 > , Option < U256 > , bool) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "create" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: EthereumRuntimeRPCApi < Block > > :: create (from , data , value , gas_limit , gas_price , nonce , estimate)
                }),
            ),
            "EthereumRuntimeRPCApi_current_transaction_statuses" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let () : () = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "current_transaction_statuses" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: EthereumRuntimeRPCApi < Block > > :: current_transaction_statuses ()
                }),
            ),
            "EthereumRuntimeRPCApi_current_block" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let () : () = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "current_block" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: EthereumRuntimeRPCApi < Block > > :: current_block ()
                }),
            ),
            "EthereumRuntimeRPCApi_current_receipts" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let () : () = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "current_receipts" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: EthereumRuntimeRPCApi < Block > > :: current_receipts ()
                }),
            ),
            "EthereumRuntimeRPCApi_current_all" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let () : () = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "current_all" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: EthereumRuntimeRPCApi < Block > > :: current_all ()
                }),
            ),
            "EthereumRuntimeRPCApi_extrinsic_filter" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (xts) : (Vec < < Block as BlockT > :: Extrinsic >) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "extrinsic_filter" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as fp_rpc :: runtime_decl_for_EthereumRuntimeRPCApi :: EthereumRuntimeRPCApi < Block > > :: extrinsic_filter (xts)
                }),
            ),
            "TransactionPaymentApi_query_info" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (uxt , len) : (< Block as BlockT > :: Extrinsic , u32) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "query_info" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as pallet_transaction_payment_rpc_runtime_api :: runtime_decl_for_TransactionPaymentApi :: TransactionPaymentApi < Block , Balance > > :: query_info (uxt , len)
                }),
            ),
            "TransactionPaymentApi_query_fee_details" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (uxt , len) : (< Block as BlockT > :: Extrinsic , u32) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "query_fee_details" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as pallet_transaction_payment_rpc_runtime_api :: runtime_decl_for_TransactionPaymentApi :: TransactionPaymentApi < Block , Balance > > :: query_fee_details (uxt , len)
                }),
            ),
            "SessionKeys_generate_session_keys" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (seed) : (Option < Vec < u8 > >) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "generate_session_keys" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as sp_session :: runtime_decl_for_SessionKeys :: SessionKeys < Block > > :: generate_session_keys (seed)
                }),
            ),
            "SessionKeys_decode_session_keys" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (encoded) : (Vec < u8 >) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "decode_session_keys" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as sp_session :: runtime_decl_for_SessionKeys :: SessionKeys < Block > > :: decode_session_keys (encoded)
                }),
            ),
            "GrandpaApi_grandpa_authorities" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let () : () = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "grandpa_authorities" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as fg_primitives :: runtime_decl_for_GrandpaApi :: GrandpaApi < Block > > :: grandpa_authorities ()
                }),
            ),
            "GrandpaApi_current_set_id" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let () : () = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "current_set_id" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as fg_primitives :: runtime_decl_for_GrandpaApi :: GrandpaApi < Block > > :: current_set_id ()
                }),
            ),
            "GrandpaApi_submit_report_equivocation_unsigned_extrinsic" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (_equivocation_proof , _key_owner_proof) : (fg_primitives :: EquivocationProof < < Block as BlockT > :: Hash , NumberFor < Block > > , fg_primitives :: OpaqueKeyOwnershipProof) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "submit_report_equivocation_unsigned_extrinsic" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as fg_primitives :: runtime_decl_for_GrandpaApi :: GrandpaApi < Block > > :: submit_report_equivocation_unsigned_extrinsic (_equivocation_proof , _key_owner_proof)
                }),
            ),
            "GrandpaApi_generate_key_ownership_proof" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (_set_id , _authority_id) : (fg_primitives :: SetId , GrandpaId) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & __sp_api__input_data) { Ok (res) => res , Err (e) => { :: std :: rt :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & match (& "generate_key_ownership_proof" , & e) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , })) } } ;
                    # [allow (deprecated)] < Runtime as fg_primitives :: runtime_decl_for_GrandpaApi :: GrandpaApi < Block > > :: generate_key_ownership_proof (_set_id , _authority_id)
                }),
            ),
            _ => None,
        }
    }
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&runtime_integrity_tests])
}
