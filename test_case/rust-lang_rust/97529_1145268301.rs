plain
DirectMap1G:    53477376 kB
+ python3 ../x.py dist --host x86_64-unknown-illumos --target x86_64-unknown-illumos
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.19s
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "--cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Zunstable-options -Csplit-debuginfo=off -Cprefer-dynamic",
        x86_64-unknown-linux-gnu,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "--cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-linux-gnu,
}
}
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "--cfg=bootstrap -Csymbol-mangling-version=v0 -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Zunstable-options -Csplit-debuginfo=off -Ztls-model=initial-exec -Zunstable-options -Wrustc::internal -Cprefer-dynamic",
        x86_64-unknown-linux-gnu,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "--cfg=bootstrap -Csymbol-mangling-version=v0 -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-linux-gnu,
}
}
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(stdarch_intel_sde) --check-cfg=values(no_fp_fmt_parse) --check-cfg=values(no_global_oom_handling) --check-cfg=values(freebsd12) --check-cfg=values(backtrace_in_libstd) --check-cfg=values(target_env,\"libnx\") --check-cfg=values(target_os,\"watchos\") --check-cfg=values(target_arch,\"asmjs\",\"spirv\",\"nvptx\",\"nvptx64\",\"le32\",\"xtensa\") --check-cfg=values(dont_compile_me) -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Zsave-analysis -Cprefer-dynamic",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(stdarch_intel_sde) --check-cfg=values(no_fp_fmt_parse) --check-cfg=values(no_global_oom_handling) --check-cfg=values(freebsd12) --check-cfg=values(backtrace_in_libstd) --check-cfg=values(target_env,\"libnx\") --check-cfg=values(target_os,\"watchos\") --check-cfg=values(target_arch,\"asmjs\",\"spirv\",\"nvptx\",\"nvptx64\",\"le32\",\"xtensa\") --check-cfg=values(dont_compile_me) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(stdarch_intel_sde) --check-cfg=values(no_fp_fmt_parse) --check-cfg=values(no_global_oom_handling) --check-cfg=values(freebsd12) --check-cfg=values(backtrace_in_libstd) --check-cfg=values(target_env,\"libnx\") --check-cfg=values(target_os,\"watchos\") --check-cfg=values(target_arch,\"asmjs\",\"spirv\",\"nvptx\",\"nvptx64\",\"le32\",\"xtensa\") --check-cfg=values(dont_compile_me) -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Zunstable-options -Csplit-debuginfo=off -Zsave-analysis -Cprefer-dynamic",
        x86_64-unknown-linux-gnu,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(stdarch_intel_sde) --check-cfg=values(no_fp_fmt_parse) --check-cfg=values(no_global_oom_handling) --check-cfg=values(freebsd12) --check-cfg=values(backtrace_in_libstd) --check-cfg=values(target_env,\"libnx\") --check-cfg=values(target_os,\"watchos\") --check-cfg=values(target_arch,\"asmjs\",\"spirv\",\"nvptx\",\"nvptx64\",\"le32\",\"xtensa\") --check-cfg=values(dont_compile_me) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-linux-gnu,
}
}
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec -Zunstable-options -Wrustc::internal -Cprefer-dynamic",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "--cfg=bootstrap -Csymbol-mangling-version=v0 -Zallow-features=binary-dep-depinfo,backtrace -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Zunstable-options -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-linux-gnu,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "--cfg=bootstrap -Csymbol-mangling-version=v0 --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-linux-gnu,
}
}
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
    rustflags: Rustflags(
        "--cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Zunstable-options -Csplit-debuginfo=off -Cprefer-dynamic",
        x86_64-unknown-linux-gnu,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "--cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-linux-gnu,
}
---
   Compiling getopts v0.2.21
   Compiling test v0.0.0 (/checkout/library/test)
    Finished release [optimized + debuginfo] target(s) in 52.90s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "--cfg=bootstrap -Csymbol-mangling-version=v0 -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Zunstable-options -Csplit-debuginfo=off -Ztls-model=initial-exec -Zunstable-options -Wrustc::internal -Cprefer-dynamic",
        x86_64-unknown-linux-gnu,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "--cfg=bootstrap -Csymbol-mangling-version=v0 -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-linux-gnu,
    ),
}
running: "cmake" "/checkout/src/llvm-project/llvm" "-G" "Ninja" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_ENABLE_PLUGINS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;BPF;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR;M68k" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=16" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_INSTALL_UTILS=ON" "-DLLVM_ENABLE_ZLIB=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-1.63.0-nightly" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=cc" "-DCMAKE_CXX_COMPILER=c++" "-DCMAKE_ASM_COMPILER=cc" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_SHARED_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_MODULE_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_EXE_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_BUILD_TYPE=Release"
---
   Compiling rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
    Finished release [optimized] target(s) in 2m 40s
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(stdarch_intel_sde) --check-cfg=values(no_fp_fmt_parse) --check-cfg=values(no_global_oom_handling) --check-cfg=values(freebsd12) --check-cfg=values(backtrace_in_libstd) --check-cfg=values(target_env,\"libnx\") --check-cfg=values(target_os,\"watchos\") --check-cfg=values(target_arch,\"asmjs\",\"spirv\",\"nvptx\",\"nvptx64\",\"le32\",\"xtensa\") --check-cfg=values(dont_compile_me) -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Zsave-analysis -Cprefer-dynamic",
        x86_64-unknown-illumos,
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-illumos)
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(stdarch_intel_sde) --check-cfg=values(no_fp_fmt_parse) --check-cfg=values(no_global_oom_handling) --check-cfg=values(freebsd12) --check-cfg=values(backtrace_in_libstd) --check-cfg=values(target_env,\"libnx\") --check-cfg=values(target_os,\"watchos\") --check-cfg=values(target_arch,\"asmjs\",\"spirv\",\"nvptx\",\"nvptx64\",\"le32\",\"xtensa\") --check-cfg=values(dont_compile_me) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
   Compiling cc v1.0.69
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.126
---
   Compiling test v0.0.0 (/checkout/library/test)
    Finished release [optimized + debuginfo] target(s) in 1m 17s
Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-illumos)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(stdarch_intel_sde) --check-cfg=values(no_fp_fmt_parse) --check-cfg=values(no_global_oom_handling) --check-cfg=values(freebsd12) --check-cfg=values(backtrace_in_libstd) --check-cfg=values(target_env,\"libnx\") --check-cfg=values(target_os,\"watchos\") --check-cfg=values(target_arch,\"asmjs\",\"spirv\",\"nvptx\",\"nvptx64\",\"le32\",\"xtensa\") --check-cfg=values(dont_compile_me) -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Zunstable-options -Csplit-debuginfo=off -Zsave-analysis -Cprefer-dynamic",
        x86_64-unknown-linux-gnu,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(stdarch_intel_sde) --check-cfg=values(no_fp_fmt_parse) --check-cfg=values(no_global_oom_handling) --check-cfg=values(freebsd12) --check-cfg=values(backtrace_in_libstd) --check-cfg=values(target_env,\"libnx\") --check-cfg=values(target_os,\"watchos\") --check-cfg=values(target_arch,\"asmjs\",\"spirv\",\"nvptx\",\"nvptx64\",\"le32\",\"xtensa\") --check-cfg=values(dont_compile_me) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-linux-gnu,
}
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling compiler_builtins v0.1.73
   Compiling libc v0.2.126
---
   Compiling getopts v0.2.21
   Compiling test v0.0.0 (/checkout/library/test)
    Finished release [optimized + debuginfo] target(s) in 1m 18s
Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec -Zunstable-options -Wrustc::internal -Cprefer-dynamic",
        x86_64-unknown-illumos,
    ),
