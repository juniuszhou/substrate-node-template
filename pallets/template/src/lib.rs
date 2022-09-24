#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use codec::{Codec, Decode, Encode, MaxEncodedLen};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_runtime::{
		traits::{AtLeast32BitUnsigned, CheckedAdd, MaybeSerializeDeserialize, One, Zero},
		RuntimeDebug,
	};
	type ClubId = u32;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		// type ClubId: Parameter
		// 	+ Member
		// 	+ Codec
		// 	+ Default
		// 	+ Copy
		// 	+ MaybeSerializeDeserialize
		// 	+ MaxEncodedLen
		// 	+ TypeInfo;

		#[pallet::constant]
		type MaxClub: Get<u32>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn members)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Members<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<ClubId, T::MaxClub>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		ClubMemberAdded{club: ClubId, account: T::AccountId},
		ClubMemberRemoved(ClubId, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		InvalidClubId,
		/// Errors should have helpful documentation associated with them.
		AlreadyClubMember,
		NotClubMember,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Update storage.
			// <Something<T>>::put(something);

			// Emit an event.
			// Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_member(
			origin: OriginFor<T>,
			club: ClubId,
			account: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(club < T::MaxClub::get(), Error::<T>::InvalidClubId);
			ensure!(!Self::members(&account).contains(&club), Error::<T>::AlreadyClubMember);

			Members::<T>::mutate(&account, |ref mut clubs| {
				clubs.force_push(club);
			});

			// Self::deposit_event(Event::BalanceSet { who, free: new_free, reserved: new_reserved });

			// Emit an event.
			Self::deposit_event(Event::ClubMemberAdded{club, account});
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn remove_member(
			origin: OriginFor<T>,
			club: ClubId,
			account: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(club < T::MaxClub::get(), Error::<T>::InvalidClubId);
			ensure!(Self::members(&account).contains(&club), Error::<T>::NotClubMember);

			Members::<T>::mutate(&account, |ref mut clubs| {
				let index = clubs.iter().position(|&r| r == club).unwrap();
				clubs.remove(index);
			});

			// Emit an event.
			Self::deposit_event(Event::ClubMemberRemoved(club, account));
			Ok(())
		}
	}
}
