
error[E0425]: cannot find function `acquire_global_lock` in module `lock`
    --> /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/syntex_errors-0.59.1/src/emitter.rs:1242:30
     |
1242 |     let _buffer_lock = lock::acquire_global_lock("rustc_errors");
     |                              ^^^^^^^^^^^^^^^^^^^ not found in `lock`
