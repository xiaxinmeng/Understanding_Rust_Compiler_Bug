
#[derive(Clone)]
pub struct BoxError {
    inner: Box<dyn Error+Send+Sync+'static>,
}
// delegate Deref+DerefMut+Drop to inner

impl Drop for BoxError {
    fn drop(&mut self) {
        self.deref_mut().drop()
    }
}

use core::mem;
use core::raw::TraitObject;

const SMALL_ERROR_SIZE : usize = mem::size_of::<usize>();

#[derive(Clone)]
pub struct SmallError {
    vtable: *mut (),
    data: [u8; SMALL_ERROR_SIZE],
}

impl Drop for SmallError {
    fn drop(&mut self) {
        self.deref_mut().drop()
    }
}
impl Send for SmallError {}
impl Sync for SmallError {}

impl SmallError {
    pub fn new<E: Error+Clone+Send+Sync>(err: E) -> SmallError 
    // where mem::size_of::<E>() <= SMALL_ERROR_SIZE  // sigh
    {
        assert!(mem::size_of::<E>() <= SMALL_ERROR_SIZE);
        let data = [0u8; SMALL_ERROR_SIZE],
        core::intrinsics::copy(&err as *const E, &mut data as *mut [u8; SMALL_ERROR_SIZE] as *mut E , 1);
        let vtable = unsafe { mem::transmute::<&mut dyn Error,TraitObject>(&mut dyn err).vtable };
        // mem::forget(err);
        SmallError { vtable, data }
    }
}

impl Deref for SmallError {
    type Target = dyn Error+Send+Sync;  // Should we add +Clone when mutli-trait objects exist
    fn deref(&self) -> &Self::Target {
        let SmallError { vtable, ref data } = self;
        mem::transmute::<TraitObject,&mut dyn Error>(TraitObject { vtable, data })
    }
}

impl DerefMut for SmallError {
    fn deref(&mut self) -> &mut Self::Target {
        let SmallError { vtable, ref data } = self;
        mem::transmute::<TraitObject,&mut dyn Error>(TraitObject { vtable, data })
    }
}
