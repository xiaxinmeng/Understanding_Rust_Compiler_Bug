rust
                let item = $item;
                unsafe { ::core::ptr::write(x.as_mut_ptr(), item) };