running: "cmake" "/checkout/src/llvm-project/llvm" "-G" "Ninja" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_ENABLE_PLUGINS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;BPF;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR;M68k" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=16" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-illumos" "-DLLVM_INSTALL_UTILS=ON" "-DLLVM_ENABLE_ZLIB=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DCMAKE_CROSSCOMPILING=True" "-DLLVM_TABLEGEN=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-tblgen" "-DLLVM_NM=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-nm" "-DLLVM_CONFIG_PATH=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-config" "-DLLVM_VERSION_SUFFIX=-rust-1.63.0-nightly" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_SYSTEM_NAME=SunOS" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=x86_64-illumos-gcc" "-DCMAKE_CXX_COMPILER=x86_64-illumos-g++" "-DCMAKE_ASM_COMPILER=x86_64-illumos-gcc" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_SHARED_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_MODULE_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_EXE_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-illumos/llvm" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_BUILD_TYPE=Release"
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
-- The C compiler identification is GNU 8.4.0
-- The CXX compiler identification is GNU 8.4.0
-- The ASM compiler identification is GNU
---
   Compiling rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
    Finished release [optimized] target(s) in 3m 50s
Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-illumos)
Assembling stage2 compiler (x86_64-unknown-illumos)
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
---
   Compiling askama v0.11.0
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
   Compiling rustdoc-tool v0.0.0 (/checkout/src/tools/rustdoc)
    Finished release [optimized] target(s) in 1m 09s
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "--cfg=bootstrap -Csymbol-mangling-version=v0 -Zallow-features=binary-dep-depinfo,backtrace -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Zunstable-options -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-linux-gnu,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "--cfg=bootstrap -Csymbol-mangling-version=v0 --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-linux-gnu,
}
Building stage0 tool fabricate (x86_64-unknown-linux-gnu)
---
Dist rust-analysis-nightly-x86_64-unknown-illumos
 finished in 8.471 seconds
