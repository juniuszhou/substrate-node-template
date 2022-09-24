use crate::{mock::*, Error};
use crate as pallet_template;
use frame_support::{assert_noop, assert_ok};
use frame_support::traits::{ConstU16, ConstU32, ConstU64};
// use frame_support::pallet_prelude::*;
// 	use frame_system::pallet_prelude::*;
use frame_support::storage::bounded_vec::BoundedVec;
use sp_runtime::traits::{BadOrigin, Get};

// use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		let club_id = 1;
		let account_id = 2;

		let mut expected: BoundedVec::<u32, <Test as pallet_template::Config>::MaxClub> = Default::default();
		expected.force_push(1);

		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::add_member(Origin::root(), club_id, account_id));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::members(account_id), expected);
		assert_ok!(TemplateModule::remove_member(Origin::root(), club_id, account_id));
		expected.pop();
		assert_eq!(TemplateModule::members(account_id), expected);

		// assert_eq!(System::events().len(), 12);

		assert_eq!(
			events(),
			[
				Event::TemplateModule(crate::Event::ClubMemberAdded{club: club_id, account: account_id}),
			]
		);
		// System::assert_has_event(Event::Balances(crate::Event::DustLost {
		// 	account: 2,
		// 	amount: 50,
		// }));
		// Event::Balances(crate::Event::Endowed { account: 1, free_balance: 100 }),

	});
}

#[test]
fn not_root() {
	new_test_ext().execute_with(|| {
		let club_id = 1;
		let account_id = 2;

		assert_noop!(TemplateModule::add_member(Origin::signed(account_id), club_id, account_id), BadOrigin);
		// Read pallet storage and assert an expected result.
		// assert_noop!(TemplateModule::members(account_id), expected);
		assert_noop!(TemplateModule::remove_member(Origin::signed(account_id), club_id, account_id), BadOrigin);



	});
}

#[test]
fn invalid_club_id() {
	new_test_ext().execute_with(|| {
		let club_id = <Test as pallet_template::Config>::MaxClub::get();
		let account_id = 2;

		assert_noop!(TemplateModule::add_member(Origin::root(), club_id, account_id), Error::<Test>::InvalidClubId);
		// Read pallet storage and assert an expected result.
		// assert_noop!(TemplateModule::members(account_id), expected);
		assert_noop!(TemplateModule::remove_member(Origin::root(), club_id, account_id), Error::<Test>::InvalidClubId);



	});
}

#[test]
fn exists_error() {
	new_test_ext().execute_with(|| {
		let club_id = 1;
		let account_id = 2;
		assert_ok!(TemplateModule::add_member(Origin::root(), club_id, account_id));

		assert_noop!(TemplateModule::add_member(Origin::root(), club_id, account_id), Error::<Test>::AlreadyClubMember);
		// Read pallet storage and assert an expected result.
		// assert_noop!(TemplateModule::members(account_id), expected);

		assert_ok!(TemplateModule::remove_member(Origin::root(), club_id, account_id));
		assert_noop!(TemplateModule::remove_member(Origin::root(), club_id, account_id), Error::<Test>::NotClubMember);



	});
}

// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
// 	});
// }
