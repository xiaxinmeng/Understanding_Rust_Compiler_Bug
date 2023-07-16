plain
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
    Checking alloc v0.0.0 (/checkout/library/alloc)
error[E0599]: no function or associated item named `new` found for struct `ancillary::SocketCred` in the current scope
    |
    |
697 |     let mut cred1 = SocketCred::new();
    |                                 ^^^ function or associated item not found in `ancillary::SocketCred`
   ::: library/std/src/os/unix/net/ancillary.rs:189:1
    |
    |
189 | pub struct SocketCred(libc::ucred);
    | ----------------------------------- function or associated item `new` not found for this

error[E0599]: no method named `get_pid` found for struct `ancillary::SocketCred` in the current scope
    |
    |
727 |         assert_eq!(cred1.get_pid(), cred_vec[0].get_pid());
    |                                                 ^^^^^^^ method not found in `ancillary::SocketCred`
   ::: library/std/src/os/unix/net/ancillary.rs:189:1
    |
    |
189 | pub struct SocketCred(libc::ucred);
    | ----------------------------------- method `get_pid` not found for this

error[E0599]: no method named `get_uid` found for struct `ancillary::SocketCred` in the current scope
    |
    |
728 |         assert_eq!(cred1.get_uid(), cred_vec[0].get_uid());
    |                                                 ^^^^^^^ method not found in `ancillary::SocketCred`
   ::: library/std/src/os/unix/net/ancillary.rs:189:1
    |
    |
189 | pub struct SocketCred(libc::ucred);
    | ----------------------------------- method `get_uid` not found for this

error[E0599]: no method named `get_gid` found for struct `ancillary::SocketCred` in the current scope
    |
    |
729 |         assert_eq!(cred1.get_gid(), cred_vec[0].get_gid());
    |                                                 ^^^^^^^ method not found in `ancillary::SocketCred`
   ::: library/std/src/os/unix/net/ancillary.rs:189:1
    |
    |
189 | pub struct SocketCred(libc::ucred);
    | ----------------------------------- method `get_gid` not found for this
For more information about this error, try `rustc --explain E0599`.
error: could not compile `std` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
