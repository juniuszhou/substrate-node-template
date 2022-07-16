#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;

pub use pallet::*;






#[doc =
r"
			The module that hosts all the
			[FRAME](https://docs.substrate.io/v3/runtime/frame)
			types needed to add this pallet to a
			runtime.
			"]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_support::storage_alias;

    #[doc = " define data structure used in the pallet storage"]
    pub struct DataStructure {
        pub id: u32,
    }
    #[allow(deprecated)]
    const _: () =
        {
            #[automatically_derived]
            impl ::codec::Encode for DataStructure {
                fn encode_to<__CodecOutputEdqy: ::codec::Output +
                    ?::core::marker::Sized>(&self,
                    __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    ::codec::Encode::encode_to(&&self.id, __codec_dest_edqy)
                }
                fn encode(&self)
                    -> ::codec::alloc::vec::Vec<::core::primitive::u8> {
                    ::codec::Encode::encode(&&self.id)
                }
                fn using_encoded<R,
                    F: ::core::ops::FnOnce(&[::core::primitive::u8])
                    -> R>(&self, f: F) -> R {
                    ::codec::Encode::using_encoded(&&self.id, f)
                }
            }
            #[automatically_derived]
            impl ::codec::EncodeLike for DataStructure { }
        };
    #[allow(deprecated)]
    const _: () =
        {
            #[automatically_derived]
            impl ::codec::Decode for DataStructure {
                fn decode<__CodecInputEdqy: ::codec::Input>(__codec_input_edqy:
                        &mut __CodecInputEdqy)
                    -> ::core::result::Result<Self, ::codec::Error> {
                    ::core::result::Result::Ok(DataStructure {
                            id: {
                                let __codec_res_edqy =
                                    <u32 as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) =>
                                        return ::core::result::Result::Err(e.chain("Could not decode `DataStructure::id`")),
                                    ::core::result::Result::Ok(__codec_res_edqy) =>
                                        __codec_res_edqy,
                                }
                            },
                        })
                }
            }
        };
    const _: () =
        {
            impl ::codec::MaxEncodedLen for DataStructure {
                fn max_encoded_len() -> ::core::primitive::usize {
                    0_usize.saturating_add(<u32>::max_encoded_len())
                }
            }
        };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for DataStructure {
        #[inline]
        fn clone(&self) -> DataStructure {
            DataStructure { id: ::core::clone::Clone::clone(&self.id) }
        }
    }
    impl ::core::marker::StructuralPartialEq for DataStructure {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for DataStructure {
        #[inline]
        fn eq(&self, other: &DataStructure) -> bool { self.id == other.id }
        #[inline]
        fn ne(&self, other: &DataStructure) -> bool { self.id != other.id }
    }
    impl ::core::marker::StructuralEq for DataStructure {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for DataStructure {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            { let _: ::core::cmp::AssertParamIsEq<u32>; }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for DataStructure {
        #[inline]
        fn default() -> DataStructure {
            DataStructure { id: ::core::default::Default::default() }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            impl ::scale_info::TypeInfo for DataStructure {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder().path(::scale_info::Path::new("DataStructure",
                                        "pallet_trial::pallet")).type_params(











                                // Errors inform users that something went wrong.

                                // Dispatchable functions allows users to interact with the pallet and invoke state changes.
                                // These functions materialize as "extrinsics", which are often compared to transactions.
                                // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
                                // Check that the extrinsic was signed and get the signer.
                                // This function will return an error if the extrinsic is not signed.
                                // https://docs.substrate.io/v3/runtime/origins

                                // Update storage.



                                // Emit an event.
                                // Return a successful DispatchResultWithPostInfo








                                // Read a value from storage.
                                // Return an error if the value has not been set.
                                // Increment the value read from storage; will error in the event of overflow.
                                // Update the value in storage with the incremented result.


                                ::alloc::vec::Vec::new()).docs(&["define data structure used in the pallet storage"]).composite(::scale_info::build::Fields::named().field(|f|
                                f.ty::<u32>().name("id").type_name("u32").docs(&[])))
                }
            }
            ;
        };
    #[doc =
    " define a method to return a value, it could be used in a storage unit"]
    pub fn __type_value_for_get_default_value() -> u32 { 1234u32 }
    struct CounterForValidators_Storage_Instance<T: Config>(frame_support::sp_std::marker::PhantomData<(T)>);
    impl<T: Config> frame_support::traits::StorageInstance for
        CounterForValidators_Storage_Instance<T> {
        fn pallet_prefix() -> &'static str {
            <Pallet<T> as frame_support::traits::PalletInfoAccess>::name()
        }
        const STORAGE_PREFIX: &'static str = "CounterForValidators";
    }
    #[doc = " the usage of storage alias"]
    type CounterForValidators<T> =
        frame_support::storage::types::StorageValue<CounterForValidators_Storage_Instance<T>,
        u32>;
    #[doc =
    r"
			Configuration trait of this pallet.

			Implement this type for a runtime in order to customize this pallet.
			"]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> +
            IsType<<Self as frame_system::Config>::Event>;
    }
    #[doc =
    r"
			The [pallet](https://docs.substrate.io/v3/runtime/frame#pallets) implementing
			the on-chain logic.
			"]
    pub struct Pallet<T>(frame_support::sp_std::marker::PhantomData<(T)>);
    const _: () =
        {
            impl<T> core::clone::Clone for Pallet<T> {
                fn clone(&self) -> Self {
                    Self(core::clone::Clone::clone(&self.0))
                }
            }
        };
    const _: () =
        {
            impl<T> core::cmp::Eq for Pallet<T> {}
        };
    const _: () =
        {
            impl<T> core::cmp::PartialEq for Pallet<T> {
                fn eq(&self, other: &Self) -> bool {
                    true && self.0 == other.0
                }
            }
        };
    const _: () =
        {
            impl<T> core::fmt::Debug for Pallet<T> {
                fn fmt(&self, fmt: &mut core::fmt::Formatter)
                    -> core::fmt::Result {
                    fmt.debug_tuple("Pallet").field(&self.0).finish()
                }
            }
        };
    #[doc =
    " if not value query type, the default is option value, value could be some(u32) or none"]
    #[allow(type_alias_bounds)]
    pub type Something<T> =
        StorageValue<_GeneratedPrefixForStorageSomething<T>, u32>;
    #[allow(type_alias_bounds)]
    pub type OptionValue<T> =
        StorageValue<_GeneratedPrefixForStorageOptionValue<T>, u32,
        OptionQuery>;
    #[doc = " query type storage"]
    #[allow(type_alias_bounds)]
    pub type QueryValue<T> =
        StorageValue<_GeneratedPrefixForStorageQueryValue<T>, u32,
        ValueQuery>;
    #[doc = " query type with default value"]
    #[allow(type_alias_bounds)]
    pub type QueryWithDefault<T> =
        StorageValue<_GeneratedPrefixForStorageQueryWithDefault<T>, u32,
        ValueQuery, GetDefaultValue>;
    #[doc =
    " if not value query type, the default is option value, value could be some(u32) or none"]
    #[allow(type_alias_bounds)]
    pub type StrutureData<T> =
        StorageValue<_GeneratedPrefixForStorageStrutureData<T>,
        DataStructure>;
    #[doc =
    r"
			The [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted
			by this pallet.
			"]
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    pub enum Event<T: Config> {

        #[doc =
        " Event documentation should end with an array that provides descriptive names for event"]
        #[doc = " parameters. [something, who]"]
        SomethingStored(u32, T::AccountId),

        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(frame_support::sp_std::marker::PhantomData<(T)>,
            frame_support::Never),
    }
    const _: () =
        {
            impl<T: Config> core::clone::Clone for Event<T> {
                fn clone(&self) -> Self {
                    match self {
                        Self::SomethingStored(ref _0, ref _1) =>
                            Self::SomethingStored(core::clone::Clone::clone(_0),
                                core::clone::Clone::clone(_1)),
                        Self::__Ignore(ref _0, ref _1) =>
                            Self::__Ignore(core::clone::Clone::clone(_0),
                                core::clone::Clone::clone(_1)),
                    }
                }
            }
        };
    const _: () =
        {
            impl<T: Config> core::cmp::Eq for Event<T> {}
        };
    const _: () =
        {
            impl<T: Config> core::cmp::PartialEq for Event<T> {
                fn eq(&self, other: &Self) -> bool {
                    match (self, other) {
                        (Self::SomethingStored(_0, _1),
                            Self::SomethingStored(_0_other, _1_other)) =>
                            true && _0 == _0_other && _1 == _1_other,
                        (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other))
                            => true && _0 == _0_other && _1 == _1_other,
                        (Self::SomethingStored { .. }, Self::__Ignore { .. }) =>
                            false,
                        (Self::__Ignore { .. }, Self::SomethingStored { .. }) =>
                            false,
                    }
                }
            }
        };
    const _: () =
        {
            impl<T: Config> core::fmt::Debug for Event<T> {
                fn fmt(&self, fmt: &mut core::fmt::Formatter)
                    -> core::fmt::Result {
                    match *self {
                        Self::SomethingStored(ref _0, ref _1) => {
                            fmt.debug_tuple("Event::SomethingStored").field(&_0).field(&_1).finish()
                        }
                        Self::__Ignore(ref _0, ref _1) => {
                            fmt.debug_tuple("Event::__Ignore").field(&_0).field(&_1).finish()
                        }
                    }
                }
            }
        };
    #[allow(deprecated)]
    const _: () =
        {
            #[automatically_derived]
            impl<T: Config> ::codec::Encode for Event<T> where
                T::AccountId: ::codec::Encode, T::AccountId: ::codec::Encode {
                fn encode_to<__CodecOutputEdqy: ::codec::Output +
                    ?::core::marker::Sized>(&self,
                    __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    match *self {
                        Event::SomethingStored(ref aa, ref ba) => {
                            __codec_dest_edqy.push_byte(0usize as
                                    ::core::primitive::u8);
                            ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                            ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                        }
                        _ => (),
                    }
                }
            }
            #[automatically_derived]
            impl<T: Config> ::codec::EncodeLike for Event<T> where
                T::AccountId: ::codec::Encode, T::AccountId: ::codec::Encode {
            }
        };
    #[allow(deprecated)]
    const _: () =
        {
            #[automatically_derived]
            impl<T: Config> ::codec::Decode for Event<T> where
                T::AccountId: ::codec::Decode, T::AccountId: ::codec::Decode {
                fn decode<__CodecInputEdqy: ::codec::Input>(__codec_input_edqy:
                        &mut __CodecInputEdqy)
                    -> ::core::result::Result<Self, ::codec::Error> {
                    match __codec_input_edqy.read_byte().map_err(|e|
                                    e.chain("Could not decode `Event`, failed to read variant byte"))?
                        {
                        __codec_x_edqy if
                            __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                            ::core::result::Result::Ok(Event::<T>::SomethingStored({
                                        let __codec_res_edqy =
                                            <u32 as ::codec::Decode>::decode(__codec_input_edqy);
                                        match __codec_res_edqy {
                                            ::core::result::Result::Err(e) =>
                                                return ::core::result::Result::Err(e.chain("Could not decode `Event::SomethingStored.0`")),
                                            ::core::result::Result::Ok(__codec_res_edqy) =>
                                                __codec_res_edqy,
                                        }
                                    },
                                    {
                                        let __codec_res_edqy =
                                            <T::AccountId as
                                                    ::codec::Decode>::decode(__codec_input_edqy);
                                        match __codec_res_edqy {
                                            ::core::result::Result::Err(e) =>
                                                return ::core::result::Result::Err(e.chain("Could not decode `Event::SomethingStored.1`")),
                                            ::core::result::Result::Ok(__codec_res_edqy) =>
                                                __codec_res_edqy,
                                        }
                                    }))
                        }
                        _ =>
                            ::core::result::Result::Err(<_ as
                                        ::core::convert::Into<_>>::into("Could not decode `Event`, variant doesn't exist")),
                    }
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            impl<T: Config> ::scale_info::TypeInfo for Event<T> where
                T::AccountId: ::scale_info::TypeInfo + 'static,
                frame_support::sp_std::marker::PhantomData<(T)>: ::scale_info::TypeInfo +
                'static, T: Config + 'static {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder().path(::scale_info::Path::new("Event",
                                        "pallet_trial::pallet")).type_params(<[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([::scale_info::TypeParameter::new("T",
                                                    ::core::option::Option::None)]))).docs_always(&["\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]).variant(::scale_info::build::Variants::new().variant("SomethingStored",
                            |v|
                                v.index(0usize as
                                                ::core::primitive::u8).fields(::scale_info::build::Fields::unnamed().field(|f|
                                                    f.ty::<u32>().type_name("u32").docs_always(&[])).field(|f|
                                                f.ty::<T::AccountId>().type_name("T::AccountId").docs_always(&[]))).docs_always(&["Event documentation should end with an array that provides descriptive names for event",
                                                "parameters. [something, who]"])))
                }
            }
            ;
        };
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    #[doc =
    r"
			Custom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)
			of this pallet.
			"]
    pub enum Error<T> {

        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(frame_support::sp_std::marker::PhantomData<(T)>,
            frame_support::Never),

        #[doc = " Error names should be descriptive."]
        NoneValue,

        #[doc =
        " Errors should have helpful documentation associated with them."]
        StorageOverflow,
    }
    #[allow(deprecated)]
    const _: () =
        {
            #[automatically_derived]
            impl<T> ::codec::Encode for Error<T> {
                fn encode_to<__CodecOutputEdqy: ::codec::Output +
                    ?::core::marker::Sized>(&self,
                    __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    match *self {
                        Error::NoneValue => {
                            __codec_dest_edqy.push_byte(0usize as
                                    ::core::primitive::u8);
                        }
                        Error::StorageOverflow => {
                            __codec_dest_edqy.push_byte(1usize as
                                    ::core::primitive::u8);
                        }
                        _ => (),
                    }
                }
            }
            #[automatically_derived]
            impl<T> ::codec::EncodeLike for Error<T> { }
        };
    #[allow(deprecated)]
    const _: () =
        {
            #[automatically_derived]
            impl<T> ::codec::Decode for Error<T> {
                fn decode<__CodecInputEdqy: ::codec::Input>(__codec_input_edqy:
                        &mut __CodecInputEdqy)
                    -> ::core::result::Result<Self, ::codec::Error> {
                    match __codec_input_edqy.read_byte().map_err(|e|
                                    e.chain("Could not decode `Error`, failed to read variant byte"))?
                        {
                        __codec_x_edqy if
                            __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                            ::core::result::Result::Ok(Error::<T>::NoneValue)
                        }
                        __codec_x_edqy if
                            __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                            ::core::result::Result::Ok(Error::<T>::StorageOverflow)
                        }
                        _ =>
                            ::core::result::Result::Err(<_ as
                                        ::core::convert::Into<_>>::into("Could not decode `Error`, variant doesn't exist")),
                    }
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            impl<T> ::scale_info::TypeInfo for Error<T> where
                frame_support::sp_std::marker::PhantomData<(T)>: ::scale_info::TypeInfo +
                'static, T: 'static {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder().path(::scale_info::Path::new("Error",
                                        "pallet_trial::pallet")).type_params(<[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([::scale_info::TypeParameter::new("T",
                                                    ::core::option::Option::None)]))).docs_always(&["\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]).variant(::scale_info::build::Variants::new().variant("NoneValue",
                                |v|
                                    v.index(0usize as
                                                ::core::primitive::u8).docs_always(&["Error names should be descriptive."])).variant("StorageOverflow",
                            |v|
                                v.index(1usize as
                                            ::core::primitive::u8).docs_always(&["Errors should have helpful documentation associated with them."])))
                }
            }
            ;
        };
    const _: () =
        {
            impl<T> frame_support::traits::PalletError for Error<T> {
                const MAX_ENCODED_SIZE: usize = 1;
            }
        };
    impl<T: Config> Pallet<T> {
        #[doc =
        " An example dispatchable that takes a singles value as a parameter, writes the value to"]
        #[doc =
        " storage and emits an event. This function must be dispatched by a signed extrinsic."]
        pub fn set_value(origin: OriginFor<T>, something: u32)
            -> DispatchResult {
            let who = ensure_signed(origin)?;
            <Something<T>>::put(something);
            <QueryValue<T>>::put(something);
            <OptionValue<T>>::put(something);
            Self::deposit_event(Event::SomethingStored(something, who));
            Ok(())
        }
        pub fn check_value(origin: OriginFor<T>) -> DispatchResult {
            ensure_signed(origin)?;
            match <Something<T>>::get() { None => {} Some(_) => {} };
            match <QueryValue<T>>::get() { 0_u32 => {} _ => {} };
            match <OptionValue<T>>::get() { None => {} Some(_) => {} };
            Ok(())
        }
        #[doc = " An example dispatchable that may throw a custom error."]
        pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            match <Something<T>>::get() {
                None => return Err(Error::<T>::NoneValue.into()),
                Some(old) => {
                    let new =
                        old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                    <Something<T>>::put(new);
                    Ok(())
                }
            }
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn internal_fn() {}
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn pallet_constants_metadata()
            ->
                frame_support::sp_std::vec::Vec<frame_support::metadata::PalletConstantMetadata> {
            ::alloc::vec::Vec::new()
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn error_metadata()
            -> Option<frame_support::metadata::PalletErrorMetadata> {
            Some(frame_support::metadata::PalletErrorMetadata {
                    ty: frame_support::scale_info::meta_type::<Error<T>>(),
                })
        }
    }
    #[doc = r" Type alias to `Pallet`, to be used by `construct_runtime`."]
    #[doc = r""]
    #[doc = r" Generated by `pallet` attribute macro."]
    #[deprecated(note = "use `Pallet` instead")]
    #[allow(dead_code)]
    pub type Module<T> = Pallet<T>;
    impl<T: Config> frame_support::traits::GetStorageVersion for Pallet<T> {
        fn current_storage_version()
            -> frame_support::traits::StorageVersion {
            frame_support::traits::StorageVersion::default()
        }
        fn on_chain_storage_version()
            -> frame_support::traits::StorageVersion {
            frame_support::traits::StorageVersion::get::<Self>()
        }
    }
    impl<T: Config> frame_support::traits::OnGenesis for Pallet<T> {
        fn on_genesis() {
            let storage_version =
                frame_support::traits::StorageVersion::default();
            storage_version.put::<Self>();
        }
    }
    impl<T: Config> frame_support::traits::PalletInfoAccess for Pallet<T> {
        fn index() -> usize {
            <<T as frame_system::Config>::PalletInfo as
                        frame_support::traits::PalletInfo>::index::<Self>().expect("Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime")
        }
        fn name() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as
                        frame_support::traits::PalletInfo>::name::<Self>().expect("Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime")
        }
        fn module_name() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as
                        frame_support::traits::PalletInfo>::module_name::<Self>().expect("Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime")
        }
        fn crate_version() -> frame_support::traits::CrateVersion {
            frame_support::traits::CrateVersion {
                major: 0u16,
                minor: 0u8,
                patch: 1u8,
            }
        }
    }
    impl<T: Config> frame_support::traits::PalletsInfoAccess for Pallet<T> {
        fn count() -> usize { 1 }
        fn accumulate(acc:
                &mut frame_support::sp_std::vec::Vec<frame_support::traits::PalletInfoData>) {
            use frame_support::traits::PalletInfoAccess;
            let item =
                frame_support::traits::PalletInfoData {
                    index: Self::index(),
                    name: Self::name(),
                    module_name: Self::module_name(),
                    crate_version: Self::crate_version(),
                };
            acc.push(item);
        }
    }
    impl<T: Config> frame_support::traits::StorageInfoTrait for Pallet<T> {
        fn storage_info()
            ->
                frame_support::sp_std::vec::Vec<frame_support::traits::StorageInfo> {
            #[allow(unused_mut)]
            let mut res = ::alloc::vec::Vec::new();
            {
                let mut storage_info =
                    <Something<T> as
                            frame_support::traits::StorageInfoTrait>::storage_info();
                res.append(&mut storage_info);
            }
            {
                let mut storage_info =
                    <OptionValue<T> as
                            frame_support::traits::StorageInfoTrait>::storage_info();
                res.append(&mut storage_info);
            }
            {
                let mut storage_info =
                    <QueryValue<T> as
                            frame_support::traits::StorageInfoTrait>::storage_info();
                res.append(&mut storage_info);
            }
            {
                let mut storage_info =
                    <QueryWithDefault<T> as
                            frame_support::traits::StorageInfoTrait>::storage_info();
                res.append(&mut storage_info);
            }
            {
                let mut storage_info =
                    <StrutureData<T> as
                            frame_support::traits::StorageInfoTrait>::storage_info();
                res.append(&mut storage_info);
            }
            res
        }
    }
    #[doc(hidden)]
    pub mod __substrate_call_check {
        #[macro_export]
        #[doc(hidden)]
        macro_rules! __is_call_part_defined_0 {
            ($pallet_name : ident) => {} ;
        }
        #[doc(hidden)]
        pub use __is_call_part_defined_0 as is_call_part_defined;
    }
    #[doc =
    r"Contains one variant per dispatchable that can be called by an extrinsic."]
    #[codec(encode_bound())]
    #[codec(decode_bound())]
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    #[allow(non_camel_case_types)]
    pub enum Call<T: Config> {

        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(frame_support::sp_std::marker::PhantomData<(T,)>,
            frame_support::Never),

        #[doc =
        " An example dispatchable that takes a singles value as a parameter, writes the value to"]
        #[doc =
        " storage and emits an event. This function must be dispatched by a signed extrinsic."]
        #[codec(index = 0u8)]
        set_value {
            #[allow(missing_docs)]
            something: u32,
        },

        #[codec(index = 1u8)]
        check_value {},

        #[doc = " An example dispatchable that may throw a custom error."]
        #[codec(index = 2u8)]
        cause_error {},
    }
    const _: () =
        {
            impl<T: Config> core::fmt::Debug for Call<T> {
                fn fmt(&self, fmt: &mut core::fmt::Formatter)
                    -> core::fmt::Result {
                    match *self {
                        Self::__Ignore(ref _0, ref _1) => {
                            fmt.debug_tuple("Call::__Ignore").field(&_0).field(&_1).finish()
                        }
                        Self::set_value { ref something } => {
                            fmt.debug_struct("Call::set_value").field("something",
                                    &something).finish()
                        }
                        Self::check_value {} => {
                            fmt.debug_struct("Call::check_value").finish()
                        }
                        Self::cause_error {} => {
                            fmt.debug_struct("Call::cause_error").finish()
                        }
                    }
                }
            }
        };
    const _: () =
        {
            impl<T: Config> core::clone::Clone for Call<T> {
                fn clone(&self) -> Self {
                    match self {
                        Self::__Ignore(ref _0, ref _1) =>
                            Self::__Ignore(core::clone::Clone::clone(_0),
                                core::clone::Clone::clone(_1)),
                        Self::set_value { ref something } =>
                            Self::set_value {
                                something: core::clone::Clone::clone(something),
                            },
                        Self::check_value {} => Self::check_value {},
                        Self::cause_error {} => Self::cause_error {},
                    }
                }
            }
        };
    const _: () =
        {
            impl<T: Config> core::cmp::Eq for Call<T> {}
        };
    const _: () =
        {
            impl<T: Config> core::cmp::PartialEq for Call<T> {
                fn eq(&self, other: &Self) -> bool {
                    match (self, other) {
                        (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other))
                            => true && _0 == _0_other && _1 == _1_other,
                        (Self::set_value { something }, Self::set_value {
                            something: _0 }) => true && something == _0,
                        (Self::check_value {}, Self::check_value {}) => true,
                        (Self::cause_error {}, Self::cause_error {}) => true,
                        (Self::__Ignore { .. }, Self::set_value { .. }) => false,
                        (Self::__Ignore { .. }, Self::check_value { .. }) => false,
                        (Self::__Ignore { .. }, Self::cause_error { .. }) => false,
                        (Self::set_value { .. }, Self::__Ignore { .. }) => false,
                        (Self::set_value { .. }, Self::check_value { .. }) => false,
                        (Self::set_value { .. }, Self::cause_error { .. }) => false,
                        (Self::check_value { .. }, Self::__Ignore { .. }) => false,
                        (Self::check_value { .. }, Self::set_value { .. }) => false,
                        (Self::check_value { .. }, Self::cause_error { .. }) =>
                            false,
                        (Self::cause_error { .. }, Self::__Ignore { .. }) => false,
                        (Self::cause_error { .. }, Self::set_value { .. }) => false,
                        (Self::cause_error { .. }, Self::check_value { .. }) =>
                            false,
                    }
                }
            }
        };
    #[allow(deprecated)]
    const _: () =
        {
            #[allow(non_camel_case_types)]
            #[automatically_derived]
            impl<T: Config> ::codec::Encode for Call<T> {
                fn encode_to<__CodecOutputEdqy: ::codec::Output +
                    ?::core::marker::Sized>(&self,
                    __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    match *self {
                        Call::set_value { ref something } => {
                            __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                            ::codec::Encode::encode_to(something, __codec_dest_edqy);
                        }
                        Call::check_value {} => {
                            __codec_dest_edqy.push_byte(1u8 as ::core::primitive::u8);
                        }
                        Call::cause_error {} => {
                            __codec_dest_edqy.push_byte(2u8 as ::core::primitive::u8);
                        }
                        _ => (),
                    }
                }
            }
            #[automatically_derived]
            impl<T: Config> ::codec::EncodeLike for Call<T> { }
        };
    #[allow(deprecated)]
    const _: () =
        {
            #[allow(non_camel_case_types)]
            #[automatically_derived]
            impl<T: Config> ::codec::Decode for Call<T> {
                fn decode<__CodecInputEdqy: ::codec::Input>(__codec_input_edqy:
                        &mut __CodecInputEdqy)
                    -> ::core::result::Result<Self, ::codec::Error> {
                    match __codec_input_edqy.read_byte().map_err(|e|
                                    e.chain("Could not decode `Call`, failed to read variant byte"))?
                        {
                        __codec_x_edqy if
                            __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                            ::core::result::Result::Ok(Call::<T>::set_value {
                                    something: {
                                        let __codec_res_edqy =
                                            <u32 as ::codec::Decode>::decode(__codec_input_edqy);
                                        match __codec_res_edqy {
                                            ::core::result::Result::Err(e) =>
                                                return ::core::result::Result::Err(e.chain("Could not decode `Call::set_value::something`")),
                                            ::core::result::Result::Ok(__codec_res_edqy) =>
                                                __codec_res_edqy,
                                        }
                                    },
                                })
                        }
                        __codec_x_edqy if
                            __codec_x_edqy == 1u8 as ::core::primitive::u8 => {
                            ::core::result::Result::Ok(Call::<T>::check_value {})
                        }
                        __codec_x_edqy if
                            __codec_x_edqy == 2u8 as ::core::primitive::u8 => {
                            ::core::result::Result::Ok(Call::<T>::cause_error {})
                        }
                        _ =>
                            ::core::result::Result::Err(<_ as
                                        ::core::convert::Into<_>>::into("Could not decode `Call`, variant doesn't exist")),
                    }
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            impl<T: Config> ::scale_info::TypeInfo for Call<T> where
                frame_support::sp_std::marker::PhantomData<(T,)>: ::scale_info::TypeInfo +
                'static, T: Config + 'static {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder().path(::scale_info::Path::new("Call",
                                        "pallet_trial::pallet")).type_params(<[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([::scale_info::TypeParameter::new("T",
                                                    ::core::option::Option::None)]))).docs_always(&["Contains one variant per dispatchable that can be called by an extrinsic."]).variant(::scale_info::build::Variants::new().variant("set_value",
                                    |v|
                                        v.index(0u8 as
                                                        ::core::primitive::u8).fields(::scale_info::build::Fields::named().field(|f|
                                                        f.ty::<u32>().name("something").type_name("u32").docs_always(&[]))).docs_always(&["An example dispatchable that takes a singles value as a parameter, writes the value to",
                                                        "storage and emits an event. This function must be dispatched by a signed extrinsic."])).variant("check_value",
                                |v|
                                    v.index(1u8 as
                                                    ::core::primitive::u8).fields(::scale_info::build::Fields::named()).docs_always(&[])).variant("cause_error",
                            |v|
                                v.index(2u8 as
                                                ::core::primitive::u8).fields(::scale_info::build::Fields::named()).docs_always(&["An example dispatchable that may throw a custom error."])))
                }
            }
            ;
        };
    impl<T: Config> Call<T> {
        #[doc = "Create a call with the variant `set_value`."]
        pub fn new_call_variant_set_value(something: u32) -> Self {
            Self::set_value { something }
        }
        #[doc = "Create a call with the variant `check_value`."]
        pub fn new_call_variant_check_value() -> Self { Self::check_value {} }
        #[doc = "Create a call with the variant `cause_error`."]
        pub fn new_call_variant_cause_error() -> Self { Self::cause_error {} }
    }
    impl<T: Config> frame_support::dispatch::GetDispatchInfo for Call<T> {
        fn get_dispatch_info(&self) -> frame_support::dispatch::DispatchInfo {
            match *self {
                Self::set_value { ref something } => {
                    let __pallet_base_weight =
                        10_000 + T::DbWeight::get().writes(1);
                    let __pallet_weight =
                        <dyn frame_support::dispatch::WeighData<(&u32,)>>::weigh_data(&__pallet_base_weight,
                            (something,));
                    let __pallet_class =
                        <dyn frame_support::dispatch::ClassifyDispatch<(&u32,)>>::classify_dispatch(&__pallet_base_weight,
                            (something,));
                    let __pallet_pays_fee =
                        <dyn frame_support::dispatch::PaysFee<(&u32,)>>::pays_fee(&__pallet_base_weight,
                            (something,));
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::check_value {} => {
                    let __pallet_base_weight = 10_000;
                    let __pallet_weight =
                        <dyn frame_support::dispatch::WeighData<()>>::weigh_data(&__pallet_base_weight,
                            ());
                    let __pallet_class =
                        <dyn frame_support::dispatch::ClassifyDispatch<()>>::classify_dispatch(&__pallet_base_weight,
                            ());
                    let __pallet_pays_fee =
                        <dyn frame_support::dispatch::PaysFee<()>>::pays_fee(&__pallet_base_weight,
                            ());
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::cause_error {} => {
                    let __pallet_base_weight =
                        10_000 + T::DbWeight::get().reads_writes(1, 1);
                    let __pallet_weight =
                        <dyn frame_support::dispatch::WeighData<()>>::weigh_data(&__pallet_base_weight,
                            ());
                    let __pallet_class =
                        <dyn frame_support::dispatch::ClassifyDispatch<()>>::classify_dispatch(&__pallet_base_weight,
                            ());
                    let __pallet_pays_fee =
                        <dyn frame_support::dispatch::PaysFee<()>>::pays_fee(&__pallet_base_weight,
                            ());
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::__Ignore(_, _) =>
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["internal error: entered unreachable code: "],
                            &[::core::fmt::ArgumentV1::new_display(&::core::fmt::Arguments::new_v1(&["__Ignore cannot be used"],
                                                    &[]))])),
            }
        }
    }
    impl<T: Config> frame_support::dispatch::GetCallName for Call<T> {
        fn get_call_name(&self) -> &'static str {
            match *self {
                Self::set_value { .. } => "set_value",
                Self::check_value { .. } => "check_value",
                Self::cause_error { .. } => "cause_error",
                Self::__Ignore(_, _) =>
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["internal error: entered unreachable code: "],
                            &[::core::fmt::ArgumentV1::new_display(&::core::fmt::Arguments::new_v1(&["__PhantomItem cannot be used."],
                                                    &[]))])),
            }
        }
        fn get_call_names() -> &'static [&'static str] {
            &["set_value", "check_value", "cause_error"]
        }
    }
    impl<T: Config> frame_support::traits::UnfilteredDispatchable for Call<T>
        {
        type Origin = frame_system::pallet_prelude::OriginFor<T>;
        fn dispatch_bypass_filter(self, origin: Self::Origin)
            -> frame_support::dispatch::DispatchResultWithPostInfo {
            match self {
                Self::set_value { something } => {
                    let __within_span__ =
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE: ::tracing::callsite::DefaultCallsite =
                                {
                                    static META: ::tracing::Metadata<'static> =
                                        {
                                            ::tracing_core::metadata::Metadata::new("set_value",
                                                "pallet_trial::pallet", ::tracing::Level::TRACE,
                                                Some("pallets/trial/src/lib.rs"), Some(16u32),
                                                Some("pallet_trial::pallet"),
                                                ::tracing_core::field::FieldSet::new(&[],
                                                    ::tracing_core::callsite::Identifier(&CALLSITE)),
                                                ::tracing::metadata::Kind::SPAN)
                                        };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                            let mut interest = ::tracing::subscriber::Interest::never();
                            if ::tracing::Level::TRACE <=
                                                    ::tracing::level_filters::STATIC_MAX_LEVEL &&
                                                ::tracing::Level::TRACE <=
                                                    ::tracing::level_filters::LevelFilter::current() &&
                                            { interest = CALLSITE.interest(); !interest.is_never() } &&
                                        ::tracing::__macro_support::__is_enabled(CALLSITE.metadata(),
                                            interest) {
                                    let meta = CALLSITE.metadata();
                                    ::tracing::Span::new(meta,
                                        &{ meta.fields().value_set(&[]) })
                                } else {
                                   let span =
                                       ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                                   {};
                                   span
                               }
                        };
                    let __tracing_guard__ = __within_span__.enter();
                    frame_support::storage::in_storage_layer(||
                            {
                                <Pallet<T>>::set_value(origin,
                                            something).map(Into::into).map_err(Into::into)
                            })
                }
                Self::check_value {} => {
                    let __within_span__ =
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE: ::tracing::callsite::DefaultCallsite =
                                {
                                    static META: ::tracing::Metadata<'static> =
                                        {
                                            ::tracing_core::metadata::Metadata::new("check_value",
                                                "pallet_trial::pallet", ::tracing::Level::TRACE,
                                                Some("pallets/trial/src/lib.rs"), Some(16u32),
                                                Some("pallet_trial::pallet"),
                                                ::tracing_core::field::FieldSet::new(&[],
                                                    ::tracing_core::callsite::Identifier(&CALLSITE)),
                                                ::tracing::metadata::Kind::SPAN)
                                        };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                            let mut interest = ::tracing::subscriber::Interest::never();
                            if ::tracing::Level::TRACE <=
                                                    ::tracing::level_filters::STATIC_MAX_LEVEL &&
                                                ::tracing::Level::TRACE <=
                                                    ::tracing::level_filters::LevelFilter::current() &&
                                            { interest = CALLSITE.interest(); !interest.is_never() } &&
                                        ::tracing::__macro_support::__is_enabled(CALLSITE.metadata(),
                                            interest) {
                                    let meta = CALLSITE.metadata();
                                    ::tracing::Span::new(meta,
                                        &{ meta.fields().value_set(&[]) })
                                } else {
                                   let span =
                                       ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                                   {};
                                   span
                               }
                        };
                    let __tracing_guard__ = __within_span__.enter();
                    frame_support::storage::in_storage_layer(||
                            {
                                <Pallet<T>>::check_value(origin).map(Into::into).map_err(Into::into)
                            })
                }
                Self::cause_error {} => {
                    let __within_span__ =
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE: ::tracing::callsite::DefaultCallsite =
                                {
                                    static META: ::tracing::Metadata<'static> =
                                        {
                                            ::tracing_core::metadata::Metadata::new("cause_error",
                                                "pallet_trial::pallet", ::tracing::Level::TRACE,
                                                Some("pallets/trial/src/lib.rs"), Some(16u32),
                                                Some("pallet_trial::pallet"),
                                                ::tracing_core::field::FieldSet::new(&[],
                                                    ::tracing_core::callsite::Identifier(&CALLSITE)),
                                                ::tracing::metadata::Kind::SPAN)
                                        };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                            let mut interest = ::tracing::subscriber::Interest::never();
                            if ::tracing::Level::TRACE <=
                                                    ::tracing::level_filters::STATIC_MAX_LEVEL &&
                                                ::tracing::Level::TRACE <=
                                                    ::tracing::level_filters::LevelFilter::current() &&
                                            { interest = CALLSITE.interest(); !interest.is_never() } &&
                                        ::tracing::__macro_support::__is_enabled(CALLSITE.metadata(),
                                            interest) {
                                    let meta = CALLSITE.metadata();
                                    ::tracing::Span::new(meta,
                                        &{ meta.fields().value_set(&[]) })
                                } else {
                                   let span =
                                       ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                                   {};
                                   span
                               }
                        };
                    let __tracing_guard__ = __within_span__.enter();
                    frame_support::storage::in_storage_layer(||
                            {
                                <Pallet<T>>::cause_error(origin).map(Into::into).map_err(Into::into)
                            })
                }
                Self::__Ignore(_, _) => {
                    let _ = origin;
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["internal error: entered unreachable code: "],
                            &[::core::fmt::ArgumentV1::new_display(&::core::fmt::Arguments::new_v1(&["__PhantomItem cannot be used."],
                                                    &[]))]));
                }
            }
        }
    }
    impl<T: Config> frame_support::dispatch::Callable<T> for Pallet<T> {
        type Call = Call<T>;
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn call_functions()
            -> frame_support::metadata::PalletCallMetadata {
            frame_support::scale_info::meta_type::<Call<T>>().into()
        }
    }
    impl<T: Config> frame_support::sp_std::fmt::Debug for Error<T> {
        fn fmt(&self, f: &mut frame_support::sp_std::fmt::Formatter<'_>)
            -> frame_support::sp_std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl<T: Config> Error<T> {
        #[doc(hidden)]
        pub fn as_str(&self) -> &'static str {
            match &self {
                Self::__Ignore(_, _) =>
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["internal error: entered unreachable code: "],
                            &[::core::fmt::ArgumentV1::new_display(&::core::fmt::Arguments::new_v1(&["`__Ignore` can never be constructed"],
                                                    &[]))])),
                Self::NoneValue => "NoneValue",
                Self::StorageOverflow => "StorageOverflow",
            }
        }
    }
    impl<T: Config> From<Error<T>> for &'static str {
        fn from(err: Error<T>) -> &'static str { err.as_str() }
    }
    impl<T: Config> From<Error<T>> for
        frame_support::sp_runtime::DispatchError {
        fn from(err: Error<T>) -> Self {
            use frame_support::codec::Encode;
            let index =
                <<T as frame_system::Config>::PalletInfo as
                                frame_support::traits::PalletInfo>::index::<Pallet<T>>().expect("Every active module has an index in the runtime; qed")
                    as u8;
            let mut encoded = err.encode();
            encoded.resize(frame_support::MAX_MODULE_ERROR_ENCODED_SIZE, 0);
            frame_support::sp_runtime::DispatchError::Module(frame_support::sp_runtime::ModuleError {
                    index,
                    error: TryInto::try_into(encoded).expect("encoded error is resized to be equal to the maximum encoded error size; qed"),
                    message: Some(err.as_str()),
                })
        }
    }
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __tt_error_token_1 {
        { $caller : tt frame_support = [{ $($frame_support : ident) :: * }] }
        =>
        { $($frame_support ::) * tt_return! { $caller error = [{ Error }] } }
        ;
    }
    pub use __tt_error_token_1 as tt_error_token;
    #[doc(hidden)]
    pub mod __substrate_event_check {
        #[macro_export]
        #[doc(hidden)]
        macro_rules! __is_event_part_defined_2 {
            ($pallet_name : ident) => {} ;
        }
        #[doc(hidden)]
        pub use __is_event_part_defined_2 as is_event_part_defined;
    }
    impl<T: Config> Pallet<T> {
        pub(super) fn deposit_event(event: Event<T>) {
            let event = <<T as Config>::Event as From<Event<T>>>::from(event);
            let event =
                <<T as Config>::Event as
                        Into<<T as frame_system::Config>::Event>>::into(event);
            <frame_system::Pallet<T>>::deposit_event(event)
        }
    }
    impl<T: Config> From<Event<T>> for () {
        fn from(_: Event<T>) {}
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn storage_metadata()
            -> frame_support::metadata::PalletStorageMetadata {
            frame_support::metadata::PalletStorageMetadata {
                prefix: <<T as frame_system::Config>::PalletInfo as
                            frame_support::traits::PalletInfo>::name::<Pallet<T>>().expect("Every active pallet has a name in the runtime; qed"),
                entries: {
                    #[allow(unused_mut)]
                    let mut entries = ::alloc::vec::Vec::new();
                    {
                        <Something<T> as
                                frame_support::storage::StorageEntryMetadataBuilder>::build_metadata(<[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([" if not value query type, the default is option value, value could be some(u32) or none"])),
                            &mut entries);
                    }
                    {
                        <OptionValue<T> as
                                frame_support::storage::StorageEntryMetadataBuilder>::build_metadata(::alloc::vec::Vec::new(),
                            &mut entries);
                    }
                    {
                        <QueryValue<T> as
                                frame_support::storage::StorageEntryMetadataBuilder>::build_metadata(<[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([" query type storage"])),
                            &mut entries);
                    }
                    {
                        <QueryWithDefault<T> as
                                frame_support::storage::StorageEntryMetadataBuilder>::build_metadata(<[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([" query type with default value"])),
                            &mut entries);
                    }
                    {
                        <StrutureData<T> as
                                frame_support::storage::StorageEntryMetadataBuilder>::build_metadata(<[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([" if not value query type, the default is option value, value could be some(u32) or none"])),
                            &mut entries);
                    }
                    entries
                },
            }
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc =
        " if not value query type, the default is option value, value could be some(u32) or none"]
        pub fn something() -> Option<u32> {
            <Something<T> as frame_support::storage::StorageValue<u32>>::get()
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn option_value() -> Option<u32> {
            <OptionValue<T> as
                    frame_support::storage::StorageValue<u32>>::get()
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc = " query type storage"]
        pub fn query_value() -> u32 {
            <QueryValue<T> as
                    frame_support::storage::StorageValue<u32>>::get()
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc = " query type with default value"]
        pub fn query_with_default() -> u32 {
            <QueryWithDefault<T> as
                    frame_support::storage::StorageValue<u32>>::get()
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc =
        " if not value query type, the default is option value, value could be some(u32) or none"]
        pub fn data_structure() -> Option<DataStructure> {
            <StrutureData<T> as
                    frame_support::storage::StorageValue<DataStructure>>::get()
        }
    }
    #[doc(hidden)]
    pub struct _GeneratedPrefixForStorageSomething<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance for
        _GeneratedPrefixForStorageSomething<T> {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as
                        frame_support::traits::PalletInfo>::name::<Pallet<T>>().expect("Every active pallet has a name in the runtime; qed")
        }
        const STORAGE_PREFIX: &'static str = "Something";
    }
    #[doc(hidden)]
    pub struct _GeneratedPrefixForStorageOptionValue<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance for
        _GeneratedPrefixForStorageOptionValue<T> {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as
                        frame_support::traits::PalletInfo>::name::<Pallet<T>>().expect("Every active pallet has a name in the runtime; qed")
        }
        const STORAGE_PREFIX: &'static str = "OptionValue";
    }
    #[doc(hidden)]
    pub struct _GeneratedPrefixForStorageQueryValue<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance for
        _GeneratedPrefixForStorageQueryValue<T> {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as
                        frame_support::traits::PalletInfo>::name::<Pallet<T>>().expect("Every active pallet has a name in the runtime; qed")
        }
        const STORAGE_PREFIX: &'static str = "QueryValue";
    }
    #[doc(hidden)]
    pub struct _GeneratedPrefixForStorageQueryWithDefault<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance for
        _GeneratedPrefixForStorageQueryWithDefault<T> {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as
                        frame_support::traits::PalletInfo>::name::<Pallet<T>>().expect("Every active pallet has a name in the runtime; qed")
        }
        const STORAGE_PREFIX: &'static str = "QueryWithDefault";
    }
    #[doc(hidden)]
    pub struct _GeneratedPrefixForStorageStrutureData<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance for
        _GeneratedPrefixForStorageStrutureData<T> {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as
                        frame_support::traits::PalletInfo>::name::<Pallet<T>>().expect("Every active pallet has a name in the runtime; qed")
        }
        const STORAGE_PREFIX: &'static str = "StrutureData";
    }
    #[doc(hidden)]
    pub mod __substrate_inherent_check {
        #[macro_export]
        #[doc(hidden)]
        macro_rules! __is_inherent_part_defined_3 {
            ($pallet_name : ident) =>
            {
                compile_error!
                (concat!
                ("`", stringify! ($pallet_name),
                "` does not have #[pallet::inherent] defined, perhaps you should \
				remove `Inherent` from construct_runtime?",))
                ;
            }
        }
        #[doc(hidden)]
        pub use __is_inherent_part_defined_3 as is_inherent_part_defined;
    }
    #[doc =
    r" Hidden instance generated to be internally used when module is used without"]
    #[doc = r" instance."]
    #[doc(hidden)]
    pub type __InherentHiddenInstance = ();
    pub(super) trait Store {
        type Something;
        type OptionValue;
        type QueryValue;
        type QueryWithDefault;
        type StrutureData;
    }
    impl<T: Config> Store for Pallet<T> {
        type Something = Something<T>;
        type OptionValue = OptionValue<T>;
        type QueryValue = QueryValue<T>;
        type QueryWithDefault = QueryWithDefault<T>;
        type StrutureData = StrutureData<T>;
    }
    impl<T: Config>
        frame_support::traits::Hooks<<T as frame_system::Config>::BlockNumber>
        for Pallet<T> {}
    impl<T: Config>
        frame_support::traits::OnFinalize<<T as
        frame_system::Config>::BlockNumber> for Pallet<T> {
        fn on_finalize(n: <T as frame_system::Config>::BlockNumber) {
            let __within_span__ =
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite =
                        {
                            static META: ::tracing::Metadata<'static> =
                                {
                                    ::tracing_core::metadata::Metadata::new("on_finalize",
                                        "pallet_trial::pallet", ::tracing::Level::TRACE,
                                        Some("pallets/trial/src/lib.rs"), Some(16u32),
                                        Some("pallet_trial::pallet"),
                                        ::tracing_core::field::FieldSet::new(&[],
                                            ::tracing_core::callsite::Identifier(&CALLSITE)),
                                        ::tracing::metadata::Kind::SPAN)
                                };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                    let mut interest = ::tracing::subscriber::Interest::never();
                    if ::tracing::Level::TRACE <=
                                            ::tracing::level_filters::STATIC_MAX_LEVEL &&
                                        ::tracing::Level::TRACE <=
                                            ::tracing::level_filters::LevelFilter::current() &&
                                    { interest = CALLSITE.interest(); !interest.is_never() } &&
                                ::tracing::__macro_support::__is_enabled(CALLSITE.metadata(),
                                    interest) {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta,
                                &{ meta.fields().value_set(&[]) })
                        } else {
                           let span =
                               ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                           {};
                           span
                       }
                };
            let __tracing_guard__ = __within_span__.enter();
            <Self as
                    frame_support::traits::Hooks<<T as
                    frame_system::Config>::BlockNumber>>::on_finalize(n)
        }
    }
    impl<T: Config>
        frame_support::traits::OnIdle<<T as
        frame_system::Config>::BlockNumber> for Pallet<T> {
        fn on_idle(n: <T as frame_system::Config>::BlockNumber,
            remaining_weight: frame_support::weights::Weight)
            -> frame_support::weights::Weight {
            <Self as
                    frame_support::traits::Hooks<<T as
                    frame_system::Config>::BlockNumber>>::on_idle(n,
                remaining_weight)
        }
    }
    impl<T: Config>
        frame_support::traits::OnInitialize<<T as
        frame_system::Config>::BlockNumber> for Pallet<T> {
        fn on_initialize(n: <T as frame_system::Config>::BlockNumber)
            -> frame_support::weights::Weight {
            let __within_span__ =
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite =
                        {
                            static META: ::tracing::Metadata<'static> =
                                {
                                    ::tracing_core::metadata::Metadata::new("on_initialize",
                                        "pallet_trial::pallet", ::tracing::Level::TRACE,
                                        Some("pallets/trial/src/lib.rs"), Some(16u32),
                                        Some("pallet_trial::pallet"),
                                        ::tracing_core::field::FieldSet::new(&[],
                                            ::tracing_core::callsite::Identifier(&CALLSITE)),
                                        ::tracing::metadata::Kind::SPAN)
                                };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                    let mut interest = ::tracing::subscriber::Interest::never();
                    if ::tracing::Level::TRACE <=
                                            ::tracing::level_filters::STATIC_MAX_LEVEL &&
                                        ::tracing::Level::TRACE <=
                                            ::tracing::level_filters::LevelFilter::current() &&
                                    { interest = CALLSITE.interest(); !interest.is_never() } &&
                                ::tracing::__macro_support::__is_enabled(CALLSITE.metadata(),
                                    interest) {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta,
                                &{ meta.fields().value_set(&[]) })
                        } else {
                           let span =
                               ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                           {};
                           span
                       }
                };
            let __tracing_guard__ = __within_span__.enter();
            <Self as
                    frame_support::traits::Hooks<<T as
                    frame_system::Config>::BlockNumber>>::on_initialize(n)
        }
    }
    impl<T: Config> frame_support::traits::OnRuntimeUpgrade for Pallet<T> {
        fn on_runtime_upgrade() -> frame_support::weights::Weight {
            let __within_span__ =
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite =
                        {
                            static META: ::tracing::Metadata<'static> =
                                {
                                    ::tracing_core::metadata::Metadata::new("on_runtime_update",
                                        "pallet_trial::pallet", ::tracing::Level::TRACE,
                                        Some("pallets/trial/src/lib.rs"), Some(16u32),
                                        Some("pallet_trial::pallet"),
                                        ::tracing_core::field::FieldSet::new(&[],
                                            ::tracing_core::callsite::Identifier(&CALLSITE)),
                                        ::tracing::metadata::Kind::SPAN)
                                };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                    let mut interest = ::tracing::subscriber::Interest::never();
                    if ::tracing::Level::TRACE <=
                                            ::tracing::level_filters::STATIC_MAX_LEVEL &&
                                        ::tracing::Level::TRACE <=
                                            ::tracing::level_filters::LevelFilter::current() &&
                                    { interest = CALLSITE.interest(); !interest.is_never() } &&
                                ::tracing::__macro_support::__is_enabled(CALLSITE.metadata(),
                                    interest) {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta,
                                &{ meta.fields().value_set(&[]) })
                        } else {
                           let span =
                               ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                           {};
                           span
                       }
                };
            let __tracing_guard__ = __within_span__.enter();
            let pallet_name =
                <<T as frame_system::Config>::PalletInfo as
                            frame_support::traits::PalletInfo>::name::<Self>().unwrap_or("<unknown pallet name>");
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                        {
                        ::log::__private_api_log(::core::fmt::Arguments::new_v1(&["\u{2705} no migration for "],
                                &[::core::fmt::ArgumentV1::new_display(&pallet_name)]), lvl,
                            &(frame_support::LOG_TARGET, "pallet_trial::pallet",
                                    "pallets/trial/src/lib.rs", 16u32),
                            ::log::__private_api::Option::None);
                    }
            };
            <Self as
                    frame_support::traits::Hooks<<T as
                    frame_system::Config>::BlockNumber>>::on_runtime_upgrade()
        }
    }
    impl<T: Config>
        frame_support::traits::OffchainWorker<<T as
        frame_system::Config>::BlockNumber> for Pallet<T> {
        fn offchain_worker(n: <T as frame_system::Config>::BlockNumber) {
            <Self as
                    frame_support::traits::Hooks<<T as
                    frame_system::Config>::BlockNumber>>::offchain_worker(n)
        }
    }
    impl<T: Config> frame_support::traits::IntegrityTest for Pallet<T> {
        fn integrity_test() {
            <Self as
                    frame_support::traits::Hooks<<T as
                    frame_system::Config>::BlockNumber>>::integrity_test()
        }
    }
    #[doc(hidden)]
    pub mod __substrate_genesis_config_check {
        #[macro_export]
        #[doc(hidden)]
        macro_rules! __is_genesis_config_defined_4 {
            ($pallet_name : ident) =>
            {
                compile_error!
                (concat!
                ("`", stringify! ($pallet_name),
                "` does not have #[pallet::genesis_config] defined, perhaps you should \
								remove `Config` from construct_runtime?",))
                ;
            }
        }
        #[macro_export]
        #[doc(hidden)]
        macro_rules! __is_std_enabled_for_genesis_4 {
            ($pallet_name : ident, $pallet_path : expr) => {} ;
        }
        #[doc(hidden)]
        pub use __is_genesis_config_defined_4 as is_genesis_config_defined;
        #[doc(hidden)]
        pub use __is_std_enabled_for_genesis_4 as is_std_enabled_for_genesis;
    }
    #[doc =
    " define a method to return a value, it could be used in a storage unit"]
    pub struct GetDefaultValue(core::marker::PhantomData<((),)>);
    impl frame_support::traits::Get<u32> for GetDefaultValue<> {
        fn get() -> u32 { __type_value_for_get_default_value::<>() }
    }
    #[doc(hidden)]
    pub mod __substrate_origin_check {
        #[macro_export]
        #[doc(hidden)]
        macro_rules! __is_origin_part_defined_5 {
            ($pallet_name : ident) =>
            {
                compile_error!
                (concat!
                ("`", stringify! ($pallet_name),
                "` does not have #[pallet::origin] defined, perhaps you should \
				remove `Origin` from construct_runtime?",))
                ;
            }
        }
        #[doc(hidden)]
        pub use __is_origin_part_defined_5 as is_origin_part_defined;
    }
    #[doc(hidden)]
    pub mod __substrate_validate_unsigned_check {
        #[macro_export]
        #[doc(hidden)]
        macro_rules! __is_validate_unsigned_part_defined_6 {
            ($pallet_name : ident) =>
            {
                compile_error!
                (concat!
                ("`", stringify! ($pallet_name),
                "` does not have #[pallet::validate_unsigned] defined, perhaps you should \
				remove `ValidateUnsigned` from construct_runtime?",))
                ;
            }
        }
        #[doc(hidden)]
        pub use __is_validate_unsigned_part_defined_6 as is_validate_unsigned_part_defined;
    }
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __tt_default_parts_7 {
        { $caller : tt frame_support = [{ $($frame_support : ident) :: * }] }
        =>
        {
            $($frame_support) * :: tt_return!
            {
                $caller tokens =
                [{ :: { Pallet, Call, Storage, Event < T >, } }]
            }
        } ;
    }
    pub use __tt_default_parts_7 as tt_default_parts;
}
