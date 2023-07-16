
error[E0425]: cannot find function `on_resolver_failure` in module `sys::net`
   --> libstd\sys_common\net.rs:174:33
    |
174 |                 Err(::sys::net::on_resolver_failure(e))
    |                                 ^^^^^^^^^^^^^^^^^^^ not found in `sys::net`
