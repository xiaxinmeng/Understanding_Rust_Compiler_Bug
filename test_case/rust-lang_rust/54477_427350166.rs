

commit 02cb8f2a4f078025abe6ddba3cff81b383a23973
Merge: e6b35b0e11 21d2a6c986
Author: bors <bors@rust-lang.org>
Date:   Wed Aug 29 20:08:16 2018 +0000

    Auto merge of #53564 - MaloJaffre:vecdeque, r=gnzlbg

    Reoptimize VecDeque::append

    ~Unfortunately, I don't know if these changes fix the unsoundness mentioned in #53529, so it is stil a WIP.
    This is also completely untested.
    The VecDeque code contains other unsound code: one example : [reading unitialized memory](https://play.rust-lang.org/?gist=6ff47551769af61fd8adc45c44010887&version=nightly&mode=release&edition=2015) (detected by MIRI), so I think this code\
 will need a bigger refactor to make it clearer and safer.~

    Note: this is based on #53571.
    r? @SimonSapin
    Cc: #53529 #52553 @YorickPeterse @jonas-schievink @Pazzaz @shepmaster.

commit e6b35b0e1115f008796e8313574e4a4739b6d39d
Merge: ba48850409 b96fef6080
Author: bors <bors@rust-lang.org>
Date:   Wed Aug 29 13:35:25 2018 +0000

    Auto merge of #53758 - oli-obk:clippy, r=kennytm

    Update clippy submodule

    r? @Manishearth @nrc @kennytm

commit ba48850409222b2470fdc606329dc74aecbc0faa
Merge: ca0de63898 3cf6f0db1a
Author: bors <bors@rust-lang.org>
Date:   Wed Aug 29 10:55:27 2018 +0000

    Auto merge of #53245 - michaelwoerister:thinlto-rust-llvm, r=alexcrichton

    [experimental]: Build LLVM with ThinLTO enabled (2nd attempt)

    This is https://github.com/rust-lang/rust/pull/51207 revived. This time, I'd like to run actual performance tests to see if it improves compile times.

commit ca0de63898b525656ad8447cd81ccb08a05e3d6c
Merge: f4e981cfe4 025d01432f
Author: bors <bors@rust-lang.org>
Date:   Wed Aug 29 08:42:20 2018 +0000

    Auto merge of #53711 - arielb1:macro-table, r=michaelwoerister

    create a valid DefIdTable for proc macro crates

    At least the incremental compilation code, and a few other places in the
    compiler, require the CrateMetadata for a loaded target crate to contain a
    valid DefIdTable for the DefIds in the target.

    Previously, the CrateMetadata for a proc macro contained the crate's
    "host" DefIdTable, which is of course incompatible with the "target"
    DefIdTable, causing ICEs. This creates a DefIdTable that properly refers
    to the "proc macro" DefIds.

    Fixes #49482.

    r? @michaelwoerister

    Should we beta-nominate this?

commit f4e981cfe474f598b34ca07df8c8f16f042e120f
Merge: 29e6aabceb 51fd3bf6a8
Author: bors <bors@rust-lang.org>
Date:   Wed Aug 29 06:24:30 2018 +0000

    Auto merge of #53684 - alexcrichton:suggest-remove, r=oli-obk

    rustc: Suggest removing `extern crate` in 2018

    This commit updates the `unused_extern_crates` lint to make automatic
    suggestions about removing `extern crate` annotations in the 2018 edition. This
    ended up being a little easier than originally though due to what's likely been
    fixed issues in the resolver!

    Closes #52829

commit 29e6aabcebe3bdb507df22a6233024711412b343
Merge: 9d69e81e9b 8cecfa62e8
Author: bors <bors@rust-lang.org>
Date:   Wed Aug 29 04:20:01 2018 +0000

    Auto merge of #53659 - nnethercote:rm-AccumulateVec, r=Mark-Simulacrum

    Remove `AccumulateVec` and its uses.

    It's basically just a less capable version of `SmallVec`.

    FWIW, the only use of `ArrayVec` is now within `HybridIdxSet`.

    r? @Mark-Simulacrum

commit 9d69e81e9b400805b0453b9262958c8e28faa8a0
Merge: f1d02c3073 1fd45a13de
Author: bors <bors@rust-lang.org>
Date:   Wed Aug 29 02:08:02 2018 +0000

    Auto merge of #53642 - alexcrichton:fix-target-cpu-native, r=arielb1

    Fix warnings about the `native` target-cpu

    This fixes a regression from #53031 where specifying `-C target-cpu=native` is
    printing a lot of warnings from LLVM about `native` being an unknown CPU. It
    turns out that `native` is indeed an unknown CPU and we have to perform a
    mapping to an actual CPU name, but this mapping is only performed in one
    location rather than all locations we inform LLVM about the target CPU.

    This commit centralizes the mapping of `native` to LLVM's value of the native
    CPU, ensuring that all locations we inform LLVM about the `target-cpu` it's
    never `native`.

    Closes #53322

commit f1d02c307348057fd0554ad934006b186f8b6826
Merge: 7061b27757 c9b5fac7da
Author: bors <bors@rust-lang.org>
Date:   Wed Aug 29 00:02:37 2018 +0000

    Auto merge of #53671 - RalfJung:miri-refactor, r=oli-obk

    Miri engine cleanup

    * Unify the two maps in memory to store the allocation and its kind together.
    * Share the handling of statics between CTFE and miri: The miri engine always
          uses "lazy" `AllocType::Static` when encountering a static.  Acessing that
          static invokes CTFE (no matter the machine).  The machine only has any
          influence when writing to a static, which CTFE outright rejects (but miri
          makes a copy-on-write).
    * Add an `AllocId` to by-ref consts so miri can use them as operands without
          making copies.
    * Move responsibilities around for the `eval_fn_call` machine hook: The hook
          just has to find the MIR (or entirely take care of everything); pushing the
          new stack frame is taken care of by the miri engine.
    * Expose the intrinsics and lang items implemented by CTFE so miri does not
          have to reimplement them.
    * Allow Machine to hook into foreign statics (used by miri to get rid of some other hacks).
    * Clean up function calling.
    * Switch const sanity check to work on operands, not mplaces.
    * Move const_eval out of rustc_mir::interpret, to make sure that it does not access private implementation details.

    In particular, we can finally make `eval_operand` take `&self`. :-)

    Should be merged after https://github.com/rust-lang/rust/pull/53609, across which I will rebase.
