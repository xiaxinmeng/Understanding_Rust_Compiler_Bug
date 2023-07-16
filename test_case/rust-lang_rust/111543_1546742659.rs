plain
   Compiling rand v0.8.5
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling core v0.0.0 (/checkout/library/core)
error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
    |
865 |     assert!(from_utf8(&[0xc0, 0x80]).is_err());
    |                       |
    |                       |
    |                       the literal was valid UTF-8 up to the 0 bytes
    |
    = note: `-D invalid-from-utf8` implied by `-D warnings`

error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
    |
866 |     assert!(from_utf8(&[0xc0, 0xae]).is_err());
    |                       |
    |                       |
    |                       the literal was valid UTF-8 up to the 0 bytes

error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
    |
867 |     assert!(from_utf8(&[0xe0, 0x80, 0x80]).is_err());
    |                       |
    |                       |
    |                       the literal was valid UTF-8 up to the 0 bytes

error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
    |
868 |     assert!(from_utf8(&[0xe0, 0x80, 0xaf]).is_err());
    |                       |
    |                       |
    |                       the literal was valid UTF-8 up to the 0 bytes

error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
    |
869 |     assert!(from_utf8(&[0xe0, 0x81, 0x81]).is_err());
    |                       |
    |                       |
    |                       the literal was valid UTF-8 up to the 0 bytes

error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
    |
870 |     assert!(from_utf8(&[0xf0, 0x82, 0x82, 0xac]).is_err());
    |                       |
    |                       |
    |                       the literal was valid UTF-8 up to the 0 bytes

error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
    |
871 |     assert!(from_utf8(&[0xf4, 0x90, 0x80, 0x80]).is_err());
    |                       |
    |                       |
    |                       the literal was valid UTF-8 up to the 0 bytes

error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
    |
874 |     assert!(from_utf8(&[0xED, 0xA0, 0x80]).is_err());
    |                       |
    |                       |
    |                       the literal was valid UTF-8 up to the 0 bytes

error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
    |
875 |     assert!(from_utf8(&[0xED, 0xBF, 0xBF]).is_err());
    |                       |
    |                       |
    |                       the literal was valid UTF-8 up to the 0 bytes

error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
    |
891 |         assert!(from_utf8(&[0xc0, 0x80]).is_err());
    |                           |
    |                           |
    |                           the literal was valid UTF-8 up to the 0 bytes

error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
    |
892 |         assert!(from_utf8(&[0xc0, 0xae]).is_err());
    |                           |
    |                           |
    |                           the literal was valid UTF-8 up to the 0 bytes

error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
    |
893 |         assert!(from_utf8(&[0xe0, 0x80, 0x80]).is_err());
    |                           |
    |                           |
    |                           the literal was valid UTF-8 up to the 0 bytes

error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
    |
894 |         assert!(from_utf8(&[0xe0, 0x80, 0xaf]).is_err());
    |                           |
    |                           |
    |                           the literal was valid UTF-8 up to the 0 bytes

error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
    |
895 |         assert!(from_utf8(&[0xe0, 0x81, 0x81]).is_err());
    |                           |
    |                           |
    |                           the literal was valid UTF-8 up to the 0 bytes

error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
    |
896 |         assert!(from_utf8(&[0xf0, 0x82, 0x82, 0xac]).is_err());
    |                           |
    |                           |
    |                           the literal was valid UTF-8 up to the 0 bytes

error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
    |
897 |         assert!(from_utf8(&[0xf4, 0x90, 0x80, 0x80]).is_err());
    |                           |
    |                           |
    |                           the literal was valid UTF-8 up to the 0 bytes

error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
    |
900 |         assert!(from_utf8(&[0xED, 0xA0, 0x80]).is_err());
    |                           |
    |                           |
    |                           the literal was valid UTF-8 up to the 0 bytes

error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
    |
901 |         assert!(from_utf8(&[0xED, 0xBF, 0xBF]).is_err());
    |                           |
    |                           |
    |                           the literal was valid UTF-8 up to the 0 bytes

