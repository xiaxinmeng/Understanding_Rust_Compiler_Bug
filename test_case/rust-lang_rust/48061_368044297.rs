rust
    fn twice_ten_oo(f: Box<FnOnce(i32) -> i32>) {
        f(f(10));
        //[lxl]~^             ERROR cannot move a value of type
        //[lxl]~^^            ERROR cannot move a value of type
        //[lxl]~^^^           ERROR use of moved value: `*f`
        //[nll]~^^^^          ERROR cannot move a value of type
        //[nll]~^^^^^         ERROR cannot move a value of type
        //[nll]~^^^^^^        ERROR cannot move a value of type
        //[nll]~^^^^^^^       ERROR cannot move a value of type
        //[nll]~^^^^^^^^      ERROR use of moved value: `*f`
        //[g2p]~^^^^^^^^^     ERROR cannot move a value of type
        //[g2p]~^^^^^^^^^^    ERROR cannot move a value of type
        //[g2p]~^^^^^^^^^^^   ERROR cannot move a value of type
        //[g2p]~^^^^^^^^^^^^  ERROR cannot move a value of type
        //[g2p]~^^^^^^^^^^^^^ ERROR use of moved value: `*f`
    }
