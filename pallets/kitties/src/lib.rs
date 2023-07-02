#![cfg_attr(not(feature = "std"), no_std)]
mod migrations;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
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

	#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
	// pub struct Kitty(pub [u8; 16]);

	pub struct Kitty {
		pub dna: [u8; 16],
	 	pub name: [u8; 4],
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
		// type Treasure: Get<Self::AccountId>;
		type PalletId: Get<PalletId>;

		#[pallet::constant]
		type KittyPrice: Get<BalanceOf<Self>>;
	}

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	pub type NextKittyId<T> = StorageValue<_, KittyId, ValueQuery>;
	// pub type NextKittyId<T> = StorageValue<_, KittyId, ValueQuery, GetDefaultValue>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T> = StorageMap<_, Blake2_128Concat, KittyId, Kitty>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_on_sale)]
	pub type KittyOnSale<T> = StorageMap<_, Blake2_128Concat, KittyId, ()>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_parents)]
	pub type KittyParents<T> = StorageMap<_, Blake2_128Concat, KittyId, (KittyId, KittyId)>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, T::AccountId>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreated { who: T::AccountId, kitty_id: KittyId, kitty: Kitty },
		KittyBred { who: T::AccountId, kitty_id: KittyId, kitty: Kitty },
		KittyTransferred { who: T::AccountId, recipient: T::AccountId, kitty_id: KittyId },
		KittyOnSale { who: T::AccountId, kitty_id: KittyId },
		KittyBought { who: T::AccountId, kitty_id: KittyId },
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidKittyId,
		NotOwner,
		SameKittyId,
		NoOwner,
		AlreadyOnSale,
		AlreadyOwned,
		NotOnSale,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_runtime_upgrade() -> frame_support::weights::Weight {
			// log::error!("++++++++++++++++   on_runtime_upgrade    +++++++++++++");
			migrations::v1::migrate::<T>()
			// Weight::from_parts(1000000, 0)
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn create(origin: OriginFor<T>, name: [u8;4]) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let kitty_id = Self::get_next_id()?;

			let dna = Self::random_value(&who);
			// let kitty = Kitty(dna);
			let kitty = Kitty {
				dna,
				name,
			};

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

			// Emit an event.
			Self::deposit_event(Event::KittyCreated { who, kitty_id, kitty });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn breed(
			origin: OriginFor<T>,
			kitty_id_1: KittyId,
			kitty_id_2: KittyId,
			name: [u8; 4]
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// check kitty id
			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameKittyId);
			let kitty_1 = Self::kitties(kitty_id_1)
				.ok_or::<DispatchError>(Error::<T>::InvalidKittyId.into())?;

			let kitty_2 = Self::kitties(kitty_id_2)
				.ok_or::<DispatchError>(Error::<T>::InvalidKittyId.into())?;

			// get next id
			let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;

			let price = T::KittyPrice::get();

			T::Currency::transfer(
				&who,
				&Self::get_account_id(),
				price,
				frame_support::traits::ExistenceRequirement::KeepAlive,
			)?;
			T::Currency::reserve(&who, price)?;

			// selector for breeding
			let selector = Self::random_value(&who);

			let dna = [0u8; 16];
			let kitty = Kitty {dna, name};

			// for i in 0..kitty_1.0.len() {
			// 	// 0 choose kitty2, and 1 choose kitty1
			// 	data[i] = (kitty_1.0[i] & selector[i]) | (kitty_2.0[i] & !selector[i]);
			// }
			// let kitty = Kitty(data);

			<Kitties<T>>::insert(kitty_id, &kitty);
			KittyOwner::<T>::insert(kitty_id, &who);
			KittyParents::<T>::insert(kitty_id, (kitty_id_1, kitty_id_2));
			NextKittyId::<T>::set(kitty_id + 1);

			Self::deposit_event(Event::KittyBred { who, kitty_id, kitty });

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]
		pub fn transfer(
			origin: OriginFor<T>,
			kitty_id: u32,
			recipient: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::kitties(kitty_id).ok_or::<DispatchError>(Error::<T>::InvalidKittyId.into())?;

			ensure!(Self::kitty_owner(kitty_id) == Some(who.clone()), Error::<T>::NotOwner);

			Self::deposit_event(Event::KittyTransferred { who, recipient, kitty_id });

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(10_000)]
		pub fn sale(origin: OriginFor<T>, kitty_id: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::kitties(kitty_id).ok_or::<DispatchError>(Error::<T>::InvalidKittyId.into())?;

			ensure!(Self::kitty_owner(kitty_id) == Some(who.clone()), Error::<T>::NotOwner);

			ensure!(Self::kitty_on_sale(kitty_id).is_some(), Error::<T>::AlreadyOnSale);

			// emit!(KittyOnSale { who, kitty_id });

			<KittyOnSale<T>>::insert(kitty_id, ());

			Self::deposit_event(Event::KittyOnSale { who, kitty_id });

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(10_000)]
		pub fn buy(origin: OriginFor<T>, kitty_id: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// check if id exists
			Self::kitties(kitty_id).ok_or::<DispatchError>(Error::<T>::InvalidKittyId.into())?;

			// check if the kitty is on sale
			ensure!(Self::kitty_on_sale(kitty_id) == Some(()), Error::<T>::NotOnSale);

			let owner =
				Self::kitty_owner(kitty_id).ok_or::<DispatchError>(Error::<T>::NoOwner.into())?;

			// check if the kitty is not owned by the buyer
			ensure!(owner != who.clone(), Error::<T>::AlreadyOwned);

			// match <KittyOwner<T>>::get(kitty_id) {
			// 	Some(owner) => {
			let price = T::KittyPrice::get();

			T::Currency::transfer(
				&who,
				&Self::get_account_id(),
				price,
				frame_support::traits::ExistenceRequirement::KeepAlive,
			)?;

			// reserve token from sender
			T::Currency::reserve(&who, price)?;

			// unreserve token from owner
			T::Currency::unreserve(&owner, price);

			// update sale status
			<KittyOnSale<T>>::remove(kitty_id);

			<KittyOwner<T>>::insert(kitty_id, &who);

			Self::deposit_event(Event::KittyBought { who, kitty_id });


			Ok(())
			// },
			// None => Error::<T>::NotOwner,
			// 	None => Ok(()),
			// }

			// result
		}
	}

	impl<T: Config> Pallet<T> {
		// get a random 256.
		fn random_value(sender: &T::AccountId) -> [u8; 16] {
			let payload = (
				T::Randomness::random_seed(),
				&sender,
				// index in current block
				<frame_system::Pallet<T>>::extrinsic_index(),
			);

			payload.using_encoded(blake2_128)
		}

		// get netx id
		fn get_next_id() -> Result<KittyId, DispatchError> {
			match Self::next_kitty_id() {
				KittyId::MAX => Err(Error::<T>::InvalidKittyId.into()),
				val => Ok(val),
			}
		}

		// get kitty via id
		// fn get_kitty(kitty_id: KittyId) -> Result<Kitty, DispatchError> {
		// 	match Self::kitties(kitty_id) {
		// 		Some(kitty) => Ok(kitty),
		// 		None => Err(Error::<T>::InvalidKittyId.into()),
		// 	}
		// }

		fn get_account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}

		// // get kitty via id
		// fn kitty(kitty_id: KittyId) -> Result<Kitty, DispatchError> {
		// 	match Self::kitties(kitty_id) {
		// 		Some(kitty) => Ok(kitty),
		// 		None => Err(Error::<T>::InvalidKittyId.into()),
		// 	}
		// }
	}
}
