use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_create() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let name = *b"abcd";

		assert_eq!(crate::Pallet::<Test>::next_kitty_id(), kitty_id);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), name));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);

		assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

		// homework
		let kitty = KittiesModule::kitties(kitty_id).unwrap();
		System::assert_last_event(Event::KittyCreated { who: account_id, kitty_id, kitty }.into());

		crate::pallet::NextKittyId::<Test>::set(crate::KittyId::max_value());
		assert_noop!(
			KittiesModule::create(RuntimeOrigin::signed(account_id), name),
			Error::<Test>::InvalidKittyId
		);
	});
}

#[test]
fn it_works_for_breed() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let name = *b"abcd";

		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1, name),
			Error::<Test>::InvalidKittyId
		);
		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id, name),
			Error::<Test>::SameKittyId
		);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), name));

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), name));
		assert_eq!(KittiesModule::next_kitty_id(), 2);

		assert_ok!(KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1, name));

		let breed_kitty_id = 2;

		// homework
		let kitty = KittiesModule::kitties(breed_kitty_id).unwrap();
		System::assert_last_event(
			Event::KittyBred { who: account_id, kitty_id: breed_kitty_id, kitty }.into(),
		);

		assert_eq!(KittiesModule::next_kitty_id(), breed_kitty_id + 1);
		assert_eq!(KittiesModule::kitties(breed_kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(breed_kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(breed_kitty_id), Some((kitty_id, kitty_id + 1)));
	});
}

#[test]
fn it_works_for_transfer() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let recipient = 2;
		let name = *b"abcd";

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), name));

		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

		assert_noop!(
			KittiesModule::transfer(RuntimeOrigin::signed(recipient), kitty_id, recipient),
			Error::<Test>::NotOwner
		);

		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id), kitty_id, recipient));

		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(recipient));

		// homework
		System::assert_last_event(
			Event::KittyTransferred { who: account_id, recipient, kitty_id }.into(),
		);
	});
}
