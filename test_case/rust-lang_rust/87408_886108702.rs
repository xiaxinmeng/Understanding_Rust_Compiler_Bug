plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking core v0.0.0 (/checkout/library/core)
error[E0308]: mismatched types
    --> library/alloc/tests/vec.rs:1546:20
     |
1546 |         if let Err(CapacityOverflow) = ten_bytes.try_reserve(MAX_USIZE) {
     |                    ^^^^^^^^^^^^^^^^    -------------------------------- this expression has type `Result<(), TryReserveError>`
     |                    |
     |                    expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1546 |         if let Err(CapacityOverflow) = ten_bytes.try_reserve(MAX_USIZE).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1174:20
     |
     |
1174 |         if let Err(CapacityOverflow) = empty_bytes.try_reserve(MAX_CAP) {
     |                    ^^^^^^^^^^^^^^^^    -------------------------------- this expression has type `Result<(), TryReserveError>`
     |                    |
     |                    expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1174 |         if let Err(CapacityOverflow) = empty_bytes.try_reserve(MAX_CAP).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1178:20
     |
     |
1178 |         if let Err(CapacityOverflow) = empty_bytes.try_reserve(MAX_CAP) {
     |                    ^^^^^^^^^^^^^^^^    -------------------------------- this expression has type `Result<(), TryReserveError>`
     |                    |
     |                    expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1178 |         if let Err(CapacityOverflow) = empty_bytes.try_reserve(MAX_CAP).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1184:24
     |
     |
1184 |             if let Err(CapacityOverflow) = empty_bytes.try_reserve(MAX_CAP + 1) {
     |                        ^^^^^^^^^^^^^^^^    ------------------------------------ this expression has type `Result<(), TryReserveError>`
     |                        |
     |                        expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1184 |             if let Err(CapacityOverflow) = empty_bytes.try_reserve(MAX_CAP + 1).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1190:24
     |
     |
1190 |             if let Err(CapacityOverflow) = empty_bytes.try_reserve(MAX_USIZE) {
     |                        ^^^^^^^^^^^^^^^^    ---------------------------------- this expression has type `Result<(), TryReserveError>`
     |                        |
     |                        expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1190 |             if let Err(CapacityOverflow) = empty_bytes.try_reserve(MAX_USIZE).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1199:24
     |
     |
1199 |             if let Err(AllocError { .. }) = empty_bytes.try_reserve(MAX_CAP) {
     |                        ^^^^^^^^^^^^^^^^^    -------------------------------- this expression has type `Result<(), TryReserveError>`
     |                        |
     |                        expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1199 |             if let Err(AllocError { .. }) = empty_bytes.try_reserve(MAX_CAP).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1210:20
     |
     |
1210 |         if let Err(CapacityOverflow) = ten_bytes.try_reserve(MAX_CAP - 10) {
     |                    ^^^^^^^^^^^^^^^^    ----------------------------------- this expression has type `Result<(), TryReserveError>`
     |                    |
     |                    expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1210 |         if let Err(CapacityOverflow) = ten_bytes.try_reserve(MAX_CAP - 10).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1213:20
     |
     |
1213 |         if let Err(CapacityOverflow) = ten_bytes.try_reserve(MAX_CAP - 10) {
     |                    ^^^^^^^^^^^^^^^^    ----------------------------------- this expression has type `Result<(), TryReserveError>`
     |                    |
     |                    expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1213 |         if let Err(CapacityOverflow) = ten_bytes.try_reserve(MAX_CAP - 10).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1217:24
     |
     |
1217 |             if let Err(CapacityOverflow) = ten_bytes.try_reserve(MAX_CAP - 9) {
     |                        ^^^^^^^^^^^^^^^^    ---------------------------------- this expression has type `Result<(), TryReserveError>`
     |                        |
     |                        expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1217 |             if let Err(CapacityOverflow) = ten_bytes.try_reserve(MAX_CAP - 9).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1222:24
     |
     |
1222 |             if let Err(AllocError { .. }) = ten_bytes.try_reserve(MAX_CAP - 9) {
     |                        ^^^^^^^^^^^^^^^^^    ---------------------------------- this expression has type `Result<(), TryReserveError>`
     |                        |
     |                        expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1222 |             if let Err(AllocError { .. }) = ten_bytes.try_reserve(MAX_CAP - 9).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1228:20
     |
     |
1228 |         if let Err(CapacityOverflow) = ten_bytes.try_reserve(MAX_USIZE) {
     |                    ^^^^^^^^^^^^^^^^    -------------------------------- this expression has type `Result<(), TryReserveError>`
     |                    |
     |                    expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1228 |         if let Err(CapacityOverflow) = ten_bytes.try_reserve(MAX_USIZE).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1238:20
     |
     |
1238 |         if let Err(CapacityOverflow) = ten_u32s.try_reserve(MAX_CAP / 4 - 10) {
     |                    ^^^^^^^^^^^^^^^^    -------------------------------------- this expression has type `Result<(), TryReserveError>`
     |                    |
     |                    expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1238 |         if let Err(CapacityOverflow) = ten_u32s.try_reserve(MAX_CAP / 4 - 10).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1241:20
     |
     |
1241 |         if let Err(CapacityOverflow) = ten_u32s.try_reserve(MAX_CAP / 4 - 10) {
     |                    ^^^^^^^^^^^^^^^^    -------------------------------------- this expression has type `Result<(), TryReserveError>`
     |                    |
     |                    expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1241 |         if let Err(CapacityOverflow) = ten_u32s.try_reserve(MAX_CAP / 4 - 10).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1245:24
     |
     |
1245 |             if let Err(CapacityOverflow) = ten_u32s.try_reserve(MAX_CAP / 4 - 9) {
     |                        ^^^^^^^^^^^^^^^^    ------------------------------------- this expression has type `Result<(), TryReserveError>`
     |                        |
     |                        expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1245 |             if let Err(CapacityOverflow) = ten_u32s.try_reserve(MAX_CAP / 4 - 9).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1250:24
     |
     |
1250 |             if let Err(AllocError { .. }) = ten_u32s.try_reserve(MAX_CAP / 4 - 9) {
     |                        ^^^^^^^^^^^^^^^^^    ------------------------------------- this expression has type `Result<(), TryReserveError>`
     |                        |
     |                        expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1250 |             if let Err(AllocError { .. }) = ten_u32s.try_reserve(MAX_CAP / 4 - 9).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1256:20
     |
     |
1256 |         if let Err(CapacityOverflow) = ten_u32s.try_reserve(MAX_USIZE - 20) {
     |                    ^^^^^^^^^^^^^^^^    ------------------------------------ this expression has type `Result<(), TryReserveError>`
     |                    |
     |                    expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1256 |         if let Err(CapacityOverflow) = ten_u32s.try_reserve(MAX_USIZE - 20).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1278:20
     |
     |
1278 |         if let Err(CapacityOverflow) = empty_bytes.try_reserve_exact(MAX_CAP) {
     |                    ^^^^^^^^^^^^^^^^    -------------------------------------- this expression has type `Result<(), TryReserveError>`
     |                    |
     |                    expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1278 |         if let Err(CapacityOverflow) = empty_bytes.try_reserve_exact(MAX_CAP).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1281:20
     |
     |
1281 |         if let Err(CapacityOverflow) = empty_bytes.try_reserve_exact(MAX_CAP) {
     |                    ^^^^^^^^^^^^^^^^    -------------------------------------- this expression has type `Result<(), TryReserveError>`
     |                    |
     |                    expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1281 |         if let Err(CapacityOverflow) = empty_bytes.try_reserve_exact(MAX_CAP).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1286:24
     |
     |
1286 |             if let Err(CapacityOverflow) = empty_bytes.try_reserve_exact(MAX_CAP + 1) {
     |                        ^^^^^^^^^^^^^^^^    ------------------------------------------ this expression has type `Result<(), TryReserveError>`
     |                        |
     |                        expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1286 |             if let Err(CapacityOverflow) = empty_bytes.try_reserve_exact(MAX_CAP + 1).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1291:24
     |
     |
1291 |             if let Err(CapacityOverflow) = empty_bytes.try_reserve_exact(MAX_USIZE) {
     |                        ^^^^^^^^^^^^^^^^    ---------------------------------------- this expression has type `Result<(), TryReserveError>`
     |                        |
     |                        expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1291 |             if let Err(CapacityOverflow) = empty_bytes.try_reserve_exact(MAX_USIZE).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1300:24
     |
     |
1300 |             if let Err(AllocError { .. }) = empty_bytes.try_reserve_exact(MAX_CAP) {
     |                        ^^^^^^^^^^^^^^^^^    -------------------------------------- this expression has type `Result<(), TryReserveError>`
     |                        |
     |                        expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1300 |             if let Err(AllocError { .. }) = empty_bytes.try_reserve_exact(MAX_CAP).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1310:20
     |
     |
1310 |         if let Err(CapacityOverflow) = ten_bytes.try_reserve_exact(MAX_CAP - 10) {
     |                    ^^^^^^^^^^^^^^^^    ----------------------------------------- this expression has type `Result<(), TryReserveError>`
     |                    |
     |                    expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1310 |         if let Err(CapacityOverflow) = ten_bytes.try_reserve_exact(MAX_CAP - 10).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1313:20
     |
     |
1313 |         if let Err(CapacityOverflow) = ten_bytes.try_reserve_exact(MAX_CAP - 10) {
     |                    ^^^^^^^^^^^^^^^^    ----------------------------------------- this expression has type `Result<(), TryReserveError>`
     |                    |
     |                    expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1313 |         if let Err(CapacityOverflow) = ten_bytes.try_reserve_exact(MAX_CAP - 10).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1317:24
     |
     |
1317 |             if let Err(CapacityOverflow) = ten_bytes.try_reserve_exact(MAX_CAP - 9) {
     |                        ^^^^^^^^^^^^^^^^    ---------------------------------------- this expression has type `Result<(), TryReserveError>`
     |                        |
     |                        expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1317 |             if let Err(CapacityOverflow) = ten_bytes.try_reserve_exact(MAX_CAP - 9).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1322:24
     |
     |
1322 |             if let Err(AllocError { .. }) = ten_bytes.try_reserve_exact(MAX_CAP - 9) {
     |                        ^^^^^^^^^^^^^^^^^    ---------------------------------------- this expression has type `Result<(), TryReserveError>`
     |                        |
     |                        expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1322 |             if let Err(AllocError { .. }) = ten_bytes.try_reserve_exact(MAX_CAP - 9).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1327:20
     |
     |
1327 |         if let Err(CapacityOverflow) = ten_bytes.try_reserve_exact(MAX_USIZE) {
     |                    ^^^^^^^^^^^^^^^^    -------------------------------------- this expression has type `Result<(), TryReserveError>`
     |                    |
     |                    expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1327 |         if let Err(CapacityOverflow) = ten_bytes.try_reserve_exact(MAX_USIZE).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1336:20
     |
     |
1336 |         if let Err(CapacityOverflow) = ten_u32s.try_reserve_exact(MAX_CAP / 4 - 10) {
     |                    ^^^^^^^^^^^^^^^^    -------------------------------------------- this expression has type `Result<(), TryReserveError>`
     |                    |
     |                    expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1336 |         if let Err(CapacityOverflow) = ten_u32s.try_reserve_exact(MAX_CAP / 4 - 10).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1339:20
     |
     |
1339 |         if let Err(CapacityOverflow) = ten_u32s.try_reserve_exact(MAX_CAP / 4 - 10) {
     |                    ^^^^^^^^^^^^^^^^    -------------------------------------------- this expression has type `Result<(), TryReserveError>`
     |                    |
     |                    expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1339 |         if let Err(CapacityOverflow) = ten_u32s.try_reserve_exact(MAX_CAP / 4 - 10).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1343:24
     |
     |
1343 |             if let Err(CapacityOverflow) = ten_u32s.try_reserve_exact(MAX_CAP / 4 - 9) {
     |                        ^^^^^^^^^^^^^^^^    ------------------------------------------- this expression has type `Result<(), TryReserveError>`
     |                        |
     |                        expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1343 |             if let Err(CapacityOverflow) = ten_u32s.try_reserve_exact(MAX_CAP / 4 - 9).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1348:24
     |
     |
1348 |             if let Err(AllocError { .. }) = ten_u32s.try_reserve_exact(MAX_CAP / 4 - 9) {
     |                        ^^^^^^^^^^^^^^^^^    ------------------------------------------- this expression has type `Result<(), TryReserveError>`
     |                        |
     |                        expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1348 |             if let Err(AllocError { .. }) = ten_u32s.try_reserve_exact(MAX_CAP / 4 - 9).kind {

error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1353:20
     |
     |
1353 |         if let Err(CapacityOverflow) = ten_u32s.try_reserve_exact(MAX_USIZE - 20) {
     |                    ^^^^^^^^^^^^^^^^    ------------------------------------------ this expression has type `Result<(), TryReserveError>`
     |                    |
     |                    expected struct `TryReserveError`, found enum `TryReserveErrorKind`
     |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
     |
1353 |         if let Err(CapacityOverflow) = ten_u32s.try_reserve_exact(MAX_USIZE - 20).kind {

error: aborting due to 31 previous errors

For more information about this error, try `rustc --explain E0308`.
