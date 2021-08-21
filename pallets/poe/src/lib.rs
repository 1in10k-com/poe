#![cfg_attr(not(feature = "std"), no_std)]

/// A module for proof of existence
pub use pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)] //使用generate store表示当前模块依赖一些存储单元，还有Store这个trait定义。 ssss

    pub struct Pallet<T>(_); //pallet 结构体承担功能模块

    #[pallet::storage]
    #[pallet::getter(fn proofs)] //用getter宏定义可选get函数proofs

    pub type Proofs<T: Config> =
        StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber)>; //存储单元Proofs用来存储存证，类型为StorageMap，key是Vec u8，表示存证的hash值。key 对应的value是(T::AccountId, T::BlockNumber) tuple。其两个元素前者表示用户id，后者表示存入时的区块，这两个类型都来自于系统模块。

    #[pallet::event] //使用这个宏定义event的枚举类型
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)] //使用此宏生成一个帮助性方法deposit event，可以很方便的进行event触发

    pub enum Event<T: Config> {
        ClaimCreated(T::AccountId, Vec<u8>), //r1
        ClaimRevoked(T::AccountId, Vec<u8>),
    }

    #[pallet::error]
    pub enum Error<T> {
        ProofAlreadyExist,
        ClaimNotExist,
        NotClaimOwner,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {} //这里是空的，表示当前功能模块没有特殊的功能函数，on initial，finalized等。

    #[pallet::call]
    //以下，在pallet这个结构体的实现里面可调用函数的构建。
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn create_claim(
            origin: OriginFor<T>, //交易发送方
            claim: Vec<u8>,       //存证哈希值
        ) -> DispatchResultWithPostInfo //定义创建存证的可调用函数，result类型别名 zzzz
        {
            let sender = ensure_signed(origin)?; //校验当前发送方是否签名，完成后获取发送方的account id给sender

            ensure!(
                !Proofs::<T>::contains_key(&claim),
                Error::<T>::ProofAlreadyExist
            );
            //确认记录里并不存在此存证,否则返回错误ProofAlreadyExist. 使用proofs存储单元的contains_key方法给定key去判断存储单元里面
            //也就是storagemap里是不是包含这个key所对应的记录 zzzz 1214

            Proofs::<T>::insert(
                &claim,
                (sender.clone(), frame_system::Pallet::<T>::block_number()),
            );
            //校验完成后调用存储单元proofs上的insert方法存储相关记录, key是claim,表示存证的哈希值,val是两元素组成的tuple,第一元素是交易发送方的sender,也是存证的owner,
            //第二元素是区块,使用system模块提供的blocknumber这个公共方法来获取当前的区块.

            Self::deposit_event(Event::ClaimCreated(sender, claim));
            //s1,保存成功后触发事件,在之前event定义时定义了一个帮助方法deposit event可以用来触发事件.此处定义了一个Event叫ClaimCreated,此时在r1添加对应信息.

            Ok(().into())
            //返回结果,是result类型,并且进行转换.
        }

        #[pallet::weight(0)]
        pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            Proofs::<T>::remove(&claim);

            Self::deposit_event(Event::ClaimRevoked(sender, claim));

            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            claim: Vec<u8>,
            dest: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            let (owner, _block_number) =
                Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            Proofs::<T>::insert(&claim, (dest, frame_system::Pallet::<T>::block_number()));

            Ok(().into())
        }
    }
}
