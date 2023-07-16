rust
impl From<String> for Rc<str> {
    #[inline]
    fn from(v: String) -> Rc<str> {
        Rc::from(&v[..])
    }
}

impl<'a, T: Clone> From<&'a [T]> for Rc<[T]> {
    #[inline]
    fn from(v: &[T]) -> Rc<[T]> {
        <Self as RcFromSlice<T>>::from_slice(v)
    }
}

impl<T: ?Sized> From<Box<T>> for Rc<T> {
    #[inline]
    fn from(v: Box<T>) -> Rc<T> {
        Rc::from_box(v)
    }
}


struct RcBox<T: ?Sized> {
    strong: Cell<usize>,
    weak: Cell<usize>,
    value: T,
}

impl<'a> From<&'a str> for Rc<str> {
    #[inline]
    fn from(v: &str) -> Rc<str> {
        let rc = Rc::<[u8]>::from(v.as_bytes());
        unsafe { Rc::from_raw(Rc::into_raw(rc) as *const str) }
    }
}

impl<T> From<Vec<T>> for Rc<[T]> {
    #[inline]
    fn from(mut v: Vec<T>) -> Rc<[T]> {
        unsafe {
            let rc = Rc::copy_from_slice(&v);

            // Allow the Vec to free its memory, but not destroy its contents
            v.set_len(0);

            rc
        }
    }
}

pub struct Rc<T: ?Sized> {
    ptr: NonNull<RcBox<T>>,
    phantom: PhantomData<T>,
}

trait RcFromSlice<T> {
    fn from_slice(slice: &[T]) -> Self;
}

impl<T: Clone> RcFromSlice<T> for Rc<[T]> {
    #[inline]
    default fn from_slice(v: &[T]) -> Self {
        // Panic guard while cloning T elements.
        // In the event of a panic, elements that have been written
        // into the new RcBox will be dropped, then the memory freed.
        struct Guard<T> {
            mem: NonNull<u8>,
            elems: *mut T,
            layout: Layout,
            n_elems: usize,
        }

        impl<T> Drop for Guard<T> {
            fn drop(&mut self) {
                use core::slice::from_raw_parts_mut;

                unsafe {
                    let slice = from_raw_parts_mut(self.elems, self.n_elems);
                    ptr::drop_in_place(slice);

                    Global.dealloc(self.mem, self.layout.clone());
                }
            }
        }

        unsafe {
            let v_ptr = v as *const [T];
            let ptr = Self::allocate_for_ptr(v_ptr);

            let mem = ptr as *mut _ as *mut u8;
            let layout = Layout::for_value(&*ptr);

            // Pointer to first element
            let elems = &mut (*ptr).value as *mut [T] as *mut T;

            let mut guard = Guard{
                mem: NonNull::new_unchecked(mem),
                elems: elems,
                layout: layout,
                n_elems: 0,
            };

            for (i, item) in v.iter().enumerate() {
                ptr::write(elems.add(i), item.clone());
                guard.n_elems += 1;
            }

            // All clear. Forget the guard so it doesn't free the new RcBox.
            forget(guard);

            Rc { ptr: NonNull::new_unchecked(ptr), phantom: PhantomData }
        }
    }
}

impl<T: Copy> RcFromSlice<T> for Rc<[T]> {
    #[inline]
    fn from_slice(v: &[T]) -> Self {
        unsafe { Rc::copy_from_slice(v) }
    }
}

impl<T> Rc<[T]> {
    // Copy elements from slice into newly allocated Rc<[T]>
    //
    // Unsafe because the caller must either take ownership or bind `T: Copy`
    unsafe fn copy_from_slice(v: &[T]) -> Rc<[T]> {
        let v_ptr = v as *const [T];
        let ptr = Self::allocate_for_ptr(v_ptr);

        ptr::copy_nonoverlapping(
            v.as_ptr(),
            &mut (*ptr).value as *mut [T] as *mut T,
            v.len());

        Rc { ptr: NonNull::new_unchecked(ptr), phantom: PhantomData }
    }
}

impl<T: ?Sized> Rc<T> {
    // Allocates an `RcBox<T>` with sufficient space for an unsized value
    unsafe fn allocate_for_ptr(ptr: *const T) -> *mut RcBox<T> {
        // Create a fake RcBox to find allocation size and alignment
        let fake_ptr = ptr as *mut RcBox<T>;

        let layout = Layout::for_value(&*fake_ptr);

        let mem = Global.alloc(layout)
            .unwrap_or_else(|_| handle_alloc_error(layout));

        // Initialize the real RcBox
        let inner = set_data_ptr(ptr as *mut T, mem.as_ptr() as *mut u8) as *mut RcBox<T>;

        ptr::write(&mut (*inner).strong, Cell::new(1));
        ptr::write(&mut (*inner).weak, Cell::new(1));

        inner
    }

    fn from_box(v: Box<T>) -> Rc<T> {
        unsafe {
            let box_unique = Box::into_unique(v);
            let bptr = box_unique.as_ptr();

            let value_size = size_of_val(&*bptr);
            let ptr = Self::allocate_for_ptr(bptr);

            // Copy value as bytes
            ptr::copy_nonoverlapping(
                bptr as *const T as *const u8,
                &mut (*ptr).value as *mut _ as *mut u8,
                value_size);

            // Free the allocation without dropping its contents
            box_free(box_unique);

            Rc { ptr: NonNull::new_unchecked(ptr), phantom: PhantomData }
        }
    }
}

// Sets the data pointer of a `?Sized` raw pointer.
//
// For a slice/trait object, this sets the `data` field and leaves the rest
// unchanged. For a sized raw pointer, this simply sets the pointer.
unsafe fn set_data_ptr<T: ?Sized, U>(mut ptr: *mut T, data: *mut U) -> *mut T {
    ptr::write(&mut ptr as *mut _ as *mut *mut u8, data as *mut u8);
    ptr
}
