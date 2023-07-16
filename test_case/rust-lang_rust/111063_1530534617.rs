
 Documenting stage2 cargo (x86_64-unknown-linux-gnu)
[...]
    Checking gix-odb v0.42.0
error[E0382]: borrow of moved value: `buf`
  --> /cargo/registry/src/index.crates.io-6f17d22bba15001f/gix-config-0.18.0/src/file/init/from_paths.rs:71:13
   |
57 |         buf: &mut Vec<u8>,
   |         --- move occurs because `buf` has type `&mut Vec<u8>`, which does not implement the `Copy` trait
...
71 |             buf.clear();
   |             ^^^^^^^^^^^ value borrowed here after move
...
78 |                 buf,
   |                 --- value moved here, in previous iteration of loop

error[E0382]: borrow of moved value: `buf`
   --> /cargo/registry/src/index.crates.io-6f17d22bba15001f/gix-config-0.18.0/src/file/includes/mod.rs:101:9
    |
88  |     buf: &mut Vec<u8>,
    |     --- move occurs because `buf` has type `&mut Vec<u8>`, which does not implement the `Copy` trait
89  | ) -> Result<(), Error> {
90  |     for (section_id, config_path) in section_ids_and_include_paths {
    |     -------------------------------------------------------------- inside of this loop
...
101 |         buf.clear();
    |         ^^^^^^^^^^^ value borrowed here after move
102 |         std::io::copy(&mut std::fs::File::open(&config_path)?, buf)?;
    |                                                                --- value moved here, in previous iteration of loop

