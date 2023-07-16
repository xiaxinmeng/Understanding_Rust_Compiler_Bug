
warning: allow(unreachable_code) incompatible with previous forbid
   --> src/lib.rs:197:9
    |
17  | #![forbid(warnings)]
    |           -------- `forbid` level set here
...
197 |         world.repo.publish_offramp(&id, false, o).await?;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overruled by previous forbid
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
