//! Unit tests for the non-fungible-token module.

#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::*;
use sp_std::default::Default;

#[test]
fn hello_world_is_ok() {
    ExtBuilder::default().build().execute_with(|| {
        assert_eq!(1,1);
    });
}


#[test]
fn force_set_admin_ok() {
    ExtBuilder::default().new_test_ext(1).execute_with(|| {
        //System::set_block_number(10);
        let user_1: <Runtime as frame_system::Config>::AccountId =1 as u64;
        let caller =Origin::root();
        assert_ok!(Farm::force_set_admin(caller, user_1));

        assert_eq!(Admin::<Runtime>::get(), vec![1 as u64]);
    });
}

#[test]
fn force_set_admin_already_ok() {
    ExtBuilder::default().new_test_ext(1).execute_with(|| {
        let root =1;
        let user_b: <Runtime as frame_system::Config>::AccountId =2 as u64;
        let caller =Origin::root();
        Farm::force_set_admin(caller.clone(), user_b);
        assert_noop!(Farm::force_set_admin(caller.clone(), user_b), Error::<Runtime>::AdminHadExist);
    });
}


#[test]
fn revert_admin_ok() {
    ExtBuilder::default().new_test_ext(1).execute_with(|| {
        let user_b: <Runtime as frame_system::Config>::AccountId =2 as u64;
        let caller =Origin::root();
        assert_ok!(Farm::force_set_admin(caller.clone(), user_b));
        assert_ok!(Farm::revert_admin(caller.clone(), user_b));
        let empty:Vec<u64> =vec![];
        assert_eq!(Admin::<Runtime>::get(), empty);
    });
}

#[test]
fn revert_not_exist_admin_failed_ok() {
    ExtBuilder::default().new_test_ext(1).execute_with(|| {
        let user_b: <Runtime as frame_system::Config>::AccountId =2 as u64;
        let caller =Origin::root();
        assert_noop!(Farm::revert_admin(caller.clone(), user_b), <Error<Runtime>>::AdminNoExist);
    });
}


fn set_1_as_admin() {
        let user_1: <Runtime as frame_system::Config>::AccountId =1 as u64;
        Farm::force_set_admin(Origin::root(), user_1);
}

#[test]
fn move_in_ok() {
    ExtBuilder::default().new_test_ext(1).execute_with(|| {
        System::set_block_number(2);
        set_1_as_admin();
        let user_admin = 1 as u64;
        let user_b: <Runtime as frame_system::Config>::AccountId =2 as u64;
        let caller =Origin::signed(user_admin);
        Farm::move_in(caller, 0 as u32, user_b);
        let red = Penguins::<Runtime>::get(0 as u32, 0);
        assert_eq!(red, Some(PenguinFarmOf::<Runtime>::RedPenguin(RedPenguin{
            owner: user_b,
            start: 2u32.into(),
            pre_eat_at: 2u32.into(),
            eat_count: 1,
            status: PenguinStatus::Active,
            eggs: Default::default(),
            asset_id: 0 as u64,
            class_id: 0 as u32,
            incubation_remain: 0 as u128
        })))
    });
}






