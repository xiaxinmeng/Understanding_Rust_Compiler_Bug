plain
   Compiling once_cell v1.7.2
   Compiling difference v2.0.0
   Compiling expect-test v1.0.1
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:12:5
   |
11 |     let buf = HtmlWithLimit::new(60);
   |         --- help: consider changing this to be mutable: `mut buf`
12 |     buf.push("Hello ");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:13:5
   |
11 |     let buf = HtmlWithLimit::new(60);
   |         --- help: consider changing this to be mutable: `mut buf`
12 |     buf.push("Hello ");
13 |     buf.open_tag("em");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:14:5
   |
11 |     let buf = HtmlWithLimit::new(60);
   |         --- help: consider changing this to be mutable: `mut buf`
...
14 |     buf.push("world");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:15:5
   |
11 |     let buf = HtmlWithLimit::new(60);
   |         --- help: consider changing this to be mutable: `mut buf`
...
15 |     buf.close_tag();
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:16:5
   |
11 |     let buf = HtmlWithLimit::new(60);
   |         --- help: consider changing this to be mutable: `mut buf`
...
16 |     buf.push("!");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:23:5
   |
22 |     let buf = HtmlWithLimit::new(60);
   |         --- help: consider changing this to be mutable: `mut buf`
23 |     buf.push("Hello");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:24:5
   |
22 |     let buf = HtmlWithLimit::new(60);
   |         --- help: consider changing this to be mutable: `mut buf`
23 |     buf.push("Hello");
24 |     buf.push(" world!");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:31:5
   |
30 |     let buf = HtmlWithLimit::new(0);
   |         --- help: consider changing this to be mutable: `mut buf`
31 |     buf.push("Hello ");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:32:5
   |
30 |     let buf = HtmlWithLimit::new(0);
   |         --- help: consider changing this to be mutable: `mut buf`
31 |     buf.push("Hello ");
32 |     buf.open_tag("em");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:33:5
   |
30 |     let buf = HtmlWithLimit::new(0);
   |         --- help: consider changing this to be mutable: `mut buf`
...
33 |     buf.push("world");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:34:5
   |
30 |     let buf = HtmlWithLimit::new(0);
   |         --- help: consider changing this to be mutable: `mut buf`
...
34 |     buf.close_tag();
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:35:5
   |
30 |     let buf = HtmlWithLimit::new(0);
   |         --- help: consider changing this to be mutable: `mut buf`
...
35 |     buf.push("!");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:42:5
   |
41 |     let buf = HtmlWithLimit::new(12);
   |         --- help: consider changing this to be mutable: `mut buf`
42 |     buf.push("Hello ");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:43:5
   |
41 |     let buf = HtmlWithLimit::new(12);
   |         --- help: consider changing this to be mutable: `mut buf`
42 |     buf.push("Hello ");
43 |     buf.open_tag("em");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:44:5
   |
41 |     let buf = HtmlWithLimit::new(12);
   |         --- help: consider changing this to be mutable: `mut buf`
...
44 |     buf.push("world");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:45:5
   |
41 |     let buf = HtmlWithLimit::new(12);
   |         --- help: consider changing this to be mutable: `mut buf`
...
45 |     buf.close_tag();
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:46:5
   |
41 |     let buf = HtmlWithLimit::new(12);
   |         --- help: consider changing this to be mutable: `mut buf`
...
46 |     buf.push("!");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:53:5
   |
52 |     let buf = HtmlWithLimit::new(60);
   |         --- help: consider changing this to be mutable: `mut buf`
53 |     buf.open_tag("p");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:54:5
   |
52 |     let buf = HtmlWithLimit::new(60);
   |         --- help: consider changing this to be mutable: `mut buf`
53 |     buf.open_tag("p");
54 |     buf.push("This is a ");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:55:5
   |
52 |     let buf = HtmlWithLimit::new(60);
   |         --- help: consider changing this to be mutable: `mut buf`
...
55 |     buf.open_tag("em");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:56:5
   |
52 |     let buf = HtmlWithLimit::new(60);
   |         --- help: consider changing this to be mutable: `mut buf`
...
56 |     buf.push("paragraph");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:57:5
   |
52 |     let buf = HtmlWithLimit::new(60);
   |         --- help: consider changing this to be mutable: `mut buf`
...
57 |     buf.open_tag("strong");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:58:5
   |
52 |     let buf = HtmlWithLimit::new(60);
   |         --- help: consider changing this to be mutable: `mut buf`
...
58 |     buf.push("!");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:59:5
   |
52 |     let buf = HtmlWithLimit::new(60);
   |         --- help: consider changing this to be mutable: `mut buf`
...
59 |     buf.close_tag();
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:60:5
   |
