rust
> use std::mem::MaybeUninit;
> 
> fn main() {
>     const N: usize = 5;
> 
>     let arr: [Vec<u8>; N] = unsafe {
>         let mut arr: MaybeUninit<[Vec<u8>; N]> = MaybeUninit::uninitialized();
>         let arr_ptr = arr.as_mut_ptr() as *mut Vec<u8>; // pointer to 1st element
>         for i in 0..N {
>             arr_ptr.add(i).write(vec![]);
>         }
>         arr.into_inner()
>     };
> 
>     for idx in 0..arr.len() {
>         println!("Index {}: {:?}", idx, arr[idx]);
>     }
> }
> 