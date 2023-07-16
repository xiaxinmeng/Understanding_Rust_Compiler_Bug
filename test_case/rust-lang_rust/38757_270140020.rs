
# Explanation of all arguments given to emscripten:
# EXPORTED_FUNCTIONS        These export the functions needed to call into Rust.
# NO_EXIT_RUNTIME=1         Prevents emscripten from unloading after main.
# DEMANGLE_SUPPORT=1        Allows rust to demangle when a panic happens, for
#                           debug info. (Can be disabled if needed)
# --memory-init-file 0      Disables the memory init file generation so
#                           emscripten can load synchronously. This is needed
#                           so it can be loaded inside the Screeps node
#                           environment.
# --pre-js/--post-js        Injects extra javascript, used to configure
#                           emscripten and export modules.
# --bind                    Adds embind javascript, allowing us to alter JS data.
# -O1/-O2/-O3               Optimization level, select as needed.
# -g2                       Disables minifying, allowing easier debugging of
#                           optimized code. (Currently disabled)
cargo rustc --release --target asmjs-unknown-emscripten -- -Clink-args="-s EXPORTED_FUNCTIONS=['_main','_load','_tick'] -s NO_EXIT_RUNTIME=1 -s DEMANGLE_SUPPORT=1 --memory-init-file 0 --pre-js pre.js --post-js post.js -s ASSERTIONS=1 --bind -O2"
