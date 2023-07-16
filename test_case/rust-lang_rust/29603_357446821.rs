
~$ cat strong.rs 
extern {
    static __progname: *const u8;
}

fn main() {
    unsafe {
        println!("  __progname:\t\t{:?}", __progname);
        println!(" &__progname:\t\t{:?}", &__progname as *const _);
        if !__progname.is_null() {
            println!(" *__progname:\t\t{:?}", *__progname as char);
        }
    }
}
~$ cat weak.rs 
#![feature(linkage)]

extern {
    #[linkage = "extern_weak"]
    static __progname: *mut *const u8;
}

fn main() {
    unsafe {
        println!("  __progname:\t\t{:?}", __progname);
        println!(" &__progname:\t\t{:?}", &__progname as *const _);
        if !__progname.is_null() {
            println!(" *__progname:\t\t{:?}", *__progname);
            if !(*__progname).is_null() {
                println!("**__progname:\t\t{:?}", **__progname as char);
            }
        }
    }
}
~$ rustc strong.rs 
~$ rustc weak.rs 
~$ ./strong 
  __progname:		0x7ffdfc7438ac
 &__progname:		0x7f10560a5360
 *__progname:		's'
~$ ./weak 
  __progname:		0x7f730b21f360
 &__progname:		0x5574a33bb008
 *__progname:		0x7fffe4ca38b2
**__progname:		'w'