error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
945 | /     macro_rules! test {
945 | /     macro_rules! test {
946 | |         ($input: expr, $expected_valid_up_to:pat, $expected_error_len:pat) => {
947 | |             let error = from_utf8($input).unwrap_err();
    | |                         ^^^^^^^^^^^^^^^^^
948 | |             assert_matches!(error.valid_up_to(), $expected_valid_up_to);
963 | |         };
964 | |     }
    | |_____- in this expansion of `test!`
    | |_____- in this expansion of `test!`
965 |       test!(b"A\xC3\xA9 \xFF ", 4, Some(1));
    |       |     |
    |       |     |
    |       |     the literal was valid UTF-8 up to the 4 bytes


error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
945 | /     macro_rules! test {
945 | /     macro_rules! test {
946 | |         ($input: expr, $expected_valid_up_to:pat, $expected_error_len:pat) => {
947 | |             let error = from_utf8($input).unwrap_err();
948 | |             assert_matches!(error.valid_up_to(), $expected_valid_up_to);
...   |
952 | |                 match from_utf8($input) {
...   |
963 | |         };
964 | |     }
    | |_____- in this expansion of `test!`
    | |_____- in this expansion of `test!`
965 |       test!(b"A\xC3\xA9 \xFF ", 4, Some(1));
    |       |     |
    |       |     |
    |       |     the literal was valid UTF-8 up to the 4 bytes


error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
945 | /     macro_rules! test {
945 | /     macro_rules! test {
946 | |         ($input: expr, $expected_valid_up_to:pat, $expected_error_len:pat) => {
947 | |             let error = from_utf8($input).unwrap_err();
    | |                         ^^^^^^^^^^^^^^^^^
948 | |             assert_matches!(error.valid_up_to(), $expected_valid_up_to);
963 | |         };
964 | |     }
    | |_____- in this expansion of `test!`
    | |_____- in this expansion of `test!`
965 |       test!(b"A\xC3\xA9 \xFF ", 4, Some(1));
966 |       test!(b"A\xC3\xA9 \x80 ", 4, Some(1));
    |       |     |
    |       |     |
    |       |     the literal was valid UTF-8 up to the 4 bytes


error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
945 | /     macro_rules! test {
945 | /     macro_rules! test {
946 | |         ($input: expr, $expected_valid_up_to:pat, $expected_error_len:pat) => {
947 | |             let error = from_utf8($input).unwrap_err();
948 | |             assert_matches!(error.valid_up_to(), $expected_valid_up_to);
...   |
952 | |                 match from_utf8($input) {
...   |
963 | |         };
964 | |     }
    | |_____- in this expansion of `test!`
    | |_____- in this expansion of `test!`
965 |       test!(b"A\xC3\xA9 \xFF ", 4, Some(1));
966 |       test!(b"A\xC3\xA9 \x80 ", 4, Some(1));
    |       |     |
    |       |     |
    |       |     the literal was valid UTF-8 up to the 4 bytes


error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
945 | /     macro_rules! test {
945 | /     macro_rules! test {
946 | |         ($input: expr, $expected_valid_up_to:pat, $expected_error_len:pat) => {
947 | |             let error = from_utf8($input).unwrap_err();
    | |                         ^^^^^^^^^^^^^^^^^
948 | |             assert_matches!(error.valid_up_to(), $expected_valid_up_to);
963 | |         };
964 | |     }
    | |_____- in this expansion of `test!`
...
...
967 |       test!(b"A\xC3\xA9 \xC1 ", 4, Some(1));
    |       |     |
    |       |     |
    |       |     the literal was valid UTF-8 up to the 4 bytes


error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
945 | /     macro_rules! test {
945 | /     macro_rules! test {
946 | |         ($input: expr, $expected_valid_up_to:pat, $expected_error_len:pat) => {
947 | |             let error = from_utf8($input).unwrap_err();
948 | |             assert_matches!(error.valid_up_to(), $expected_valid_up_to);
...   |
952 | |                 match from_utf8($input) {
...   |
963 | |         };
964 | |     }
    | |_____- in this expansion of `test!`
    | |_____- in this expansion of `test!`
...
967 |       test!(b"A\xC3\xA9 \xC1 ", 4, Some(1));
    |       |     |
    |       |     |
    |       |     the literal was valid UTF-8 up to the 4 bytes


error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
945 | /     macro_rules! test {
945 | /     macro_rules! test {
946 | |         ($input: expr, $expected_valid_up_to:pat, $expected_error_len:pat) => {
947 | |             let error = from_utf8($input).unwrap_err();
    | |                         ^^^^^^^^^^^^^^^^^
948 | |             assert_matches!(error.valid_up_to(), $expected_valid_up_to);
963 | |         };
964 | |     }
    | |_____- in this expansion of `test!`
...
...
968 |       test!(b"A\xC3\xA9 \xC1", 4, Some(1));
    |       |     |
    |       |     |
    |       |     the literal was valid UTF-8 up to the 4 bytes


error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
945 | /     macro_rules! test {
945 | /     macro_rules! test {
946 | |         ($input: expr, $expected_valid_up_to:pat, $expected_error_len:pat) => {
947 | |             let error = from_utf8($input).unwrap_err();
948 | |             assert_matches!(error.valid_up_to(), $expected_valid_up_to);
...   |
952 | |                 match from_utf8($input) {
...   |
963 | |         };
964 | |     }
    | |_____- in this expansion of `test!`
    | |_____- in this expansion of `test!`
...
968 |       test!(b"A\xC3\xA9 \xC1", 4, Some(1));
    |       |     |
    |       |     |
    |       |     the literal was valid UTF-8 up to the 4 bytes


error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
945 | /     macro_rules! test {
945 | /     macro_rules! test {
946 | |         ($input: expr, $expected_valid_up_to:pat, $expected_error_len:pat) => {
947 | |             let error = from_utf8($input).unwrap_err();
    | |                         ^^^^^^^^^^^^^^^^^
948 | |             assert_matches!(error.valid_up_to(), $expected_valid_up_to);
963 | |         };
964 | |     }
    | |_____- in this expansion of `test!`
...
...
969 |       test!(b"A\xC3\xA9 \xC2", 4, None);
    |       |     |
    |       |     |
    |       |     the literal was valid UTF-8 up to the 4 bytes


error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
945 | /     macro_rules! test {
945 | /     macro_rules! test {
946 | |         ($input: expr, $expected_valid_up_to:pat, $expected_error_len:pat) => {
947 | |             let error = from_utf8($input).unwrap_err();
948 | |             assert_matches!(error.valid_up_to(), $expected_valid_up_to);
...   |
952 | |                 match from_utf8($input) {
...   |
963 | |         };
964 | |     }
    | |_____- in this expansion of `test!`
    | |_____- in this expansion of `test!`
...
969 |       test!(b"A\xC3\xA9 \xC2", 4, None);
    |       |     |
    |       |     |
    |       |     the literal was valid UTF-8 up to the 4 bytes


error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
945 | /     macro_rules! test {
945 | /     macro_rules! test {
946 | |         ($input: expr, $expected_valid_up_to:pat, $expected_error_len:pat) => {
947 | |             let error = from_utf8($input).unwrap_err();
    | |                         ^^^^^^^^^^^^^^^^^
948 | |             assert_matches!(error.valid_up_to(), $expected_valid_up_to);
963 | |         };
964 | |     }
    | |_____- in this expansion of `test!`
...
...
970 |       test!(b"A\xC3\xA9 \xC2 ", 4, Some(1));
    |       |     |
    |       |     |
    |       |     the literal was valid UTF-8 up to the 4 bytes


error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
945 | /     macro_rules! test {
945 | /     macro_rules! test {
946 | |         ($input: expr, $expected_valid_up_to:pat, $expected_error_len:pat) => {
947 | |             let error = from_utf8($input).unwrap_err();
948 | |             assert_matches!(error.valid_up_to(), $expected_valid_up_to);
...   |
952 | |                 match from_utf8($input) {
...   |
963 | |         };
964 | |     }
    | |_____- in this expansion of `test!`
    | |_____- in this expansion of `test!`
...
970 |       test!(b"A\xC3\xA9 \xC2 ", 4, Some(1));
    |       |     |
    |       |     |
    |       |     the literal was valid UTF-8 up to the 4 bytes


error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
945 | /     macro_rules! test {
945 | /     macro_rules! test {
946 | |         ($input: expr, $expected_valid_up_to:pat, $expected_error_len:pat) => {
947 | |             let error = from_utf8($input).unwrap_err();
    | |                         ^^^^^^^^^^^^^^^^^
948 | |             assert_matches!(error.valid_up_to(), $expected_valid_up_to);
963 | |         };
964 | |     }
    | |_____- in this expansion of `test!`
...
...
971 |       test!(b"A\xC3\xA9 \xC2\xC0", 4, Some(1));
    |       |     |
    |       |     |
    |       |     the literal was valid UTF-8 up to the 4 bytes


error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
945 | /     macro_rules! test {
945 | /     macro_rules! test {
946 | |         ($input: expr, $expected_valid_up_to:pat, $expected_error_len:pat) => {
947 | |             let error = from_utf8($input).unwrap_err();
948 | |             assert_matches!(error.valid_up_to(), $expected_valid_up_to);
...   |
952 | |                 match from_utf8($input) {
...   |
963 | |         };
964 | |     }
    | |_____- in this expansion of `test!`
    | |_____- in this expansion of `test!`
...
971 |       test!(b"A\xC3\xA9 \xC2\xC0", 4, Some(1));
    |       |     |
    |       |     |
    |       |     the literal was valid UTF-8 up to the 4 bytes


error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
945 | /     macro_rules! test {
945 | /     macro_rules! test {
946 | |         ($input: expr, $expected_valid_up_to:pat, $expected_error_len:pat) => {
947 | |             let error = from_utf8($input).unwrap_err();
    | |                         ^^^^^^^^^^^^^^^^^
948 | |             assert_matches!(error.valid_up_to(), $expected_valid_up_to);
963 | |         };
964 | |     }
    | |_____- in this expansion of `test!`
...
...
972 |       test!(b"A\xC3\xA9 \xE0", 4, None);
    |       |     |
    |       |     |
    |       |     the literal was valid UTF-8 up to the 4 bytes


error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
945 | /     macro_rules! test {
945 | /     macro_rules! test {
946 | |         ($input: expr, $expected_valid_up_to:pat, $expected_error_len:pat) => {
947 | |             let error = from_utf8($input).unwrap_err();
948 | |             assert_matches!(error.valid_up_to(), $expected_valid_up_to);
...   |
952 | |                 match from_utf8($input) {
...   |
963 | |         };
964 | |     }
    | |_____- in this expansion of `test!`
    | |_____- in this expansion of `test!`
...
972 |       test!(b"A\xC3\xA9 \xE0", 4, None);
    |       |     |
    |       |     |
    |       |     the literal was valid UTF-8 up to the 4 bytes


error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
945 | /     macro_rules! test {
945 | /     macro_rules! test {
946 | |         ($input: expr, $expected_valid_up_to:pat, $expected_error_len:pat) => {
947 | |             let error = from_utf8($input).unwrap_err();
    | |                         ^^^^^^^^^^^^^^^^^
948 | |             assert_matches!(error.valid_up_to(), $expected_valid_up_to);
963 | |         };
964 | |     }
    | |_____- in this expansion of `test!`
...
...
973 |       test!(b"A\xC3\xA9 \xE0\x9F", 4, Some(1));
    |       |     |
    |       |     |
    |       |     the literal was valid UTF-8 up to the 4 bytes


error: calls to `std::str::from_utf8` with a invalid literal -> always return an error
    |
945 | /     macro_rules! test {
945 | /     macro_rules! test {
946 | |         ($input: expr, $expected_valid_up_to:pat, $expected_error_len:pat) => {
947 | |             let error = from_utf8($input).unwrap_err();
948 | |             assert_matches!(error.valid_up_to(), $expected_valid_up_to);
...   |
952 | |                 match from_utf8($input) {
...   |
963 | |         };
964 | |     }
    | |_____- in this expansion of `test!`
