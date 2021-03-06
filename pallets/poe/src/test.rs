use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

#[test]
fn create_claim_works() {
    new_test_ext().execute_with( ||{
        let claim = vec![0,1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone()));
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((1,frame_system::Pallet::<Test>::block_number()))
        );
    });
}

#[test]
fn create_claim_failed_when_exist(){
    new_test_ext().execute_with(||{
        let claim = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    });
}

#[test]
fn revoke_claim_works(){
    new_test_ext().execute_with(||{
        let claim = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
        assert_eq!(
            Proofs::<Test>::get(&claim),
            None
        );
    });
}

#[test]
fn revoke_claim_failed_when_not_exist(){
    new_test_ext().execute_with(||{
        let claim = vec![0,1];
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    });
}

#[test]
fn vec_len_test(){
    new_test_ext().execute_with(||{
        let claim = vec![0,1,2,3,4,5,6,7,8,9,10,11,12];
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimTooLong
        );
    });
}