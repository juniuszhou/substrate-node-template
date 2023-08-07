#![feature(prelude_import)]
#![recursion_limit = "256"]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern std;
use pallet_grandpa::AuthorityId as GrandpaId;
use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{
        AccountIdLookup, BlakeTwo256, Block as BlockT, IdentifyAccount, NumberFor, One,
        Verify,
    },
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, MultiSignature,
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;
pub use frame_support::{
    construct_runtime, parameter_types,
    traits::{
        ConstU128, ConstU32, ConstU64, ConstU8, KeyOwnerProofSystem, Randomness,
        StorageInfo,
    },
    weights::{
        constants::{
            BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight,
            WEIGHT_REF_TIME_PER_SECOND,
        },
        IdentityFee, Weight,
    },
    StorageValue,
};
pub use frame_system::Call as SystemCall;
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::{ConstFeeMultiplier, CurrencyAdapter, Multiplier};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};
/// Import the template pallet.
pub use pallet_template;
/// An index to a block.
pub type BlockNumber = u32;
/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;
/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
/// Balance of an account.
pub type Balance = u128;
/// Index of a transaction in the chain.
pub type Index = u32;
/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;
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
    impl ::core::clone::Clone for SessionKeys {
        #[inline]
        fn clone(&self) -> SessionKeys {
            SessionKeys {
                aura: ::core::clone::Clone::clone(&self.aura),
                grandpa: ::core::clone::Clone::clone(&self.grandpa),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SessionKeys {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SessionKeys {
        #[inline]
        fn eq(&self, other: &SessionKeys) -> bool {
            self.aura == other.aura && self.grandpa == other.grandpa
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for SessionKeys {}
    #[automatically_derived]
    impl ::core::cmp::Eq for SessionKeys {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                <Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
            >;
        }
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::codec::Encode for SessionKeys {
            fn size_hint(&self) -> usize {
                0_usize
                    .saturating_add(::codec::Encode::size_hint(&self.aura))
                    .saturating_add(::codec::Encode::size_hint(&self.grandpa))
            }
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                ::codec::Encode::encode_to(&self.aura, __codec_dest_edqy);
                ::codec::Encode::encode_to(&self.grandpa, __codec_dest_edqy);
            }
        }
        #[automatically_derived]
        impl ::codec::EncodeLike for SessionKeys {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::codec::Decode for SessionKeys {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                ::core::result::Result::Ok(SessionKeys {
                    aura: {
                        let __codec_res_edqy = <<Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public as ::codec::Decode>::decode(
                            __codec_input_edqy,
                        );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `SessionKeys::aura`"),
                                );
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                __codec_res_edqy
                            }
                        }
                    },
                    grandpa: {
                        let __codec_res_edqy = <<Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public as ::codec::Decode>::decode(
                            __codec_input_edqy,
                        );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `SessionKeys::grandpa`"),
                                );
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                __codec_res_edqy
                            }
                        }
                    },
                })
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for SessionKeys {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(
                        ::scale_info::Path::new(
                            "SessionKeys",
                            "node_template_runtime::opaque",
                        ),
                    )
                    .type_params(::alloc::vec::Vec::new())
                    .composite(
                        ::scale_info::build::Fields::named()
                            .field(|f| {
                                f
                                    .ty::<
                                        <Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                                    >()
                                    .name("aura")
                                    .type_name(
                                        "<Aura as $crate::BoundToRuntimeAppPublic>::Public",
                                    )
                            })
                            .field(|f| {
                                f
                                    .ty::<
                                        <Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                                    >()
                                    .name("grandpa")
                                    .type_name(
                                        "<Grandpa as $crate::BoundToRuntimeAppPublic>::Public",
                                    )
                            }),
                    )
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
            ) -> __opaque_keys_serde_import__SessionKeys::__private::Result<
                __S::Ok,
                __S::Error,
            >
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
        impl<'de> __opaque_keys_serde_import__SessionKeys::Deserialize<'de>
        for SessionKeys {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> __opaque_keys_serde_import__SessionKeys::__private::Result<
                Self,
                __D::Error,
            >
            where
                __D: __opaque_keys_serde_import__SessionKeys::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
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
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
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
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct SessionKeys",
                        )
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
                        >(&mut __seq) {
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
                                        &"struct SessionKeys with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            <Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct SessionKeys with 2 elements",
                                    ),
                                );
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
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("aura"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            <Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                                        >(&mut __map) {
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
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            <Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
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
                #[doc(hidden)]
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
            let keys = Self {
                aura: <<Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public as ::sp_runtime::RuntimeAppPublic>::generate_pair(
                    seed.clone(),
                ),
                grandpa: <<Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public as ::sp_runtime::RuntimeAppPublic>::generate_pair(
                    seed.clone(),
                ),
            };
            ::sp_runtime::codec::Encode::encode(&keys)
        }
        /// Converts `Self` into a `Vec` of `(raw public key, KeyTypeId)`.
        pub fn into_raw_public_keys(
            self,
        ) -> ::sp_runtime::sp_std::vec::Vec<
            (::sp_runtime::sp_std::vec::Vec<u8>, ::sp_runtime::KeyTypeId),
        > {
            let mut keys = Vec::new();
            keys.push((
                ::sp_runtime::RuntimeAppPublic::to_raw_vec(&self.aura),
                <<Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public as ::sp_runtime::RuntimeAppPublic>::ID,
            ));
            keys.push((
                ::sp_runtime::RuntimeAppPublic::to_raw_vec(&self.grandpa),
                <<Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public as ::sp_runtime::RuntimeAppPublic>::ID,
            ));
            keys
        }
        /// Decode `Self` from the given `encoded` slice and convert `Self` into the raw public
        /// keys (see [`Self::into_raw_public_keys`]).
        ///
        /// Returns `None` when the decoding failed, otherwise `Some(_)`.
        pub fn decode_into_raw_public_keys(
            encoded: &[u8],
        ) -> Option<
            ::sp_runtime::sp_std::vec::Vec<
                (::sp_runtime::sp_std::vec::Vec<u8>, ::sp_runtime::KeyTypeId),
            >,
        > {
            <Self as ::sp_runtime::codec::Decode>::decode(&mut &encoded[..])
                .ok()
                .map(|s| s.into_raw_public_keys())
        }
    }
    impl ::sp_runtime::traits::OpaqueKeys for SessionKeys {
        type KeyTypeIdProviders = (Aura, Grandpa);
        fn key_ids() -> &'static [::sp_runtime::KeyTypeId] {
            &[
                <<Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public as ::sp_runtime::RuntimeAppPublic>::ID,
                <<Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public as ::sp_runtime::RuntimeAppPublic>::ID,
            ]
        }
        fn get_raw(&self, i: ::sp_runtime::KeyTypeId) -> &[u8] {
            match i {
                i if i
                    == <<Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public as ::sp_runtime::RuntimeAppPublic>::ID => {
                    self.aura.as_ref()
                }
                i if i
                    == <<Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public as ::sp_runtime::RuntimeAppPublic>::ID => {
                    self.grandpa.as_ref()
                }
                _ => &[],
            }
        }
    }
}
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: { ::sp_runtime::RuntimeString::Borrowed("node-template") },
    impl_name: { ::sp_runtime::RuntimeString::Borrowed("node-template") },
    authoring_version: 1,
    spec_version: 100,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};
