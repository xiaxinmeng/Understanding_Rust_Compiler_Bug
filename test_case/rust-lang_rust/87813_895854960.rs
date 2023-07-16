rust
#![feature(once_cell)]

pub mod addr {
    use std::convert::TryFrom;
    use std::marker::PhantomData as PD;

    extern "C" {
        pub static _ENARX_SALLYPORT_START: u8;
    }

    pub struct Address<T, U>(T, PD<U>);
    impl<T, U> Address<T, U> {
        pub fn f<Z>(_: &Z) -> Self {
            todo!()
        }
    }

    pub struct ShimVirtAddr<U>(Address<u64, U>);
    impl<U> TryFrom<Address<u64, U>> for ShimVirtAddr<U> {
        type Error = ();
        fn try_from(_: Address<u64, U>) -> Result<Self, Self::Error> {
            Err(())
        }
    }

    pub struct ShimPhysUnencryptedAddr<U>(Address<u64, U>);

    impl<U> ShimPhysUnencryptedAddr<U> {
        pub fn into_mut<'a>(self) -> &'a mut U {
            unsafe { &mut *(self.0 .0 as *mut U) }
        }
    }

    impl<U> TryFrom<ShimVirtAddr<U>> for ShimPhysUnencryptedAddr<U> {
        type Error = ();

        #[inline(always)]
        fn try_from(_: ShimVirtAddr<U>) -> Result<Self, Self::Error> {
            let _a = 0 < unsafe { &_ENARX_SALLYPORT_START as *const _ as u64 };

            Err(())
        }
    }
}
pub mod hostcall {
    extern "C" {
        pub static _ENARX_SALLYPORT_START: Block;
    }
    use std::convert::TryFrom;
    use std::lazy::SyncLazy;

    use crate::addr::{Address, ShimPhysUnencryptedAddr, ShimVirtAddr};

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Block {
        buf: [u8; 100],
    }

    pub static HOST_CALL_ALLOC: SyncLazy<HostCallAllocator> = SyncLazy::new(|| {
        let _a: *mut Block = unsafe {
            let address = Address::<u64, Block>::f(&_ENARX_SALLYPORT_START);
            let shim_virt = ShimVirtAddr::<Block>::try_from(address).unwrap();

            ShimPhysUnencryptedAddr::<Block>::try_from(shim_virt)
                .unwrap()
                .into_mut() as *mut _
        };
        HostCallAllocator([None, None])
    });

    pub struct HostCallAllocator([Option<&'static mut Block>; 2]);
}

fn main() {}