/*

#[test]
fn create_class_should_work() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(NonFungibleTokenModule::create_class(&ALICE, vec![1], ()));
    });
}

#[test]
fn create_class_should_fail() {
    ExtBuilder::default().build().execute_with(|| {
        NextClassId::<Runtime>::mutate(|id| *id = <Runtime as Config>::ClassId::max_value());
        assert_noop!(
            NonFungibleTokenModule::create_class(&ALICE, vec![1], ()),
            Error::<Runtime>::NoAvailableClassId
        );
    });
}

#[test]
fn mint_should_work() {
    ExtBuilder::default().build().execute_with(|| {
        let next_class_id = NonFungibleTokenModule::next_class_id();
        assert_eq!(next_class_id, CLASS_ID);
        assert_ok!(NonFungibleTokenModule::create_class(&ALICE, vec![1], ()));
        assert_eq!(NonFungibleTokenModule::next_token_id(CLASS_ID), 0);
        assert_ok!(NonFungibleTokenModule::mint(&BOB, CLASS_ID, vec![1], ()));
        assert_eq!(NonFungibleTokenModule::next_token_id(CLASS_ID), 1);
        assert_ok!(NonFungibleTokenModule::mint(&BOB, CLASS_ID, vec![1], ()));
        assert_eq!(NonFungibleTokenModule::next_token_id(CLASS_ID), 2);

        let next_class_id = NonFungibleTokenModule::next_class_id();
        assert_ok!(NonFungibleTokenModule::create_class(&ALICE, vec![1], ()));
        assert_eq!(NonFungibleTokenModule::next_token_id(next_class_id), 0);
        assert_ok!(NonFungibleTokenModule::mint(
            &BOB,
            next_class_id,
            vec![1],
            ()
        ));
        assert_eq!(NonFungibleTokenModule::next_token_id(next_class_id), 1);

        assert_eq!(NonFungibleTokenModule::next_token_id(CLASS_ID), 2);
    });
}

#[test]
fn mint_should_fail() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(NonFungibleTokenModule::create_class(&ALICE, vec![1], ()));
        Classes::<Runtime>::mutate(CLASS_ID, |class_info| {
            class_info.as_mut().unwrap().total_issuance = <Runtime as Config>::TokenId::max_value();
        });
        assert_noop!(
            NonFungibleTokenModule::mint(&BOB, CLASS_ID, vec![1], ()),
            ArithmeticError::Overflow,
        );

        NextTokenId::<Runtime>::mutate(CLASS_ID, |id| {
            *id = <Runtime as Config>::TokenId::max_value()
        });
        assert_noop!(
            NonFungibleTokenModule::mint(&BOB, CLASS_ID, vec![1], ()),
            Error::<Runtime>::NoAvailableTokenId
        );
    });
}

#[test]
fn transfer_should_work() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(NonFungibleTokenModule::create_class(&ALICE, vec![1], ()));
        assert_ok!(NonFungibleTokenModule::mint(&BOB, CLASS_ID, vec![1], ()));
        assert_ok!(NonFungibleTokenModule::transfer(
            &BOB,
            &BOB,
            (CLASS_ID, TOKEN_ID)
        ));
        assert_ok!(NonFungibleTokenModule::transfer(
            &BOB,
            &ALICE,
            (CLASS_ID, TOKEN_ID)
        ));
        assert_ok!(NonFungibleTokenModule::transfer(
            &ALICE,
            &BOB,
            (CLASS_ID, TOKEN_ID)
        ));
        assert!(NonFungibleTokenModule::is_owner(&BOB, (CLASS_ID, TOKEN_ID)));
    });
}

#[test]
fn transfer_should_fail() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(NonFungibleTokenModule::create_class(&ALICE, vec![1], ()));
        assert_ok!(NonFungibleTokenModule::mint(&BOB, CLASS_ID, vec![1], ()));
        assert_noop!(
            NonFungibleTokenModule::transfer(&BOB, &ALICE, (CLASS_ID, TOKEN_ID_NOT_EXIST)),
            Error::<Runtime>::TokenNotFound
        );
        assert_noop!(
            NonFungibleTokenModule::transfer(&ALICE, &BOB, (CLASS_ID, TOKEN_ID)),
            Error::<Runtime>::NoPermission
        );
        assert_noop!(
            NonFungibleTokenModule::mint(&BOB, CLASS_ID_NOT_EXIST, vec![1], ()),
            Error::<Runtime>::ClassNotFound
        );
        assert_noop!(
            NonFungibleTokenModule::transfer(&ALICE, &ALICE, (CLASS_ID, TOKEN_ID)),
            Error::<Runtime>::NoPermission
        );
    });
}

#[test]
fn burn_should_work() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(NonFungibleTokenModule::create_class(&ALICE, vec![1], ()));
        assert_ok!(NonFungibleTokenModule::mint(&BOB, CLASS_ID, vec![1], ()));
        assert_ok!(NonFungibleTokenModule::burn(&BOB, (CLASS_ID, TOKEN_ID)));
    });
}

#[test]
fn burn_should_fail() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(NonFungibleTokenModule::create_class(&ALICE, vec![1], ()));
        assert_ok!(NonFungibleTokenModule::mint(&BOB, CLASS_ID, vec![1], ()));
        assert_noop!(
            NonFungibleTokenModule::burn(&BOB, (CLASS_ID, TOKEN_ID_NOT_EXIST)),
            Error::<Runtime>::TokenNotFound
        );

        assert_noop!(
            NonFungibleTokenModule::burn(&ALICE, (CLASS_ID, TOKEN_ID)),
            Error::<Runtime>::NoPermission
        );
    });

    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(NonFungibleTokenModule::create_class(&ALICE, vec![1], ()));
        assert_ok!(NonFungibleTokenModule::mint(&BOB, CLASS_ID, vec![1], ()));

        Classes::<Runtime>::mutate(CLASS_ID, |class_info| {
            class_info.as_mut().unwrap().total_issuance = 0;
        });
        assert_noop!(
            NonFungibleTokenModule::burn(&BOB, (CLASS_ID, TOKEN_ID)),
            ArithmeticError::Overflow,
        );
    });
}

#[test]
fn destroy_class_should_work() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(NonFungibleTokenModule::create_class(&ALICE, vec![1], ()));
        assert_ok!(NonFungibleTokenModule::mint(&BOB, CLASS_ID, vec![1], ()));
        assert_ok!(NonFungibleTokenModule::burn(&BOB, (CLASS_ID, TOKEN_ID)));
        assert_ok!(NonFungibleTokenModule::destroy_class(&ALICE, CLASS_ID));
        assert_eq!(Classes::<Runtime>::contains_key(CLASS_ID), false);
        assert_eq!(NextTokenId::<Runtime>::contains_key(CLASS_ID), false);
    });
}

#[test]
fn destroy_class_should_fail() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(NonFungibleTokenModule::create_class(&ALICE, vec![1], ()));
        assert_ok!(NonFungibleTokenModule::mint(&BOB, CLASS_ID, vec![1], ()));
        assert_noop!(
            NonFungibleTokenModule::destroy_class(&ALICE, CLASS_ID_NOT_EXIST),
            Error::<Runtime>::ClassNotFound
        );

        assert_noop!(
            NonFungibleTokenModule::destroy_class(&BOB, CLASS_ID),
            Error::<Runtime>::NoPermission
        );

        assert_noop!(
            NonFungibleTokenModule::destroy_class(&ALICE, CLASS_ID),
            Error::<Runtime>::CannotDestroyClass
        );

        assert_ok!(NonFungibleTokenModule::burn(&BOB, (CLASS_ID, TOKEN_ID)));
        assert_ok!(NonFungibleTokenModule::destroy_class(&ALICE, CLASS_ID));
        assert_eq!(Classes::<Runtime>::contains_key(CLASS_ID), false);
    });
}*/