const _: () = {};
/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 6000;
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;
/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
pub struct BlockHashCount;
impl BlockHashCount {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        2400
    }
}
impl<_I: From<BlockNumber>> ::frame_support::traits::Get<_I> for BlockHashCount {
    fn get() -> _I {
        _I::from(Self::get())
    }
}
impl ::frame_support::traits::TypedGet for BlockHashCount {
    type Type = BlockNumber;
    fn get() -> BlockNumber {
        Self::get()
    }
}
pub struct Version;
impl Version {
    /// Returns the value of this parameter type.
    pub const fn get() -> RuntimeVersion {
        VERSION
    }
}
impl<_I: From<RuntimeVersion>> ::frame_support::traits::Get<_I> for Version {
    fn get() -> _I {
        _I::from(Self::get())
    }
}
impl ::frame_support::traits::TypedGet for Version {
    type Type = RuntimeVersion;
    fn get() -> RuntimeVersion {
        Self::get()
    }
}
/// We allow for 2 seconds of compute with a 6 second average block time.
pub struct BlockWeights;
impl BlockWeights {
    /// Returns the value of this parameter type.
    pub fn get() -> frame_system::limits::BlockWeights {
        frame_system::limits::BlockWeights::with_sensible_defaults(
            Weight::from_parts(2u64 * WEIGHT_REF_TIME_PER_SECOND, u64::MAX),
            NORMAL_DISPATCH_RATIO,
        )
    }
}
impl<_I: From<frame_system::limits::BlockWeights>> ::frame_support::traits::Get<_I>
for BlockWeights {
    fn get() -> _I {
        _I::from(Self::get())
    }
}
impl ::frame_support::traits::TypedGet for BlockWeights {
    type Type = frame_system::limits::BlockWeights;
    fn get() -> frame_system::limits::BlockWeights {
        Self::get()
    }
}
pub struct BlockLength;
impl BlockLength {
    /// Returns the value of this parameter type.
    pub fn get() -> frame_system::limits::BlockLength {
        frame_system::limits::BlockLength::max_with_normal_ratio(
            5 * 1024 * 1024,
            NORMAL_DISPATCH_RATIO,
        )
    }
}
impl<_I: From<frame_system::limits::BlockLength>> ::frame_support::traits::Get<_I>
for BlockLength {
    fn get() -> _I {
        _I::from(Self::get())
    }
}
impl ::frame_support::traits::TypedGet for BlockLength {
    type Type = frame_system::limits::BlockLength;
    fn get() -> frame_system::limits::BlockLength {
        Self::get()
    }
}
pub struct SS58Prefix;
impl SS58Prefix {
    /// Returns the value of this parameter type.
    pub const fn get() -> u8 {
        42
    }
}
impl<_I: From<u8>> ::frame_support::traits::Get<_I> for SS58Prefix {
    fn get() -> _I {
        _I::from(Self::get())
    }
}
impl ::frame_support::traits::TypedGet for SS58Prefix {
    type Type = u8;
    fn get() -> u8 {
        Self::get()
    }
}
impl frame_system::Config for Runtime {
    /// The basic call filter to use in dispatchable.
    type BaseCallFilter = frame_support::traits::Everything;
    /// Block & extrinsics weights: base values and limits.
    type BlockWeights = BlockWeights;
    /// The maximum length of a block (in bytes).
    type BlockLength = BlockLength;
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The aggregated dispatch type that is available for extrinsics.
    type RuntimeCall = RuntimeCall;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = AccountIdLookup<AccountId, ()>;
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
    type RuntimeEvent = RuntimeEvent;
    /// The ubiquitous origin type.
    type RuntimeOrigin = RuntimeOrigin;
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
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}
impl pallet_aura::Config for Runtime {
    type AuthorityId = AuraId;
    type DisabledValidators = ();
    type MaxAuthorities = ConstU32<32>;
}
impl pallet_grandpa::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxAuthorities = ConstU32<32>;
    type MaxSetIdSessionEntries = ConstU64<0>;
    type KeyOwnerProof = sp_core::Void;
    type EquivocationReportSystem = ();
}
impl pallet_timestamp::Config for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = Aura;
    type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
    type WeightInfo = ();
}
/// Existential deposit.
pub const EXISTENTIAL_DEPOSIT: u128 = 500;
impl pallet_balances::Config for Runtime {
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type HoldIdentifier = ();
    type MaxHolds = ();
}
pub struct FeeMultiplier;
impl FeeMultiplier {
    /// Returns the value of this parameter type.
    pub fn get() -> Multiplier {
        Multiplier::one()
    }
}
impl<_I: From<Multiplier>> ::frame_support::traits::Get<_I> for FeeMultiplier {
    fn get() -> _I {
        _I::from(Self::get())
    }
}
impl ::frame_support::traits::TypedGet for FeeMultiplier {
    type Type = Multiplier;
    fn get() -> Multiplier {
        Self::get()
    }
}
impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
    type OperationalFeeMultiplier = ConstU8<5>;
    type WeightToFee = IdentityFee<Balance>;
    type LengthToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ConstFeeMultiplier<FeeMultiplier>;
}
impl pallet_sudo::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
}
/// Configure the pallet-template in pallets/template.
impl pallet_template::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_template::weights::SubstrateWeight<Runtime>;
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
impl ::core::clone::Clone for Runtime {
    #[inline]
    fn clone(&self) -> Runtime {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Runtime {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Runtime {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Runtime {
    #[inline]
    fn eq(&self, other: &Runtime) -> bool {
        true
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for Runtime {}
#[automatically_derived]
impl ::core::cmp::Eq for Runtime {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
impl core::fmt::Debug for Runtime {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_tuple("Runtime").finish()
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for Runtime {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(::scale_info::Path::new("Runtime", "node_template_runtime"))
                .type_params(::alloc::vec::Vec::new())
                .composite(::scale_info::build::Fields::unit())
        }
    }
};
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::GetNodeBlockType
for Runtime {
    type NodeBlock = opaque::Block;
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::GetRuntimeBlockType
for Runtime {
    type RuntimeBlock = Block;
}
#[doc(hidden)]
trait InternalConstructRuntime {
    #[inline(always)]
    fn runtime_metadata(
        &self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::vec::Vec<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata_ir::RuntimeApiMetadataIR,
    > {
        Default::default()
    }
}
#[doc(hidden)]
impl InternalConstructRuntime for &Runtime {}
#[allow(non_camel_case_types)]
pub enum RuntimeEvent {
    #[codec(index = 0u8)]
    System(frame_system::Event<Runtime>),
    #[codec(index = 3u8)]
    Grandpa(pallet_grandpa::Event),
    #[codec(index = 4u8)]
    Balances(pallet_balances::Event<Runtime>),
    #[codec(index = 5u8)]
    TransactionPayment(pallet_transaction_payment::Event<Runtime>),
    #[codec(index = 6u8)]
    Sudo(pallet_sudo::Event<Runtime>),
    #[codec(index = 7u8)]
    TemplateModule(pallet_template::Event<Runtime>),
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::clone::Clone for RuntimeEvent {
    #[inline]
    fn clone(&self) -> RuntimeEvent {
        match self {
            RuntimeEvent::System(__self_0) => {
                RuntimeEvent::System(::core::clone::Clone::clone(__self_0))
            }
            RuntimeEvent::Grandpa(__self_0) => {
                RuntimeEvent::Grandpa(::core::clone::Clone::clone(__self_0))
            }
            RuntimeEvent::Balances(__self_0) => {
                RuntimeEvent::Balances(::core::clone::Clone::clone(__self_0))
            }
            RuntimeEvent::TransactionPayment(__self_0) => {
                RuntimeEvent::TransactionPayment(::core::clone::Clone::clone(__self_0))
            }
            RuntimeEvent::Sudo(__self_0) => {
                RuntimeEvent::Sudo(::core::clone::Clone::clone(__self_0))
            }
            RuntimeEvent::TemplateModule(__self_0) => {
                RuntimeEvent::TemplateModule(::core::clone::Clone::clone(__self_0))
            }
        }
    }
}
#[allow(non_camel_case_types)]
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for RuntimeEvent {}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::cmp::PartialEq for RuntimeEvent {
    #[inline]
    fn eq(&self, other: &RuntimeEvent) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
            && match (self, other) {
                (RuntimeEvent::System(__self_0), RuntimeEvent::System(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                (RuntimeEvent::Grandpa(__self_0), RuntimeEvent::Grandpa(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                (RuntimeEvent::Balances(__self_0), RuntimeEvent::Balances(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                (
                    RuntimeEvent::TransactionPayment(__self_0),
                    RuntimeEvent::TransactionPayment(__arg1_0),
                ) => *__self_0 == *__arg1_0,
                (RuntimeEvent::Sudo(__self_0), RuntimeEvent::Sudo(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                (
                    RuntimeEvent::TemplateModule(__self_0),
                    RuntimeEvent::TemplateModule(__arg1_0),
                ) => *__self_0 == *__arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() }
            }
    }
}
#[allow(non_camel_case_types)]
#[automatically_derived]
impl ::core::marker::StructuralEq for RuntimeEvent {}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::cmp::Eq for RuntimeEvent {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<frame_system::Event<Runtime>>;
        let _: ::core::cmp::AssertParamIsEq<pallet_grandpa::Event>;
        let _: ::core::cmp::AssertParamIsEq<pallet_balances::Event<Runtime>>;
        let _: ::core::cmp::AssertParamIsEq<pallet_transaction_payment::Event<Runtime>>;
        let _: ::core::cmp::AssertParamIsEq<pallet_sudo::Event<Runtime>>;
        let _: ::core::cmp::AssertParamIsEq<pallet_template::Event<Runtime>>;
    }
}
#[allow(deprecated)]
const _: () = {
    #[allow(non_camel_case_types)]
    #[automatically_derived]
    impl ::codec::Encode for RuntimeEvent {
        fn size_hint(&self) -> usize {
            1_usize
                + match *self {
                    RuntimeEvent::System(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    RuntimeEvent::Grandpa(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    RuntimeEvent::Balances(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    RuntimeEvent::TransactionPayment(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    RuntimeEvent::Sudo(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    RuntimeEvent::TemplateModule(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    _ => 0_usize,
                }
        }
        fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                RuntimeEvent::System(ref aa) => {
                    __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                RuntimeEvent::Grandpa(ref aa) => {
                    __codec_dest_edqy.push_byte(3u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                RuntimeEvent::Balances(ref aa) => {
                    __codec_dest_edqy.push_byte(4u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                RuntimeEvent::TransactionPayment(ref aa) => {
                    __codec_dest_edqy.push_byte(5u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                RuntimeEvent::Sudo(ref aa) => {
                    __codec_dest_edqy.push_byte(6u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                RuntimeEvent::TemplateModule(ref aa) => {
                    __codec_dest_edqy.push_byte(7u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                _ => {}
            }
        }
    }
    #[automatically_derived]
    impl ::codec::EncodeLike for RuntimeEvent {}
};
#[allow(deprecated)]
const _: () = {
    #[allow(non_camel_case_types)]
    #[automatically_derived]
    impl ::codec::Decode for RuntimeEvent {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| {
                    e
                        .chain(
                            "Could not decode `RuntimeEvent`, failed to read variant byte",
                        )
                })?
            {
                #[allow(clippy::unnecessary_cast)]
                __codec_x_edqy if __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeEvent::System({
                                let __codec_res_edqy = <frame_system::Event<
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `RuntimeEvent::System.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                #[allow(clippy::unnecessary_cast)]
                __codec_x_edqy if __codec_x_edqy == 3u8 as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeEvent::Grandpa({
                                let __codec_res_edqy = <pallet_grandpa::Event as ::codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `RuntimeEvent::Grandpa.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                #[allow(clippy::unnecessary_cast)]
                __codec_x_edqy if __codec_x_edqy == 4u8 as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeEvent::Balances({
                                let __codec_res_edqy = <pallet_balances::Event<
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `RuntimeEvent::Balances.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                #[allow(clippy::unnecessary_cast)]
                __codec_x_edqy if __codec_x_edqy == 5u8 as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeEvent::TransactionPayment({
                                let __codec_res_edqy = <pallet_transaction_payment::Event<
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e
                                                .chain(
                                                    "Could not decode `RuntimeEvent::TransactionPayment.0`",
                                                ),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                #[allow(clippy::unnecessary_cast)]
                __codec_x_edqy if __codec_x_edqy == 6u8 as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeEvent::Sudo({
                                let __codec_res_edqy = <pallet_sudo::Event<
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `RuntimeEvent::Sudo.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                #[allow(clippy::unnecessary_cast)]
                __codec_x_edqy if __codec_x_edqy == 7u8 as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeEvent::TemplateModule({
                                let __codec_res_edqy = <pallet_template::Event<
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `RuntimeEvent::TemplateModule.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                _ => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into(
                                "Could not decode `RuntimeEvent`, variant doesn't exist",
                            ),
                        )
                    })();
                }
            }
        }
    }
};
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for RuntimeEvent {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(::scale_info::Path::new("RuntimeEvent", "node_template_runtime"))
                .type_params(::alloc::vec::Vec::new())
                .variant(
                    ::scale_info::build::Variants::new()
                        .variant(
                            "System",
                            |v| {
                                v
                                    .index(0u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<frame_system::Event<Runtime>>()
                                                    .type_name("frame_system::Event<Runtime>")
                                            }),
                                    )
                            },
                        )
                        .variant(
                            "Grandpa",
                            |v| {
                                v
                                    .index(3u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<pallet_grandpa::Event>()
                                                    .type_name("pallet_grandpa::Event")
                                            }),
                                    )
                            },
                        )
                        .variant(
                            "Balances",
                            |v| {
                                v
                                    .index(4u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<pallet_balances::Event<Runtime>>()
                                                    .type_name("pallet_balances::Event<Runtime>")
                                            }),
                                    )
                            },
                        )
                        .variant(
                            "TransactionPayment",
                            |v| {
                                v
                                    .index(5u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<pallet_transaction_payment::Event<Runtime>>()
                                                    .type_name("pallet_transaction_payment::Event<Runtime>")
                                            }),
                                    )
                            },
                        )
                        .variant(
                            "Sudo",
                            |v| {
                                v
                                    .index(6u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<pallet_sudo::Event<Runtime>>()
                                                    .type_name("pallet_sudo::Event<Runtime>")
                                            }),
                                    )
                            },
                        )
                        .variant(
                            "TemplateModule",
                            |v| {
                                v
                                    .index(7u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<pallet_template::Event<Runtime>>()
                                                    .type_name("pallet_template::Event<Runtime>")
                                            }),
                                    )
                            },
                        ),
                )
        }
    }
};
impl core::fmt::Debug for RuntimeEvent {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::System(ref a0) => {
                fmt.debug_tuple("RuntimeEvent::System").field(a0).finish()
            }
            Self::Grandpa(ref a0) => {
                fmt.debug_tuple("RuntimeEvent::Grandpa").field(a0).finish()
            }
            Self::Balances(ref a0) => {
                fmt.debug_tuple("RuntimeEvent::Balances").field(a0).finish()
            }
            Self::TransactionPayment(ref a0) => {
                fmt.debug_tuple("RuntimeEvent::TransactionPayment").field(a0).finish()
            }
            Self::Sudo(ref a0) => {
                fmt.debug_tuple("RuntimeEvent::Sudo").field(a0).finish()
            }
            Self::TemplateModule(ref a0) => {
                fmt.debug_tuple("RuntimeEvent::TemplateModule").field(a0).finish()
            }
            _ => Ok(()),
        }
    }
}
impl From<frame_system::Event<Runtime>> for RuntimeEvent {
    fn from(x: frame_system::Event<Runtime>) -> Self {
        RuntimeEvent::System(x)
    }
}
impl TryInto<frame_system::Event<Runtime>> for RuntimeEvent {
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
impl From<pallet_grandpa::Event> for RuntimeEvent {
    fn from(x: pallet_grandpa::Event) -> Self {
        RuntimeEvent::Grandpa(x)
    }
}
impl TryInto<pallet_grandpa::Event> for RuntimeEvent {
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
impl From<pallet_balances::Event<Runtime>> for RuntimeEvent {
    fn from(x: pallet_balances::Event<Runtime>) -> Self {
        RuntimeEvent::Balances(x)
    }
}
impl TryInto<pallet_balances::Event<Runtime>> for RuntimeEvent {
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
impl From<pallet_transaction_payment::Event<Runtime>> for RuntimeEvent {
    fn from(x: pallet_transaction_payment::Event<Runtime>) -> Self {
        RuntimeEvent::TransactionPayment(x)
    }
}
impl TryInto<pallet_transaction_payment::Event<Runtime>> for RuntimeEvent {
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_transaction_payment::Event<Runtime>,
        Self::Error,
    > {
        match self {
            Self::TransactionPayment(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_sudo::Event<Runtime>> for RuntimeEvent {
    fn from(x: pallet_sudo::Event<Runtime>) -> Self {
        RuntimeEvent::Sudo(x)
    }
}
impl TryInto<pallet_sudo::Event<Runtime>> for RuntimeEvent {
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
impl From<pallet_template::Event<Runtime>> for RuntimeEvent {
    fn from(x: pallet_template::Event<Runtime>) -> Self {
        RuntimeEvent::TemplateModule(x)
    }
}
impl TryInto<pallet_template::Event<Runtime>> for RuntimeEvent {
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_template::Event<Runtime>,
        Self::Error,
    > {
        match self {
            Self::TemplateModule(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
/// The runtime origin type representing the origin of a call.
///
/// Origin is always created with the base filter configured in [`frame_system::Config::BaseCallFilter`].
pub struct RuntimeOrigin {
    caller: OriginCaller,
    filter: self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc<
        Box<dyn Fn(&<Runtime as frame_system::Config>::RuntimeCall) -> bool>,
    >,
}
#[automatically_derived]
impl ::core::clone::Clone for RuntimeOrigin {
    #[inline]
    fn clone(&self) -> RuntimeOrigin {
        RuntimeOrigin {
            caller: ::core::clone::Clone::clone(&self.caller),
            filter: ::core::clone::Clone::clone(&self.filter),
        }
    }
}
#[cfg(feature = "std")]
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::fmt::Debug
for RuntimeOrigin {
    fn fmt(
        &self,
        fmt: &mut self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::fmt::Formatter,
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
for RuntimeOrigin {
    type Call = <Runtime as frame_system::Config>::RuntimeCall;
    type PalletsOrigin = OriginCaller;
    type AccountId = <Runtime as frame_system::Config>::AccountId;
    fn add_filter(&mut self, filter: impl Fn(&Self::Call) -> bool + 'static) {
        let f = self.filter.clone();
        self
            .filter = self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc::new(
            Box::new(move |call| { f(call) && filter(call) }),
        );
    }
    fn reset_filter(&mut self) {
        let filter = <<Runtime as frame_system::Config>::BaseCallFilter as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::Contains<
            <Runtime as frame_system::Config>::RuntimeCall,
        >>::contains;
        self
            .filter = self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc::new(
            Box::new(filter),
        );
    }
    fn set_caller_from(&mut self, other: impl Into<Self>) {
        self.caller = other.into().caller;
    }
    fn filter_call(&self, call: &Self::Call) -> bool {
        match self.caller {
            OriginCaller::system(frame_system::Origin::<Runtime>::Root) => true,
            _ => (self.filter)(call),
        }
    }
    fn caller(&self) -> &Self::PalletsOrigin {
        &self.caller
    }
    fn into_caller(self) -> Self::PalletsOrigin {
        self.caller
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
    fn none() -> Self {
        frame_system::RawOrigin::None.into()
    }
    fn root() -> Self {
        frame_system::RawOrigin::Root.into()
    }
    fn signed(by: Self::AccountId) -> Self {
        frame_system::RawOrigin::Signed(by).into()
    }
}
#[allow(non_camel_case_types)]
pub enum OriginCaller {
    #[codec(index = 0u8)]
    system(frame_system::Origin<Runtime>),
    #[allow(dead_code)]
    Void(self::sp_api_hidden_includes_construct_runtime::hidden_include::Void),
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::clone::Clone for OriginCaller {
    #[inline]
    fn clone(&self) -> OriginCaller {
        match self {
            OriginCaller::system(__self_0) => {
                OriginCaller::system(::core::clone::Clone::clone(__self_0))
            }
            OriginCaller::Void(__self_0) => {
                OriginCaller::Void(::core::clone::Clone::clone(__self_0))
            }
        }
    }
}
#[allow(non_camel_case_types)]
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for OriginCaller {}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::cmp::PartialEq for OriginCaller {
    #[inline]
    fn eq(&self, other: &OriginCaller) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
            && match (self, other) {
                (OriginCaller::system(__self_0), OriginCaller::system(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                (OriginCaller::Void(__self_0), OriginCaller::Void(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                _ => unsafe { ::core::intrinsics::unreachable() }
            }
    }
}
#[allow(non_camel_case_types)]
#[automatically_derived]
impl ::core::marker::StructuralEq for OriginCaller {}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::cmp::Eq for OriginCaller {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<frame_system::Origin<Runtime>>;
        let _: ::core::cmp::AssertParamIsEq<
            self::sp_api_hidden_includes_construct_runtime::hidden_include::Void,
        >;
    }
}
impl core::fmt::Debug for OriginCaller {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::system(ref a0) => {
                fmt.debug_tuple("OriginCaller::system").field(a0).finish()
            }
            Self::Void(ref a0) => {
                fmt.debug_tuple("OriginCaller::Void").field(a0).finish()
            }
            _ => Ok(()),
        }
    }
}
#[allow(deprecated)]
const _: () = {
    #[allow(non_camel_case_types)]
    #[automatically_derived]
    impl ::codec::Encode for OriginCaller {
        fn size_hint(&self) -> usize {
            1_usize
                + match *self {
                    OriginCaller::system(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    OriginCaller::Void(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    _ => 0_usize,
                }
        }
        fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                OriginCaller::system(ref aa) => {
                    __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                OriginCaller::Void(ref aa) => {
                    __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                _ => {}
            }
        }
    }
    #[automatically_derived]
    impl ::codec::EncodeLike for OriginCaller {}
};
#[allow(deprecated)]
const _: () = {
    #[allow(non_camel_case_types)]
    #[automatically_derived]
    impl ::codec::Decode for OriginCaller {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| {
                    e
                        .chain(
                            "Could not decode `OriginCaller`, failed to read variant byte",
                        )
                })?
            {
                #[allow(clippy::unnecessary_cast)]
                __codec_x_edqy if __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            OriginCaller::system({
                                let __codec_res_edqy = <frame_system::Origin<
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `OriginCaller::system.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                #[allow(clippy::unnecessary_cast)]
                __codec_x_edqy if __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            OriginCaller::Void({
                                let __codec_res_edqy = <self::sp_api_hidden_includes_construct_runtime::hidden_include::Void as ::codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `OriginCaller::Void.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                _ => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into(
                                "Could not decode `OriginCaller`, variant doesn't exist",
                            ),
                        )
                    })();
                }
            }
        }
    }
};
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for OriginCaller {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(::scale_info::Path::new("OriginCaller", "node_template_runtime"))
                .type_params(::alloc::vec::Vec::new())
                .variant(
                    ::scale_info::build::Variants::new()
                        .variant(
                            "system",
                            |v| {
                                v
                                    .index(0u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<frame_system::Origin<Runtime>>()
                                                    .type_name("frame_system::Origin<Runtime>")
                                            }),
                                    )
                            },
                        )
                        .variant(
                            "Void",
                            |v| {
                                v
                                    .index(1usize as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<
                                                        self::sp_api_hidden_includes_construct_runtime::hidden_include::Void,
                                                    >()
                                                    .type_name(
                                                        "self::sp_api_hidden_includes_construct_runtime::hidden_include::Void",
                                                    )
                                            }),
                                    )
                            },
                        ),
                )
        }
    }
};
const _: () = {
    impl ::codec::MaxEncodedLen for OriginCaller {
        fn max_encoded_len() -> ::core::primitive::usize {
            0_usize
                .max(
                    0_usize
                        .saturating_add(
                            <frame_system::Origin<Runtime>>::max_encoded_len(),
                        ),
                )
                .max(
                    0_usize
                        .saturating_add(
                            <self::sp_api_hidden_includes_construct_runtime::hidden_include::Void>::max_encoded_len(),
                        ),
                )
                .saturating_add(1)
        }
    }
};
#[allow(dead_code)]
impl RuntimeOrigin {
    /// Create with system none origin and [`frame_system::Config::BaseCallFilter`].
    pub fn none() -> Self {
        <RuntimeOrigin as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait>::none()
    }
    /// Create with system root origin and [`frame_system::Config::BaseCallFilter`].
    pub fn root() -> Self {
        <RuntimeOrigin as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait>::root()
    }
    /// Create with system signed origin and [`frame_system::Config::BaseCallFilter`].
    pub fn signed(by: <Runtime as frame_system::Config>::AccountId) -> Self {
        <RuntimeOrigin as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait>::signed(
            by,
        )
    }
}
impl From<frame_system::Origin<Runtime>> for OriginCaller {
    fn from(x: frame_system::Origin<Runtime>) -> Self {
        OriginCaller::system(x)
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::CallerTrait<
    <Runtime as frame_system::Config>::AccountId,
> for OriginCaller {
    fn into_system(
        self,
    ) -> Option<frame_system::RawOrigin<<Runtime as frame_system::Config>::AccountId>> {
        match self {
            OriginCaller::system(x) => Some(x),
            _ => None,
        }
    }
    fn as_system_ref(
        &self,
    ) -> Option<&frame_system::RawOrigin<<Runtime as frame_system::Config>::AccountId>> {
        match &self {
            OriginCaller::system(o) => Some(o),
            _ => None,
        }
    }
}
impl TryFrom<OriginCaller> for frame_system::Origin<Runtime> {
    type Error = OriginCaller;
    fn try_from(
        x: OriginCaller,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        frame_system::Origin<Runtime>,
        OriginCaller,
    > {
        if let OriginCaller::system(l) = x { Ok(l) } else { Err(x) }
    }
}
impl From<frame_system::Origin<Runtime>> for RuntimeOrigin {
    /// Convert to runtime origin, using as filter: [`frame_system::Config::BaseCallFilter`].
    fn from(x: frame_system::Origin<Runtime>) -> Self {
        let o: OriginCaller = x.into();
        o.into()
    }
}
impl From<OriginCaller> for RuntimeOrigin {
    fn from(x: OriginCaller) -> Self {
        let mut o = RuntimeOrigin {
            caller: x,
            filter: self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc::new(
                Box::new(|_| true),
            ),
        };
        self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait::reset_filter(
            &mut o,
        );
        o
    }
}
impl From<RuntimeOrigin>
for self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
    frame_system::Origin<Runtime>,
    RuntimeOrigin,
> {
    /// NOTE: converting to pallet origin loses the origin filter information.
    fn from(val: RuntimeOrigin) -> Self {
        if let OriginCaller::system(l) = val.caller { Ok(l) } else { Err(val) }
    }
}
impl From<Option<<Runtime as frame_system::Config>::AccountId>> for RuntimeOrigin {
    /// Convert to runtime origin with caller being system signed or none and use filter [`frame_system::Config::BaseCallFilter`].
    fn from(x: Option<<Runtime as frame_system::Config>::AccountId>) -> Self {
        <frame_system::Origin<Runtime>>::from(x).into()
    }
}
pub type System = frame_system::Pallet<Runtime>;
pub type Timestamp = pallet_timestamp::Pallet<Runtime>;
pub type Aura = pallet_aura::Pallet<Runtime>;
pub type Grandpa = pallet_grandpa::Pallet<Runtime>;
pub type Balances = pallet_balances::Pallet<Runtime>;
pub type TransactionPayment = pallet_transaction_payment::Pallet<Runtime>;
pub type Sudo = pallet_sudo::Pallet<Runtime>;
pub type TemplateModule = pallet_template::Pallet<Runtime>;
/// All pallets included in the runtime as a nested tuple of types.
#[deprecated(
    note = "The type definition has changed from representing all pallets \
			excluding system, in reversed order to become the representation of all pallets \
			including system pallet in regular order. For this reason it is encouraged to use \
			explicitly one of `AllPalletsWithSystem`, `AllPalletsWithoutSystem`, \
			`AllPalletsWithSystemReversed`, `AllPalletsWithoutSystemReversed`. \
			Note that the type `frame_executive::Executive` expects one of `AllPalletsWithSystem` \
			, `AllPalletsWithSystemReversed`, `AllPalletsReversedWithSystemFirst`. More details in \
			https://github.com/paritytech/substrate/pull/10043"
)]
pub type AllPallets = AllPalletsWithSystem;
#[cfg(all())]
/// All pallets included in the runtime as a nested tuple of types.
pub type AllPalletsWithSystem = (
    System,
    Timestamp,
    Aura,
    Grandpa,
    Balances,
    TransactionPayment,
    Sudo,
    TemplateModule,
);
#[cfg(all())]
/// All pallets included in the runtime as a nested tuple of types.
/// Excludes the System pallet.
pub type AllPalletsWithoutSystem = (
    Timestamp,
    Aura,
    Grandpa,
    Balances,
    TransactionPayment,
    Sudo,
    TemplateModule,
);
#[cfg(all())]
/// All pallets included in the runtime as a nested tuple of types in reversed order.
#[deprecated(
    note = "Using reverse pallet orders is deprecated. use only \
			`AllPalletsWithSystem or AllPalletsWithoutSystem`"
)]
pub type AllPalletsWithSystemReversed = (
    TemplateModule,
    Sudo,
    TransactionPayment,
    Balances,
    Grandpa,
    Aura,
    Timestamp,
    System,
);
#[cfg(all())]
/// All pallets included in the runtime as a nested tuple of types in reversed order.
/// Excludes the System pallet.
#[deprecated(
    note = "Using reverse pallet orders is deprecated. use only \
			`AllPalletsWithSystem or AllPalletsWithoutSystem`"
)]
pub type AllPalletsWithoutSystemReversed = (
    TemplateModule,
    Sudo,
    TransactionPayment,
    Balances,
    Grandpa,
    Aura,
    Timestamp,
);
#[cfg(all())]
/// All pallets included in the runtime as a nested tuple of types in reversed order.
/// With the system pallet first.
#[deprecated(
    note = "Using reverse pallet orders is deprecated. use only \
			`AllPalletsWithSystem or AllPalletsWithoutSystem`"
)]
pub type AllPalletsReversedWithSystemFirst = (
    System,
    TemplateModule,
    Sudo,
    TransactionPayment,
    Balances,
    Grandpa,
    Aura,
    Timestamp,
);
/// Provides an implementation of `PalletInfo` to provide information
/// about the pallet setup in the runtime.
pub struct PalletInfo;
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfo
for PalletInfo {
    fn index<P: 'static>() -> Option<usize> {
        let type_id = self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
            P,
        >();
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                System,
            >()
        {
            return Some(0usize);
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Timestamp,
            >()
        {
            return Some(1usize);
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Aura,
            >()
        {
            return Some(2usize);
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Grandpa,
            >()
        {
            return Some(3usize);
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Balances,
            >()
        {
            return Some(4usize);
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                TransactionPayment,
            >()
        {
            return Some(5usize);
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Sudo,
            >()
        {
            return Some(6usize);
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                TemplateModule,
            >()
        {
            return Some(7usize);
        }
        None
    }
    fn name<P: 'static>() -> Option<&'static str> {
        let type_id = self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
            P,
        >();
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                System,
            >()
        {
            return Some("System");
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Timestamp,
            >()
        {
            return Some("Timestamp");
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Aura,
            >()
        {
            return Some("Aura");
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Grandpa,
            >()
        {
            return Some("Grandpa");
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Balances,
            >()
        {
            return Some("Balances");
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                TransactionPayment,
            >()
        {
            return Some("TransactionPayment");
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Sudo,
            >()
        {
            return Some("Sudo");
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                TemplateModule,
            >()
        {
            return Some("TemplateModule");
        }
        None
    }
    fn module_name<P: 'static>() -> Option<&'static str> {
        let type_id = self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
            P,
        >();
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                System,
            >()
        {
            return Some("frame_system");
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Timestamp,
            >()
        {
            return Some("pallet_timestamp");
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Aura,
            >()
        {
            return Some("pallet_aura");
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Grandpa,
            >()
        {
            return Some("pallet_grandpa");
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Balances,
            >()
        {
            return Some("pallet_balances");
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                TransactionPayment,
            >()
        {
            return Some("pallet_transaction_payment");
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Sudo,
            >()
        {
            return Some("pallet_sudo");
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                TemplateModule,
            >()
        {
            return Some("pallet_template");
        }
        None
    }
    fn crate_version<P: 'static>() -> Option<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::CrateVersion,
    > {
        let type_id = self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
            P,
        >();
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                System,
            >()
        {
            return Some(
                <frame_system::Pallet<
                    Runtime,
                > as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version(),
            );
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Timestamp,
            >()
        {
            return Some(
                <pallet_timestamp::Pallet<
                    Runtime,
                > as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version(),
            );
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Aura,
            >()
        {
            return Some(
                <pallet_aura::Pallet<
                    Runtime,
                > as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version(),
            );
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Grandpa,
            >()
        {
            return Some(
                <pallet_grandpa::Pallet<
                    Runtime,
                > as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version(),
            );
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Balances,
            >()
        {
            return Some(
                <pallet_balances::Pallet<
                    Runtime,
                > as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version(),
            );
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                TransactionPayment,
            >()
        {
            return Some(
                <pallet_transaction_payment::Pallet<
                    Runtime,
                > as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version(),
            );
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Sudo,
            >()
        {
            return Some(
                <pallet_sudo::Pallet<
                    Runtime,
                > as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version(),
            );
        }
        if type_id
            == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                TemplateModule,
            >()
        {
            return Some(
                <pallet_template::Pallet<
                    Runtime,
                > as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version(),
            );
        }
        None
    }
}
pub enum RuntimeCall {
    #[codec(index = 0u8)]
    System(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            System,
            Runtime,
        >,
    ),
    #[codec(index = 1u8)]
    Timestamp(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Timestamp,
            Runtime,
        >,
    ),
    #[codec(index = 3u8)]
    Grandpa(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Grandpa,
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
    #[codec(index = 6u8)]
    Sudo(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Sudo,
            Runtime,
        >,
    ),
    #[codec(index = 7u8)]
    TemplateModule(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            TemplateModule,
            Runtime,
        >,
    ),
}
#[automatically_derived]
impl ::core::clone::Clone for RuntimeCall {
    #[inline]
    fn clone(&self) -> RuntimeCall {
        match self {
            RuntimeCall::System(__self_0) => {
                RuntimeCall::System(::core::clone::Clone::clone(__self_0))
            }
            RuntimeCall::Timestamp(__self_0) => {
                RuntimeCall::Timestamp(::core::clone::Clone::clone(__self_0))
            }
            RuntimeCall::Grandpa(__self_0) => {
                RuntimeCall::Grandpa(::core::clone::Clone::clone(__self_0))
            }
            RuntimeCall::Balances(__self_0) => {
                RuntimeCall::Balances(::core::clone::Clone::clone(__self_0))
            }
            RuntimeCall::Sudo(__self_0) => {
                RuntimeCall::Sudo(::core::clone::Clone::clone(__self_0))
            }
            RuntimeCall::TemplateModule(__self_0) => {
                RuntimeCall::TemplateModule(::core::clone::Clone::clone(__self_0))
            }
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for RuntimeCall {}
#[automatically_derived]
impl ::core::cmp::PartialEq for RuntimeCall {
    #[inline]
    fn eq(&self, other: &RuntimeCall) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
            && match (self, other) {
                (RuntimeCall::System(__self_0), RuntimeCall::System(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                (RuntimeCall::Timestamp(__self_0), RuntimeCall::Timestamp(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                (RuntimeCall::Grandpa(__self_0), RuntimeCall::Grandpa(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                (RuntimeCall::Balances(__self_0), RuntimeCall::Balances(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                (RuntimeCall::Sudo(__self_0), RuntimeCall::Sudo(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                (
                    RuntimeCall::TemplateModule(__self_0),
                    RuntimeCall::TemplateModule(__arg1_0),
                ) => *__self_0 == *__arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() }
            }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for RuntimeCall {}
#[automatically_derived]
impl ::core::cmp::Eq for RuntimeCall {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<
            self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                System,
                Runtime,
            >,
        >;
        let _: ::core::cmp::AssertParamIsEq<
            self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                Timestamp,
                Runtime,
            >,
        >;
        let _: ::core::cmp::AssertParamIsEq<
            self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                Grandpa,
                Runtime,
            >,
        >;
        let _: ::core::cmp::AssertParamIsEq<
            self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                Balances,
                Runtime,
            >,
        >;
        let _: ::core::cmp::AssertParamIsEq<
            self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                Sudo,
                Runtime,
            >,
        >;
        let _: ::core::cmp::AssertParamIsEq<
            self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                TemplateModule,
                Runtime,
            >,
        >;
    }
}
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Encode for RuntimeCall {
        fn size_hint(&self) -> usize {
            1_usize
                + match *self {
                    RuntimeCall::System(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    RuntimeCall::Timestamp(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    RuntimeCall::Grandpa(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    RuntimeCall::Balances(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    RuntimeCall::Sudo(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    RuntimeCall::TemplateModule(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    _ => 0_usize,
                }
        }
        fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                RuntimeCall::System(ref aa) => {
                    __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                RuntimeCall::Timestamp(ref aa) => {
                    __codec_dest_edqy.push_byte(1u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                RuntimeCall::Grandpa(ref aa) => {
                    __codec_dest_edqy.push_byte(3u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                RuntimeCall::Balances(ref aa) => {
                    __codec_dest_edqy.push_byte(4u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                RuntimeCall::Sudo(ref aa) => {
                    __codec_dest_edqy.push_byte(6u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                RuntimeCall::TemplateModule(ref aa) => {
                    __codec_dest_edqy.push_byte(7u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                _ => {}
            }
        }
    }
    #[automatically_derived]
    impl ::codec::EncodeLike for RuntimeCall {}
};
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Decode for RuntimeCall {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| {
                    e
                        .chain(
                            "Could not decode `RuntimeCall`, failed to read variant byte",
                        )
                })?
            {
                #[allow(clippy::unnecessary_cast)]
                __codec_x_edqy if __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeCall::System({
                                let __codec_res_edqy = <self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                    System,
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `RuntimeCall::System.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                #[allow(clippy::unnecessary_cast)]
                __codec_x_edqy if __codec_x_edqy == 1u8 as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeCall::Timestamp({
                                let __codec_res_edqy = <self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                    Timestamp,
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `RuntimeCall::Timestamp.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                #[allow(clippy::unnecessary_cast)]
                __codec_x_edqy if __codec_x_edqy == 3u8 as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeCall::Grandpa({
                                let __codec_res_edqy = <self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                    Grandpa,
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `RuntimeCall::Grandpa.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                #[allow(clippy::unnecessary_cast)]
                __codec_x_edqy if __codec_x_edqy == 4u8 as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeCall::Balances({
                                let __codec_res_edqy = <self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                    Balances,
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `RuntimeCall::Balances.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                #[allow(clippy::unnecessary_cast)]
                __codec_x_edqy if __codec_x_edqy == 6u8 as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeCall::Sudo({
                                let __codec_res_edqy = <self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                    Sudo,
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `RuntimeCall::Sudo.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                #[allow(clippy::unnecessary_cast)]
                __codec_x_edqy if __codec_x_edqy == 7u8 as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeCall::TemplateModule({
                                let __codec_res_edqy = <self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                    TemplateModule,
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `RuntimeCall::TemplateModule.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                _ => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into(
                                "Could not decode `RuntimeCall`, variant doesn't exist",
                            ),
                        )
                    })();
                }
            }
        }
    }
};
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for RuntimeCall {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(::scale_info::Path::new("RuntimeCall", "node_template_runtime"))
                .type_params(::alloc::vec::Vec::new())
                .variant(
                    ::scale_info::build::Variants::new()
                        .variant(
                            "System",
                            |v| {
                                v
                                    .index(0u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<
                                                        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                                            System,
                                                            Runtime,
                                                        >,
                                                    >()
                                                    .type_name(
                                                        "self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<System, Runtime>",
                                                    )
                                            }),
                                    )
                            },
                        )
                        .variant(
                            "Timestamp",
                            |v| {
                                v
                                    .index(1u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<
                                                        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                                            Timestamp,
                                                            Runtime,
                                                        >,
                                                    >()
                                                    .type_name(
                                                        "self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<Timestamp, Runtime>",
                                                    )
                                            }),
                                    )
                            },
                        )
                        .variant(
                            "Grandpa",
                            |v| {
                                v
                                    .index(3u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<
                                                        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                                            Grandpa,
                                                            Runtime,
                                                        >,
                                                    >()
                                                    .type_name(
                                                        "self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<Grandpa, Runtime>",
                                                    )
                                            }),
                                    )
                            },
                        )
                        .variant(
                            "Balances",
                            |v| {
                                v
                                    .index(4u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<
                                                        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                                            Balances,
                                                            Runtime,
                                                        >,
                                                    >()
                                                    .type_name(
                                                        "self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<Balances, Runtime>",
                                                    )
                                            }),
                                    )
                            },
                        )
                        .variant(
                            "Sudo",
                            |v| {
                                v
                                    .index(6u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<
                                                        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                                            Sudo,
                                                            Runtime,
                                                        >,
                                                    >()
                                                    .type_name(
                                                        "self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<Sudo, Runtime>",
                                                    )
                                            }),
                                    )
                            },
                        )
                        .variant(
                            "TemplateModule",
                            |v| {
                                v
                                    .index(7u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<
                                                        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                                            TemplateModule,
                                                            Runtime,
                                                        >,
                                                    >()
                                                    .type_name(
                                                        "self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<TemplateModule, Runtime>",
                                                    )
                                            }),
                                    )
                            },
                        ),
                )
        }
    }
};
impl core::fmt::Debug for RuntimeCall {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::System(ref a0) => {
                fmt.debug_tuple("RuntimeCall::System").field(a0).finish()
            }
            Self::Timestamp(ref a0) => {
                fmt.debug_tuple("RuntimeCall::Timestamp").field(a0).finish()
            }
            Self::Grandpa(ref a0) => {
                fmt.debug_tuple("RuntimeCall::Grandpa").field(a0).finish()
            }
            Self::Balances(ref a0) => {
                fmt.debug_tuple("RuntimeCall::Balances").field(a0).finish()
            }
            Self::Sudo(ref a0) => fmt.debug_tuple("RuntimeCall::Sudo").field(a0).finish(),
            Self::TemplateModule(ref a0) => {
                fmt.debug_tuple("RuntimeCall::TemplateModule").field(a0).finish()
            }
            _ => Ok(()),
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::GetDispatchInfo
for RuntimeCall {
    fn get_dispatch_info(
        &self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::DispatchInfo {
        match self {
            RuntimeCall::System(call) => call.get_dispatch_info(),
            RuntimeCall::Timestamp(call) => call.get_dispatch_info(),
            RuntimeCall::Grandpa(call) => call.get_dispatch_info(),
            RuntimeCall::Balances(call) => call.get_dispatch_info(),
            RuntimeCall::Sudo(call) => call.get_dispatch_info(),
            RuntimeCall::TemplateModule(call) => call.get_dispatch_info(),
        }
    }
}
#[allow(deprecated)]
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::weights::GetDispatchInfo
for RuntimeCall {}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::GetCallMetadata
for RuntimeCall {
    fn get_call_metadata(
        &self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::GetCallName;
        match self {
            RuntimeCall::System(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "System";
                self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata {
                    function_name,
                    pallet_name,
                }
            }
            RuntimeCall::Timestamp(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Timestamp";
                self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata {
                    function_name,
                    pallet_name,
                }
            }
            RuntimeCall::Grandpa(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Grandpa";
                self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata {
                    function_name,
                    pallet_name,
                }
            }
            RuntimeCall::Balances(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Balances";
                self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata {
                    function_name,
                    pallet_name,
                }
            }
            RuntimeCall::Sudo(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Sudo";
                self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata {
                    function_name,
                    pallet_name,
                }
            }
            RuntimeCall::TemplateModule(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "TemplateModule";
                self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata {
                    function_name,
                    pallet_name,
                }
            }
        }
    }
    fn get_module_names() -> &'static [&'static str] {
        &["System", "Timestamp", "Grandpa", "Balances", "Sudo", "TemplateModule"]
    }
    fn get_call_names(module: &str) -> &'static [&'static str] {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::{
            Callable, GetCallName,
        };
        match module {
            "System" => {
                <<System as Callable<
                    Runtime,
                >>::RuntimeCall as GetCallName>::get_call_names()
            }
            "Timestamp" => {
                <<Timestamp as Callable<
                    Runtime,
                >>::RuntimeCall as GetCallName>::get_call_names()
            }
            "Grandpa" => {
                <<Grandpa as Callable<
                    Runtime,
                >>::RuntimeCall as GetCallName>::get_call_names()
            }
            "Balances" => {
                <<Balances as Callable<
                    Runtime,
                >>::RuntimeCall as GetCallName>::get_call_names()
            }
            "Sudo" => {
                <<Sudo as Callable<
                    Runtime,
                >>::RuntimeCall as GetCallName>::get_call_names()
            }
            "TemplateModule" => {
                <<TemplateModule as Callable<
                    Runtime,
                >>::RuntimeCall as GetCallName>::get_call_names()
            }
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::Dispatchable
for RuntimeCall {
    type RuntimeOrigin = RuntimeOrigin;
    type Config = RuntimeCall;
    type Info = self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::DispatchInfo;
    type PostInfo = self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::PostDispatchInfo;
    fn dispatch(
        self,
        origin: RuntimeOrigin,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::DispatchResultWithPostInfo {
        if !<Self::RuntimeOrigin as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait>::filter_call(
            &origin,
            &self,
        ) {
            return self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result::Err(
                frame_system::Error::<Runtime>::CallFiltered.into(),
            );
        }
        self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(
            self,
            origin,
        )
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable
for RuntimeCall {
    type RuntimeOrigin = RuntimeOrigin;
    fn dispatch_bypass_filter(
        self,
        origin: RuntimeOrigin,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::DispatchResultWithPostInfo {
        match self {
            RuntimeCall::System(call) => {
                self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                    call,
                    origin,
                )
            }
            RuntimeCall::Timestamp(call) => {
                self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                    call,
                    origin,
                )
            }
            RuntimeCall::Grandpa(call) => {
                self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                    call,
                    origin,
                )
            }
            RuntimeCall::Balances(call) => {
                self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                    call,
                    origin,
                )
            }
            RuntimeCall::Sudo(call) => {
                self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                    call,
                    origin,
                )
            }
            RuntimeCall::TemplateModule(call) => {
                self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                    call,
                    origin,
                )
            }
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
    self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
        System,
        Runtime,
    >,
> for RuntimeCall {
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
            RuntimeCall::System(call) => Some(call),
            _ => None,
        }
    }
}
impl From<
    self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
        System,
        Runtime,
    >,
> for RuntimeCall {
    fn from(
        call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            System,
            Runtime,
        >,
    ) -> Self {
        RuntimeCall::System(call)
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
    self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
        Timestamp,
        Runtime,
    >,
> for RuntimeCall {
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
            RuntimeCall::Timestamp(call) => Some(call),
            _ => None,
        }
    }
}
impl From<
    self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
        Timestamp,
        Runtime,
    >,
> for RuntimeCall {
    fn from(
        call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Timestamp,
            Runtime,
        >,
    ) -> Self {
        RuntimeCall::Timestamp(call)
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
    self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
        Grandpa,
        Runtime,
    >,
> for RuntimeCall {
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
            RuntimeCall::Grandpa(call) => Some(call),
            _ => None,
        }
    }
}
impl From<
    self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
        Grandpa,
        Runtime,
    >,
> for RuntimeCall {
    fn from(
        call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Grandpa,
            Runtime,
        >,
    ) -> Self {
        RuntimeCall::Grandpa(call)
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
    self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
        Balances,
        Runtime,
    >,
> for RuntimeCall {
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
            RuntimeCall::Balances(call) => Some(call),
            _ => None,
        }
    }
}
impl From<
    self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
        Balances,
        Runtime,
    >,
> for RuntimeCall {
    fn from(
        call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Balances,
            Runtime,
        >,
    ) -> Self {
        RuntimeCall::Balances(call)
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
    self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
        Sudo,
        Runtime,
    >,
> for RuntimeCall {
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
            RuntimeCall::Sudo(call) => Some(call),
            _ => None,
        }
    }
}
impl From<
    self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
        Sudo,
        Runtime,
    >,
> for RuntimeCall {
    fn from(
        call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Sudo,
            Runtime,
        >,
    ) -> Self {
        RuntimeCall::Sudo(call)
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
    self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
        TemplateModule,
        Runtime,
    >,
> for RuntimeCall {
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            TemplateModule,
            Runtime,
        >,
    > {
        match self {
            RuntimeCall::TemplateModule(call) => Some(call),
            _ => None,
        }
    }
}
impl From<
    self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
        TemplateModule,
        Runtime,
    >,
> for RuntimeCall {
    fn from(
        call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            TemplateModule,
            Runtime,
        >,
    ) -> Self {
        RuntimeCall::TemplateModule(call)
    }
}
impl Runtime {
    fn metadata_ir() -> self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata_ir::MetadataIR {
        let rt = Runtime;
        self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata_ir::MetadataIR {
            pallets: <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata_ir::PalletMetadataIR {
                        name: "System",
                        index: 0u8,
                        storage: Some(
                            frame_system::Pallet::<Runtime>::storage_metadata(),
                        ),
                        calls: Some(frame_system::Pallet::<Runtime>::call_functions()),
                        event: Some(self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata_ir::PalletEventMetadataIR {
                            ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<
                                frame_system::Event<Runtime>,
                            >(),
                        }),
                        constants: frame_system::Pallet::<
                            Runtime,
                        >::pallet_constants_metadata(),
                        error: frame_system::Pallet::<Runtime>::error_metadata(),
                        docs: frame_system::Pallet::<
                            Runtime,
                        >::pallet_documentation_metadata(),
                    },
                    self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata_ir::PalletMetadataIR {
                        name: "Timestamp",
                        index: 1u8,
                        storage: Some(
                            pallet_timestamp::Pallet::<Runtime>::storage_metadata(),
                        ),
                        calls: Some(
                            pallet_timestamp::Pallet::<Runtime>::call_functions(),
                        ),
                        event: None,
                        constants: pallet_timestamp::Pallet::<
                            Runtime,
                        >::pallet_constants_metadata(),
                        error: pallet_timestamp::Pallet::<Runtime>::error_metadata(),
                        docs: pallet_timestamp::Pallet::<
                            Runtime,
                        >::pallet_documentation_metadata(),
                    },
                    self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata_ir::PalletMetadataIR {
                        name: "Aura",
                        index: 2u8,
                        storage: Some(
                            pallet_aura::Pallet::<Runtime>::storage_metadata(),
                        ),
                        calls: None,
                        event: None,
                        constants: pallet_aura::Pallet::<
                            Runtime,
                        >::pallet_constants_metadata(),
                        error: pallet_aura::Pallet::<Runtime>::error_metadata(),
                        docs: pallet_aura::Pallet::<
                            Runtime,
                        >::pallet_documentation_metadata(),
                    },
                    self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata_ir::PalletMetadataIR {
                        name: "Grandpa",
                        index: 3u8,
                        storage: Some(
                            pallet_grandpa::Pallet::<Runtime>::storage_metadata(),
                        ),
                        calls: Some(pallet_grandpa::Pallet::<Runtime>::call_functions()),
                        event: Some(self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata_ir::PalletEventMetadataIR {
                            ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<
                                pallet_grandpa::Event,
                            >(),
                        }),
                        constants: pallet_grandpa::Pallet::<
                            Runtime,
                        >::pallet_constants_metadata(),
                        error: pallet_grandpa::Pallet::<Runtime>::error_metadata(),
                        docs: pallet_grandpa::Pallet::<
                            Runtime,
                        >::pallet_documentation_metadata(),
                    },
                    self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata_ir::PalletMetadataIR {
                        name: "Balances",
                        index: 4u8,
                        storage: Some(
                            pallet_balances::Pallet::<Runtime>::storage_metadata(),
                        ),
                        calls: Some(
                            pallet_balances::Pallet::<Runtime>::call_functions(),
                        ),
                        event: Some(self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata_ir::PalletEventMetadataIR {
                            ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<
                                pallet_balances::Event<Runtime>,
                            >(),
                        }),
                        constants: pallet_balances::Pallet::<
                            Runtime,
                        >::pallet_constants_metadata(),
                        error: pallet_balances::Pallet::<Runtime>::error_metadata(),
                        docs: pallet_balances::Pallet::<
                            Runtime,
                        >::pallet_documentation_metadata(),
                    },
                    self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata_ir::PalletMetadataIR {
                        name: "TransactionPayment",
                        index: 5u8,
                        storage: Some(
                            pallet_transaction_payment::Pallet::<
                                Runtime,
                            >::storage_metadata(),
                        ),
                        calls: None,
                        event: Some(self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata_ir::PalletEventMetadataIR {
                            ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<
                                pallet_transaction_payment::Event<Runtime>,
                            >(),
                        }),
                        constants: pallet_transaction_payment::Pallet::<
                            Runtime,
                        >::pallet_constants_metadata(),
                        error: pallet_transaction_payment::Pallet::<
                            Runtime,
                        >::error_metadata(),
                        docs: pallet_transaction_payment::Pallet::<
                            Runtime,
                        >::pallet_documentation_metadata(),
                    },
                    self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata_ir::PalletMetadataIR {
                        name: "Sudo",
                        index: 6u8,
                        storage: Some(
                            pallet_sudo::Pallet::<Runtime>::storage_metadata(),
                        ),
                        calls: Some(pallet_sudo::Pallet::<Runtime>::call_functions()),
                        event: Some(self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata_ir::PalletEventMetadataIR {
                            ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<
                                pallet_sudo::Event<Runtime>,
                            >(),
                        }),
                        constants: pallet_sudo::Pallet::<
                            Runtime,
                        >::pallet_constants_metadata(),
                        error: pallet_sudo::Pallet::<Runtime>::error_metadata(),
                        docs: pallet_sudo::Pallet::<
                            Runtime,
                        >::pallet_documentation_metadata(),
                    },
                    self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata_ir::PalletMetadataIR {
                        name: "TemplateModule",
                        index: 7u8,
                        storage: Some(
                            pallet_template::Pallet::<Runtime>::storage_metadata(),
                        ),
                        calls: Some(
                            pallet_template::Pallet::<Runtime>::call_functions(),
                        ),
                        event: Some(self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata_ir::PalletEventMetadataIR {
                            ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<
                                pallet_template::Event<Runtime>,
                            >(),
                        }),
                        constants: pallet_template::Pallet::<
                            Runtime,
                        >::pallet_constants_metadata(),
                        error: pallet_template::Pallet::<Runtime>::error_metadata(),
                        docs: pallet_template::Pallet::<
                            Runtime,
                        >::pallet_documentation_metadata(),
                    },
                ]),
            ),
            extrinsic: self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata_ir::ExtrinsicMetadataIR {
                ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<
                    UncheckedExtrinsic,
                >(),
                version: <UncheckedExtrinsic as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::ExtrinsicMetadata>::VERSION,
                signed_extensions: <<UncheckedExtrinsic as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::ExtrinsicMetadata>::SignedExtensions as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::SignedExtension>::metadata()
                    .into_iter()
                    .map(|meta| self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata_ir::SignedExtensionMetadataIR {
                        identifier: meta.identifier,
                        ty: meta.ty,
                        additional_signed: meta.additional_signed,
                    })
                    .collect(),
            },
            ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<
                Runtime,
            >(),
            apis: (&rt).runtime_metadata(),
        }
    }
    pub fn metadata() -> self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::RuntimeMetadataPrefixed {
        self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata_ir::into_latest(
            Runtime::metadata_ir(),
        )
    }
    pub fn metadata_at_version(
        version: u32,
    ) -> Option<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::OpaqueMetadata,
    > {
        self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata_ir::into_version(
                Runtime::metadata_ir(),
                version,
            )
            .map(|prefixed| {
                self::sp_api_hidden_includes_construct_runtime::hidden_include::OpaqueMetadata::new(
                    prefixed.into(),
                )
            })
    }
    pub fn metadata_versions() -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::vec::Vec<
        u32,
    > {
        self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata_ir::supported_versions()
    }
}
#[cfg(any(feature = "std", test))]
pub type SystemConfig = frame_system::GenesisConfig;
#[cfg(any(feature = "std", test))]
pub type AuraConfig = pallet_aura::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type GrandpaConfig = pallet_grandpa::GenesisConfig;
#[cfg(any(feature = "std", test))]
pub type BalancesConfig = pallet_balances::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type TransactionPaymentConfig = pallet_transaction_payment::GenesisConfig;
#[cfg(any(feature = "std", test))]
pub type SudoConfig = pallet_sudo::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
use self::sp_api_hidden_includes_construct_runtime::hidden_include::serde as __genesis_config_serde_import__;
#[cfg(any(feature = "std", test))]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(crate = "__genesis_config_serde_import__")]
pub struct GenesisConfig {
    pub system: SystemConfig,
    pub aura: AuraConfig,
    pub grandpa: GrandpaConfig,
    pub balances: BalancesConfig,
    pub transaction_payment: TransactionPaymentConfig,
    pub sudo: SudoConfig,
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
                false as usize + 1 + 1 + 1 + 1 + 1 + 1,
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
                "transactionPayment",
                &self.transaction_payment,
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
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
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
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        4u64 => _serde::__private::Ok(__Field::__field4),
                        5u64 => _serde::__private::Ok(__Field::__field5),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"field index 0 <= i < 6",
                                ),
                            )
                        }
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
                        "aura" => _serde::__private::Ok(__Field::__field1),
                        "grandpa" => _serde::__private::Ok(__Field::__field2),
                        "balances" => _serde::__private::Ok(__Field::__field3),
                        "transactionPayment" => _serde::__private::Ok(__Field::__field4),
                        "sudo" => _serde::__private::Ok(__Field::__field5),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_field(__value, FIELDS),
                            )
                        }
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
                        b"aura" => _serde::__private::Ok(__Field::__field1),
                        b"grandpa" => _serde::__private::Ok(__Field::__field2),
                        b"balances" => _serde::__private::Ok(__Field::__field3),
                        b"transactionPayment" => _serde::__private::Ok(__Field::__field4),
                        b"sudo" => _serde::__private::Ok(__Field::__field5),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_field(__value, FIELDS),
                            )
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
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
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
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct GenesisConfig",
                    )
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
                        SystemConfig,
                    >(&mut __seq) {
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
                                    &"struct GenesisConfig with 6 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match match _serde::de::SeqAccess::next_element::<
                        AuraConfig,
                    >(&mut __seq) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct GenesisConfig with 6 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match match _serde::de::SeqAccess::next_element::<
                        GrandpaConfig,
                    >(&mut __seq) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct GenesisConfig with 6 elements",
                                ),
                            );
                        }
                    };
                    let __field3 = match match _serde::de::SeqAccess::next_element::<
                        BalancesConfig,
                    >(&mut __seq) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct GenesisConfig with 6 elements",
                                ),
                            );
                        }
                    };
                    let __field4 = match match _serde::de::SeqAccess::next_element::<
                        TransactionPaymentConfig,
                    >(&mut __seq) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    4usize,
                                    &"struct GenesisConfig with 6 elements",
                                ),
                            );
                        }
                    };
                    let __field5 = match match _serde::de::SeqAccess::next_element::<
                        SudoConfig,
                    >(&mut __seq) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    5usize,
                                    &"struct GenesisConfig with 6 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(GenesisConfig {
                        system: __field0,
                        aura: __field1,
                        grandpa: __field2,
                        balances: __field3,
                        transaction_payment: __field4,
                        sudo: __field5,
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
                    let mut __field0: _serde::__private::Option<SystemConfig> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<AuraConfig> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<GrandpaConfig> = _serde::__private::None;
                    let mut __field3: _serde::__private::Option<BalancesConfig> = _serde::__private::None;
                    let mut __field4: _serde::__private::Option<
                        TransactionPaymentConfig,
                    > = _serde::__private::None;
                    let mut __field5: _serde::__private::Option<SudoConfig> = _serde::__private::None;
                    while let _serde::__private::Some(__key)
                        = match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("system"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<
                                        SystemConfig,
                                    >(&mut __map) {
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
                                        <__A::Error as _serde::de::Error>::duplicate_field("aura"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<
                                        AuraConfig,
                                    >(&mut __map) {
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
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "grandpa",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<
                                        GrandpaConfig,
                                    >(&mut __map) {
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
                                            "balances",
                                        ),
                                    );
                                }
                                __field3 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<
                                        BalancesConfig,
                                    >(&mut __map) {
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
                                            "transactionPayment",
                                        ),
                                    );
                                }
                                __field4 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<
                                        TransactionPaymentConfig,
                                    >(&mut __map) {
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
                                        <__A::Error as _serde::de::Error>::duplicate_field("sudo"),
                                    );
                                }
                                __field5 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<
                                        SudoConfig,
                                    >(&mut __map) {
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
                            match _serde::__private::de::missing_field("aura") {
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
                            match _serde::__private::de::missing_field("grandpa") {
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
                            match _serde::__private::de::missing_field("balances") {
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
                            match _serde::__private::de::missing_field(
                                "transactionPayment",
                            ) {
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
                            match _serde::__private::de::missing_field("sudo") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(GenesisConfig {
                        system: __field0,
                        aura: __field1,
                        grandpa: __field2,
                        balances: __field3,
                        transaction_payment: __field4,
                        sudo: __field5,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &[
                "system",
                "aura",
                "grandpa",
                "balances",
                "transactionPayment",
                "sudo",
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
impl ::core::default::Default for GenesisConfig {
    #[inline]
    fn default() -> GenesisConfig {
        GenesisConfig {
            system: ::core::default::Default::default(),
            aura: ::core::default::Default::default(),
            grandpa: ::core::default::Default::default(),
            balances: ::core::default::Default::default(),
            transaction_payment: ::core::default::Default::default(),
            sudo: ::core::default::Default::default(),
        }
    }
}
#[cfg(any(feature = "std", test))]
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildStorage
for GenesisConfig {
    fn assimilate_storage(
        &self,
        storage: &mut self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::Storage,
    ) -> std::result::Result<(), String> {
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildModuleGenesisStorage::<
            Runtime,
            frame_system::__InherentHiddenInstance,
        >::build_module_genesis_storage(&self.system, storage)?;
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildModuleGenesisStorage::<
            Runtime,
            pallet_aura::__InherentHiddenInstance,
        >::build_module_genesis_storage(&self.aura, storage)?;
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildModuleGenesisStorage::<
            Runtime,
            pallet_grandpa::__InherentHiddenInstance,
        >::build_module_genesis_storage(&self.grandpa, storage)?;
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildModuleGenesisStorage::<
            Runtime,
            pallet_balances::__InherentHiddenInstance,
        >::build_module_genesis_storage(&self.balances, storage)?;
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildModuleGenesisStorage::<
            Runtime,
            pallet_transaction_payment::__InherentHiddenInstance,
        >::build_module_genesis_storage(&self.transaction_payment, storage)?;
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildModuleGenesisStorage::<
            Runtime,
            pallet_sudo::__InherentHiddenInstance,
        >::build_module_genesis_storage(&self.sudo, storage)?;
        self::sp_api_hidden_includes_construct_runtime::hidden_include::BasicExternalities::execute_with_storage(
            storage,
            || {
                <AllPalletsWithSystem as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OnGenesis>::on_genesis();
            },
        );
        Ok(())
    }
}
trait InherentDataExt {
    fn create_extrinsics(
        &self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::Vec<
        <Block as self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::BlockT>::Extrinsic,
    >;
    fn check_extrinsics(
        &self,
        block: &Block,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::CheckInherentsResult;
}
impl InherentDataExt
for self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::InherentData {
    fn create_extrinsics(
        &self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::Vec<
        <Block as self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::BlockT>::Extrinsic,
    > {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::ProvideInherent;
        let mut inherents = Vec::new();
        if let Some(inherent) = Timestamp::create_inherent(self) {
            let inherent = <UncheckedExtrinsic as self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::Extrinsic>::new(
                    inherent.into(),
                    None,
                )
                .expect(
                    "Runtime UncheckedExtrinsic is not Opaque, so it has to return \
							`Some`; qed",
                );
            inherents.push(inherent);
        }
        inherents
    }
    fn check_extrinsics(
        &self,
        block: &Block,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::CheckInherentsResult {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::{
            ProvideInherent, IsFatalError,
        };
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::{
            IsSubType, ExtrinsicCall,
        };
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block as _;
        let mut result = self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::CheckInherentsResult::new();
        for xt in block.extrinsics() {
            if self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::Extrinsic::is_signed(
                    xt,
                )
                .unwrap_or(false)
            {
                break;
            }
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
            if !is_inherent {
                break;
            }
        }
        match Timestamp::is_inherent_required(self) {
            Ok(Some(e)) => {
                let found = block
                    .extrinsics()
                    .iter()
                    .any(|xt| {
                        let is_signed = self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::Extrinsic::is_signed(
                                xt,
                            )
                            .unwrap_or(false);
                        if !is_signed {
                            let call = <UncheckedExtrinsic as ExtrinsicCall>::call(xt);
                            if let Some(call) = IsSubType::<_>::is_sub_type(call) {
                                Timestamp::is_inherent(&call)
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    });
                if !found {
                    result
                        .put_error(Timestamp::INHERENT_IDENTIFIER, &e)
                        .expect("There is only one fatal error; qed");
                    if e.is_fatal_error() {
                        return result;
                    }
                }
            }
            Ok(None) => {}
            Err(e) => {
                result
                    .put_error(Timestamp::INHERENT_IDENTIFIER, &e)
                    .expect("There is only one fatal error; qed");
                if e.is_fatal_error() {
                    return result;
                }
            }
        }
        result
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::EnsureInherentsAreFirst<
    Block,
> for Runtime {
    fn ensure_inherents_are_first(block: &Block) -> Result<(), u32> {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::ProvideInherent;
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::{
            IsSubType, ExtrinsicCall,
        };
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block as _;
        let mut first_signed_observed = false;
        for (i, xt) in block.extrinsics().iter().enumerate() {
            let is_signed = self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::Extrinsic::is_signed(
                    xt,
                )
                .unwrap_or(false);
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
for Runtime {
    type Call = RuntimeCall;
    fn pre_dispatch(
        call: &Self::Call,
    ) -> Result<
        (),
        self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::TransactionValidityError,
    > {
        #[allow(unreachable_patterns)]
        match call {
            RuntimeCall::Grandpa(inner_call) => Grandpa::pre_dispatch(inner_call),
            _ => Ok(()),
        }
    }
    fn validate_unsigned(
        #[allow(unused_variables)]
        source: self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::TransactionSource,
        call: &Self::Call,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::TransactionValidity {
        #[allow(unreachable_patterns)]
        match call {
            RuntimeCall::Grandpa(inner_call) => {
                Grandpa::validate_unsigned(source, inner_call)
            }
            _ => {
                self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::UnknownTransaction::NoUnsignedValidator
                    .into()
            }
        }
    }
}
/// A reason for placing a freeze on funds.
pub enum RuntimeFreezeReason {}
#[automatically_derived]
impl ::core::marker::Copy for RuntimeFreezeReason {}
#[automatically_derived]
impl ::core::clone::Clone for RuntimeFreezeReason {
    #[inline]
    fn clone(&self) -> RuntimeFreezeReason {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for RuntimeFreezeReason {}
#[automatically_derived]
impl ::core::cmp::Eq for RuntimeFreezeReason {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for RuntimeFreezeReason {}
#[automatically_derived]
impl ::core::cmp::PartialEq for RuntimeFreezeReason {
    #[inline]
    fn eq(&self, other: &RuntimeFreezeReason) -> bool {
        unsafe { ::core::intrinsics::unreachable() }
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for RuntimeFreezeReason {
    #[inline]
    fn cmp(&self, other: &RuntimeFreezeReason) -> ::core::cmp::Ordering {
        unsafe { ::core::intrinsics::unreachable() }
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for RuntimeFreezeReason {
    #[inline]
    fn partial_cmp(
        &self,
        other: &RuntimeFreezeReason,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        unsafe { ::core::intrinsics::unreachable() }
    }
}
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Encode for RuntimeFreezeReason {}
    #[automatically_derived]
    impl ::codec::EncodeLike for RuntimeFreezeReason {}
};
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Decode for RuntimeFreezeReason {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| {
                    e
                        .chain(
                            "Could not decode `RuntimeFreezeReason`, failed to read variant byte",
                        )
                })?
            {
                _ => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into(
                                "Could not decode `RuntimeFreezeReason`, variant doesn't exist",
                            ),
                        )
                    })();
                }
            }
        }
    }
};
const _: () = {
    impl ::codec::MaxEncodedLen for RuntimeFreezeReason {
        fn max_encoded_len() -> ::core::primitive::usize {
            0_usize.saturating_add(1)
        }
    }
};
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for RuntimeFreezeReason {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(
                    ::scale_info::Path::new(
                        "RuntimeFreezeReason",
                        "node_template_runtime",
                    ),
                )
                .type_params(::alloc::vec::Vec::new())
                .docs(&["A reason for placing a freeze on funds."])
                .variant(::scale_info::build::Variants::new())
        }
    }
};
impl core::fmt::Debug for RuntimeFreezeReason {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            _ => Ok(()),
        }
    }
}
/// A reason for placing a hold on funds.
pub enum RuntimeHoldReason {}
#[automatically_derived]
impl ::core::marker::Copy for RuntimeHoldReason {}
#[automatically_derived]
impl ::core::clone::Clone for RuntimeHoldReason {
    #[inline]
    fn clone(&self) -> RuntimeHoldReason {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for RuntimeHoldReason {}
#[automatically_derived]
impl ::core::cmp::Eq for RuntimeHoldReason {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for RuntimeHoldReason {}
#[automatically_derived]
impl ::core::cmp::PartialEq for RuntimeHoldReason {
    #[inline]
    fn eq(&self, other: &RuntimeHoldReason) -> bool {
        unsafe { ::core::intrinsics::unreachable() }
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for RuntimeHoldReason {
    #[inline]
    fn cmp(&self, other: &RuntimeHoldReason) -> ::core::cmp::Ordering {
        unsafe { ::core::intrinsics::unreachable() }
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for RuntimeHoldReason {
    #[inline]
    fn partial_cmp(
        &self,
        other: &RuntimeHoldReason,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        unsafe { ::core::intrinsics::unreachable() }
    }
}
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Encode for RuntimeHoldReason {}
    #[automatically_derived]
    impl ::codec::EncodeLike for RuntimeHoldReason {}
};
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Decode for RuntimeHoldReason {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| {
                    e
                        .chain(
                            "Could not decode `RuntimeHoldReason`, failed to read variant byte",
                        )
                })?
            {
                _ => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into(
                                "Could not decode `RuntimeHoldReason`, variant doesn't exist",
                            ),
                        )
                    })();
                }
            }
        }
    }
};
const _: () = {
    impl ::codec::MaxEncodedLen for RuntimeHoldReason {
        fn max_encoded_len() -> ::core::primitive::usize {
            0_usize.saturating_add(1)
        }
    }
};
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for RuntimeHoldReason {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(
                    ::scale_info::Path::new("RuntimeHoldReason", "node_template_runtime"),
                )
                .type_params(::alloc::vec::Vec::new())
                .docs(&["A reason for placing a hold on funds."])
                .variant(::scale_info::build::Variants::new())
        }
    }
};
impl core::fmt::Debug for RuntimeHoldReason {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            _ => Ok(()),
        }
    }
}
/// An identifier for each lock placed on funds.
pub enum RuntimeLockId {}
#[automatically_derived]
impl ::core::marker::Copy for RuntimeLockId {}
#[automatically_derived]
impl ::core::clone::Clone for RuntimeLockId {
    #[inline]
    fn clone(&self) -> RuntimeLockId {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for RuntimeLockId {}
#[automatically_derived]
impl ::core::cmp::Eq for RuntimeLockId {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for RuntimeLockId {}
#[automatically_derived]
impl ::core::cmp::PartialEq for RuntimeLockId {
    #[inline]
    fn eq(&self, other: &RuntimeLockId) -> bool {
        unsafe { ::core::intrinsics::unreachable() }
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for RuntimeLockId {
    #[inline]
    fn cmp(&self, other: &RuntimeLockId) -> ::core::cmp::Ordering {
        unsafe { ::core::intrinsics::unreachable() }
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for RuntimeLockId {
    #[inline]
    fn partial_cmp(
        &self,
        other: &RuntimeLockId,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        unsafe { ::core::intrinsics::unreachable() }
    }
}
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Encode for RuntimeLockId {}
    #[automatically_derived]
    impl ::codec::EncodeLike for RuntimeLockId {}
};
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Decode for RuntimeLockId {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| {
                    e
                        .chain(
                            "Could not decode `RuntimeLockId`, failed to read variant byte",
                        )
                })?
            {
                _ => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into(
                                "Could not decode `RuntimeLockId`, variant doesn't exist",
                            ),
                        )
                    })();
                }
            }
        }
    }
};
const _: () = {
    impl ::codec::MaxEncodedLen for RuntimeLockId {
        fn max_encoded_len() -> ::core::primitive::usize {
            0_usize.saturating_add(1)
        }
    }
};
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for RuntimeLockId {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(::scale_info::Path::new("RuntimeLockId", "node_template_runtime"))
                .type_params(::alloc::vec::Vec::new())
                .docs(&["An identifier for each lock placed on funds."])
                .variant(::scale_info::build::Variants::new())
        }
    }
};
impl core::fmt::Debug for RuntimeLockId {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            _ => Ok(()),
        }
    }
}
/// A reason for slashing funds.
pub enum RuntimeSlashReason {}
#[automatically_derived]
impl ::core::marker::Copy for RuntimeSlashReason {}
#[automatically_derived]
impl ::core::clone::Clone for RuntimeSlashReason {
    #[inline]
    fn clone(&self) -> RuntimeSlashReason {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for RuntimeSlashReason {}
#[automatically_derived]
impl ::core::cmp::Eq for RuntimeSlashReason {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for RuntimeSlashReason {}
#[automatically_derived]
impl ::core::cmp::PartialEq for RuntimeSlashReason {
    #[inline]
    fn eq(&self, other: &RuntimeSlashReason) -> bool {
        unsafe { ::core::intrinsics::unreachable() }
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for RuntimeSlashReason {
    #[inline]
    fn cmp(&self, other: &RuntimeSlashReason) -> ::core::cmp::Ordering {
        unsafe { ::core::intrinsics::unreachable() }
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for RuntimeSlashReason {
    #[inline]
    fn partial_cmp(
        &self,
        other: &RuntimeSlashReason,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        unsafe { ::core::intrinsics::unreachable() }
    }
}
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Encode for RuntimeSlashReason {}
    #[automatically_derived]
    impl ::codec::EncodeLike for RuntimeSlashReason {}
};
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Decode for RuntimeSlashReason {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| {
                    e
                        .chain(
                            "Could not decode `RuntimeSlashReason`, failed to read variant byte",
                        )
                })?
            {
                _ => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into(
                                "Could not decode `RuntimeSlashReason`, variant doesn't exist",
                            ),
                        )
                    })();
                }
            }
        }
    }
};
const _: () = {
    impl ::codec::MaxEncodedLen for RuntimeSlashReason {
        fn max_encoded_len() -> ::core::primitive::usize {
            0_usize.saturating_add(1)
        }
    }
};
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for RuntimeSlashReason {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(
                    ::scale_info::Path::new(
                        "RuntimeSlashReason",
                        "node_template_runtime",
                    ),
                )
                .type_params(::alloc::vec::Vec::new())
                .docs(&["A reason for slashing funds."])
                .variant(::scale_info::build::Variants::new())
        }
    }
};
impl core::fmt::Debug for RuntimeSlashReason {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            _ => Ok(()),
        }
    }
}
const _: () = if !(<frame_system::Error<
    Runtime,
> as ::frame_support::traits::PalletError>::MAX_ENCODED_SIZE
    <= ::frame_support::MAX_MODULE_ERROR_ENCODED_SIZE)
{
    {
        ::core::panicking::panic_fmt(
            format_args!(
                "The maximum encoded size of the error type in the `System` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`",
            ),
        );
    }
};
const _: () = if !(<pallet_grandpa::Error<
    Runtime,
> as ::frame_support::traits::PalletError>::MAX_ENCODED_SIZE
    <= ::frame_support::MAX_MODULE_ERROR_ENCODED_SIZE)
{
    {
        ::core::panicking::panic_fmt(
            format_args!(
                "The maximum encoded size of the error type in the `Grandpa` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`",
            ),
        );
    }
};
const _: () = if !(<pallet_balances::Error<
    Runtime,
> as ::frame_support::traits::PalletError>::MAX_ENCODED_SIZE
    <= ::frame_support::MAX_MODULE_ERROR_ENCODED_SIZE)
{
    {
        ::core::panicking::panic_fmt(
            format_args!(
                "The maximum encoded size of the error type in the `Balances` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`",
            ),
        );
    }
};
const _: () = if !(<pallet_sudo::Error<
    Runtime,
> as ::frame_support::traits::PalletError>::MAX_ENCODED_SIZE
    <= ::frame_support::MAX_MODULE_ERROR_ENCODED_SIZE)
{
    {
        ::core::panicking::panic_fmt(
            format_args!(
                "The maximum encoded size of the error type in the `Sudo` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`",
            ),
        );
    }
};
const _: () = if !(<pallet_template::Error<
    Runtime,
> as ::frame_support::traits::PalletError>::MAX_ENCODED_SIZE
    <= ::frame_support::MAX_MODULE_ERROR_ENCODED_SIZE)
{
    {
        ::core::panicking::panic_fmt(
            format_args!(
                "The maximum encoded size of the error type in the `TemplateModule` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`",
            ),
        );
    }
};
/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckNonZeroSender<Runtime>,
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<
    Address,
    RuntimeCall,
    Signature,
    SignedExtra,
>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;
pub struct RuntimeApi {}
/// Implements all runtime apis for the client side.
#[cfg(any(feature = "std", test))]
pub struct RuntimeApiImpl<Block: sp_api::BlockT, C: sp_api::CallApiAt<Block> + 'static> {
    call: &'static C,
    commit_on_success: std::cell::RefCell<bool>,
    changes: std::cell::RefCell<sp_api::OverlayedChanges>,
    storage_transaction_cache: std::cell::RefCell<
        sp_api::StorageTransactionCache<Block, C::StateBackend>,
    >,
    recorder: std::option::Option<sp_api::ProofRecorder<Block>>,
}
#[cfg(any(feature = "std", test))]
impl<Block: sp_api::BlockT, C: sp_api::CallApiAt<Block>> sp_api::ApiExt<Block>
for RuntimeApiImpl<Block, C> {
    type StateBackend = C::StateBackend;
    fn execute_in_transaction<F: FnOnce(&Self) -> sp_api::TransactionOutcome<R>, R>(
        &self,
        call: F,
    ) -> R
    where
        Self: Sized,
    {
        self.start_transaction();
        *std::cell::RefCell::borrow_mut(&self.commit_on_success) = false;
        let res = call(self);
        *std::cell::RefCell::borrow_mut(&self.commit_on_success) = true;
        self.commit_or_rollback(
            match res {
                sp_api::TransactionOutcome::Commit(_) => true,
                _ => false,
            },
        );
        res.into_inner()
    }
    fn has_api<A: sp_api::RuntimeApiInfo + ?Sized>(
        &self,
        at: <Block as sp_api::BlockT>::Hash,
    ) -> std::result::Result<bool, sp_api::ApiError>
    where
        Self: Sized,
    {
        sp_api::CallApiAt::<Block>::runtime_version_at(self.call, at)
            .map(|v| sp_api::RuntimeVersion::has_api_with(
                &v,
                &A::ID,
                |v| v == A::VERSION,
            ))
    }
    fn has_api_with<A: sp_api::RuntimeApiInfo + ?Sized, P: Fn(u32) -> bool>(
        &self,
        at: <Block as sp_api::BlockT>::Hash,
        pred: P,
    ) -> std::result::Result<bool, sp_api::ApiError>
    where
        Self: Sized,
    {
        sp_api::CallApiAt::<Block>::runtime_version_at(self.call, at)
            .map(|v| sp_api::RuntimeVersion::has_api_with(&v, &A::ID, pred))
    }
    fn api_version<A: sp_api::RuntimeApiInfo + ?Sized>(
        &self,
        at: <Block as sp_api::BlockT>::Hash,
    ) -> std::result::Result<Option<u32>, sp_api::ApiError>
    where
        Self: Sized,
    {
        sp_api::CallApiAt::<Block>::runtime_version_at(self.call, at)
            .map(|v| sp_api::RuntimeVersion::api_version(&v, &A::ID))
    }
    fn record_proof(&mut self) {
        self.recorder = std::option::Option::Some(std::default::Default::default());
    }
    fn proof_recorder(&self) -> std::option::Option<sp_api::ProofRecorder<Block>> {
        std::clone::Clone::clone(&self.recorder)
    }
    fn extract_proof(&mut self) -> std::option::Option<sp_api::StorageProof> {
        let recorder = std::option::Option::take(&mut self.recorder);
        std::option::Option::map(
            recorder,
            |recorder| { sp_api::ProofRecorder::<Block>::drain_storage_proof(recorder) },
        )
    }
    fn into_storage_changes(
        &self,
        backend: &Self::StateBackend,
        parent_hash: Block::Hash,
    ) -> core::result::Result<sp_api::StorageChanges<C::StateBackend, Block>, String>
    where
        Self: Sized,
    {
        let state_version = sp_api::CallApiAt::<
            Block,
        >::runtime_version_at(self.call, std::clone::Clone::clone(&parent_hash))
            .map(|v| sp_api::RuntimeVersion::state_version(&v))
            .map_err(|e| {
                let res = ::alloc::fmt::format(
                    format_args!("Failed to get state version: {0}", e),
                );
                res
            })?;
        sp_api::OverlayedChanges::into_storage_changes(
            std::cell::RefCell::take(&self.changes),
            backend,
            core::cell::RefCell::take(&self.storage_transaction_cache),
            state_version,
        )
    }
}
#[cfg(any(feature = "std", test))]
impl<Block: sp_api::BlockT, C> sp_api::ConstructRuntimeApi<Block, C> for RuntimeApi
where
    C: sp_api::CallApiAt<Block> + 'static,
{
    type RuntimeApi = RuntimeApiImpl<Block, C>;
    fn construct_runtime_api<'a>(call: &'a C) -> sp_api::ApiRef<'a, Self::RuntimeApi> {
        RuntimeApiImpl {
            call: unsafe { std::mem::transmute(call) },
            commit_on_success: true.into(),
            changes: std::default::Default::default(),
            recorder: std::default::Default::default(),
            storage_transaction_cache: std::default::Default::default(),
        }
            .into()
    }
}
#[cfg(any(feature = "std", test))]
impl<Block: sp_api::BlockT, C: sp_api::CallApiAt<Block>> RuntimeApiImpl<Block, C> {
    fn commit_or_rollback(&self, commit: bool) {
        let proof = "\
					We only close a transaction when we opened one ourself.
					Other parts of the runtime that make use of transactions (state-machine)
					also balance their transactions. The runtime cannot close client initiated
					transactions; qed";
        if *std::cell::RefCell::borrow(&self.commit_on_success) {
            let res = if commit {
                let res = if let Some(recorder) = &self.recorder {
                    sp_api::ProofRecorder::<Block>::commit_transaction(&recorder)
                } else {
                    Ok(())
                };
                let res2 = sp_api::OverlayedChanges::commit_transaction(
                    &mut std::cell::RefCell::borrow_mut(&self.changes),
                );
                std::result::Result::and(res, std::result::Result::map_err(res2, drop))
            } else {
                let res = if let Some(recorder) = &self.recorder {
                    sp_api::ProofRecorder::<Block>::rollback_transaction(&recorder)
                } else {
                    Ok(())
                };
                let res2 = sp_api::OverlayedChanges::rollback_transaction(
                    &mut std::cell::RefCell::borrow_mut(&self.changes),
                );
                std::result::Result::and(res, std::result::Result::map_err(res2, drop))
            };
            std::result::Result::expect(res, proof);
        }
    }
    fn start_transaction(&self) {
        if !*std::cell::RefCell::borrow(&self.commit_on_success) {
            return;
        }
        sp_api::OverlayedChanges::start_transaction(
            &mut std::cell::RefCell::borrow_mut(&self.changes),
        );
        if let Some(recorder) = &self.recorder {
            sp_api::ProofRecorder::<Block>::start_transaction(&recorder);
        }
    }
}
impl sp_api::runtime_decl_for_core::Core<Block> for Runtime {
    fn version() -> RuntimeVersion {
        VERSION
    }
    fn execute_block(block: Block) {
        Executive::execute_block(block);
    }
    fn initialize_block(header: &<Block as BlockT>::Header) {
        Executive::initialize_block(header)
    }
}
impl sp_api::runtime_decl_for_metadata::Metadata<Block> for Runtime {
    fn metadata() -> OpaqueMetadata {
        OpaqueMetadata::new(Runtime::metadata().into())
    }
    fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
        Runtime::metadata_at_version(version)
    }
    fn metadata_versions() -> sp_std::vec::Vec<u32> {
        Runtime::metadata_versions()
    }
}
impl sp_block_builder::runtime_decl_for_block_builder::BlockBuilder<Block> for Runtime {
    fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
        Executive::apply_extrinsic(extrinsic)
    }
    fn finalize_block() -> <Block as BlockT>::Header {
        Executive::finalize_block()
    }
    fn inherent_extrinsics(
        data: sp_inherents::InherentData,
    ) -> Vec<<Block as BlockT>::Extrinsic> {
        data.create_extrinsics()
    }
    fn check_inherents(
        block: Block,
        data: sp_inherents::InherentData,
    ) -> sp_inherents::CheckInherentsResult {
        data.check_extrinsics(&block)
    }
}
impl sp_transaction_pool::runtime_api::runtime_decl_for_tagged_transaction_queue::TaggedTransactionQueue<
    Block,
> for Runtime {
    fn validate_transaction(
        source: TransactionSource,
        tx: <Block as BlockT>::Extrinsic,
        block_hash: <Block as BlockT>::Hash,
    ) -> TransactionValidity {
        Executive::validate_transaction(source, tx, block_hash)
    }
}
impl sp_offchain::runtime_decl_for_offchain_worker_api::OffchainWorkerApi<Block>
for Runtime {
    fn offchain_worker(header: &<Block as BlockT>::Header) {
        Executive::offchain_worker(header)
    }
}
impl sp_consensus_aura::runtime_decl_for_aura_api::AuraApi<Block, AuraId> for Runtime {
    fn slot_duration() -> sp_consensus_aura::SlotDuration {
        sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
    }
    fn authorities() -> Vec<AuraId> {
        Aura::authorities().into_inner()
    }
}
impl sp_session::runtime_decl_for_session_keys::SessionKeys<Block> for Runtime {
    fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
        opaque::SessionKeys::generate(seed)
    }
    fn decode_session_keys(encoded: Vec<u8>) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
        opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
    }
}
impl sp_consensus_grandpa::runtime_decl_for_grandpa_api::GrandpaApi<Block> for Runtime {
    fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
        Grandpa::grandpa_authorities()
    }
    fn current_set_id() -> sp_consensus_grandpa::SetId {
        Grandpa::current_set_id()
    }
    fn submit_report_equivocation_unsigned_extrinsic(
        _equivocation_proof: sp_consensus_grandpa::EquivocationProof<
            <Block as BlockT>::Hash,
            NumberFor<Block>,
        >,
        _key_owner_proof: sp_consensus_grandpa::OpaqueKeyOwnershipProof,
    ) -> Option<()> {
        None
    }
    fn generate_key_ownership_proof(
        _set_id: sp_consensus_grandpa::SetId,
        _authority_id: GrandpaId,
    ) -> Option<sp_consensus_grandpa::OpaqueKeyOwnershipProof> {
        None
    }
}
impl frame_system_rpc_runtime_api::runtime_decl_for_account_nonce_api::AccountNonceApi<
    Block,
    AccountId,
    Index,
> for Runtime {
    fn account_nonce(account: AccountId) -> Index {
        System::account_nonce(account)
    }
}
impl pallet_transaction_payment_rpc_runtime_api::runtime_decl_for_transaction_payment_api::TransactionPaymentApi<
    Block,
    Balance,
> for Runtime {
    fn query_info(
        uxt: <Block as BlockT>::Extrinsic,
        len: u32,
    ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
        TransactionPayment::query_info(uxt, len)
    }
    fn query_fee_details(
        uxt: <Block as BlockT>::Extrinsic,
        len: u32,
    ) -> pallet_transaction_payment::FeeDetails<Balance> {
        TransactionPayment::query_fee_details(uxt, len)
    }
    fn query_weight_to_fee(weight: Weight) -> Balance {
        TransactionPayment::weight_to_fee(weight)
    }
    fn query_length_to_fee(length: u32) -> Balance {
        TransactionPayment::length_to_fee(length)
    }
}
impl pallet_transaction_payment_rpc_runtime_api::runtime_decl_for_transaction_payment_call_api::TransactionPaymentCallApi<
    Block,
    Balance,
    RuntimeCall,
> for Runtime {
    fn query_call_info(
        call: RuntimeCall,
        len: u32,
    ) -> pallet_transaction_payment::RuntimeDispatchInfo<Balance> {
        TransactionPayment::query_call_info(call, len)
    }
    fn query_call_fee_details(
        call: RuntimeCall,
        len: u32,
    ) -> pallet_transaction_payment::FeeDetails<Balance> {
        TransactionPayment::query_call_fee_details(call, len)
    }
    fn query_weight_to_fee(weight: Weight) -> Balance {
        TransactionPayment::weight_to_fee(weight)
    }
    fn query_length_to_fee(length: u32) -> Balance {
        TransactionPayment::length_to_fee(length)
    }
}
#[cfg(any(feature = "std", test))]
impl<
    __SrApiBlock__: sp_api::BlockT + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    RuntimeApiImplCall: sp_api::CallApiAt<__SrApiBlock__> + 'static,
> sp_api::Core<__SrApiBlock__> for RuntimeApiImpl<__SrApiBlock__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend: sp_api::StateBackend<
        sp_api::HashFor<__SrApiBlock__>,
    >,
    &'static RuntimeApiImplCall: Send,
    RuntimeVersion: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SrApiBlock__: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    <__SrApiBlock__ as BlockT>::Header: std::panic::UnwindSafe
        + std::panic::RefUnwindSafe,
    __SrApiBlock__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn __runtime_api_internal_call_api_at(
        &self,
        at: <__SrApiBlock__ as sp_api::BlockT>::Hash,
        context: sp_api::ExecutionContext,
        params: std::vec::Vec<u8>,
        fn_name: &dyn Fn(sp_api::RuntimeVersion) -> &'static str,
    ) -> std::result::Result<std::vec::Vec<u8>, sp_api::ApiError> {
        self.start_transaction();
        let res = (|| {
            let version = sp_api::CallApiAt::<
                __SrApiBlock__,
            >::runtime_version_at(self.call, at)?;
            let params = sp_api::CallApiAtParams {
                at,
                function: (*fn_name)(version),
                arguments: params,
                overlayed_changes: &self.changes,
                storage_transaction_cache: &self.storage_transaction_cache,
                context,
                recorder: &self.recorder,
            };
            sp_api::CallApiAt::<__SrApiBlock__>::call_api_at(self.call, params)
        })();
        self.commit_or_rollback(std::result::Result::is_ok(&res));
        res
    }
}
#[cfg(any(feature = "std", test))]
impl<
    __SrApiBlock__: sp_api::BlockT + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    RuntimeApiImplCall: sp_api::CallApiAt<__SrApiBlock__> + 'static,
> sp_api::Metadata<__SrApiBlock__> for RuntimeApiImpl<__SrApiBlock__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend: sp_api::StateBackend<
        sp_api::HashFor<__SrApiBlock__>,
    >,
    &'static RuntimeApiImplCall: Send,
    OpaqueMetadata: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    u32: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Option<OpaqueMetadata>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    sp_std::vec::Vec<u32>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SrApiBlock__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn __runtime_api_internal_call_api_at(
        &self,
        at: <__SrApiBlock__ as sp_api::BlockT>::Hash,
        context: sp_api::ExecutionContext,
        params: std::vec::Vec<u8>,
        fn_name: &dyn Fn(sp_api::RuntimeVersion) -> &'static str,
    ) -> std::result::Result<std::vec::Vec<u8>, sp_api::ApiError> {
        self.start_transaction();
        let res = (|| {
            let version = sp_api::CallApiAt::<
                __SrApiBlock__,
            >::runtime_version_at(self.call, at)?;
            let params = sp_api::CallApiAtParams {
                at,
                function: (*fn_name)(version),
                arguments: params,
                overlayed_changes: &self.changes,
                storage_transaction_cache: &self.storage_transaction_cache,
                context,
                recorder: &self.recorder,
            };
            sp_api::CallApiAt::<__SrApiBlock__>::call_api_at(self.call, params)
        })();
        self.commit_or_rollback(std::result::Result::is_ok(&res));
        res
    }
}
#[cfg(any(feature = "std", test))]
impl<
    __SrApiBlock__: sp_api::BlockT + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    RuntimeApiImplCall: sp_api::CallApiAt<__SrApiBlock__> + 'static,
> sp_block_builder::BlockBuilder<__SrApiBlock__>
for RuntimeApiImpl<__SrApiBlock__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend: sp_api::StateBackend<
        sp_api::HashFor<__SrApiBlock__>,
    >,
    &'static RuntimeApiImplCall: Send,
    <__SrApiBlock__ as BlockT>::Extrinsic: std::panic::UnwindSafe
        + std::panic::RefUnwindSafe,
    ApplyExtrinsicResult: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    <__SrApiBlock__ as BlockT>::Header: std::panic::UnwindSafe
        + std::panic::RefUnwindSafe,
    sp_inherents::InherentData: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Vec<
        <__SrApiBlock__ as BlockT>::Extrinsic,
    >: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SrApiBlock__: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    sp_inherents::InherentData: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    sp_inherents::CheckInherentsResult: std::panic::UnwindSafe
        + std::panic::RefUnwindSafe,
    __SrApiBlock__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn __runtime_api_internal_call_api_at(
        &self,
        at: <__SrApiBlock__ as sp_api::BlockT>::Hash,
        context: sp_api::ExecutionContext,
        params: std::vec::Vec<u8>,
        fn_name: &dyn Fn(sp_api::RuntimeVersion) -> &'static str,
    ) -> std::result::Result<std::vec::Vec<u8>, sp_api::ApiError> {
        self.start_transaction();
        let res = (|| {
            let version = sp_api::CallApiAt::<
                __SrApiBlock__,
            >::runtime_version_at(self.call, at)?;
            let params = sp_api::CallApiAtParams {
                at,
                function: (*fn_name)(version),
                arguments: params,
                overlayed_changes: &self.changes,
                storage_transaction_cache: &self.storage_transaction_cache,
                context,
                recorder: &self.recorder,
            };
            sp_api::CallApiAt::<__SrApiBlock__>::call_api_at(self.call, params)
        })();
        self.commit_or_rollback(std::result::Result::is_ok(&res));
        res
    }
}
#[cfg(any(feature = "std", test))]
impl<
    __SrApiBlock__: sp_api::BlockT + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    RuntimeApiImplCall: sp_api::CallApiAt<__SrApiBlock__> + 'static,
> sp_transaction_pool::runtime_api::TaggedTransactionQueue<__SrApiBlock__>
for RuntimeApiImpl<__SrApiBlock__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend: sp_api::StateBackend<
        sp_api::HashFor<__SrApiBlock__>,
    >,
    &'static RuntimeApiImplCall: Send,
    TransactionSource: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    <__SrApiBlock__ as BlockT>::Extrinsic: std::panic::UnwindSafe
        + std::panic::RefUnwindSafe,
    <__SrApiBlock__ as BlockT>::Hash: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    TransactionValidity: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SrApiBlock__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn __runtime_api_internal_call_api_at(
        &self,
        at: <__SrApiBlock__ as sp_api::BlockT>::Hash,
        context: sp_api::ExecutionContext,
        params: std::vec::Vec<u8>,
        fn_name: &dyn Fn(sp_api::RuntimeVersion) -> &'static str,
    ) -> std::result::Result<std::vec::Vec<u8>, sp_api::ApiError> {
        self.start_transaction();
        let res = (|| {
            let version = sp_api::CallApiAt::<
                __SrApiBlock__,
            >::runtime_version_at(self.call, at)?;
            let params = sp_api::CallApiAtParams {
                at,
                function: (*fn_name)(version),
                arguments: params,
                overlayed_changes: &self.changes,
                storage_transaction_cache: &self.storage_transaction_cache,
                context,
                recorder: &self.recorder,
            };
            sp_api::CallApiAt::<__SrApiBlock__>::call_api_at(self.call, params)
        })();
        self.commit_or_rollback(std::result::Result::is_ok(&res));
        res
    }
}
#[cfg(any(feature = "std", test))]
impl<
    __SrApiBlock__: sp_api::BlockT + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    RuntimeApiImplCall: sp_api::CallApiAt<__SrApiBlock__> + 'static,
> sp_offchain::OffchainWorkerApi<__SrApiBlock__>
for RuntimeApiImpl<__SrApiBlock__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend: sp_api::StateBackend<
        sp_api::HashFor<__SrApiBlock__>,
    >,
    &'static RuntimeApiImplCall: Send,
    <__SrApiBlock__ as BlockT>::Header: std::panic::UnwindSafe
        + std::panic::RefUnwindSafe,
    __SrApiBlock__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn __runtime_api_internal_call_api_at(
        &self,
        at: <__SrApiBlock__ as sp_api::BlockT>::Hash,
        context: sp_api::ExecutionContext,
        params: std::vec::Vec<u8>,
        fn_name: &dyn Fn(sp_api::RuntimeVersion) -> &'static str,
    ) -> std::result::Result<std::vec::Vec<u8>, sp_api::ApiError> {
        self.start_transaction();
        let res = (|| {
            let version = sp_api::CallApiAt::<
                __SrApiBlock__,
            >::runtime_version_at(self.call, at)?;
            let params = sp_api::CallApiAtParams {
                at,
                function: (*fn_name)(version),
                arguments: params,
                overlayed_changes: &self.changes,
                storage_transaction_cache: &self.storage_transaction_cache,
                context,
                recorder: &self.recorder,
            };
            sp_api::CallApiAt::<__SrApiBlock__>::call_api_at(self.call, params)
        })();
        self.commit_or_rollback(std::result::Result::is_ok(&res));
        res
    }
}
#[cfg(any(feature = "std", test))]
impl<
    __SrApiBlock__: sp_api::BlockT + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    RuntimeApiImplCall: sp_api::CallApiAt<__SrApiBlock__> + 'static,
> sp_consensus_aura::AuraApi<__SrApiBlock__, AuraId>
for RuntimeApiImpl<__SrApiBlock__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend: sp_api::StateBackend<
        sp_api::HashFor<__SrApiBlock__>,
    >,
    &'static RuntimeApiImplCall: Send,
    sp_consensus_aura::SlotDuration: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Vec<AuraId>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SrApiBlock__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn __runtime_api_internal_call_api_at(
        &self,
        at: <__SrApiBlock__ as sp_api::BlockT>::Hash,
        context: sp_api::ExecutionContext,
        params: std::vec::Vec<u8>,
        fn_name: &dyn Fn(sp_api::RuntimeVersion) -> &'static str,
    ) -> std::result::Result<std::vec::Vec<u8>, sp_api::ApiError> {
        self.start_transaction();
        let res = (|| {
            let version = sp_api::CallApiAt::<
                __SrApiBlock__,
            >::runtime_version_at(self.call, at)?;
            let params = sp_api::CallApiAtParams {
                at,
                function: (*fn_name)(version),
                arguments: params,
                overlayed_changes: &self.changes,
                storage_transaction_cache: &self.storage_transaction_cache,
                context,
                recorder: &self.recorder,
            };
            sp_api::CallApiAt::<__SrApiBlock__>::call_api_at(self.call, params)
        })();
        self.commit_or_rollback(std::result::Result::is_ok(&res));
        res
    }
}
#[cfg(any(feature = "std", test))]
impl<
    __SrApiBlock__: sp_api::BlockT + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    RuntimeApiImplCall: sp_api::CallApiAt<__SrApiBlock__> + 'static,
> sp_session::SessionKeys<__SrApiBlock__>
for RuntimeApiImpl<__SrApiBlock__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend: sp_api::StateBackend<
        sp_api::HashFor<__SrApiBlock__>,
    >,
    &'static RuntimeApiImplCall: Send,
    Option<Vec<u8>>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Vec<u8>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Vec<u8>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Option<
        Vec<(Vec<u8>, KeyTypeId)>,
    >: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SrApiBlock__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn __runtime_api_internal_call_api_at(
        &self,
        at: <__SrApiBlock__ as sp_api::BlockT>::Hash,
        context: sp_api::ExecutionContext,
        params: std::vec::Vec<u8>,
        fn_name: &dyn Fn(sp_api::RuntimeVersion) -> &'static str,
    ) -> std::result::Result<std::vec::Vec<u8>, sp_api::ApiError> {
        self.start_transaction();
        let res = (|| {
            let version = sp_api::CallApiAt::<
                __SrApiBlock__,
            >::runtime_version_at(self.call, at)?;
            let params = sp_api::CallApiAtParams {
                at,
                function: (*fn_name)(version),
                arguments: params,
                overlayed_changes: &self.changes,
                storage_transaction_cache: &self.storage_transaction_cache,
                context,
                recorder: &self.recorder,
            };
            sp_api::CallApiAt::<__SrApiBlock__>::call_api_at(self.call, params)
        })();
        self.commit_or_rollback(std::result::Result::is_ok(&res));
        res
    }
}
#[cfg(any(feature = "std", test))]
impl<
    __SrApiBlock__: sp_api::BlockT + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    RuntimeApiImplCall: sp_api::CallApiAt<__SrApiBlock__> + 'static,
> sp_consensus_grandpa::GrandpaApi<__SrApiBlock__>
for RuntimeApiImpl<__SrApiBlock__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend: sp_api::StateBackend<
        sp_api::HashFor<__SrApiBlock__>,
    >,
    &'static RuntimeApiImplCall: Send,
    sp_consensus_grandpa::AuthorityList: std::panic::UnwindSafe
        + std::panic::RefUnwindSafe,
    sp_consensus_grandpa::SetId: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    sp_consensus_grandpa::EquivocationProof<
        <__SrApiBlock__ as BlockT>::Hash,
        NumberFor<__SrApiBlock__>,
    >: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    sp_consensus_grandpa::OpaqueKeyOwnershipProof: std::panic::UnwindSafe
        + std::panic::RefUnwindSafe,
    Option<()>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    sp_consensus_grandpa::SetId: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    GrandpaId: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Option<
        sp_consensus_grandpa::OpaqueKeyOwnershipProof,
    >: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SrApiBlock__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn __runtime_api_internal_call_api_at(
        &self,
        at: <__SrApiBlock__ as sp_api::BlockT>::Hash,
        context: sp_api::ExecutionContext,
        params: std::vec::Vec<u8>,
        fn_name: &dyn Fn(sp_api::RuntimeVersion) -> &'static str,
    ) -> std::result::Result<std::vec::Vec<u8>, sp_api::ApiError> {
        self.start_transaction();
        let res = (|| {
            let version = sp_api::CallApiAt::<
                __SrApiBlock__,
            >::runtime_version_at(self.call, at)?;
            let params = sp_api::CallApiAtParams {
                at,
                function: (*fn_name)(version),
                arguments: params,
                overlayed_changes: &self.changes,
                storage_transaction_cache: &self.storage_transaction_cache,
                context,
                recorder: &self.recorder,
            };
            sp_api::CallApiAt::<__SrApiBlock__>::call_api_at(self.call, params)
        })();
        self.commit_or_rollback(std::result::Result::is_ok(&res));
        res
    }
}
#[cfg(any(feature = "std", test))]
impl<
    __SrApiBlock__: sp_api::BlockT + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    RuntimeApiImplCall: sp_api::CallApiAt<__SrApiBlock__> + 'static,
> frame_system_rpc_runtime_api::AccountNonceApi<__SrApiBlock__, AccountId, Index>
for RuntimeApiImpl<__SrApiBlock__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend: sp_api::StateBackend<
        sp_api::HashFor<__SrApiBlock__>,
    >,
    &'static RuntimeApiImplCall: Send,
    AccountId: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Index: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SrApiBlock__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn __runtime_api_internal_call_api_at(
        &self,
        at: <__SrApiBlock__ as sp_api::BlockT>::Hash,
        context: sp_api::ExecutionContext,
        params: std::vec::Vec<u8>,
        fn_name: &dyn Fn(sp_api::RuntimeVersion) -> &'static str,
    ) -> std::result::Result<std::vec::Vec<u8>, sp_api::ApiError> {
        self.start_transaction();
        let res = (|| {
            let version = sp_api::CallApiAt::<
                __SrApiBlock__,
            >::runtime_version_at(self.call, at)?;
            let params = sp_api::CallApiAtParams {
                at,
                function: (*fn_name)(version),
                arguments: params,
                overlayed_changes: &self.changes,
                storage_transaction_cache: &self.storage_transaction_cache,
                context,
                recorder: &self.recorder,
            };
            sp_api::CallApiAt::<__SrApiBlock__>::call_api_at(self.call, params)
        })();
        self.commit_or_rollback(std::result::Result::is_ok(&res));
        res
    }
}
#[cfg(any(feature = "std", test))]
impl<
    __SrApiBlock__: sp_api::BlockT + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    RuntimeApiImplCall: sp_api::CallApiAt<__SrApiBlock__> + 'static,
> pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<
    __SrApiBlock__,
    Balance,
> for RuntimeApiImpl<__SrApiBlock__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend: sp_api::StateBackend<
        sp_api::HashFor<__SrApiBlock__>,
    >,
    &'static RuntimeApiImplCall: Send,
    <__SrApiBlock__ as BlockT>::Extrinsic: std::panic::UnwindSafe
        + std::panic::RefUnwindSafe,
    u32: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<
        Balance,
    >: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    <__SrApiBlock__ as BlockT>::Extrinsic: std::panic::UnwindSafe
        + std::panic::RefUnwindSafe,
    u32: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    pallet_transaction_payment::FeeDetails<
        Balance,
    >: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Weight: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Balance: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    u32: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Balance: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SrApiBlock__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn __runtime_api_internal_call_api_at(
        &self,
        at: <__SrApiBlock__ as sp_api::BlockT>::Hash,
        context: sp_api::ExecutionContext,
        params: std::vec::Vec<u8>,
        fn_name: &dyn Fn(sp_api::RuntimeVersion) -> &'static str,
    ) -> std::result::Result<std::vec::Vec<u8>, sp_api::ApiError> {
        self.start_transaction();
        let res = (|| {
            let version = sp_api::CallApiAt::<
                __SrApiBlock__,
            >::runtime_version_at(self.call, at)?;
            let params = sp_api::CallApiAtParams {
                at,
                function: (*fn_name)(version),
                arguments: params,
                overlayed_changes: &self.changes,
                storage_transaction_cache: &self.storage_transaction_cache,
                context,
                recorder: &self.recorder,
            };
            sp_api::CallApiAt::<__SrApiBlock__>::call_api_at(self.call, params)
        })();
        self.commit_or_rollback(std::result::Result::is_ok(&res));
        res
    }
}
#[cfg(any(feature = "std", test))]
impl<
    __SrApiBlock__: sp_api::BlockT + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    RuntimeApiImplCall: sp_api::CallApiAt<__SrApiBlock__> + 'static,
> pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<
    __SrApiBlock__,
    Balance,
    RuntimeCall,
> for RuntimeApiImpl<__SrApiBlock__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend: sp_api::StateBackend<
        sp_api::HashFor<__SrApiBlock__>,
    >,
    &'static RuntimeApiImplCall: Send,
    RuntimeCall: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    u32: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    pallet_transaction_payment::RuntimeDispatchInfo<
        Balance,
    >: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    RuntimeCall: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    u32: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    pallet_transaction_payment::FeeDetails<
        Balance,
    >: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Weight: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Balance: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    u32: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Balance: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SrApiBlock__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn __runtime_api_internal_call_api_at(
        &self,
        at: <__SrApiBlock__ as sp_api::BlockT>::Hash,
        context: sp_api::ExecutionContext,
        params: std::vec::Vec<u8>,
        fn_name: &dyn Fn(sp_api::RuntimeVersion) -> &'static str,
    ) -> std::result::Result<std::vec::Vec<u8>, sp_api::ApiError> {
        self.start_transaction();
        let res = (|| {
            let version = sp_api::CallApiAt::<
                __SrApiBlock__,
            >::runtime_version_at(self.call, at)?;
            let params = sp_api::CallApiAtParams {
                at,
                function: (*fn_name)(version),
                arguments: params,
                overlayed_changes: &self.changes,
                storage_transaction_cache: &self.storage_transaction_cache,
                context,
                recorder: &self.recorder,
            };
            sp_api::CallApiAt::<__SrApiBlock__>::call_api_at(self.call, params)
        })();
        self.commit_or_rollback(std::result::Result::is_ok(&res));
        res
    }
}
const RUNTIME_API_VERSIONS: sp_api::ApisVec = ::sp_version::sp_std::borrow::Cow::Borrowed(
    &[
        (sp_api::runtime_decl_for_core::ID, sp_api::runtime_decl_for_core::VERSION),
        (
            sp_api::runtime_decl_for_metadata::ID,
            sp_api::runtime_decl_for_metadata::VERSION,
        ),
        (
            sp_block_builder::runtime_decl_for_block_builder::ID,
            sp_block_builder::runtime_decl_for_block_builder::VERSION,
        ),
        (
            sp_transaction_pool::runtime_api::runtime_decl_for_tagged_transaction_queue::ID,
            sp_transaction_pool::runtime_api::runtime_decl_for_tagged_transaction_queue::VERSION,
        ),
        (
            sp_offchain::runtime_decl_for_offchain_worker_api::ID,
            sp_offchain::runtime_decl_for_offchain_worker_api::VERSION,
        ),
        (
            sp_consensus_aura::runtime_decl_for_aura_api::ID,
            sp_consensus_aura::runtime_decl_for_aura_api::VERSION,
        ),
        (
            sp_session::runtime_decl_for_session_keys::ID,
            sp_session::runtime_decl_for_session_keys::VERSION,
        ),
        (
            sp_consensus_grandpa::runtime_decl_for_grandpa_api::ID,
            sp_consensus_grandpa::runtime_decl_for_grandpa_api::VERSION,
        ),
        (
            frame_system_rpc_runtime_api::runtime_decl_for_account_nonce_api::ID,
            frame_system_rpc_runtime_api::runtime_decl_for_account_nonce_api::VERSION,
        ),
        (
            pallet_transaction_payment_rpc_runtime_api::runtime_decl_for_transaction_payment_api::ID,
            pallet_transaction_payment_rpc_runtime_api::runtime_decl_for_transaction_payment_api::VERSION,
        ),
        (
            pallet_transaction_payment_rpc_runtime_api::runtime_decl_for_transaction_payment_call_api::ID,
            pallet_transaction_payment_rpc_runtime_api::runtime_decl_for_transaction_payment_call_api::VERSION,
        ),
    ],
);
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
#[doc(hidden)]
trait InternalImplRuntimeApis {
    #[inline(always)]
    fn runtime_metadata(
        &self,
    ) -> sp_api::vec::Vec<sp_api::metadata_ir::RuntimeApiMetadataIR> {
        <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                sp_api::runtime_decl_for_core::runtime_metadata::<Block>(),
                sp_api::runtime_decl_for_metadata::runtime_metadata::<Block>(),
                sp_block_builder::runtime_decl_for_block_builder::runtime_metadata::<
                    Block,
                >(),
                sp_transaction_pool::runtime_api::runtime_decl_for_tagged_transaction_queue::runtime_metadata::<
                    Block,
                >(),
                sp_offchain::runtime_decl_for_offchain_worker_api::runtime_metadata::<
                    Block,
                >(),
                sp_consensus_aura::runtime_decl_for_aura_api::runtime_metadata::<
                    Block,
                    AuraId,
                >(),
                sp_session::runtime_decl_for_session_keys::runtime_metadata::<Block>(),
                sp_consensus_grandpa::runtime_decl_for_grandpa_api::runtime_metadata::<
                    Block,
                >(),
                frame_system_rpc_runtime_api::runtime_decl_for_account_nonce_api::runtime_metadata::<
                    Block,
                    AccountId,
                    Index,
                >(),
                pallet_transaction_payment_rpc_runtime_api::runtime_decl_for_transaction_payment_api::runtime_metadata::<
                    Block,
                    Balance,
                >(),
                pallet_transaction_payment_rpc_runtime_api::runtime_decl_for_transaction_payment_call_api::runtime_metadata::<
                    Block,
                    Balance,
                    RuntimeCall,
                >(),
            ]),
        )
    }
}
#[doc(hidden)]
impl InternalImplRuntimeApis for Runtime {}
pub mod api {
    use super::*;
    #[cfg(feature = "std")]
    pub fn dispatch(method: &str, mut _sp_api_input_data_: &[u8]) -> Option<Vec<u8>> {
        match method {
            "Core_version" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            if !_sp_api_input_data_.is_empty() {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: expected no parameters, but input buffer is not empty.",
                                            "version",
                                        ),
                                    );
                                };
                            }
                            #[allow(deprecated)]
                            <Runtime as sp_api::runtime_decl_for_core::Core<
                                Block,
                            >>::version()
                        },
                    ),
                )
            }
            "Core_execute_block" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            let block: Block = match sp_api::DecodeLimit::decode_all_with_depth_limit(
                                sp_api::MAX_EXTRINSIC_DEPTH,
                                &mut _sp_api_input_data_,
                            ) {
                                Ok(res) => res,
                                Err(e) => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: {1}",
                                            "execute_block",
                                            e,
                                        ),
                                    );
                                }
                            };
                            #[allow(deprecated)]
                            <Runtime as sp_api::runtime_decl_for_core::Core<
                                Block,
                            >>::execute_block(block)
                        },
                    ),
                )
            }
            "Core_initialize_block" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            let header: <Block as BlockT>::Header = match sp_api::DecodeLimit::decode_all_with_depth_limit(
                                sp_api::MAX_EXTRINSIC_DEPTH,
                                &mut _sp_api_input_data_,
                            ) {
                                Ok(res) => res,
                                Err(e) => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: {1}",
                                            "initialize_block",
                                            e,
                                        ),
                                    );
                                }
                            };
                            #[allow(deprecated)]
                            <Runtime as sp_api::runtime_decl_for_core::Core<
                                Block,
                            >>::initialize_block(&header)
                        },
                    ),
                )
            }
            "Metadata_metadata" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            if !_sp_api_input_data_.is_empty() {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: expected no parameters, but input buffer is not empty.",
                                            "metadata",
                                        ),
                                    );
                                };
                            }
                            #[allow(deprecated)]
                            <Runtime as sp_api::runtime_decl_for_metadata::Metadata<
                                Block,
                            >>::metadata()
                        },
                    ),
                )
            }
            "Metadata_metadata_at_version" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            let version: u32 = match sp_api::DecodeLimit::decode_all_with_depth_limit(
                                sp_api::MAX_EXTRINSIC_DEPTH,
                                &mut _sp_api_input_data_,
                            ) {
                                Ok(res) => res,
                                Err(e) => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: {1}",
                                            "metadata_at_version",
                                            e,
                                        ),
                                    );
                                }
                            };
                            #[allow(deprecated)]
                            <Runtime as sp_api::runtime_decl_for_metadata::Metadata<
                                Block,
                            >>::metadata_at_version(version)
                        },
                    ),
                )
            }
            "Metadata_metadata_versions" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            if !_sp_api_input_data_.is_empty() {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: expected no parameters, but input buffer is not empty.",
                                            "metadata_versions",
                                        ),
                                    );
                                };
                            }
                            #[allow(deprecated)]
                            <Runtime as sp_api::runtime_decl_for_metadata::Metadata<
                                Block,
                            >>::metadata_versions()
                        },
                    ),
                )
            }
            "BlockBuilder_apply_extrinsic" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            let extrinsic: <Block as BlockT>::Extrinsic = match sp_api::DecodeLimit::decode_all_with_depth_limit(
                                sp_api::MAX_EXTRINSIC_DEPTH,
                                &mut _sp_api_input_data_,
                            ) {
                                Ok(res) => res,
                                Err(e) => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: {1}",
                                            "apply_extrinsic",
                                            e,
                                        ),
                                    );
                                }
                            };
                            #[allow(deprecated)]
                            <Runtime as sp_block_builder::runtime_decl_for_block_builder::BlockBuilder<
                                Block,
                            >>::apply_extrinsic(extrinsic)
                        },
                    ),
                )
            }
            "BlockBuilder_finalize_block" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            if !_sp_api_input_data_.is_empty() {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: expected no parameters, but input buffer is not empty.",
                                            "finalize_block",
                                        ),
                                    );
                                };
                            }
                            #[allow(deprecated)]
                            <Runtime as sp_block_builder::runtime_decl_for_block_builder::BlockBuilder<
                                Block,
                            >>::finalize_block()
                        },
                    ),
                )
            }
            "BlockBuilder_inherent_extrinsics" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            let data: sp_inherents::InherentData = match sp_api::DecodeLimit::decode_all_with_depth_limit(
                                sp_api::MAX_EXTRINSIC_DEPTH,
                                &mut _sp_api_input_data_,
                            ) {
                                Ok(res) => res,
                                Err(e) => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: {1}",
                                            "inherent_extrinsics",
                                            e,
                                        ),
                                    );
                                }
                            };
                            #[allow(deprecated)]
                            <Runtime as sp_block_builder::runtime_decl_for_block_builder::BlockBuilder<
                                Block,
                            >>::inherent_extrinsics(data)
                        },
                    ),
                )
            }
            "BlockBuilder_check_inherents" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            let (block, data): (Block, sp_inherents::InherentData) = match sp_api::DecodeLimit::decode_all_with_depth_limit(
                                sp_api::MAX_EXTRINSIC_DEPTH,
                                &mut _sp_api_input_data_,
                            ) {
                                Ok(res) => res,
                                Err(e) => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: {1}",
                                            "check_inherents",
                                            e,
                                        ),
                                    );
                                }
                            };
                            #[allow(deprecated)]
                            <Runtime as sp_block_builder::runtime_decl_for_block_builder::BlockBuilder<
                                Block,
                            >>::check_inherents(block, data)
                        },
                    ),
                )
            }
            "TaggedTransactionQueue_validate_transaction" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            let (
                                source,
                                tx,
                                block_hash,
                            ): (
                                TransactionSource,
                                <Block as BlockT>::Extrinsic,
                                <Block as BlockT>::Hash,
                            ) = match sp_api::DecodeLimit::decode_all_with_depth_limit(
                                sp_api::MAX_EXTRINSIC_DEPTH,
                                &mut _sp_api_input_data_,
                            ) {
                                Ok(res) => res,
                                Err(e) => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: {1}",
                                            "validate_transaction",
                                            e,
                                        ),
                                    );
                                }
                            };
                            #[allow(deprecated)]
                            <Runtime as sp_transaction_pool::runtime_api::runtime_decl_for_tagged_transaction_queue::TaggedTransactionQueue<
                                Block,
                            >>::validate_transaction(source, tx, block_hash)
                        },
                    ),
                )
            }
            "OffchainWorkerApi_offchain_worker" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            let header: <Block as BlockT>::Header = match sp_api::DecodeLimit::decode_all_with_depth_limit(
                                sp_api::MAX_EXTRINSIC_DEPTH,
                                &mut _sp_api_input_data_,
                            ) {
                                Ok(res) => res,
                                Err(e) => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: {1}",
                                            "offchain_worker",
                                            e,
                                        ),
                                    );
                                }
                            };
                            #[allow(deprecated)]
                            <Runtime as sp_offchain::runtime_decl_for_offchain_worker_api::OffchainWorkerApi<
                                Block,
                            >>::offchain_worker(&header)
                        },
                    ),
                )
            }
            "AuraApi_slot_duration" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            if !_sp_api_input_data_.is_empty() {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: expected no parameters, but input buffer is not empty.",
                                            "slot_duration",
                                        ),
                                    );
                                };
                            }
                            #[allow(deprecated)]
                            <Runtime as sp_consensus_aura::runtime_decl_for_aura_api::AuraApi<
                                Block,
                                AuraId,
                            >>::slot_duration()
                        },
                    ),
                )
            }
            "AuraApi_authorities" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            if !_sp_api_input_data_.is_empty() {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: expected no parameters, but input buffer is not empty.",
                                            "authorities",
                                        ),
                                    );
                                };
                            }
                            #[allow(deprecated)]
                            <Runtime as sp_consensus_aura::runtime_decl_for_aura_api::AuraApi<
                                Block,
                                AuraId,
                            >>::authorities()
                        },
                    ),
                )
            }
            "SessionKeys_generate_session_keys" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            let seed: Option<Vec<u8>> = match sp_api::DecodeLimit::decode_all_with_depth_limit(
                                sp_api::MAX_EXTRINSIC_DEPTH,
                                &mut _sp_api_input_data_,
                            ) {
                                Ok(res) => res,
                                Err(e) => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: {1}",
                                            "generate_session_keys",
                                            e,
                                        ),
                                    );
                                }
                            };
                            #[allow(deprecated)]
                            <Runtime as sp_session::runtime_decl_for_session_keys::SessionKeys<
                                Block,
                            >>::generate_session_keys(seed)
                        },
                    ),
                )
            }
            "SessionKeys_decode_session_keys" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            let encoded: Vec<u8> = match sp_api::DecodeLimit::decode_all_with_depth_limit(
                                sp_api::MAX_EXTRINSIC_DEPTH,
                                &mut _sp_api_input_data_,
                            ) {
                                Ok(res) => res,
                                Err(e) => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: {1}",
                                            "decode_session_keys",
                                            e,
                                        ),
                                    );
                                }
                            };
                            #[allow(deprecated)]
                            <Runtime as sp_session::runtime_decl_for_session_keys::SessionKeys<
                                Block,
                            >>::decode_session_keys(encoded)
                        },
                    ),
                )
            }
            "GrandpaApi_grandpa_authorities" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            if !_sp_api_input_data_.is_empty() {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: expected no parameters, but input buffer is not empty.",
                                            "grandpa_authorities",
                                        ),
                                    );
                                };
                            }
                            #[allow(deprecated)]
                            <Runtime as sp_consensus_grandpa::runtime_decl_for_grandpa_api::GrandpaApi<
                                Block,
                            >>::grandpa_authorities()
                        },
                    ),
                )
            }
            "GrandpaApi_current_set_id" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            if !_sp_api_input_data_.is_empty() {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: expected no parameters, but input buffer is not empty.",
                                            "current_set_id",
                                        ),
                                    );
                                };
                            }
                            #[allow(deprecated)]
                            <Runtime as sp_consensus_grandpa::runtime_decl_for_grandpa_api::GrandpaApi<
                                Block,
                            >>::current_set_id()
                        },
                    ),
                )
            }
            "GrandpaApi_submit_report_equivocation_unsigned_extrinsic" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            let (
                                _equivocation_proof,
                                _key_owner_proof,
                            ): (
                                sp_consensus_grandpa::EquivocationProof<
                                    <Block as BlockT>::Hash,
                                    NumberFor<Block>,
                                >,
                                sp_consensus_grandpa::OpaqueKeyOwnershipProof,
                            ) = match sp_api::DecodeLimit::decode_all_with_depth_limit(
                                sp_api::MAX_EXTRINSIC_DEPTH,
                                &mut _sp_api_input_data_,
                            ) {
                                Ok(res) => res,
                                Err(e) => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: {1}",
                                            "submit_report_equivocation_unsigned_extrinsic",
                                            e,
                                        ),
                                    );
                                }
                            };
                            #[allow(deprecated)]
                            <Runtime as sp_consensus_grandpa::runtime_decl_for_grandpa_api::GrandpaApi<
                                Block,
                            >>::submit_report_equivocation_unsigned_extrinsic(
                                _equivocation_proof,
                                _key_owner_proof,
                            )
                        },
                    ),
                )
            }
            "GrandpaApi_generate_key_ownership_proof" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            let (
                                _set_id,
                                _authority_id,
                            ): (sp_consensus_grandpa::SetId, GrandpaId) = match sp_api::DecodeLimit::decode_all_with_depth_limit(
                                sp_api::MAX_EXTRINSIC_DEPTH,
                                &mut _sp_api_input_data_,
                            ) {
                                Ok(res) => res,
                                Err(e) => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: {1}",
                                            "generate_key_ownership_proof",
                                            e,
                                        ),
                                    );
                                }
                            };
                            #[allow(deprecated)]
                            <Runtime as sp_consensus_grandpa::runtime_decl_for_grandpa_api::GrandpaApi<
                                Block,
                            >>::generate_key_ownership_proof(_set_id, _authority_id)
                        },
                    ),
                )
            }
            "AccountNonceApi_account_nonce" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            let account: AccountId = match sp_api::DecodeLimit::decode_all_with_depth_limit(
                                sp_api::MAX_EXTRINSIC_DEPTH,
                                &mut _sp_api_input_data_,
                            ) {
                                Ok(res) => res,
                                Err(e) => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: {1}",
                                            "account_nonce",
                                            e,
                                        ),
                                    );
                                }
                            };
                            #[allow(deprecated)]
                            <Runtime as frame_system_rpc_runtime_api::runtime_decl_for_account_nonce_api::AccountNonceApi<
                                Block,
                                AccountId,
                                Index,
                            >>::account_nonce(account)
                        },
                    ),
                )
            }
            "TransactionPaymentApi_query_info" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            let (uxt, len): (<Block as BlockT>::Extrinsic, u32) = match sp_api::DecodeLimit::decode_all_with_depth_limit(
                                sp_api::MAX_EXTRINSIC_DEPTH,
                                &mut _sp_api_input_data_,
                            ) {
                                Ok(res) => res,
                                Err(e) => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: {1}",
                                            "query_info",
                                            e,
                                        ),
                                    );
                                }
                            };
                            #[allow(deprecated)]
                            <Runtime as pallet_transaction_payment_rpc_runtime_api::runtime_decl_for_transaction_payment_api::TransactionPaymentApi<
                                Block,
                                Balance,
                            >>::query_info(uxt, len)
                        },
                    ),
                )
            }
            "TransactionPaymentApi_query_fee_details" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            let (uxt, len): (<Block as BlockT>::Extrinsic, u32) = match sp_api::DecodeLimit::decode_all_with_depth_limit(
                                sp_api::MAX_EXTRINSIC_DEPTH,
                                &mut _sp_api_input_data_,
                            ) {
                                Ok(res) => res,
                                Err(e) => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: {1}",
                                            "query_fee_details",
                                            e,
                                        ),
                                    );
                                }
                            };
                            #[allow(deprecated)]
                            <Runtime as pallet_transaction_payment_rpc_runtime_api::runtime_decl_for_transaction_payment_api::TransactionPaymentApi<
                                Block,
                                Balance,
                            >>::query_fee_details(uxt, len)
                        },
                    ),
                )
            }
            "TransactionPaymentApi_query_weight_to_fee" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            let weight: Weight = match sp_api::DecodeLimit::decode_all_with_depth_limit(
                                sp_api::MAX_EXTRINSIC_DEPTH,
                                &mut _sp_api_input_data_,
                            ) {
                                Ok(res) => res,
                                Err(e) => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: {1}",
                                            "query_weight_to_fee",
                                            e,
                                        ),
                                    );
                                }
                            };
                            #[allow(deprecated)]
                            <Runtime as pallet_transaction_payment_rpc_runtime_api::runtime_decl_for_transaction_payment_api::TransactionPaymentApi<
                                Block,
                                Balance,
                            >>::query_weight_to_fee(weight)
                        },
                    ),
                )
            }
            "TransactionPaymentApi_query_length_to_fee" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            let length: u32 = match sp_api::DecodeLimit::decode_all_with_depth_limit(
                                sp_api::MAX_EXTRINSIC_DEPTH,
                                &mut _sp_api_input_data_,
                            ) {
                                Ok(res) => res,
                                Err(e) => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: {1}",
                                            "query_length_to_fee",
                                            e,
                                        ),
                                    );
                                }
                            };
                            #[allow(deprecated)]
                            <Runtime as pallet_transaction_payment_rpc_runtime_api::runtime_decl_for_transaction_payment_api::TransactionPaymentApi<
                                Block,
                                Balance,
                            >>::query_length_to_fee(length)
                        },
                    ),
                )
            }
            "TransactionPaymentCallApi_query_call_info" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            let (call, len): (RuntimeCall, u32) = match sp_api::DecodeLimit::decode_all_with_depth_limit(
                                sp_api::MAX_EXTRINSIC_DEPTH,
                                &mut _sp_api_input_data_,
                            ) {
                                Ok(res) => res,
                                Err(e) => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: {1}",
                                            "query_call_info",
                                            e,
                                        ),
                                    );
                                }
                            };
                            #[allow(deprecated)]
                            <Runtime as pallet_transaction_payment_rpc_runtime_api::runtime_decl_for_transaction_payment_call_api::TransactionPaymentCallApi<
                                Block,
                                Balance,
                                RuntimeCall,
                            >>::query_call_info(call, len)
                        },
                    ),
                )
            }
            "TransactionPaymentCallApi_query_call_fee_details" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            let (call, len): (RuntimeCall, u32) = match sp_api::DecodeLimit::decode_all_with_depth_limit(
                                sp_api::MAX_EXTRINSIC_DEPTH,
                                &mut _sp_api_input_data_,
                            ) {
                                Ok(res) => res,
                                Err(e) => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: {1}",
                                            "query_call_fee_details",
                                            e,
                                        ),
                                    );
                                }
                            };
                            #[allow(deprecated)]
                            <Runtime as pallet_transaction_payment_rpc_runtime_api::runtime_decl_for_transaction_payment_call_api::TransactionPaymentCallApi<
                                Block,
                                Balance,
                                RuntimeCall,
                            >>::query_call_fee_details(call, len)
                        },
                    ),
                )
            }
            "TransactionPaymentCallApi_query_weight_to_fee" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            let weight: Weight = match sp_api::DecodeLimit::decode_all_with_depth_limit(
                                sp_api::MAX_EXTRINSIC_DEPTH,
                                &mut _sp_api_input_data_,
                            ) {
                                Ok(res) => res,
                                Err(e) => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: {1}",
                                            "query_weight_to_fee",
                                            e,
                                        ),
                                    );
                                }
                            };
                            #[allow(deprecated)]
                            <Runtime as pallet_transaction_payment_rpc_runtime_api::runtime_decl_for_transaction_payment_call_api::TransactionPaymentCallApi<
                                Block,
                                Balance,
                                RuntimeCall,
                            >>::query_weight_to_fee(weight)
                        },
                    ),
                )
            }
            "TransactionPaymentCallApi_query_length_to_fee" => {
                Some(
                    sp_api::Encode::encode(
                        &{
                            let length: u32 = match sp_api::DecodeLimit::decode_all_with_depth_limit(
                                sp_api::MAX_EXTRINSIC_DEPTH,
                                &mut _sp_api_input_data_,
                            ) {
                                Ok(res) => res,
                                Err(e) => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Bad input data provided to {0}: {1}",
                                            "query_length_to_fee",
                                            e,
                                        ),
                                    );
                                }
                            };
                            #[allow(deprecated)]
                            <Runtime as pallet_transaction_payment_rpc_runtime_api::runtime_decl_for_transaction_payment_call_api::TransactionPaymentCallApi<
                                Block,
                                Balance,
                                RuntimeCall,
                            >>::query_length_to_fee(length)
                        },
                    ),
                )
            }
            _ => None,
        }
    }
}
