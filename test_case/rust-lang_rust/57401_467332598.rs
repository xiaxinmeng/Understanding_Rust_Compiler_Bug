rust
pub struct Ref<'b, T: ?Sized + 'b> {
    value: &'b T, // Immutable reference
    borrow: BorrowRef<'b>, // Refcell is immutably borrowed
}

pub struct RefMut<'b, T: ?Sized + 'b> {
    value: &'b mut T, // Mutable reference
    borrow: BorrowRefMut<'b>, // Refcell is mutably borrowed
}

pub struct DowngradedRef<'b, T: ?Sized + 'b> {
    value: &'b T, // Immutable reference
    borrow: BorrowRefMut<'b>, // Refcell is mutably borrowed
}
