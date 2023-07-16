plain
---- [pretty] src/test/pretty/stmt_expr_attributes.rs stdout ----

error: pretty-printing failed in round 0 revision None
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/pretty/stmt_expr_attributes.rs" "-Z" "unpretty=normal" "--target" "x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty/stmt_expr_attributes/auxiliary.pretty" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers"
--- stdout -------------------------------
// pp-exact
#![feature(box_syntax)]
#![feature(inline_const)]
#![feature(inline_const_pat)]
#![feature(rustc_attrs)]
#![feature(rustc_attrs)]
#![feature(stmt_expr_attributes)]

fn main() {}

fn _0() {

    #[rustc_dummy]
    foo();

fn _1() {


    #[rustc_dummy]
    unsafe {
        #![rustc_dummy]
        // code
}

fn _2() {


    #[rustc_dummy]
    { foo(); }
    {
    {
        #![rustc_dummy]
        foo()
    }
}


fn _3() {

    #[rustc_dummy]
    match () { _ => {} }

fn _4() {


    #[rustc_dummy]
    match () {
        #![rustc_dummy]
        _ => (),

    let _ =
    let _ =
        #[rustc_dummy] match () {
            #![rustc_dummy]
            () => (),
}

fn _5() {


    #[rustc_dummy]
    let x = 1;

    let x = #[rustc_dummy] 1;
    let y = ();
    let y = ();
    let z = ();

    foo3(x, #[rustc_dummy] y, z);

    qux(3 + #[rustc_dummy] 2);

fn _6() {


    #[rustc_dummy]
    [1, 2, 3];

    let _ = #[rustc_dummy] [1, 2, 3];

    #[rustc_dummy]
    [1; 4];

    let _ = #[rustc_dummy] [1; 4];

struct Foo {
    data: (),
}
}

struct Bar(());

fn _7() {

    #[rustc_dummy]
    Foo { data: () };

    let _ = #[rustc_dummy] Foo { data: () };

fn _8() {


    #[rustc_dummy]
    ();

    #[rustc_dummy]
    (0);

    #[rustc_dummy]
    (0,);

    #[rustc_dummy]
    (0, 1);

fn _9() {
fn _9() {
    macro_rules! stmt_mac { () => { let _ = () ; } }

    #[rustc_dummy]
    stmt_mac!();

    #[rustc_dummy]
    stmt_mac! {};

    #[rustc_dummy]
    stmt_mac![];

    #[rustc_dummy]
    stmt_mac! {}
    let _ = ();
}


macro_rules! expr_mac { () => { () } }
fn _10() {
fn _10() {
    let _ = #[rustc_dummy] expr_mac!();
    let _ = #[rustc_dummy] expr_mac![];
    let _ = #[rustc_dummy] expr_mac! {};

fn _11() {
fn _11() {
    let _ = #[rustc_dummy] box 0;
    let _: [(); 0] = #[rustc_dummy] [];
    let _ = #[rustc_dummy] [0, 0];
    let _ = #[rustc_dummy] [0; 0];
    let _ = #[rustc_dummy] foo();
    let _ = #[rustc_dummy] 1i32.clone();
    let _ = #[rustc_dummy] ();
    let _ = #[rustc_dummy] (0);
    let _ = #[rustc_dummy] (0,);
    let _ = #[rustc_dummy] (0, 0);
    let _ = #[rustc_dummy] 0 + #[rustc_dummy] 0;
    let _ = #[rustc_dummy] !0;
    let _ = #[rustc_dummy] -0i32;
    let _ = #[rustc_dummy] false;
    let _ = #[rustc_dummy] 'c';
    let _ = #[rustc_dummy] 0;
    let _ = #[rustc_dummy] 0 as usize;
    let _ =
        #[rustc_dummy] while false {
            #![rustc_dummy]
    let _ =
    let _ =
        #[rustc_dummy] while let None = Some(()) {
            #![rustc_dummy]
    let _ =
    let _ =
        #[rustc_dummy] for _ in 0..0 {
            #![rustc_dummy]
    let _ =
    let _ =
        #[rustc_dummy] loop {
            #![rustc_dummy]
    let _ =
    let _ =
        #[rustc_dummy] match false {
            #![rustc_dummy]
            _ => (),
        };
    let _ = #[rustc_dummy] || #[rustc_dummy] ();
    let _ = #[rustc_dummy] move || #[rustc_dummy] ();
    let _ =
        #[rustc_dummy] ||
            {
                #![rustc_dummy]
                #[rustc_dummy]
            };
    let _ =
    let _ =
        #[rustc_dummy] move ||
            {
                #![rustc_dummy]
                #[rustc_dummy]
            };
    let _ =
    let _ =
        #[rustc_dummy] {
            #![rustc_dummy]
    let _ =
    let _ =
        #[rustc_dummy] {
            #![rustc_dummy]
            let _ = ();
    let _ =
    let _ =
        #[rustc_dummy] {
            #![rustc_dummy]
            let _ = ();
        };
        };
    (/*ERROR*/)

fn _12() {
fn _12() {
    #[rustc_dummy]
    let _ = 0;

    #[rustc_dummy]
    0;

    #[rustc_dummy]
    expr_mac!();

    #[rustc_dummy]
    {
        #![rustc_dummy]
}

fn foo() {}
fn foo() {}
fn foo3(_: i32, _: (), _: ()) {}
fn qux(_: i32) {}
--- stderr -------------------------------
error: expected pattern, found `{`
   --> /checkout/src/test/pretty/stmt_expr_attributes.rs:212:15
    |
