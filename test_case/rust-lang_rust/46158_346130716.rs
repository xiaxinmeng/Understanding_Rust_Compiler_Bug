patch
diff -rU10 /home/ariel/.cargo/registry/src/github.com-1ecc6299db9ec823/x11-dl-2.15.0/src/link.rs ./src/link.rs
--- /home/ariel/.cargo/registry/src/github.com-1ecc6299db9ec823/x11-dl-2.15.0/src/link.rs       2017-08-04 21:44:07.000000000 +0300
+++ ./src/link.rs       2017-11-21 21:04:48.688374887 +0200
@@ -29,44 +29,45 @@
       lib: ::link::DynamicLibrary,
       $(pub $fn_name: unsafe extern "C" fn ($($param_type),*) -> $ret_type,)*
       $(pub $vfn_name: unsafe extern "C" fn ($($vparam_type),+, ...) -> $vret_type,)*
       $(pub $var_name: *mut $var_type,)*
     }
 
     unsafe impl Send for $struct_name {}
     unsafe impl Sync for $struct_name {}
 
     impl $struct_name {
-      unsafe fn init (&mut self) -> Result<(), $crate::error::OpenError> {
+      unsafe fn init (this: *mut Self) -> Result<(), $crate::error::OpenError> {
         lazy_static! {
           static ref SYMS: [(&'static str, usize); $nsyms] = unsafe {[
             $((stringify!($fn_name), &((*(0 as *const $struct_name)).$fn_name) as *const _ as usize),)*
             $((stringify!($vfn_name), &((*(0 as *const $struct_name)).$vfn_name) as *const _ as usize),)*
             $((stringify!($var_name), &((*(0 as *const $struct_name)).$var_name) as *const _ as usize),)*
           ]};
         }
-        let offset = self as *mut $struct_name as usize;
+        let offset = this as usize;
         for &(name, sym_offset) in SYMS.iter() {
-          *((offset + sym_offset) as *mut *mut _) = try!(self.lib.symbol(name));
+          *((offset + sym_offset) as *mut *mut _) = try!((*this).lib.symbol(name));
         }
         Ok(())
       }
 
       pub fn open () -> Result<$struct_name, $crate::error::OpenError> {
         unsafe {
           let libdir = $crate::link::config::libdir::$pkg_name;
-          let mut lib = try!($crate::link::DynamicLibrary::open_multi(libdir, &[$($lib_name),*]));
-          let mut this: $struct_name = ::std::mem::zeroed();
-          ::std::mem::swap(&mut lib, &mut this.lib);
-          ::std::mem::forget(lib);
-          try!(this.init());
-          Ok(this)
+          let lib = try!($crate::link::DynamicLibrary::open_multi(libdir, &[$($lib_name),*]));
+          let mut this: ::std::mem::ManuallyDrop<$struct_name>
+              = ::std::mem::zeroed();
+          let this_ptr = &mut this as *mut _ as *mut $struct_name;
+          ::std::ptr::write(&mut (*this_ptr).lib, lib);
+          try!(Self::init(this_ptr));
+          Ok(::std::mem::ManuallyDrop::into_inner(this))
         }
       }
     }
   };
 }
