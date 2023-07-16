
#![no_std]
#![feature(lang_items, alloc_system, alloc_error_handler, alloc)]

extern crate alloc_system;
extern crate alloc;

use crate::alloc_system::System;
use crate::alloc::boxed::Box;
use crate::alloc::vec::Vec;
use core::panic::PanicInfo;
