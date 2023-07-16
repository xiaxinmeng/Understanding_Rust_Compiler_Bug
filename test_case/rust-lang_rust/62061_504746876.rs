
error: any use of this value will cause an error
   --> /home/alexgaynor/projects/linux-kernel-module-rust/src/chrdev.rs:125:5
    |
125 |       intrinsics::panic_if_uninhabited::<T>();
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |       |
    |       "calling intrinsic `panic_if_uninhabited`" needs an rfc before being allowed inside constants
    |       inside call to `linux_kernel_module::chrdev::zeroed::<linux_kernel_module::bindings::file_operations>` at /home/alexgaynor/projects/linux-kernel-module-rust/src/chrdev.rs:132:71
    |       inside call to `linux_kernel_module::chrdev::FileOperationsVtable::new::<CycleFile>` at src/lib.rs:14:9
    | 
   ::: src/lib.rs:13:5
    |
13  | /     const VTABLE: linux_kernel_module::chrdev::FileOperationsVtable =
14  | |         linux_kernel_module::chrdev::FileOperationsVtable::new::<Self>();
    | |_________________________________________________________________________-
    |
    = note: #[deny(const_err)] on by default
