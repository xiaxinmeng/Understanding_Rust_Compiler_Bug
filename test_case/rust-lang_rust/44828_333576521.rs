
// MIR for `main`
// source = Fn(NodeId(29))
// pass_name = CleanEndRegions
// disambiguator = before

fn main() -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let mut _1: isize;               // "x" in scope 1 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:23:9: 23:14
        scope 3 {
            let mut _2: S<'31_0rs>;      // "y" in scope 3 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:26:13: 26:18
            scope 5 {
                let _5: S<'31_1rs>;      // "z" in scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:27:13: 27:14
            }
            scope 6 {
            }
        }
        scope 4 {
        }
    }
    scope 2 {
    }
    let mut _3: &'31_0rs mut isize;
    let mut _4: &'31_0rs mut isize;
    let mut _6: &'31_0rs mut S<'31_0rs>;
    let mut _7: &'31_0rs mut S<'31_0rs>;
    let mut _8: (isize, bool);
    let mut _9: (isize, bool);

    bb0: {
        StorageLive(_1);                 // scope 1 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:23:9: 23:14
        _1 = const 1isize;               // scope 1 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:23:17: 23:18
        EndRegion('3s);                  // scope 1 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:23:17: 23:18
        EndRegion('4s);                  // scope 0 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:22:11: 31:2
        EndRegion('4ds);                 // scope 0 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:22:11: 31:2
        StorageLive(_2);                 // scope 3 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:26:13: 26:18
        StorageLive(_3);                 // scope 3 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:26:34: 26:40
        StorageLive(_4);                 // scope 3 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:26:34: 26:40
        EndRegion('8s);                  // scope 3 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:26:39: 26:40
        _4 = &'31_0rs mut _1;            // scope 3 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:26:34: 26:40
        _3 = &'31_0rs mut (*_4);         // scope 3 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:26:34: 26:40
        EndRegion('9s);                  // scope 3 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:26:34: 26:40
        _2 = S<'31_0rs> { pointer: _3 }; // scope 3 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:26:21: 26:42
        EndRegion('10s);                 // scope 3 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:26:21: 26:42
        StorageDead(_3);                 // scope 3 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:26:42: 26:42
        EndRegion('11s);                 // scope 1 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:25:5: 30:6
        StorageDead(_4);                 // scope 3 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:26:43: 26:43
        EndRegion('11ds);                // scope 1 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:25:5: 30:6
        StorageLive(_5);                 // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:27:13: 27:14
        EndRegion('14s);                 // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:27:17: 27:34
        StorageLive(_6);                 // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:27:35: 27:41
        StorageLive(_7);                 // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:27:35: 27:41
        EndRegion('15s);                 // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:27:40: 27:41
        _7 = &'31_0rs mut _2;            // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:27:35: 27:41
        _6 = &'31_0rs mut (*_7);         // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:27:35: 27:41
        EndRegion('16s);                 // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:27:35: 27:41
        _5 = const copy_borrowed_ptr(_6) -> bb1; // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:27:17: 27:42
    }

    bb1: {
        EndRegion('17s);                 // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:27:17: 27:42
        StorageDead(_6);                 // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:27:42: 27:42
        EndRegion('18s);                 // scope 1 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:25:5: 30:6
        StorageDead(_7);                 // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:27:43: 27:43
        EndRegion('18ds);                // scope 1 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:25:5: 30:6
        EndRegion('22s);                 // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:28:23: 28:24
        EndRegion('19s);                 // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:28:10: 28:11
        EndRegion('20s);                 // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:28:10: 28:19
        EndRegion('21s);                 // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:28:9: 28:19
        _8 = CheckedAdd((*(_2.0: &'31_0rs mut isize)), const 1isize); // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:28:9: 28:24
        assert(!(_8.1: bool), "attempt to add with overflow") -> bb2; // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:28:9: 28:24
    }

    bb2: {
        (*(_2.0: &'31_0rs mut isize)) = (_8.0: isize); // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:28:9: 28:24
        EndRegion('23s);                 // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:28:9: 28:24
        EndRegion('24s);                 // scope 1 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:25:5: 30:6
        EndRegion('24ds);                // scope 1 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:25:5: 30:6
        EndRegion('28s);                 // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:29:23: 29:24
        EndRegion('25s);                 // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:29:10: 29:11
        EndRegion('26s);                 // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:29:10: 29:19
        EndRegion('27s);                 // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:29:9: 29:19
        _9 = CheckedAdd((*(_5.0: &'31_1rs mut isize)), const 1isize); // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:29:9: 29:24
        assert(!(_9.1: bool), "attempt to add with overflow") -> bb3; // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:29:9: 29:24
    }

    bb3: {
        (*(_5.0: &'31_1rs mut isize)) = (_9.0: isize); // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:29:9: 29:24
        EndRegion('29s);                 // scope 5 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:29:9: 29:24
        EndRegion('30s);                 // scope 1 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:25:5: 30:6
        EndRegion('30ds);                // scope 1 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:25:5: 30:6
        _0 = ();                         // scope 1 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:25:5: 30:6
        EndRegion('31_1rs);              // scope 1 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:25:5: 30:6
        StorageDead(_5);                 // scope 3 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:30:6: 30:6
        EndRegion('31_0rs);              // scope 1 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:25:5: 30:6
        StorageDead(_2);                 // scope 1 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:30:6: 30:6
        EndRegion('31s);                 // scope 1 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:25:5: 30:6
        EndRegion('32s);                 // scope 1 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:25:5: 30:6
        EndRegion('33_0rs);              // scope 0 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:22:11: 31:2
        StorageDead(_1);                 // scope 0 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:31:2: 31:2
        EndRegion('33s);                 // scope 0 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:22:11: 31:2
        EndRegion('34s);                 // scope 0 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:22:11: 31:2
        EndRegion('34ds);                // scope 0 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:22:11: 31:2
        EndRegion('34as);                // scope 0 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:22:1: 31:2
        goto -> bb4;                     // scope 0 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:31:2: 31:2
    }

    bb4: {
        EndRegion('34cs);                // scope 0 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:22:1: 31:2
        return;                          // scope 0 at ../src/test/compile-fail/borrowck/borrowck-assign-to-andmut-in-borrowed-loc.rs:31:2: 31:2
    }
}
