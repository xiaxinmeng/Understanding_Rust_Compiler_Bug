rust
#[repr(C)]
#[derive(Clone, Copy)]
pub union MyResult {
    ok: MyOk,
    err: MyErr,
}

#[repr(usize)]
#[derive(Clone, Copy)]
enum MyResultTag { Ok, Err }

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MyOk(MyResultTag, usize);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MyErr(MyResultTag, isize);

#[repr(usize)]
#[derive(Clone, Copy)]
pub enum MyResult2 {
    Ok(usize),
    Err(isize),
}

pub fn size() -> usize {
    std::mem::size_of::<MyResult>()
}

pub fn size2() -> usize {
    std::mem::size_of::<MyResult2>()
}

extern {
    fn t(_: MyResult);
    fn t2(_: MyResult2);
}

pub fn try() {
    unsafe { t(MyResult{ok:MyOk(MyResultTag::Ok, 1)}) };
}

pub fn try2() {
    unsafe { t2(MyResult2::Ok(1)) };
}