Dist rust-src-nightly
 finished in 9.047 seconds
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
Building stage1 tool cargo (x86_64-unknown-illumos)
---
   Compiling cargo-util v0.1.3 (/checkout/src/tools/cargo/crates/cargo-util)
   Compiling git2 v0.14.2
   Compiling git2-curl v0.15.0
    Finished release [optimized] target(s) in 1m 50s
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
Building stage1 tool cargo-credential-1password (x86_64-unknown-illumos)
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
   Compiling syn v1.0.91
   Compiling cargo-credential v0.1.0 (/checkout/src/tools/cargo/crates/credential/cargo-credential)
   Compiling serde_derive v1.0.125
   Compiling serde_derive v1.0.125
   Compiling serde v1.0.125
   Compiling serde_json v1.0.59
   Compiling cargo-credential-1password v0.1.0 (/checkout/src/tools/cargo/crates/credential/cargo-credential-1password)
    Finished release [optimized] target(s) in 14.47s
Dist cargo-nightly-x86_64-unknown-illumos
 finished in 12.740 seconds
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
Building stage1 tool clippy-driver (x86_64-unknown-illumos)
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
---
   Compiling clippy v0.1.63 (/checkout/src/tools/clippy)
   Compiling cargo_metadata v0.14.0
   Compiling clippy_lints v0.1.63 (/checkout/src/tools/clippy/clippy_lints)
    Finished release [optimized] target(s) in 59.48s
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
Building stage1 tool rls (x86_64-unknown-illumos)
---
   Compiling jsonrpc-server-utils v18.0.0
warning: unexpected `cfg` condition value
 --> src/tools/rls/racer/src/racer/lib.rs:1:13
  |
1 | #![cfg_attr(feature = "nightly", feature(test))]
  |
  = note: `#[warn(unexpected_cfgs)]` on by default
  = note: expected values for `feature` are: default, metadata


warning: unexpected `cfg` condition value
  --> src/tools/rls/racer/src/racer/lib.rs:59:11
   |
59 | #[cfg(all(feature = "nightly", test))]
   |
   = note: expected values for `feature` are: default, metadata

   Compiling jsonrpc-pubsub v18.0.0
---
warning: `racer` (lib) generated 3 warnings
    Finished release [optimized] target(s) in 56.55s
Dist rls-nightly-x86_64-unknown-illumos
 finished in 14.620 seconds
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
Building stage1 tool rust-analyzer (x86_64-unknown-illumos)
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
---
   Compiling ide v0.0.0 (/checkout/src/tools/rust-analyzer/crates/ide)
    Finished release [optimized] target(s) in 1m 52s
Dist rust-analyzer-nightly-x86_64-unknown-illumos
 finished in 14.156 seconds
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    ),
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
   Compiling cc v1.0.69
   Compiling openssl-src v111.18.0+1.1.1n
   Compiling openssl-sys v0.9.72
   Compiling openssl-sys v0.9.72
   Compiling libz-sys v1.1.3
   Compiling libnghttp2-sys v0.1.4+1.41.0
   Compiling curl-sys v0.4.55+curl-7.83.1
   Compiling openssl v0.10.38
   Compiling rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
   Compiling rustfmt-nightly v1.4.38 (/checkout/src/tools/rustfmt)
    Finished release [optimized] target(s) in 49.95s
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
    Finished release [optimized] target(s) in 0.23s
Dist rustfmt-nightly-x86_64-unknown-illumos
 finished in 12.546 seconds
 finished in 12.546 seconds
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
Building stage1 tool rust-demangler (x86_64-unknown-illumos)
   Compiling rustc-demangle v0.1.21
   Compiling rust-demangler v0.0.1 (/checkout/src/tools/rust-demangler)
   Compiling rust-demangler v0.0.1 (/checkout/src/tools/rust-demangler)
    Finished release [optimized] target(s) in 2.27s
Dist rust-demangler-nightly-x86_64-unknown-illumos
 finished in 2.874 seconds
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
    ),
}
    Finished release [optimized] target(s) in 0.19s
    Finished release [optimized] target(s) in 0.19s
Dist clippy-nightly-x86_64-unknown-illumos
 finished in 8.930 seconds
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
Building stage1 tool miri (x86_64-unknown-illumos)
---
    = note: `#[warn(rustc::pass_by_value)]` on by default

warning: `miri` (lib) generated 1 warning
    Finished release [optimized] target(s) in 32.30s
[src/bootstrap/builder.rs:1985] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    ),
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=names() --check-cfg=values() --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) --crate-version 1.63.0-nightly\n(2fc487a26\n2022-06-02)",
        x86_64-unknown-illumos,
}
