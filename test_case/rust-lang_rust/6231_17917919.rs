
// -*- rust -*-
// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::libc::c_void;
use core::libc;
use core::cast;

pub type rust_task = c_void;

pub type LocalDataKey<'self,T> = &'self fn(v: @T);

pub trait LocalData { }
impl<T: 'static> LocalData for @T { }

impl Eq for @LocalData {
    fn eq(&self, other: &@LocalData) -> bool {
        unsafe {
            let ptr_a: &(uint, uint) = cast::transmute(self);
            let ptr_b: &(uint, uint) = cast::transmute(other);
            return ptr_a == ptr_b;
        }
    }
    fn ne(&self, other: &@LocalData) -> bool { !(*self).eq(other) }
}

type TaskLocalElement = (*libc::c_void, *libc::c_void, @LocalData);
// Has to be a pointer at outermost layer; the foreign call returns void *.
type TaskLocalMap = @mut ~[Option<TaskLocalElement>];

fn cleanup_task_local_map(map_ptr: *libc::c_void) {
    unsafe {
        ::io::println("cleaning up task local map");
        assert!(!map_ptr.is_null());
        // Get and keep the single reference that was created at the
        // beginning.
        let map: TaskLocalMap = cast::transmute(map_ptr);
        *map == ~[];
        // All local_data will be destroyed along with the map.
    }
}

unsafe fn get_task_local_map(task: *rust_task) -> TaskLocalMap {

    extern fn cleanup_task_local_map_extern_cb(map_ptr: *libc::c_void) {
        cleanup_task_local_map(map_ptr);
    }

    // Relies on the runtime initialising the pointer to null.
    // Note: The map's box lives in TLS invisibly referenced once. Each time
    // we retrieve it for get/set, we make another reference, which get/set
    // drop when they finish. No "re-storing after modifying" is needed.
    let map_ptr = rust_get_task_local_data(task);
    if map_ptr.is_null() {
        let map: TaskLocalMap = @mut ~[];
        rust_set_task_local_data(task, cast::transmute(map));
        rust_task_local_data_atexit(task, cleanup_task_local_map_extern_cb);
        // Also need to reference it an extra time to keep it for now.
        let nonmut = cast::transmute::<TaskLocalMap,
                                       @~[Option<TaskLocalElement>]>(map);
        cast::bump_box_refcount(nonmut);
        map
    } else {
        let map = cast::transmute(map_ptr);
        let nonmut = cast::transmute::<TaskLocalMap,
                                       @~[Option<TaskLocalElement>]>(map);
        cast::bump_box_refcount(nonmut);
        map
    }
}

extern {
    #[rust_stack]
    pub fn rust_get_task() -> *rust_task;
    #[rust_stack]
    pub fn rust_get_task_local_data(task: *rust_task) -> *libc::c_void;
    #[rust_stack]
    pub fn rust_set_task_local_data(task: *rust_task, map: *libc::c_void);
    #[rust_stack]
    pub fn rust_task_local_data_atexit(task: *rust_task, cleanup_fn: *u8);
}

pub fn main() {
    unsafe {
        get_task_local_map(rust_get_task());
    }
}
