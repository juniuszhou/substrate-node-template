#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod migrations {
    pub mod v1 {
        use crate::*;
        use frame_support::{
            pallet_prelude::*, storage::StoragePrefixedMap, traits::GetStorageVersion,
            weights::Weight,
        };
        use frame_system::pallet_prelude::*;
        use frame_support::{migration::storage_key_iter, Blake2_128Concat};
        pub struct OldKitty(pub [u8; 16]);
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Encode for OldKitty {
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
            #[automatically_derived]
            impl ::codec::EncodeLike for OldKitty {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Decode for OldKitty {
                fn decode<__CodecInputEdqy: ::codec::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::codec::Error> {
                    ::core::result::Result::Ok(OldKitty({
                        let __codec_res_edqy =
                            <[u8; 16] as ::codec::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `OldKitty.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
            }
        };
        #[automatically_derived]
        impl ::core::clone::Clone for OldKitty {
            #[inline]
            fn clone(&self) -> OldKitty {
                OldKitty(::core::clone::Clone::clone(&self.0))
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for OldKitty {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for OldKitty {
            #[inline]
            fn eq(&self, other: &OldKitty) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for OldKitty {}
        #[automatically_derived]
        impl ::core::cmp::Eq for OldKitty {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<[u8; 16]>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for OldKitty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "OldKitty", &&self.0)
            }
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            impl ::scale_info::TypeInfo for OldKitty {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(::scale_info::Path::new(
                            "OldKitty",
                            "pallet_kitties::migrations::v1",
                        ))
                        .type_params(::alloc::vec::Vec::new())
                        .composite(
                            ::scale_info::build::Fields::unnamed()
                                .field(|f| f.ty::<[u8; 16]>().type_name("[u8; 16]")),
                        )
                }
            };
        };
        const _: () = {
            impl ::codec::MaxEncodedLen for OldKitty {
                fn max_encoded_len() -> ::core::primitive::usize {
                    0_usize.saturating_add(<[u8; 16]>::max_encoded_len())
                }
            }
        };
        pub fn migrate<T: Config>() -> Weight {
            let on_chain_storage_version = Pallet::<T>::on_chain_storage_version();
            let current_version = Pallet::<T>::current_storage_version();
            if current_version != 1 {
                return Weight::zero();
            }
            if on_chain_storage_version != 0 {
                return Weight::zero();
            }
            let module = Kitties::<T>::module_prefix();
            let item = Kitties::<T>::storage_prefix();
            for (index, kitty) in
                storage_key_iter::<KittyId, OldKitty, Blake2_128Concat>(module, item).drain()
            {
                let new_kitty = Kitty {
                    dna: kitty.0,
                    name: *b"abcd",
                };
                Kitties::<T>::insert(index, &new_kitty);
            }
            Weight::zero()
        }
    }
}
pub use pallet::*;
///
///			The module that hosts all the
///			[FRAME](https://docs.substrate.io/main-docs/build/events-errors/)
///			types needed to add this pallet to a
///			runtime.
///
pub mod pallet {
    use crate::migrations;
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, Randomness, ReservableCurrency},
        PalletId,
    };
    use frame_system::pallet_prelude::*;
    use sp_io::hashing::blake2_128;
    use sp_runtime::traits::AccountIdConversion;
    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);
    pub(crate) type KittyId = u32;
    pub struct Kitty {
        pub dna: [u8; 16],
        pub name: [u8; 4],
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::codec::Encode for Kitty {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                ::codec::Encode::encode_to(&self.dna, __codec_dest_edqy);
                ::codec::Encode::encode_to(&self.name, __codec_dest_edqy);
            }
        }
        #[automatically_derived]
        impl ::codec::EncodeLike for Kitty {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::codec::Decode for Kitty {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                ::core::result::Result::Ok(Kitty {
                    dna: {
                        let __codec_res_edqy =
                            <[u8; 16] as ::codec::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Kitty::dna`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                    name: {
                        let __codec_res_edqy =
                            <[u8; 4] as ::codec::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Kitty::name`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                })
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for Kitty {
        #[inline]
        fn clone(&self) -> Kitty {
            Kitty {
                dna: ::core::clone::Clone::clone(&self.dna),
                name: ::core::clone::Clone::clone(&self.name),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Kitty {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Kitty {
        #[inline]
        fn eq(&self, other: &Kitty) -> bool {
            self.dna == other.dna && self.name == other.name
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Kitty {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Kitty {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<[u8; 16]>;
            let _: ::core::cmp::AssertParamIsEq<[u8; 4]>;
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Kitty {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Kitty",
                "dna",
                &self.dna,
                "name",
                &&self.name,
            )
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for Kitty {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new("Kitty", "pallet_kitties::pallet"))
                    .type_params(::alloc::vec::Vec::new())
                    .composite(
                        ::scale_info::build::Fields::named()
                            .field(|f| f.ty::<[u8; 16]>().name("dna").type_name("[u8; 16]"))
                            .field(|f| f.ty::<[u8; 4]>().name("name").type_name("[u8; 4]")),
                    )
            }
        };
    };
    const _: () = {
        impl ::codec::MaxEncodedLen for Kitty {
            fn max_encoded_len() -> ::core::primitive::usize {
                0_usize
                    .saturating_add(<[u8; 16]>::max_encoded_len())
                    .saturating_add(<[u8; 4]>::max_encoded_len())
            }
        }
    };
    ///
    ///			Configuration trait of this pallet.
    ///
    ///			Implement this type for a runtime in order to customize this pallet.
    ///
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
        type PalletId: Get<PalletId>;
        type KittyPrice: Get<BalanceOf<Self>>;
    }
    ///
    ///			The [pallet](https://docs.substrate.io/reference/frame-pallets/#pallets) implementing
    ///			the on-chain logic.
    ///
    pub struct Pallet<T>(frame_support::sp_std::marker::PhantomData<(T)>);
    const _: () = {
        impl<T> core::clone::Clone for Pallet<T> {
            fn clone(&self) -> Self {
                Self(core::clone::Clone::clone(&self.0))
            }
        }
    };
    const _: () = {
        impl<T> core::cmp::Eq for Pallet<T> {}
    };
    const _: () = {
        impl<T> core::cmp::PartialEq for Pallet<T> {
            fn eq(&self, other: &Self) -> bool {
                true && self.0 == other.0
            }
        }
    };
    const _: () = {
        impl<T> core::fmt::Debug for Pallet<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_tuple("Pallet").field(&self.0).finish()
            }
        }
    };
    #[allow(type_alias_bounds)]
    pub type NextKittyId<T> =
        StorageValue<_GeneratedPrefixForStorageNextKittyId<T>, KittyId, ValueQuery>;
    #[allow(type_alias_bounds)]
    pub type Kitties<T> =
        StorageMap<_GeneratedPrefixForStorageKitties<T>, Blake2_128Concat, KittyId, Kitty>;
    #[allow(type_alias_bounds)]
    pub type KittyOnSale<T> =
        StorageMap<_GeneratedPrefixForStorageKittyOnSale<T>, Blake2_128Concat, KittyId, ()>;
    #[allow(type_alias_bounds)]
    pub type KittyParents<T> = StorageMap<
        _GeneratedPrefixForStorageKittyParents<T>,
        Blake2_128Concat,
        KittyId,
        (KittyId, KittyId),
    >;
    #[allow(type_alias_bounds)]
    pub type KittyOwner<T: Config> = StorageMap<
        _GeneratedPrefixForStorageKittyOwner<T>,
        Blake2_128Concat,
        KittyId,
        T::AccountId,
    >;
    ///
    ///			The [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted
    ///			by this pallet.
    ///
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    pub enum Event<T: Config> {
        KittyCreated {
            who: T::AccountId,
            kitty_id: KittyId,
            kitty: Kitty,
        },
        KittyBred {
            who: T::AccountId,
            kitty_id: KittyId,
            kitty: Kitty,
        },
        KittyTransferred {
            who: T::AccountId,
            recipient: T::AccountId,
            kitty_id: KittyId,
        },
        KittyOnSale {
            who: T::AccountId,
            kitty_id: KittyId,
        },
        KittyBought {
            who: T::AccountId,
            kitty_id: KittyId,
        },
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(
            frame_support::sp_std::marker::PhantomData<(T)>,
            frame_support::Never,
        ),
    }
    const _: () = {
        impl<T: Config> core::clone::Clone for Event<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::KittyCreated {
                        ref who,
                        ref kitty_id,
                        ref kitty,
                    } => Self::KittyCreated {
                        who: core::clone::Clone::clone(who),
                        kitty_id: core::clone::Clone::clone(kitty_id),
                        kitty: core::clone::Clone::clone(kitty),
                    },
                    Self::KittyBred {
                        ref who,
                        ref kitty_id,
                        ref kitty,
                    } => Self::KittyBred {
                        who: core::clone::Clone::clone(who),
                        kitty_id: core::clone::Clone::clone(kitty_id),
                        kitty: core::clone::Clone::clone(kitty),
                    },
                    Self::KittyTransferred {
                        ref who,
                        ref recipient,
                        ref kitty_id,
                    } => Self::KittyTransferred {
                        who: core::clone::Clone::clone(who),
                        recipient: core::clone::Clone::clone(recipient),
                        kitty_id: core::clone::Clone::clone(kitty_id),
                    },
                    Self::KittyOnSale {
                        ref who,
                        ref kitty_id,
                    } => Self::KittyOnSale {
                        who: core::clone::Clone::clone(who),
                        kitty_id: core::clone::Clone::clone(kitty_id),
                    },
                    Self::KittyBought {
                        ref who,
                        ref kitty_id,
                    } => Self::KittyBought {
                        who: core::clone::Clone::clone(who),
                        kitty_id: core::clone::Clone::clone(kitty_id),
                    },
                    Self::__Ignore(ref _0, ref _1) => {
                        Self::__Ignore(core::clone::Clone::clone(_0), core::clone::Clone::clone(_1))
                    }
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::cmp::Eq for Event<T> {}
    };
    const _: () = {
        impl<T: Config> core::cmp::PartialEq for Event<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (
                        Self::KittyCreated {
                            who,
                            kitty_id,
                            kitty,
                        },
                        Self::KittyCreated {
                            who: _0,
                            kitty_id: _1,
                            kitty: _2,
                        },
                    ) => true && who == _0 && kitty_id == _1 && kitty == _2,
                    (
                        Self::KittyBred {
                            who,
                            kitty_id,
                            kitty,
                        },
                        Self::KittyBred {
                            who: _0,
                            kitty_id: _1,
                            kitty: _2,
                        },
                    ) => true && who == _0 && kitty_id == _1 && kitty == _2,
                    (
                        Self::KittyTransferred {
                            who,
                            recipient,
                            kitty_id,
                        },
                        Self::KittyTransferred {
                            who: _0,
                            recipient: _1,
                            kitty_id: _2,
                        },
                    ) => true && who == _0 && recipient == _1 && kitty_id == _2,
                    (
                        Self::KittyOnSale { who, kitty_id },
                        Self::KittyOnSale {
                            who: _0,
                            kitty_id: _1,
                        },
                    ) => true && who == _0 && kitty_id == _1,
                    (
                        Self::KittyBought { who, kitty_id },
                        Self::KittyBought {
                            who: _0,
                            kitty_id: _1,
                        },
                    ) => true && who == _0 && kitty_id == _1,
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::KittyCreated { .. }, Self::KittyBred { .. }) => false,
                    (Self::KittyCreated { .. }, Self::KittyTransferred { .. }) => false,
                    (Self::KittyCreated { .. }, Self::KittyOnSale { .. }) => false,
                    (Self::KittyCreated { .. }, Self::KittyBought { .. }) => false,
                    (Self::KittyCreated { .. }, Self::__Ignore { .. }) => false,
                    (Self::KittyBred { .. }, Self::KittyCreated { .. }) => false,
                    (Self::KittyBred { .. }, Self::KittyTransferred { .. }) => false,
                    (Self::KittyBred { .. }, Self::KittyOnSale { .. }) => false,
                    (Self::KittyBred { .. }, Self::KittyBought { .. }) => false,
                    (Self::KittyBred { .. }, Self::__Ignore { .. }) => false,
                    (Self::KittyTransferred { .. }, Self::KittyCreated { .. }) => false,
                    (Self::KittyTransferred { .. }, Self::KittyBred { .. }) => false,
                    (Self::KittyTransferred { .. }, Self::KittyOnSale { .. }) => false,
                    (Self::KittyTransferred { .. }, Self::KittyBought { .. }) => false,
                    (Self::KittyTransferred { .. }, Self::__Ignore { .. }) => false,
                    (Self::KittyOnSale { .. }, Self::KittyCreated { .. }) => false,
                    (Self::KittyOnSale { .. }, Self::KittyBred { .. }) => false,
                    (Self::KittyOnSale { .. }, Self::KittyTransferred { .. }) => false,
                    (Self::KittyOnSale { .. }, Self::KittyBought { .. }) => false,
                    (Self::KittyOnSale { .. }, Self::__Ignore { .. }) => false,
                    (Self::KittyBought { .. }, Self::KittyCreated { .. }) => false,
                    (Self::KittyBought { .. }, Self::KittyBred { .. }) => false,
                    (Self::KittyBought { .. }, Self::KittyTransferred { .. }) => false,
                    (Self::KittyBought { .. }, Self::KittyOnSale { .. }) => false,
                    (Self::KittyBought { .. }, Self::__Ignore { .. }) => false,
                    (Self::__Ignore { .. }, Self::KittyCreated { .. }) => false,
                    (Self::__Ignore { .. }, Self::KittyBred { .. }) => false,
                    (Self::__Ignore { .. }, Self::KittyTransferred { .. }) => false,
                    (Self::__Ignore { .. }, Self::KittyOnSale { .. }) => false,
                    (Self::__Ignore { .. }, Self::KittyBought { .. }) => false,
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::fmt::Debug for Event<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    Self::KittyCreated {
                        ref who,
                        ref kitty_id,
                        ref kitty,
                    } => fmt
                        .debug_struct("Event::KittyCreated")
                        .field("who", &who)
                        .field("kitty_id", &kitty_id)
                        .field("kitty", &kitty)
                        .finish(),
                    Self::KittyBred {
                        ref who,
                        ref kitty_id,
                        ref kitty,
                    } => fmt
                        .debug_struct("Event::KittyBred")
                        .field("who", &who)
                        .field("kitty_id", &kitty_id)
                        .field("kitty", &kitty)
                        .finish(),
                    Self::KittyTransferred {
                        ref who,
                        ref recipient,
                        ref kitty_id,
                    } => fmt
                        .debug_struct("Event::KittyTransferred")
                        .field("who", &who)
                        .field("recipient", &recipient)
                        .field("kitty_id", &kitty_id)
                        .finish(),
                    Self::KittyOnSale {
                        ref who,
                        ref kitty_id,
                    } => fmt
                        .debug_struct("Event::KittyOnSale")
                        .field("who", &who)
                        .field("kitty_id", &kitty_id)
                        .finish(),
                    Self::KittyBought {
                        ref who,
                        ref kitty_id,
                    } => fmt
                        .debug_struct("Event::KittyBought")
                        .field("who", &who)
                        .field("kitty_id", &kitty_id)
                        .finish(),
                    Self::__Ignore(ref _0, ref _1) => fmt
                        .debug_tuple("Event::__Ignore")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                }
            }
        }
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<T: Config> ::codec::Encode for Event<T>
        where
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
        {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Event::KittyCreated {
                        ref who,
                        ref kitty_id,
                        ref kitty,
                    } => {
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(who, __codec_dest_edqy);
                        ::codec::Encode::encode_to(kitty_id, __codec_dest_edqy);
                        ::codec::Encode::encode_to(kitty, __codec_dest_edqy);
                    }
                    Event::KittyBred {
                        ref who,
                        ref kitty_id,
                        ref kitty,
                    } => {
                        __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(who, __codec_dest_edqy);
                        ::codec::Encode::encode_to(kitty_id, __codec_dest_edqy);
                        ::codec::Encode::encode_to(kitty, __codec_dest_edqy);
                    }
                    Event::KittyTransferred {
                        ref who,
                        ref recipient,
                        ref kitty_id,
                    } => {
                        __codec_dest_edqy.push_byte(2usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(who, __codec_dest_edqy);
                        ::codec::Encode::encode_to(recipient, __codec_dest_edqy);
                        ::codec::Encode::encode_to(kitty_id, __codec_dest_edqy);
                    }
                    Event::KittyOnSale {
                        ref who,
                        ref kitty_id,
                    } => {
                        __codec_dest_edqy.push_byte(3usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(who, __codec_dest_edqy);
                        ::codec::Encode::encode_to(kitty_id, __codec_dest_edqy);
                    }
                    Event::KittyBought {
                        ref who,
                        ref kitty_id,
                    } => {
                        __codec_dest_edqy.push_byte(4usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(who, __codec_dest_edqy);
                        ::codec::Encode::encode_to(kitty_id, __codec_dest_edqy);
                    }
                    _ => (),
                }
            }
        }
        #[automatically_derived]
        impl<T: Config> ::codec::EncodeLike for Event<T>
        where
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
        {
        }
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<T: Config> ::codec::Decode for Event<T>
        where
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
        {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Event`, failed to read variant byte"))?
                {
                    __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Event::<T>::KittyCreated {
                            who: {
                                let __codec_res_edqy =
                                    <T::AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::KittyCreated::who`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            kitty_id: {
                                let __codec_res_edqy =
                                    <KittyId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(e.chain(
                                            "Could not decode `Event::KittyCreated::kitty_id`",
                                        ))
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            kitty: {
                                let __codec_res_edqy =
                                    <Kitty as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(e.chain(
                                            "Could not decode `Event::KittyCreated::kitty`",
                                        ))
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        })
                    }
                    __codec_x_edqy if __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Event::<T>::KittyBred {
                            who: {
                                let __codec_res_edqy =
                                    <T::AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::KittyBred::who`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            kitty_id: {
                                let __codec_res_edqy =
                                    <KittyId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(e.chain(
                                            "Could not decode `Event::KittyBred::kitty_id`",
                                        ))
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            kitty: {
                                let __codec_res_edqy =
                                    <Kitty as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::KittyBred::kitty`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        })
                    }
                    __codec_x_edqy if __codec_x_edqy == 2usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Event::<T>::KittyTransferred {
                            who: {
                                let __codec_res_edqy =
                                    <T::AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(e.chain(
                                            "Could not decode `Event::KittyTransferred::who`",
                                        ))
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            recipient: {
                                let __codec_res_edqy =
                                    <T::AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(e.chain(
                                            "Could not decode `Event::KittyTransferred::recipient`",
                                        ))
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            kitty_id: {
                                let __codec_res_edqy =
                                    <KittyId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(e.chain(
                                            "Could not decode `Event::KittyTransferred::kitty_id`",
                                        ))
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        })
                    }
                    __codec_x_edqy if __codec_x_edqy == 3usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Event::<T>::KittyOnSale {
                            who: {
                                let __codec_res_edqy =
                                    <T::AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::KittyOnSale::who`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            kitty_id: {
                                let __codec_res_edqy =
                                    <KittyId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(e.chain(
                                            "Could not decode `Event::KittyOnSale::kitty_id`",
                                        ))
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        })
                    }
                    __codec_x_edqy if __codec_x_edqy == 4usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Event::<T>::KittyBought {
                            who: {
                                let __codec_res_edqy =
                                    <T::AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::KittyBought::who`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            kitty_id: {
                                let __codec_res_edqy =
                                    <KittyId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(e.chain(
                                            "Could not decode `Event::KittyBought::kitty_id`",
                                        ))
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        })
                    }
                    _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                        "Could not decode `Event`, variant doesn't exist",
                    )),
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<T: Config> ::scale_info::TypeInfo for Event<T>
        where
            T::AccountId: ::scale_info::TypeInfo + 'static,
            T::AccountId: ::scale_info::TypeInfo + 'static,
            T::AccountId: ::scale_info::TypeInfo + 'static,
            T::AccountId: ::scale_info::TypeInfo + 'static,
            T::AccountId: ::scale_info::TypeInfo + 'static,
            T::AccountId: ::scale_info::TypeInfo + 'static,
            frame_support::sp_std::marker::PhantomData<(T)>: ::scale_info::TypeInfo + 'static,
            T: Config + 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                :: scale_info :: Type :: builder () . path (:: scale_info :: Path :: new ("Event" , "pallet_kitties::pallet")) . type_params (< [_] > :: into_vec (# [rustc_box] :: alloc :: boxed :: Box :: new ([:: scale_info :: TypeParameter :: new ("T" , :: core :: option :: Option :: None)]))) . docs_always (& ["\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]) . variant (:: scale_info :: build :: Variants :: new () . variant ("KittyCreated" , | v | v . index (0usize as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: named () . field (| f | f . ty :: < T :: AccountId > () . name ("who") . type_name ("T::AccountId")) . field (| f | f . ty :: < KittyId > () . name ("kitty_id") . type_name ("KittyId")) . field (| f | f . ty :: < Kitty > () . name ("kitty") . type_name ("Kitty")))) . variant ("KittyBred" , | v | v . index (1usize as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: named () . field (| f | f . ty :: < T :: AccountId > () . name ("who") . type_name ("T::AccountId")) . field (| f | f . ty :: < KittyId > () . name ("kitty_id") . type_name ("KittyId")) . field (| f | f . ty :: < Kitty > () . name ("kitty") . type_name ("Kitty")))) . variant ("KittyTransferred" , | v | v . index (2usize as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: named () . field (| f | f . ty :: < T :: AccountId > () . name ("who") . type_name ("T::AccountId")) . field (| f | f . ty :: < T :: AccountId > () . name ("recipient") . type_name ("T::AccountId")) . field (| f | f . ty :: < KittyId > () . name ("kitty_id") . type_name ("KittyId")))) . variant ("KittyOnSale" , | v | v . index (3usize as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: named () . field (| f | f . ty :: < T :: AccountId > () . name ("who") . type_name ("T::AccountId")) . field (| f | f . ty :: < KittyId > () . name ("kitty_id") . type_name ("KittyId")))) . variant ("KittyBought" , | v | v . index (4usize as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: named () . field (| f | f . ty :: < T :: AccountId > () . name ("who") . type_name ("T::AccountId")) . field (| f | f . ty :: < KittyId > () . name ("kitty_id") . type_name ("KittyId")))))
            }
        };
    };
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    ///
    ///			Custom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)
    ///			of this pallet.
    ///
    pub enum Error<T> {
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(
            frame_support::sp_std::marker::PhantomData<(T)>,
            frame_support::Never,
        ),
        InvalidKittyId,
        NotOwner,
        SameKittyId,
        NoOwner,
        AlreadyOnSale,
        AlreadyOwned,
        NotOnSale,
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<T> ::codec::Encode for Error<T> {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Error::InvalidKittyId => {
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                    }
                    Error::NotOwner => {
                        __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                    }
                    Error::SameKittyId => {
                        __codec_dest_edqy.push_byte(2usize as ::core::primitive::u8);
                    }
                    Error::NoOwner => {
                        __codec_dest_edqy.push_byte(3usize as ::core::primitive::u8);
                    }
                    Error::AlreadyOnSale => {
                        __codec_dest_edqy.push_byte(4usize as ::core::primitive::u8);
                    }
                    Error::AlreadyOwned => {
                        __codec_dest_edqy.push_byte(5usize as ::core::primitive::u8);
                    }
                    Error::NotOnSale => {
                        __codec_dest_edqy.push_byte(6usize as ::core::primitive::u8);
                    }
                    _ => (),
                }
            }
        }
        #[automatically_derived]
        impl<T> ::codec::EncodeLike for Error<T> {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<T> ::codec::Decode for Error<T> {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Error`, failed to read variant byte"))?
                {
                    __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Error::<T>::InvalidKittyId)
                    }
                    __codec_x_edqy if __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Error::<T>::NotOwner)
                    }
                    __codec_x_edqy if __codec_x_edqy == 2usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Error::<T>::SameKittyId)
                    }
                    __codec_x_edqy if __codec_x_edqy == 3usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Error::<T>::NoOwner)
                    }
                    __codec_x_edqy if __codec_x_edqy == 4usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Error::<T>::AlreadyOnSale)
                    }
                    __codec_x_edqy if __codec_x_edqy == 5usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Error::<T>::AlreadyOwned)
                    }
                    __codec_x_edqy if __codec_x_edqy == 6usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Error::<T>::NotOnSale)
                    }
                    _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                        "Could not decode `Error`, variant doesn't exist",
                    )),
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<T> ::scale_info::TypeInfo for Error<T>
        where
            frame_support::sp_std::marker::PhantomData<(T)>: ::scale_info::TypeInfo + 'static,
            T: 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                :: scale_info :: Type :: builder () . path (:: scale_info :: Path :: new ("Error" , "pallet_kitties::pallet")) . type_params (< [_] > :: into_vec (# [rustc_box] :: alloc :: boxed :: Box :: new ([:: scale_info :: TypeParameter :: new ("T" , :: core :: option :: Option :: None)]))) . docs_always (& ["\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]) . variant (:: scale_info :: build :: Variants :: new () . variant ("InvalidKittyId" , | v | v . index (0usize as :: core :: primitive :: u8)) . variant ("NotOwner" , | v | v . index (1usize as :: core :: primitive :: u8)) . variant ("SameKittyId" , | v | v . index (2usize as :: core :: primitive :: u8)) . variant ("NoOwner" , | v | v . index (3usize as :: core :: primitive :: u8)) . variant ("AlreadyOnSale" , | v | v . index (4usize as :: core :: primitive :: u8)) . variant ("AlreadyOwned" , | v | v . index (5usize as :: core :: primitive :: u8)) . variant ("NotOnSale" , | v | v . index (6usize as :: core :: primitive :: u8)))
            }
        };
    };
    const _: () = {
        impl<T> frame_support::traits::PalletError for Error<T> {
            const MAX_ENCODED_SIZE: usize = 1;
        }
    };
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_runtime_upgrade() -> frame_support::weights::Weight {
            migrations::v1::migrate::<T>()
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn create(origin: OriginFor<T>, name: [u8; 4]) -> DispatchResult {
            frame_support::storage::with_storage_layer(|| {
                let who = ensure_signed(origin)?;
                let kitty_id = Self::get_next_id()?;
                let dna = Self::random_value(&who);
                let kitty = Kitty { dna, name };
                let price = T::KittyPrice::get();
                T::Currency::transfer(
                    &who,
                    &Self::get_account_id(),
                    price,
                    frame_support::traits::ExistenceRequirement::KeepAlive,
                )?;
                T::Currency::reserve(&who, price)?;
                Kitties::<T>::insert(kitty_id, &kitty);
                KittyOwner::<T>::insert(kitty_id, &who);
                NextKittyId::<T>::set(kitty_id + 1);
                Self::deposit_event(Event::KittyCreated {
                    who,
                    kitty_id,
                    kitty,
                });
                Ok(())
            })
        }
        pub fn breed(
            origin: OriginFor<T>,
            kitty_id_1: KittyId,
            kitty_id_2: KittyId,
            name: [u8; 4],
        ) -> DispatchResult {
            frame_support::storage::with_storage_layer(|| {
                let who = ensure_signed(origin)?;
                {
                    if !(kitty_id_1 != kitty_id_2) {
                        {
                            return Err(Error::<T>::SameKittyId.into());
                        };
                    }
                };
                let kitty_1 = Self::kitties(kitty_id_1)
                    .ok_or::<DispatchError>(Error::<T>::InvalidKittyId.into())?;
                let kitty_2 = Self::kitties(kitty_id_2)
                    .ok_or::<DispatchError>(Error::<T>::InvalidKittyId.into())?;
                let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;
                let price = T::KittyPrice::get();
                T::Currency::transfer(
                    &who,
                    &Self::get_account_id(),
                    price,
                    frame_support::traits::ExistenceRequirement::KeepAlive,
                )?;
                T::Currency::reserve(&who, price)?;
                let selector = Self::random_value(&who);
                let dna = [0u8; 16];
                let kitty = Kitty { dna, name };
                <Kitties<T>>::insert(kitty_id, &kitty);
                KittyOwner::<T>::insert(kitty_id, &who);
                KittyParents::<T>::insert(kitty_id, (kitty_id_1, kitty_id_2));
                NextKittyId::<T>::set(kitty_id + 1);
                Self::deposit_event(Event::KittyBred {
                    who,
                    kitty_id,
                    kitty,
                });
                Ok(())
            })
        }
        pub fn transfer(
            origin: OriginFor<T>,
            kitty_id: u32,
            recipient: T::AccountId,
        ) -> DispatchResult {
            frame_support::storage::with_storage_layer(|| {
                let who = ensure_signed(origin)?;
                Self::kitties(kitty_id)
                    .ok_or::<DispatchError>(Error::<T>::InvalidKittyId.into())?;
                {
                    if !(Self::kitty_owner(kitty_id) == Some(who.clone())) {
                        {
                            return Err(Error::<T>::NotOwner.into());
                        };
                    }
                };
                Self::deposit_event(Event::KittyTransferred {
                    who,
                    recipient,
                    kitty_id,
                });
                Ok(())
            })
        }
        pub fn sale(origin: OriginFor<T>, kitty_id: u32) -> DispatchResult {
            frame_support::storage::with_storage_layer(|| {
                let who = ensure_signed(origin)?;
                Self::kitties(kitty_id)
                    .ok_or::<DispatchError>(Error::<T>::InvalidKittyId.into())?;
                {
                    if !(Self::kitty_owner(kitty_id) == Some(who.clone())) {
                        {
                            return Err(Error::<T>::NotOwner.into());
                        };
                    }
                };
                {
                    if !Self::kitty_on_sale(kitty_id).is_some() {
                        {
                            return Err(Error::<T>::AlreadyOnSale.into());
                        };
                    }
                };
                <KittyOnSale<T>>::insert(kitty_id, ());
                Self::deposit_event(Event::KittyOnSale { who, kitty_id });
                Ok(())
            })
        }
        pub fn buy(origin: OriginFor<T>, kitty_id: u32) -> DispatchResult {
            frame_support::storage::with_storage_layer(|| {
                let who = ensure_signed(origin)?;
                Self::kitties(kitty_id)
                    .ok_or::<DispatchError>(Error::<T>::InvalidKittyId.into())?;
                {
                    if !(Self::kitty_on_sale(kitty_id) == Some(())) {
                        {
                            return Err(Error::<T>::NotOnSale.into());
                        };
                    }
                };
                let owner = Self::kitty_owner(kitty_id)
                    .ok_or::<DispatchError>(Error::<T>::NoOwner.into())?;
                {
                    if !(owner != who.clone()) {
                        {
                            return Err(Error::<T>::AlreadyOwned.into());
                        };
                    }
                };
                let price = T::KittyPrice::get();
                T::Currency::transfer(
                    &who,
                    &Self::get_account_id(),
                    price,
                    frame_support::traits::ExistenceRequirement::KeepAlive,
                )?;
                T::Currency::reserve(&who, price)?;
                T::Currency::unreserve(&owner, price);
                <KittyOnSale<T>>::remove(kitty_id);
                <KittyOwner<T>>::insert(kitty_id, &who);
                Self::deposit_event(Event::KittyBought { who, kitty_id });
                Ok(())
            })
        }
    }
    impl<T: Config> Pallet<T> {
        fn random_value(sender: &T::AccountId) -> [u8; 16] {
            let payload = (
                T::Randomness::random_seed(),
                &sender,
                <frame_system::Pallet<T>>::extrinsic_index(),
            );
            payload.using_encoded(blake2_128)
        }
        fn get_next_id() -> Result<KittyId, DispatchError> {
            match Self::next_kitty_id() {
                KittyId::MAX => Err(Error::<T>::InvalidKittyId.into()),
                val => Ok(val),
            }
        }
        fn get_account_id() -> T::AccountId {
            T::PalletId::get().into_account_truncating()
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn pallet_documentation_metadata() -> frame_support::sp_std::vec::Vec<&'static str> {
            ::alloc::vec::Vec::new()
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn pallet_constants_metadata(
        ) -> frame_support::sp_std::vec::Vec<frame_support::metadata::PalletConstantMetadata>
        {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([{
                    frame_support::metadata::PalletConstantMetadata {
                        name: "KittyPrice",
                        ty: frame_support::scale_info::meta_type::<BalanceOf<T>>(),
                        value: {
                            let value = <<T as Config>::KittyPrice as frame_support::traits::Get<
                                BalanceOf<T>,
                            >>::get();
                            frame_support::codec::Encode::encode(&value)
                        },
                        docs: ::alloc::vec::Vec::new(),
                    }
                }]),
            )
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn error_metadata() -> Option<frame_support::metadata::PalletErrorMetadata> {
            Some(frame_support::metadata::PalletErrorMetadata {
                ty: frame_support::scale_info::meta_type::<Error<T>>(),
            })
        }
    }
    /// Type alias to `Pallet`, to be used by `construct_runtime`.
    ///
    /// Generated by `pallet` attribute macro.
    #[deprecated(note = "use `Pallet` instead")]
    #[allow(dead_code)]
    pub type Module<T> = Pallet<T>;
    impl<T: Config> frame_support::traits::GetStorageVersion for Pallet<T> {
        fn current_storage_version() -> frame_support::traits::StorageVersion {
            STORAGE_VERSION
        }
        fn on_chain_storage_version() -> frame_support::traits::StorageVersion {
            frame_support::traits::StorageVersion::get::<Self>()
        }
    }
    impl<T: Config> frame_support::traits::OnGenesis for Pallet<T> {
        fn on_genesis() {
            let storage_version = STORAGE_VERSION;
            storage_version.put::<Self>();
        }
    }
    impl<T: Config> frame_support::traits::PalletInfoAccess for Pallet<T> {
        fn index() -> usize {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::index::<
                Self,
            >()
            .expect(
                "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
            )
        }
        fn name() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Self,
            >()
            .expect(
                "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
            )
        }
        fn module_name() -> &'static str {
            < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: module_name :: < Self > () . expect ("Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime")
        }
        fn crate_version() -> frame_support::traits::CrateVersion {
            frame_support::traits::CrateVersion {
                major: 0u16,
                minor: 1u8,
                patch: 0u8,
            }
        }
    }
    impl<T: Config> frame_support::traits::PalletsInfoAccess for Pallet<T> {
        fn count() -> usize {
            1
        }
        fn infos() -> frame_support::sp_std::vec::Vec<frame_support::traits::PalletInfoData> {
            use frame_support::traits::PalletInfoAccess;
            let item = frame_support::traits::PalletInfoData {
                index: Self::index(),
                name: Self::name(),
                module_name: Self::module_name(),
                crate_version: Self::crate_version(),
            };
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([item]),
            )
        }
    }
    impl<T: Config> frame_support::traits::StorageInfoTrait for Pallet<T> {
        fn storage_info() -> frame_support::sp_std::vec::Vec<frame_support::traits::StorageInfo> {
            #[allow(unused_mut)]
            let mut res = ::alloc::vec::Vec::new();
            {
                let mut storage_info =
                    <NextKittyId<T> as frame_support::traits::StorageInfoTrait>::storage_info();
                res.append(&mut storage_info);
            }
            {
                let mut storage_info =
                    <Kitties<T> as frame_support::traits::StorageInfoTrait>::storage_info();
                res.append(&mut storage_info);
            }
            {
                let mut storage_info =
                    <KittyOnSale<T> as frame_support::traits::StorageInfoTrait>::storage_info();
                res.append(&mut storage_info);
            }
            {
                let mut storage_info =
                    <KittyParents<T> as frame_support::traits::StorageInfoTrait>::storage_info();
                res.append(&mut storage_info);
            }
            {
                let mut storage_info =
                    <KittyOwner<T> as frame_support::traits::StorageInfoTrait>::storage_info();
                res.append(&mut storage_info);
            }
            res
        }
    }
    use frame_support::traits::{StorageInfoTrait, TrackedStorageKey, WhitelistedStorageKeys};
    impl<T: Config> WhitelistedStorageKeys for Pallet<T> {
        fn whitelisted_storage_keys() -> frame_support::sp_std::vec::Vec<TrackedStorageKey> {
            use frame_support::sp_std::vec;
            ::alloc::vec::Vec::new()
        }
    }
    mod warnings {}
    #[doc(hidden)]
    pub mod __substrate_call_check {
        #[doc(hidden)]
        pub use __is_call_part_defined_0 as is_call_part_defined;
    }
    ///Contains one variant per dispatchable that can be called by an extrinsic.
    #[codec(encode_bound())]
    #[codec(decode_bound())]
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    #[allow(non_camel_case_types)]
    pub enum Call<T: Config> {
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(
            frame_support::sp_std::marker::PhantomData<(T,)>,
            frame_support::Never,
        ),
        #[codec(index = 0u8)]
        create {
            #[allow(missing_docs)]
            name: [u8; 4],
        },
        #[codec(index = 1u8)]
        breed {
            #[allow(missing_docs)]
            kitty_id_1: KittyId,
            #[allow(missing_docs)]
            kitty_id_2: KittyId,
            #[allow(missing_docs)]
            name: [u8; 4],
        },
        #[codec(index = 2u8)]
        transfer {
            #[allow(missing_docs)]
            kitty_id: u32,
            #[allow(missing_docs)]
            recipient: T::AccountId,
        },
        #[codec(index = 3u8)]
        sale {
            #[allow(missing_docs)]
            kitty_id: u32,
        },
        #[codec(index = 4u8)]
        buy {
            #[allow(missing_docs)]
            kitty_id: u32,
        },
    }
    const _: () = {
        impl<T: Config> core::fmt::Debug for Call<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    Self::__Ignore(ref _0, ref _1) => fmt
                        .debug_tuple("Call::__Ignore")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                    Self::create { ref name } => fmt
                        .debug_struct("Call::create")
                        .field("name", &name)
                        .finish(),
                    Self::breed {
                        ref kitty_id_1,
                        ref kitty_id_2,
                        ref name,
                    } => fmt
                        .debug_struct("Call::breed")
                        .field("kitty_id_1", &kitty_id_1)
                        .field("kitty_id_2", &kitty_id_2)
                        .field("name", &name)
                        .finish(),
                    Self::transfer {
                        ref kitty_id,
                        ref recipient,
                    } => fmt
                        .debug_struct("Call::transfer")
                        .field("kitty_id", &kitty_id)
                        .field("recipient", &recipient)
                        .finish(),
                    Self::sale { ref kitty_id } => fmt
                        .debug_struct("Call::sale")
                        .field("kitty_id", &kitty_id)
                        .finish(),
                    Self::buy { ref kitty_id } => fmt
                        .debug_struct("Call::buy")
                        .field("kitty_id", &kitty_id)
                        .finish(),
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::clone::Clone for Call<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::__Ignore(ref _0, ref _1) => {
                        Self::__Ignore(core::clone::Clone::clone(_0), core::clone::Clone::clone(_1))
                    }
                    Self::create { ref name } => Self::create {
                        name: core::clone::Clone::clone(name),
                    },
                    Self::breed {
                        ref kitty_id_1,
                        ref kitty_id_2,
                        ref name,
                    } => Self::breed {
                        kitty_id_1: core::clone::Clone::clone(kitty_id_1),
                        kitty_id_2: core::clone::Clone::clone(kitty_id_2),
                        name: core::clone::Clone::clone(name),
                    },
                    Self::transfer {
                        ref kitty_id,
                        ref recipient,
                    } => Self::transfer {
                        kitty_id: core::clone::Clone::clone(kitty_id),
                        recipient: core::clone::Clone::clone(recipient),
                    },
                    Self::sale { ref kitty_id } => Self::sale {
                        kitty_id: core::clone::Clone::clone(kitty_id),
                    },
                    Self::buy { ref kitty_id } => Self::buy {
                        kitty_id: core::clone::Clone::clone(kitty_id),
                    },
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::cmp::Eq for Call<T> {}
    };
    const _: () = {
        impl<T: Config> core::cmp::PartialEq for Call<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::create { name }, Self::create { name: _0 }) => true && name == _0,
                    (
                        Self::breed {
                            kitty_id_1,
                            kitty_id_2,
                            name,
                        },
                        Self::breed {
                            kitty_id_1: _0,
                            kitty_id_2: _1,
                            name: _2,
                        },
                    ) => true && kitty_id_1 == _0 && kitty_id_2 == _1 && name == _2,
                    (
                        Self::transfer {
                            kitty_id,
                            recipient,
                        },
                        Self::transfer {
                            kitty_id: _0,
                            recipient: _1,
                        },
                    ) => true && kitty_id == _0 && recipient == _1,
                    (Self::sale { kitty_id }, Self::sale { kitty_id: _0 }) => {
                        true && kitty_id == _0
                    }
                    (Self::buy { kitty_id }, Self::buy { kitty_id: _0 }) => true && kitty_id == _0,
                    (Self::__Ignore { .. }, Self::create { .. }) => false,
                    (Self::__Ignore { .. }, Self::breed { .. }) => false,
                    (Self::__Ignore { .. }, Self::transfer { .. }) => false,
                    (Self::__Ignore { .. }, Self::sale { .. }) => false,
                    (Self::__Ignore { .. }, Self::buy { .. }) => false,
                    (Self::create { .. }, Self::__Ignore { .. }) => false,
                    (Self::create { .. }, Self::breed { .. }) => false,
                    (Self::create { .. }, Self::transfer { .. }) => false,
                    (Self::create { .. }, Self::sale { .. }) => false,
                    (Self::create { .. }, Self::buy { .. }) => false,
                    (Self::breed { .. }, Self::__Ignore { .. }) => false,
                    (Self::breed { .. }, Self::create { .. }) => false,
                    (Self::breed { .. }, Self::transfer { .. }) => false,
                    (Self::breed { .. }, Self::sale { .. }) => false,
                    (Self::breed { .. }, Self::buy { .. }) => false,
                    (Self::transfer { .. }, Self::__Ignore { .. }) => false,
                    (Self::transfer { .. }, Self::create { .. }) => false,
                    (Self::transfer { .. }, Self::breed { .. }) => false,
                    (Self::transfer { .. }, Self::sale { .. }) => false,
                    (Self::transfer { .. }, Self::buy { .. }) => false,
                    (Self::sale { .. }, Self::__Ignore { .. }) => false,
                    (Self::sale { .. }, Self::create { .. }) => false,
                    (Self::sale { .. }, Self::breed { .. }) => false,
                    (Self::sale { .. }, Self::transfer { .. }) => false,
                    (Self::sale { .. }, Self::buy { .. }) => false,
                    (Self::buy { .. }, Self::__Ignore { .. }) => false,
                    (Self::buy { .. }, Self::create { .. }) => false,
                    (Self::buy { .. }, Self::breed { .. }) => false,
                    (Self::buy { .. }, Self::transfer { .. }) => false,
                    (Self::buy { .. }, Self::sale { .. }) => false,
                }
            }
        }
    };
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl<T: Config> ::codec::Encode for Call<T> {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Call::create { ref name } => {
                        __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(name, __codec_dest_edqy);
                    }
                    Call::breed {
                        ref kitty_id_1,
                        ref kitty_id_2,
                        ref name,
                    } => {
                        __codec_dest_edqy.push_byte(1u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(kitty_id_1, __codec_dest_edqy);
                        ::codec::Encode::encode_to(kitty_id_2, __codec_dest_edqy);
                        ::codec::Encode::encode_to(name, __codec_dest_edqy);
                    }
                    Call::transfer {
                        ref kitty_id,
                        ref recipient,
                    } => {
                        __codec_dest_edqy.push_byte(2u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(kitty_id, __codec_dest_edqy);
                        ::codec::Encode::encode_to(recipient, __codec_dest_edqy);
                    }
                    Call::sale { ref kitty_id } => {
                        __codec_dest_edqy.push_byte(3u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(kitty_id, __codec_dest_edqy);
                    }
                    Call::buy { ref kitty_id } => {
                        __codec_dest_edqy.push_byte(4u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(kitty_id, __codec_dest_edqy);
                    }
                    _ => (),
                }
            }
        }
        #[automatically_derived]
        impl<T: Config> ::codec::EncodeLike for Call<T> {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl<T: Config> ::codec::Decode for Call<T> {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Call`, failed to read variant byte"))?
                {
                    __codec_x_edqy if __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::create {
                            name: {
                                let __codec_res_edqy =
                                    <[u8; 4] as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::create::name`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        })
                    }
                    __codec_x_edqy if __codec_x_edqy == 1u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::breed {
                            kitty_id_1: {
                                let __codec_res_edqy =
                                    <KittyId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::breed::kitty_id_1`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            kitty_id_2: {
                                let __codec_res_edqy =
                                    <KittyId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::breed::kitty_id_2`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            name: {
                                let __codec_res_edqy =
                                    <[u8; 4] as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::breed::name`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        })
                    }
                    __codec_x_edqy if __codec_x_edqy == 2u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::transfer {
                            kitty_id: {
                                let __codec_res_edqy =
                                    <u32 as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::transfer::kitty_id`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            recipient: {
                                let __codec_res_edqy =
                                    <T::AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::transfer::recipient`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        })
                    }
                    __codec_x_edqy if __codec_x_edqy == 3u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::sale {
                            kitty_id: {
                                let __codec_res_edqy =
                                    <u32 as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::sale::kitty_id`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        })
                    }
                    __codec_x_edqy if __codec_x_edqy == 4u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::buy {
                            kitty_id: {
                                let __codec_res_edqy =
                                    <u32 as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::buy::kitty_id`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        })
                    }
                    _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                        "Could not decode `Call`, variant doesn't exist",
                    )),
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<T: Config> ::scale_info::TypeInfo for Call<T>
        where
            frame_support::sp_std::marker::PhantomData<(T,)>: ::scale_info::TypeInfo + 'static,
            T::AccountId: ::scale_info::TypeInfo + 'static,
            T: Config + 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new("Call", "pallet_kitties::pallet"))
                    .type_params(<[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([::scale_info::TypeParameter::new(
                            "T",
                            ::core::option::Option::None,
                        )]),
                    ))
                    .docs_always(&[
                        "Contains one variant per dispatchable that can be called by an extrinsic.",
                    ])
                    .variant(
                        ::scale_info::build::Variants::new()
                            .variant("create", |v| {
                                v.index(0u8 as ::core::primitive::u8).fields(
                                    ::scale_info::build::Fields::named().field(|f| {
                                        f.ty::<[u8; 4]>().name("name").type_name("[u8; 4]")
                                    }),
                                )
                            })
                            .variant("breed", |v| {
                                v.index(1u8 as ::core::primitive::u8).fields(
                                    ::scale_info::build::Fields::named()
                                        .field(|f| {
                                            f.ty::<KittyId>()
                                                .name("kitty_id_1")
                                                .type_name("KittyId")
                                        })
                                        .field(|f| {
                                            f.ty::<KittyId>()
                                                .name("kitty_id_2")
                                                .type_name("KittyId")
                                        })
                                        .field(|f| {
                                            f.ty::<[u8; 4]>().name("name").type_name("[u8; 4]")
                                        }),
                                )
                            })
                            .variant("transfer", |v| {
                                v.index(2u8 as ::core::primitive::u8).fields(
                                    ::scale_info::build::Fields::named()
                                        .field(|f| f.ty::<u32>().name("kitty_id").type_name("u32"))
                                        .field(|f| {
                                            f.ty::<T::AccountId>()
                                                .name("recipient")
                                                .type_name("T::AccountId")
                                        }),
                                )
                            })
                            .variant("sale", |v| {
                                v.index(3u8 as ::core::primitive::u8).fields(
                                    ::scale_info::build::Fields::named()
                                        .field(|f| f.ty::<u32>().name("kitty_id").type_name("u32")),
                                )
                            })
                            .variant("buy", |v| {
                                v.index(4u8 as ::core::primitive::u8).fields(
                                    ::scale_info::build::Fields::named()
                                        .field(|f| f.ty::<u32>().name("kitty_id").type_name("u32")),
                                )
                            }),
                    )
            }
        };
    };
    impl<T: Config> Call<T> {
        ///Create a call with the variant `create`.
        pub fn new_call_variant_create(name: [u8; 4]) -> Self {
            Self::create { name }
        }
        ///Create a call with the variant `breed`.
        pub fn new_call_variant_breed(
            kitty_id_1: KittyId,
            kitty_id_2: KittyId,
            name: [u8; 4],
        ) -> Self {
            Self::breed {
                kitty_id_1,
                kitty_id_2,
                name,
            }
        }
        ///Create a call with the variant `transfer`.
        pub fn new_call_variant_transfer(kitty_id: u32, recipient: T::AccountId) -> Self {
            Self::transfer {
                kitty_id,
                recipient,
            }
        }
        ///Create a call with the variant `sale`.
        pub fn new_call_variant_sale(kitty_id: u32) -> Self {
            Self::sale { kitty_id }
        }
        ///Create a call with the variant `buy`.
        pub fn new_call_variant_buy(kitty_id: u32) -> Self {
            Self::buy { kitty_id }
        }
    }
    impl<T: Config> frame_support::dispatch::GetDispatchInfo for Call<T> {
        fn get_dispatch_info(&self) -> frame_support::dispatch::DispatchInfo {
            match *self {
                Self::create { ref name } => {
                    let __pallet_base_weight = 10_000;
                    let __pallet_weight =
                        <dyn frame_support::dispatch::WeighData<(&[u8; 4],)>>::weigh_data(
                            &__pallet_base_weight,
                            (name,),
                        );
                    let __pallet_class = < dyn frame_support :: dispatch :: ClassifyDispatch < (& [u8 ; 4] ,) > > :: classify_dispatch (& __pallet_base_weight , (name ,)) ;
                    let __pallet_pays_fee =
                        <dyn frame_support::dispatch::PaysFee<(&[u8; 4],)>>::pays_fee(
                            &__pallet_base_weight,
                            (name,),
                        );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::breed {
                    ref kitty_id_1,
                    ref kitty_id_2,
                    ref name,
                } => {
                    let __pallet_base_weight = 10_000;
                    let __pallet_weight = <dyn frame_support::dispatch::WeighData<(
                        &KittyId,
                        &KittyId,
                        &[u8; 4],
                    )>>::weigh_data(
                        &__pallet_base_weight, (kitty_id_1, kitty_id_2, name)
                    );
                    let __pallet_class = <dyn frame_support::dispatch::ClassifyDispatch<(
                        &KittyId,
                        &KittyId,
                        &[u8; 4],
                    )>>::classify_dispatch(
                        &__pallet_base_weight, (kitty_id_1, kitty_id_2, name)
                    );
                    let __pallet_pays_fee = <dyn frame_support::dispatch::PaysFee<(
                        &KittyId,
                        &KittyId,
                        &[u8; 4],
                    )>>::pays_fee(
                        &__pallet_base_weight, (kitty_id_1, kitty_id_2, name)
                    );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::transfer {
                    ref kitty_id,
                    ref recipient,
                } => {
                    let __pallet_base_weight = 10_000;
                    let __pallet_weight = <dyn frame_support::dispatch::WeighData<(
                        &u32,
                        &T::AccountId,
                    )>>::weigh_data(
                        &__pallet_base_weight, (kitty_id, recipient)
                    );
                    let __pallet_class = <dyn frame_support::dispatch::ClassifyDispatch<(
                        &u32,
                        &T::AccountId,
                    )>>::classify_dispatch(
                        &__pallet_base_weight, (kitty_id, recipient)
                    );
                    let __pallet_pays_fee =
                        <dyn frame_support::dispatch::PaysFee<(&u32, &T::AccountId)>>::pays_fee(
                            &__pallet_base_weight,
                            (kitty_id, recipient),
                        );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::sale { ref kitty_id } => {
                    let __pallet_base_weight = 10_000;
                    let __pallet_weight =
                        <dyn frame_support::dispatch::WeighData<(&u32,)>>::weigh_data(
                            &__pallet_base_weight,
                            (kitty_id,),
                        );
                    let __pallet_class =
                        <dyn frame_support::dispatch::ClassifyDispatch<(&u32,)>>::classify_dispatch(
                            &__pallet_base_weight,
                            (kitty_id,),
                        );
                    let __pallet_pays_fee =
                        <dyn frame_support::dispatch::PaysFee<(&u32,)>>::pays_fee(
                            &__pallet_base_weight,
                            (kitty_id,),
                        );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::buy { ref kitty_id } => {
                    let __pallet_base_weight = 10_000;
                    let __pallet_weight =
                        <dyn frame_support::dispatch::WeighData<(&u32,)>>::weigh_data(
                            &__pallet_base_weight,
                            (kitty_id,),
                        );
                    let __pallet_class =
                        <dyn frame_support::dispatch::ClassifyDispatch<(&u32,)>>::classify_dispatch(
                            &__pallet_base_weight,
                            (kitty_id,),
                        );
                    let __pallet_pays_fee =
                        <dyn frame_support::dispatch::PaysFee<(&u32,)>>::pays_fee(
                            &__pallet_base_weight,
                            (kitty_id,),
                        );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(format_args!(
                        "internal error: entered unreachable code: {0}",
                        format_args!("__Ignore cannot be used")
                    ));
                }
            }
        }
    }
    #[allow(deprecated)]
    impl<T: Config> frame_support::weights::GetDispatchInfo for Call<T> {}
    impl<T: Config> frame_support::dispatch::GetCallName for Call<T> {
        fn get_call_name(&self) -> &'static str {
            match *self {
                Self::create { .. } => "create",
                Self::breed { .. } => "breed",
                Self::transfer { .. } => "transfer",
                Self::sale { .. } => "sale",
                Self::buy { .. } => "buy",
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(format_args!(
                        "internal error: entered unreachable code: {0}",
                        format_args!("__PhantomItem cannot be used.")
                    ));
                }
            }
        }
        fn get_call_names() -> &'static [&'static str] {
            &["create", "breed", "transfer", "sale", "buy"]
        }
    }
    impl<T: Config> frame_support::traits::UnfilteredDispatchable for Call<T> {
        type RuntimeOrigin = frame_system::pallet_prelude::OriginFor<T>;
        fn dispatch_bypass_filter(
            self,
            origin: Self::RuntimeOrigin,
        ) -> frame_support::dispatch::DispatchResultWithPostInfo {
            frame_support::dispatch_context::run_in_context(|| match self {
                Self::create { name } => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "create",
                                    "pallet_kitties::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("pallets/kitties/src/lib.rs"),
                                    Some(12u32),
                                    Some("pallet_kitties::pallet"),
                                    ::tracing_core::field::FieldSet::new(
                                        &[],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && ::tracing::__macro_support::__is_enabled(
                                CALLSITE.metadata(),
                                interest,
                            )
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                        } else {
                            let span =
                                ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                            {};
                            span
                        }
                    };
                    let __tracing_guard__ = __within_span__.enter();
                    <Pallet<T>>::create(origin, name)
                        .map(Into::into)
                        .map_err(Into::into)
                }
                Self::breed {
                    kitty_id_1,
                    kitty_id_2,
                    name,
                } => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "breed",
                                    "pallet_kitties::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("pallets/kitties/src/lib.rs"),
                                    Some(12u32),
                                    Some("pallet_kitties::pallet"),
                                    ::tracing_core::field::FieldSet::new(
                                        &[],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && ::tracing::__macro_support::__is_enabled(
                                CALLSITE.metadata(),
                                interest,
                            )
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                        } else {
                            let span =
                                ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                            {};
                            span
                        }
                    };
                    let __tracing_guard__ = __within_span__.enter();
                    <Pallet<T>>::breed(origin, kitty_id_1, kitty_id_2, name)
                        .map(Into::into)
                        .map_err(Into::into)
                }
                Self::transfer {
                    kitty_id,
                    recipient,
                } => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "transfer",
                                    "pallet_kitties::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("pallets/kitties/src/lib.rs"),
                                    Some(12u32),
                                    Some("pallet_kitties::pallet"),
                                    ::tracing_core::field::FieldSet::new(
                                        &[],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && ::tracing::__macro_support::__is_enabled(
                                CALLSITE.metadata(),
                                interest,
                            )
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                        } else {
                            let span =
                                ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                            {};
                            span
                        }
                    };
                    let __tracing_guard__ = __within_span__.enter();
                    <Pallet<T>>::transfer(origin, kitty_id, recipient)
                        .map(Into::into)
                        .map_err(Into::into)
                }
                Self::sale { kitty_id } => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "sale",
                                    "pallet_kitties::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("pallets/kitties/src/lib.rs"),
                                    Some(12u32),
                                    Some("pallet_kitties::pallet"),
                                    ::tracing_core::field::FieldSet::new(
                                        &[],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && ::tracing::__macro_support::__is_enabled(
                                CALLSITE.metadata(),
                                interest,
                            )
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                        } else {
                            let span =
                                ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                            {};
                            span
                        }
                    };
                    let __tracing_guard__ = __within_span__.enter();
                    <Pallet<T>>::sale(origin, kitty_id)
                        .map(Into::into)
                        .map_err(Into::into)
                }
                Self::buy { kitty_id } => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "buy",
                                    "pallet_kitties::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("pallets/kitties/src/lib.rs"),
                                    Some(12u32),
                                    Some("pallet_kitties::pallet"),
                                    ::tracing_core::field::FieldSet::new(
                                        &[],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && ::tracing::__macro_support::__is_enabled(
                                CALLSITE.metadata(),
                                interest,
                            )
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                        } else {
                            let span =
                                ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                            {};
                            span
                        }
                    };
                    let __tracing_guard__ = __within_span__.enter();
                    <Pallet<T>>::buy(origin, kitty_id)
                        .map(Into::into)
                        .map_err(Into::into)
                }
                Self::__Ignore(_, _) => {
                    let _ = origin;
                    {
                        ::core::panicking::panic_fmt(format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("__PhantomItem cannot be used.")
                        ));
                    };
                }
            })
        }
    }
    impl<T: Config> frame_support::dispatch::Callable<T> for Pallet<T> {
        type RuntimeCall = Call<T>;
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn call_functions() -> frame_support::metadata::PalletCallMetadata {
            frame_support::scale_info::meta_type::<Call<T>>().into()
        }
    }
    impl<T: Config> frame_support::sp_std::fmt::Debug for Error<T> {
        fn fmt(
            &self,
            f: &mut frame_support::sp_std::fmt::Formatter<'_>,
        ) -> frame_support::sp_std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl<T: Config> Error<T> {
        #[doc(hidden)]
        pub fn as_str(&self) -> &'static str {
            match &self {
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(format_args!(
                        "internal error: entered unreachable code: {0}",
                        format_args!("`__Ignore` can never be constructed")
                    ));
                }
                Self::InvalidKittyId => "InvalidKittyId",
                Self::NotOwner => "NotOwner",
                Self::SameKittyId => "SameKittyId",
                Self::NoOwner => "NoOwner",
                Self::AlreadyOnSale => "AlreadyOnSale",
                Self::AlreadyOwned => "AlreadyOwned",
                Self::NotOnSale => "NotOnSale",
            }
        }
    }
    impl<T: Config> From<Error<T>> for &'static str {
        fn from(err: Error<T>) -> &'static str {
            err.as_str()
        }
    }
    impl<T: Config> From<Error<T>> for frame_support::sp_runtime::DispatchError {
        fn from(err: Error<T>) -> Self {
            use frame_support::codec::Encode;
            let index = < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: index :: < Pallet < T > > () . expect ("Every active module has an index in the runtime; qed") as u8 ;
            let mut encoded = err.encode();
            encoded.resize(frame_support::MAX_MODULE_ERROR_ENCODED_SIZE, 0);
            frame_support :: sp_runtime :: DispatchError :: Module (frame_support :: sp_runtime :: ModuleError { index , error : TryInto :: try_into (encoded) . expect ("encoded error is resized to be equal to the maximum encoded error size; qed") , message : Some (err . as_str ()) , })
        }
    }
    pub use __tt_error_token_1 as tt_error_token;
    #[doc(hidden)]
    pub mod __substrate_event_check {
        #[doc(hidden)]
        pub use __is_event_part_defined_2 as is_event_part_defined;
    }
    impl<T: Config> Pallet<T> {
        pub(super) fn deposit_event(event: Event<T>) {
            let event = <<T as Config>::RuntimeEvent as From<Event<T>>>::from(event);
            let event = <<T as Config>::RuntimeEvent as Into<
                <T as frame_system::Config>::RuntimeEvent,
            >>::into(event);
            <frame_system::Pallet<T>>::deposit_event(event)
        }
    }
    impl<T: Config> From<Event<T>> for () {
        fn from(_: Event<T>) {}
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn storage_metadata() -> frame_support::metadata::PalletStorageMetadata {
            frame_support :: metadata :: PalletStorageMetadata { prefix : < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: name :: < Pallet < T > > () . expect ("No name found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`.") , entries : { # [allow (unused_mut)] let mut entries = :: alloc :: vec :: Vec :: new () ; { < NextKittyId < T > as frame_support :: storage :: StorageEntryMetadataBuilder > :: build_metadata (:: alloc :: vec :: Vec :: new () , & mut entries) ; } { < Kitties < T > as frame_support :: storage :: StorageEntryMetadataBuilder > :: build_metadata (:: alloc :: vec :: Vec :: new () , & mut entries) ; } { < KittyOnSale < T > as frame_support :: storage :: StorageEntryMetadataBuilder > :: build_metadata (:: alloc :: vec :: Vec :: new () , & mut entries) ; } { < KittyParents < T > as frame_support :: storage :: StorageEntryMetadataBuilder > :: build_metadata (:: alloc :: vec :: Vec :: new () , & mut entries) ; } { < KittyOwner < T > as frame_support :: storage :: StorageEntryMetadataBuilder > :: build_metadata (:: alloc :: vec :: Vec :: new () , & mut entries) ; } entries } , }
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn next_kitty_id() -> KittyId {
            <NextKittyId<T> as frame_support::storage::StorageValue<KittyId>>::get()
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn kitties<KArg>(k: KArg) -> Option<Kitty>
        where
            KArg: frame_support::codec::EncodeLike<KittyId>,
        {
            <Kitties<T> as frame_support::storage::StorageMap<KittyId, Kitty>>::get(k)
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn kitty_on_sale<KArg>(k: KArg) -> Option<()>
        where
            KArg: frame_support::codec::EncodeLike<KittyId>,
        {
            <KittyOnSale<T> as frame_support::storage::StorageMap<KittyId, ()>>::get(k)
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn kitty_parents<KArg>(k: KArg) -> Option<(KittyId, KittyId)>
        where
            KArg: frame_support::codec::EncodeLike<KittyId>,
        {
            < KittyParents < T > as frame_support :: storage :: StorageMap < KittyId , (KittyId , KittyId) > > :: get (k)
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn kitty_owner<KArg>(k: KArg) -> Option<T::AccountId>
        where
            KArg: frame_support::codec::EncodeLike<KittyId>,
        {
            <KittyOwner<T> as frame_support::storage::StorageMap<KittyId, T::AccountId>>::get(k)
        }
    }
    #[doc(hidden)]
    pub struct _GeneratedPrefixForStorageNextKittyId<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance
        for _GeneratedPrefixForStorageNextKittyId<T>
    {
        fn pallet_prefix() -> &'static str {
            < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: name :: < Pallet < T > > () . expect ("No name found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`.")
        }
        const STORAGE_PREFIX: &'static str = "NextKittyId";
    }
    #[doc(hidden)]
    pub struct _GeneratedPrefixForStorageKitties<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance for _GeneratedPrefixForStorageKitties<T> {
        fn pallet_prefix() -> &'static str {
            < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: name :: < Pallet < T > > () . expect ("No name found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`.")
        }
        const STORAGE_PREFIX: &'static str = "Kitties";
    }
    #[doc(hidden)]
    pub struct _GeneratedPrefixForStorageKittyOnSale<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance
        for _GeneratedPrefixForStorageKittyOnSale<T>
    {
        fn pallet_prefix() -> &'static str {
            < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: name :: < Pallet < T > > () . expect ("No name found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`.")
        }
        const STORAGE_PREFIX: &'static str = "KittyOnSale";
    }
    #[doc(hidden)]
    pub struct _GeneratedPrefixForStorageKittyParents<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance
        for _GeneratedPrefixForStorageKittyParents<T>
    {
        fn pallet_prefix() -> &'static str {
            < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: name :: < Pallet < T > > () . expect ("No name found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`.")
        }
        const STORAGE_PREFIX: &'static str = "KittyParents";
    }
    #[doc(hidden)]
    pub struct _GeneratedPrefixForStorageKittyOwner<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance for _GeneratedPrefixForStorageKittyOwner<T> {
        fn pallet_prefix() -> &'static str {
            < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: name :: < Pallet < T > > () . expect ("No name found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`.")
        }
        const STORAGE_PREFIX: &'static str = "KittyOwner";
    }
    #[doc(hidden)]
    pub mod __substrate_inherent_check {
        #[doc(hidden)]
        pub use __is_inherent_part_defined_3 as is_inherent_part_defined;
    }
    /// Hidden instance generated to be internally used when module is used without
    /// instance.
    #[doc(hidden)]
    pub type __InherentHiddenInstance = ();
    impl<T: Config> frame_support::traits::OnFinalize<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_finalize(n: <T as frame_system::Config>::BlockNumber) {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_finalize",
                            "pallet_kitties::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/kitties/src/lib.rs"),
                            Some(12u32),
                            Some("pallet_kitties::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(CALLSITE.metadata(), interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_finalize (n)
        }
    }
    impl<T: Config> frame_support::traits::OnIdle<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_idle(
            n: <T as frame_system::Config>::BlockNumber,
            remaining_weight: frame_support::weights::Weight,
        ) -> frame_support::weights::Weight {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_idle (n , remaining_weight)
        }
    }
    impl<T: Config> frame_support::traits::OnInitialize<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_initialize(
            n: <T as frame_system::Config>::BlockNumber,
        ) -> frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_initialize",
                            "pallet_kitties::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/kitties/src/lib.rs"),
                            Some(12u32),
                            Some("pallet_kitties::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(CALLSITE.metadata(), interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_initialize (n)
        }
    }
    impl<T: Config> frame_support::traits::OnRuntimeUpgrade for Pallet<T> {
        fn on_runtime_upgrade() -> frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_runtime_update",
                            "pallet_kitties::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/kitties/src/lib.rs"),
                            Some(12u32),
                            Some("pallet_kitties::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(CALLSITE.metadata(), interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            let pallet_name = < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: name :: < Self > () . unwrap_or ("<unknown pallet name>") ;
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    :: log :: __private_api_log (format_args ! ("⚠️ {0} declares internal migrations (which *might* execute). On-chain `{1:?}` vs current storage version `{2:?}`" , pallet_name , < Self as frame_support :: traits :: GetStorageVersion >:: on_chain_storage_version () , < Self as frame_support :: traits :: GetStorageVersion >:: current_storage_version ()) , lvl , & (frame_support :: LOG_TARGET , "pallet_kitties::pallet" , "pallets/kitties/src/lib.rs" , 12u32) , :: log :: __private_api :: Option :: None) ;
                }
            };
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_runtime_upgrade ()
        }
    }
    impl<T: Config> frame_support::traits::OffchainWorker<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn offchain_worker(n: <T as frame_system::Config>::BlockNumber) {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: offchain_worker (n)
        }
    }
    impl<T: Config> frame_support::traits::IntegrityTest for Pallet<T> {
        fn integrity_test() {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: integrity_test ()
        }
    }
    #[doc(hidden)]
    pub mod __substrate_genesis_config_check {
        #[doc(hidden)]
        pub use __is_genesis_config_defined_4 as is_genesis_config_defined;
        #[doc(hidden)]
        pub use __is_std_enabled_for_genesis_4 as is_std_enabled_for_genesis;
    }
    #[doc(hidden)]
    pub mod __substrate_origin_check {
        #[doc(hidden)]
        pub use __is_origin_part_defined_5 as is_origin_part_defined;
    }
    #[doc(hidden)]
    pub mod __substrate_validate_unsigned_check {
        #[doc(hidden)]
        pub use __is_validate_unsigned_part_defined_6 as is_validate_unsigned_part_defined;
    }
    pub use __tt_default_parts_7 as tt_default_parts;
}
