rust
/// Methods of this trait signifies a point where CTFE evaluation would fail
/// and some use case dependent behaviour can instead be applied.
pub trait Machine<'a, 'mir, 'tcx>: Sized {
    /// Additional memory kinds a machine wishes to distinguish from the builtin ones
    type MemoryKinds: ::std::fmt::Debug + MayLeak + Eq + 'static;

    /// Tag tracked alongside every pointer.  This is used to implement "Stacked Borrows"
    /// <https://www.ralfj.de/blog/2018/08/07/stacked-borrows.html>.
    /// The `default()` is used for pointers to consts, statics, vtables and functions.
    type PointerTag: ::std::fmt::Debug + Default + Copy + Eq + Hash + 'static;

    /// Extra data stored in every call frame.
    type FrameExtra;

    /// Extra data stored in memory.  A reference to this is available when `AllocExtra`
    /// gets initialized, so you can e.g. have an `Rc` here if there is global state you
    /// need access to in the `AllocExtra` hooks.
    type MemoryExtra: Default;

    /// Extra data stored in every allocation.
    type AllocExtra: AllocationExtra<Self::PointerTag, Self::MemoryExtra>;

    /// Memory's allocation map
    type MemoryMap:
        AllocMap<
            AllocId,
            (MemoryKind<Self::MemoryKinds>, Allocation<Self::PointerTag, Self::AllocExtra>)
        > +
        Default +
        Clone;
