plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: macro has missing stability attribute
    --> library/core/src/mem/mod.rs:1213:1
     |
1213 | / macro_rules! type_info {
1214 | |     (<$ty:ty>) => {
1215 | |         format!(
1216 | |             "{}: size {}, alignment {}",
1241 | |     }};
1242 | | }
     | |_^

