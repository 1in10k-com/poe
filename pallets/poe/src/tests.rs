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
            Some((1, frame_system::Pallet::<Test>::block_number())
        ));
        //ccc,对链上状态断言.claim对应的存储信息应该是两个元素组成的tuple.第一个元素是accountid,交易发送方1.第二个元素是区块数返回的值.
    })
}
//aaa,测试create_claim可调用函数 zzzz 0807
//使用test标签表示它是测试用例,new_test_ext()是之前说到的测试帮助函数,用来构建测试环境.