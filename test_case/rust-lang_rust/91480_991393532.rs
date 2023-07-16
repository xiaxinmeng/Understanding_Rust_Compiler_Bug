plain
   Compiling rustdoc-themes v0.1.0 (/checkout/src/tools/rustdoc-themes)
    Finished release [optimized] target(s) in 0.64s
rustdoc: [check-theme] Starting tests! (Ignoring all other arguments)
 - Checking "/checkout/src/librustdoc/html/static/css/themes/dark.css"... FAILED
  Missing ".content span.associatedtype,.content a.associatedtype,.block a.current.associatedtype" rule
 - Checking "/checkout/src/librustdoc/html/static/css/themes/ayu.css"... FAILED
  Missing ".content span.type,.content a.type,.block a.current.type" rule

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustdoc-themes" "/checkout/obj/build/bootstrap/debug/rustdoc" "/checkout/src/librustdoc/html/static/css/themes"
expected success, got: exit status: 1

