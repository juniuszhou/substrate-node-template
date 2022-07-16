#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;



#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::storage_alias;

	/// define data structure used in the pallet storage
	#[derive(Encode, Decode, MaxEncodedLen, Clone, PartialEq, Eq, Default, TypeInfo)]
	pub struct DataStructure {
			pub id: u32,
	}

	/// define a method to return a value, it could be used in a storage unit
	#[pallet::type_value]
	pub fn GetDefaultValue() -> u32 {
		1234u32
	}

	/// the usage of storage alias
	#[storage_alias]
	type CounterForValidators<T: Config> = StorageValue<Pallet<T>, u32>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// if not value query type, the default is option value, value could be some(u32) or none
	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn option_value)]
	pub type OptionValue<T> = StorageValue<_, u32, OptionQuery>;

	/// query type storage
	#[pallet::storage]
	#[pallet::getter(fn query_value)]
	pub type QueryValue<T> = StorageValue<_, u32, ValueQuery>;

	/// query type with default value
	#[pallet::storage]
	#[pallet::getter(fn query_with_default)]
	pub type QueryWithDefault<T> = StorageValue<_, u32, ValueQuery, GetDefaultValue>;

	/// if not value query type, the default is option value, value could be some(u32) or none
	#[pallet::storage]
	#[pallet::getter(fn data_structure)]
	pub type StrutureData<T> = StorageValue<_, DataStructure>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn set_value(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			<QueryValue<T>>::put(something);

			<OptionValue<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn check_value(origin: OriginFor<T>) -> DispatchResult {

			ensure_signed(origin)?;

			match <Something<T>>::get() {
				None => {},
				Some(_) => {},
			};

			match <QueryValue<T>>::get() {
				0_u32 => {},
				_ => {},
			};

			match <OptionValue<T>>::get() {
				None => {},
				Some(_) => {},
			};

			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn internal_fn() {

		}
	}
}
