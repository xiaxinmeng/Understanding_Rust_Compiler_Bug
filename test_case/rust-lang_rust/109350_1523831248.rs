plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.91
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0277]: the trait bound `usize: ~const Ord` is not satisfied
   --> library/core/src/str/validations.rs:183:29
    |
183 |     let limit = bytes.len().min(4);
    |                             ^^^^^^ the trait `~const Ord` is not implemented for `usize`
    |
note: the trait `Ord` is implemented for `usize`, but that implementation is not `const`
   --> library/core/src/str/validations.rs:183:29
    |
183 |     let limit = bytes.len().min(4);


error[E0015]: cannot call non-const fn `<usize as Ord>::min` in constant functions
   --> library/core/src/str/validations.rs:183:29
    |
183 |     let limit = bytes.len().min(4);
    |
    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants


error[E0277]: the trait bound `[u8]: ~const Index<RangeFrom<usize>>` is not satisfied
   --> library/core/src/str/validations.rs:188:56
    |
188 |         if let Some((chr, len)) = try_next_code_point(&bytes[bytes.len() - n..]) {
    |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^ `[u8]` cannot be indexed by `RangeFrom<usize>`
    |
    = help: the trait `~const Index<RangeFrom<usize>>` is not implemented for `[u8]`
note: the trait `Index<RangeFrom<usize>>` is implemented for `[u8]`, but that implementation is not `const`
   --> library/core/src/str/validations.rs:188:56
    |
188 |         if let Some((chr, len)) = try_next_code_point(&bytes[bytes.len() - n..]) {

error[E0015]: cannot call non-const operator in constant functions
   --> library/core/src/str/validations.rs:188:56
    |
    |
188 |         if let Some((chr, len)) = try_next_code_point(&bytes[bytes.len() - n..]) {
    |
    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants


error[E0015]: cannot call non-const fn `slice::<impl [u8]>::get::<usize>` in constant functions
   --> library/core/src/str/validations.rs:270:25
268 | /     macro_rules! get {
268 | /     macro_rules! get {
269 | |         (raw $offset:expr) => {
270 | |             match bytes.get(index + $offset) {
    | |                         ^^^^^^^^^^^^^^^^^^^^
271 | |                 Some(byte) => *byte,
...   |
276 | |             let byte = get!(raw $offset);
...   |
281 | |         }}
282 | |     }
    | |     -
    | |     -
    | |     |
    | |_____in this expansion of `get!` (#1)
    |       in this expansion of `get!` (#2)
...
304 |               let second = get!(cont 1);
    |
    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants


error[E0015]: cannot call non-const fn `slice::<impl [u8]>::get::<usize>` in constant functions
   --> library/core/src/str/validations.rs:270:25
268 | /     macro_rules! get {
268 | /     macro_rules! get {
269 | |         (raw $offset:expr) => {
270 | |             match bytes.get(index + $offset) {
    | |                         ^^^^^^^^^^^^^^^^^^^^
271 | |                 Some(byte) => *byte,
281 | |         }}
282 | |     }
    | |_____- in this expansion of `get!`
...
...
310 |               let second = get!(raw 1);
    |
    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants


error[E0015]: cannot call non-const fn `slice::<impl [u8]>::get::<usize>` in constant functions
   --> library/core/src/str/validations.rs:270:25
268 | /     macro_rules! get {
268 | /     macro_rules! get {
269 | |         (raw $offset:expr) => {
270 | |             match bytes.get(index + $offset) {
    | |                         ^^^^^^^^^^^^^^^^^^^^
271 | |                 Some(byte) => *byte,
...   |
276 | |             let byte = get!(raw $offset);
...   |
281 | |         }}
282 | |     }
    | |     -
    | |     -
    | |     |
    | |_____in this expansion of `get!` (#1)
    |       in this expansion of `get!` (#2)
...
320 |               let value = utf8_acc_cont_byte(value, get!(cont 2));
    |
    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants


error[E0015]: cannot call non-const fn `slice::<impl [u8]>::get::<usize>` in constant functions
   --> library/core/src/str/validations.rs:270:25
268 | /     macro_rules! get {
268 | /     macro_rules! get {
269 | |         (raw $offset:expr) => {
270 | |             match bytes.get(index + $offset) {
    | |                         ^^^^^^^^^^^^^^^^^^^^
271 | |                 Some(byte) => *byte,
281 | |         }}
282 | |     }
    | |_____- in this expansion of `get!`
...
...
324 |               let second = get!(raw 1);
    |
    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants


error[E0015]: cannot call non-const fn `slice::<impl [u8]>::get::<usize>` in constant functions
   --> library/core/src/str/validations.rs:270:25
268 | /     macro_rules! get {
268 | /     macro_rules! get {
269 | |         (raw $offset:expr) => {
270 | |             match bytes.get(index + $offset) {
    | |                         ^^^^^^^^^^^^^^^^^^^^
271 | |                 Some(byte) => *byte,
...   |
276 | |             let byte = get!(raw $offset);
...   |
281 | |         }}
282 | |     }
    | |     -
    | |     -
    | |     |
    | |_____in this expansion of `get!` (#1)
    |       in this expansion of `get!` (#2)
...
331 |               let value = utf8_acc_cont_byte(value, get!(cont 2));
    |
    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants


error[E0015]: cannot call non-const fn `slice::<impl [u8]>::get::<usize>` in constant functions
   --> library/core/src/str/validations.rs:270:25
268 | /     macro_rules! get {
268 | /     macro_rules! get {
269 | |         (raw $offset:expr) => {
270 | |             match bytes.get(index + $offset) {
    | |                         ^^^^^^^^^^^^^^^^^^^^
271 | |                 Some(byte) => *byte,
...   |
276 | |             let byte = get!(raw $offset);
...   |
281 | |         }}
282 | |     }
    | |     -
    | |     -
    | |     |
    | |_____in this expansion of `get!` (#1)
    |       in this expansion of `get!` (#2)
...
332 |               let value = utf8_acc_cont_byte(value, get!(cont 3));
    |
    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants

Some errors have detailed explanations: E0015, E0277.