52 |     let buf = HtmlWithLimit::new(60);
   |         --- help: consider changing this to be mutable: `mut buf`
...
60 |     buf.close_tag();
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:61:5
   |
52 |     let buf = HtmlWithLimit::new(60);
   |         --- help: consider changing this to be mutable: `mut buf`
...
61 |     buf.close_tag();
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:68:5
   |
67 |     let buf = HtmlWithLimit::new(60);
   |         --- help: consider changing this to be mutable: `mut buf`
68 |     buf.open_tag("p");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:69:5
   |
67 |     let buf = HtmlWithLimit::new(60);
   |         --- help: consider changing this to be mutable: `mut buf`
68 |     buf.open_tag("p");
69 |     buf.push("This is a ");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:70:5
   |
67 |     let buf = HtmlWithLimit::new(60);
   |         --- help: consider changing this to be mutable: `mut buf`
...
70 |     buf.open_tag("em");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:71:5
   |
67 |     let buf = HtmlWithLimit::new(60);
   |         --- help: consider changing this to be mutable: `mut buf`
...
71 |     buf.push("paragraph");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:72:5
   |
67 |     let buf = HtmlWithLimit::new(60);
   |         --- help: consider changing this to be mutable: `mut buf`
...
72 |     buf.open_tag("strong");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:73:5
   |
67 |     let buf = HtmlWithLimit::new(60);
   |         --- help: consider changing this to be mutable: `mut buf`
...
73 |     buf.push("!");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:82:9
   |
79 |     let buf = HtmlWithLimit::new(20);
   |         --- help: consider changing this to be mutable: `mut buf`
...
82 |         buf.open_tag("strong");
   |         ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:83:9
   |
79 |     let buf = HtmlWithLimit::new(20);
   |         --- help: consider changing this to be mutable: `mut buf`
...
83 |         buf.push("word#")?;
   |         ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:84:9
   |
79 |     let buf = HtmlWithLimit::new(20);
   |         --- help: consider changing this to be mutable: `mut buf`
...
84 |         buf.push(&n.to_string())?;
   |         ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:85:9
   |
79 |     let buf = HtmlWithLimit::new(20);
   |         --- help: consider changing this to be mutable: `mut buf`
...
85 |         buf.close_tag();
   |         ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:80:5
   |
79 |     let buf = HtmlWithLimit::new(20);
   |         --- help: consider changing this to be mutable: `mut buf`
80 |     buf.open_tag("p");
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
  --> src/librustdoc/html/length_limit/tests.rs:88:5
   |
79 |     let buf = HtmlWithLimit::new(20);
   |         --- help: consider changing this to be mutable: `mut buf`
...
88 |     buf.close_tag();
   |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
   --> src/librustdoc/html/length_limit/tests.rs:102:5
    |
101 |     let buf = HtmlWithLimit::new(6);
    |         --- help: consider changing this to be mutable: `mut buf`
102 |     buf.open_tag("p");
    |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
   --> src/librustdoc/html/length_limit/tests.rs:103:5
    |
101 |     let buf = HtmlWithLimit::new(6);
    |         --- help: consider changing this to be mutable: `mut buf`
102 |     buf.open_tag("p");
103 |     buf.push("Hello");
    |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
   --> src/librustdoc/html/length_limit/tests.rs:104:5
    |
101 |     let buf = HtmlWithLimit::new(6);
    |         --- help: consider changing this to be mutable: `mut buf`
...
104 |     buf.push(" World");
    |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
   --> src/librustdoc/html/length_limit/tests.rs:113:5
    |
112 |     let buf = HtmlWithLimit::new(60);
    |         --- help: consider changing this to be mutable: `mut buf`
113 |     buf.open_tag("p");
    |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
   --> src/librustdoc/html/length_limit/tests.rs:114:5
    |
112 |     let buf = HtmlWithLimit::new(60);
    |         --- help: consider changing this to be mutable: `mut buf`
113 |     buf.open_tag("p");
114 |     buf.push("Hello");
    |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
   --> src/librustdoc/html/length_limit/tests.rs:115:5
    |
112 |     let buf = HtmlWithLimit::new(60);
    |         --- help: consider changing this to be mutable: `mut buf`
...
115 |     buf.close_tag();
    |     ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
   --> src/librustdoc/html/length_limit/tests.rs:116:5
    |
112 |     let buf = HtmlWithLimit::new(60);
    |         --- help: consider changing this to be mutable: `mut buf`
...
116 |     buf.close_tag();
    |     ^^^ cannot borrow as mutable
For more information about this error, try `rustc --explain E0596`.
error: could not compile `rustdoc` due to 45 previous errors



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


Build completed unsuccessfully in 0:25:49
