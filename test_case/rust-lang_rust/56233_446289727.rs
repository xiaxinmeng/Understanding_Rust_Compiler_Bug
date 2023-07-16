
commit dcbd573f5014c28610c0e6e3476d40e5042bc844
Author: kenta7777 <k.hasegw7@gmail.com>
Date:   Mon Dec 10 23:45:33 2018 +0900

    reduce some code repetitions

commit 3499575282b5cda1e98220baae4f6c87e1863926
Merge: 3a31213 c28c287
Author: bors <bors@rust-lang.org>
Date:   Tue Dec 11 14:04:15 2018 +0000

    Auto merge of #56243 - RalfJung:test-deterministic, r=alexcrichton
    
    libtest: Use deterministic HashMap, avoid spawning thread if there is no concurrency
    
    It seems desirable to make a test and bench runner deterministic, which this achieves by using a deterministic hasher. Also, we we only have 1 thread, we don't bother spawning one and just use the main thread.
    
    The motivation for this is to be able to run the test harness in miri, where we can neither access the OS RNG, nor spawn threads.

commit c28c28779c082b6e1d0e7007a222392dc5d6c052
Author: Ralf Jung <post@ralfj.de>
Date:   Tue Dec 11 11:02:23 2018 +0100

    use an enum instead of bool

