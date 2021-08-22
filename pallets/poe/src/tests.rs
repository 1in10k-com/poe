use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;
//ddd,如不加入此行,运行测试会报错.找不到Proofs存证单元.所以需要引入模块里的内容.因为测试用例是在子模块里,所以需要引入父级模块. zzzz 1030. 之后测试会显示2passed.

#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        //bbb,调用create claim 传入信息交易的发送方是1这个账户.系统模块里配置的accountid是u64,所以可以用1来表示. 调用结果应该是返回ok.
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((1, frame_system::Pallet::<Test>::block_number()))
        );
        //ccc,对链上状态断言.claim对应的存储信息应该是两个元素组成的tuple.第一个元素是accountid,交易发送方1.第二个元素是区块数返回的值.
    })
}
//aaa,测试create_claim可调用函数 zzzz 0807
//使用test标签表示它是测试用例,new_test_ext()是之前说到的测试帮助函数,用来构建测试环境.

#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    })
}
//aaa,存证如已经存在，再创建一个相同存证将抛出错误信息。先create一个,然后再次create相同的,希望它报错.
//assert_noop是frame support提供的断言方法.它表示操作结果是一个错误的信息,并且对链上状态不做修改.

#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), None)
    })
}
//bbb,1300 创建存证后,对存证进行撤销.应该返回ok并且结果被撤销.

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    })
}
//ccc,撤销存证时的错误场景.
