plain
   Compiling difference v2.0.0
   Compiling expect-test v1.0.1
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0061]: this function takes 4 arguments but 5 arguments were supplied
   --> src/librustdoc/html/highlight/tests.rs:26:13
    |
26  |             write_code(&mut out, src, Edition::Edition2018, None, None);
    |             ^^^^^^^^^^                -------------------- argument of type `Edition` unexpected
note: function defined here
   --> src/librustdoc/html/highlight.rs:125:4
    |
125 | fn write_code(
125 | fn write_code(
    |    ^^^^^^^^^^
126 |     out: &mut Buffer,
    |     ----------------
127 |     src: &str,
    |     ---------
128 |     href_context: Option<HrefContext<'_, '_, '_>>,
129 |     decoration_info: Option<DecorationInfo>,
    |     ---------------------------------------
help: remove the extra argument
    |
    |
26  |             write_code(&mut out, src, None, None);

error[E0061]: this function takes 4 arguments but 5 arguments were supplied
error[E0061]: this function takes 4 arguments but 5 arguments were supplied
   --> src/librustdoc/html/highlight/tests.rs:40:9
    |
40  |         write_code(&mut html, src, Edition::Edition2018, None, None);
    |         ^^^^^^^^^^                 -------------------- argument of type `Edition` unexpected
note: function defined here
   --> src/librustdoc/html/highlight.rs:125:4
    |
125 | fn write_code(
125 | fn write_code(
    |    ^^^^^^^^^^
126 |     out: &mut Buffer,
    |     ----------------
127 |     src: &str,
    |     ---------
128 |     href_context: Option<HrefContext<'_, '_, '_>>,
129 |     decoration_info: Option<DecorationInfo>,
    |     ---------------------------------------
help: remove the extra argument
    |
    |
40  |         write_code(&mut html, src, None, None);
    |         ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

error[E0061]: this function takes 4 arguments but 5 arguments were supplied
   --> src/librustdoc/html/highlight/tests.rs:54:9
    |
54  |         write_code(&mut html, src, Edition::Edition2018, None, None);
    |         ^^^^^^^^^^                 -------------------- argument of type `Edition` unexpected
note: function defined here
   --> src/librustdoc/html/highlight.rs:125:4
    |
125 | fn write_code(
125 | fn write_code(
    |    ^^^^^^^^^^
126 |     out: &mut Buffer,
    |     ----------------
127 |     src: &str,
    |     ---------
128 |     href_context: Option<HrefContext<'_, '_, '_>>,
129 |     decoration_info: Option<DecorationInfo>,
    |     ---------------------------------------
help: remove the extra argument
    |
    |
54  |         write_code(&mut html, src, None, None);
    |         ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

error[E0061]: this function takes 4 arguments but 5 arguments were supplied
   --> src/librustdoc/html/highlight/tests.rs:64:9
    |
64  |         write_code(&mut html, src, Edition::Edition2018, None, None);
    |         ^^^^^^^^^^                 -------------------- argument of type `Edition` unexpected
note: function defined here
   --> src/librustdoc/html/highlight.rs:125:4
    |
125 | fn write_code(
125 | fn write_code(
    |    ^^^^^^^^^^
126 |     out: &mut Buffer,
    |     ----------------
127 |     src: &str,
    |     ---------
128 |     href_context: Option<HrefContext<'_, '_, '_>>,
129 |     decoration_info: Option<DecorationInfo>,
    |     ---------------------------------------
help: remove the extra argument
    |
    |
64  |         write_code(&mut html, src, None, None);
    |         ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

error[E0061]: this function takes 4 arguments but 5 arguments were supplied
   --> src/librustdoc/html/highlight/tests.rs:78:9
    |
78  |         write_code(&mut html, src, Edition::Edition2018, None, Some(DecorationInfo(decorations)));
    |         ^^^^^^^^^^                 --------------------        --------------------------------- argument of type `std::option::Option<DecorationInfo>` unexpected
    |                                    expected enum `std::option::Option`, found enum `Edition`
    |
    |
    = note: expected enum `std::option::Option<HrefContext<'_, '_, '_>>`
               found enum `Edition`
   --> src/librustdoc/html/highlight.rs:125:4
    |
125 | fn write_code(
    |    ^^^^^^^^^^
    |    ^^^^^^^^^^
126 |     out: &mut Buffer,
    |     ----------------
127 |     src: &str,
    |     ---------
128 |     href_context: Option<HrefContext<'_, '_, '_>>,
129 |     decoration_info: Option<DecorationInfo>,
    |     ---------------------------------------
help: remove the extra argument
    |
    |
78  |         write_code(&mut html, src, /* std::option::Option<HrefContext<'_, '_, '_>> */, None);

For more information about this error, try `rustc --explain E0061`.
error: could not compile `rustdoc` due to 5 previous errors
 finished in 9.119 seconds
