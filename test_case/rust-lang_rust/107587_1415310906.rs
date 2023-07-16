plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:055e3b93d15803815fe6f9cbc1b02b11be094e54)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
   Compiling rand_xorshift v0.3.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling alloc v0.0.0 (/checkout/library/alloc)
error: use of deprecated constant `std::f32::NAN`: replaced by the `NAN` associated constant on `f32`
    |
    |
657 |     let array3: [f32; 3] = [1.0, std::f32::NAN, 3.0];
    |
    |
    = note: `-D deprecated` implied by `-D warnings`

error: use of deprecated module `std::i128`: all constants in this module replaced by associated constants on `i128`
 --> library/core/tests/num/i128.rs:1:13
  |
1 | int_module!(i128);


error: use of deprecated constant `std::i128::MAX`: replaced by the `MAX` associated constant on this type
   --> library/core/tests/num/int_macros.rs:12:25
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
12  | |                 assert!(MAX > 0);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MIN`: replaced by the `MIN` associated constant on this type
   --> library/core/tests/num/int_macros.rs:13:25
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
13  | |                 assert!(MIN <= 0);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MIN`: replaced by the `MIN` associated constant on this type
   --> library/core/tests/num/int_macros.rs:14:28
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
14  | |                 assert_eq!(MIN + MAX + 1, 0);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MAX`: replaced by the `MAX` associated constant on this type
   --> library/core/tests/num/int_macros.rs:14:34
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
14  | |                 assert_eq!(MIN + MAX + 1, 0);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MIN`: replaced by the `MIN` associated constant on this type
   --> library/core/tests/num/int_macros.rs:24:50
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
24  | |                 assert_eq!((-1 as $T).rem_euclid(MIN), MAX);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MAX`: replaced by the `MAX` associated constant on this type
   --> library/core/tests/num/int_macros.rs:24:56
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
24  | |                 assert_eq!((-1 as $T).rem_euclid(MIN), MAX);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MAX`: replaced by the `MAX` associated constant on this type
   --> library/core/tests/num/int_macros.rs:101:28
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
101 | |                 assert_eq!(MAX.leading_ones(), 0);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MAX`: replaced by the `MAX` associated constant on this type
   --> library/core/tests/num/int_macros.rs:104:28
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
104 | |                 assert_eq!(MAX.trailing_ones(), $T::BITS - 1);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MAX`: replaced by the `MAX` associated constant on this type
   --> library/core/tests/num/int_macros.rs:184:29
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
184 | |                 assert_eq!((MAX - 2).saturating_abs(), MAX - 2);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MAX`: replaced by the `MAX` associated constant on this type
   --> library/core/tests/num/int_macros.rs:184:56
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
184 | |                 assert_eq!((MAX - 2).saturating_abs(), MAX - 2);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MAX`: replaced by the `MAX` associated constant on this type
   --> library/core/tests/num/int_macros.rs:185:29
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
185 | |                 assert_eq!((MAX - 1).saturating_abs(), MAX - 1);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MAX`: replaced by the `MAX` associated constant on this type
   --> library/core/tests/num/int_macros.rs:185:56
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
185 | |                 assert_eq!((MAX - 1).saturating_abs(), MAX - 1);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MAX`: replaced by the `MAX` associated constant on this type
   --> library/core/tests/num/int_macros.rs:186:28
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
186 | |                 assert_eq!(MAX.saturating_abs(), MAX);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MAX`: replaced by the `MAX` associated constant on this type
   --> library/core/tests/num/int_macros.rs:186:50
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
186 | |                 assert_eq!(MAX.saturating_abs(), MAX);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MIN`: replaced by the `MIN` associated constant on this type
   --> library/core/tests/num/int_macros.rs:187:29
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
187 | |                 assert_eq!((MIN + 2).saturating_abs(), MAX - 1);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MAX`: replaced by the `MAX` associated constant on this type
   --> library/core/tests/num/int_macros.rs:187:56
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
187 | |                 assert_eq!((MIN + 2).saturating_abs(), MAX - 1);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MIN`: replaced by the `MIN` associated constant on this type
   --> library/core/tests/num/int_macros.rs:188:29
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
188 | |                 assert_eq!((MIN + 1).saturating_abs(), MAX);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MAX`: replaced by the `MAX` associated constant on this type
   --> library/core/tests/num/int_macros.rs:188:56
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
188 | |                 assert_eq!((MIN + 1).saturating_abs(), MAX);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MIN`: replaced by the `MIN` associated constant on this type
   --> library/core/tests/num/int_macros.rs:189:28
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
189 | |                 assert_eq!(MIN.saturating_abs(), MAX);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MAX`: replaced by the `MAX` associated constant on this type
   --> library/core/tests/num/int_macros.rs:189:50
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
189 | |                 assert_eq!(MIN.saturating_abs(), MAX);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MAX`: replaced by the `MAX` associated constant on this type
   --> library/core/tests/num/int_macros.rs:197:29
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
197 | |                 assert_eq!((MAX - 2).saturating_neg(), MIN + 3);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MIN`: replaced by the `MIN` associated constant on this type
   --> library/core/tests/num/int_macros.rs:197:56
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
197 | |                 assert_eq!((MAX - 2).saturating_neg(), MIN + 3);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MAX`: replaced by the `MAX` associated constant on this type
   --> library/core/tests/num/int_macros.rs:198:29
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
198 | |                 assert_eq!((MAX - 1).saturating_neg(), MIN + 2);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MIN`: replaced by the `MIN` associated constant on this type
   --> library/core/tests/num/int_macros.rs:198:56
1   | / macro_rules! int_module {
1   | / macro_rules! int_module {
2   | |     ($T:ident) => {
3   | |         #[cfg(test)]
4   | |         mod tests {
...   |
198 | |                 assert_eq!((MAX - 1).saturating_neg(), MIN + 2);
...   |
368 | |     };
369 | | }
    | |_- in this expansion of `int_module!`
    | |_- in this expansion of `int_module!`
    |
   ::: library/core/tests/num/i128.rs:1:1
    |
1   |   int_module!(i128);


error: use of deprecated constant `std::i128::MAX`: replaced by the `MAX` associated constant on this type
   --> library/core/tests/num/int_macros.rs:199:28
