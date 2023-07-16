plain
DirectMap2M:     6121472 kB
DirectMap1G:    53477376 kB
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.23s
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "--cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Zunstable-options -Csplit-debuginfo=off -Cprefer-dynamic",
        x86_64-unknown-linux-gnu,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "--cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-linux-gnu,
}
}
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "--cfg=bootstrap -Csymbol-mangling-version=v0 -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Zunstable-options -Csplit-debuginfo=off -Ztls-model=initial-exec -Zunstable-options -Wrustc::internal -Cprefer-dynamic",
        x86_64-unknown-linux-gnu,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "--cfg=bootstrap -Csymbol-mangling-version=v0 -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-linux-gnu,
}
}
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=names,values" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=legacy -Zunstable-options --check-cfg=values(bootstrap) --check-cfg=values(stdarch_intel_sde) --check-cfg=values(no_fp_fmt_parse) --check-cfg=values(no_global_oom_handling) --check-cfg=values(freebsd12) --check-cfg=values(backtrace_in_libstd) --check-cfg=values(target_env,\"libnx\") --check-cfg=values(target_os,\"watchos\") --check-cfg=values(target_arch,\"asmjs\",\"spirv\",\"nvptx\",\"nvptx64\",\"le32\",\"xtensa\") --check-cfg=values(dont_compile_me) -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Zsave-analysis -Cprefer-dynamic",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=legacy -Zunstable-options --check-cfg=values(bootstrap) --check-cfg=values(stdarch_intel_sde) --check-cfg=values(no_fp_fmt_parse) --check-cfg=values(no_global_oom_handling) --check-cfg=values(freebsd12) --check-cfg=values(backtrace_in_libstd) --check-cfg=values(target_env,\"libnx\") --check-cfg=values(target_os,\"watchos\") --check-cfg=values(target_arch,\"asmjs\",\"spirv\",\"nvptx\",\"nvptx64\",\"le32\",\"xtensa\") --check-cfg=values(dont_compile_me) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zcheck-cfg=names,values" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=legacy -Zunstable-options --check-cfg=values(bootstrap) --check-cfg=values(stdarch_intel_sde) --check-cfg=values(no_fp_fmt_parse) --check-cfg=values(no_global_oom_handling) --check-cfg=values(freebsd12) --check-cfg=values(backtrace_in_libstd) --check-cfg=values(target_env,\"libnx\") --check-cfg=values(target_os,\"watchos\") --check-cfg=values(target_arch,\"asmjs\",\"spirv\",\"nvptx\",\"nvptx64\",\"le32\",\"xtensa\") --check-cfg=values(dont_compile_me) -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Zunstable-options -Csplit-debuginfo=off -Zsave-analysis -Cprefer-dynamic",
        x86_64-unknown-linux-gnu,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=legacy -Zunstable-options --check-cfg=values(bootstrap) --check-cfg=values(stdarch_intel_sde) --check-cfg=values(no_fp_fmt_parse) --check-cfg=values(no_global_oom_handling) --check-cfg=values(freebsd12) --check-cfg=values(backtrace_in_libstd) --check-cfg=values(target_env,\"libnx\") --check-cfg=values(target_os,\"watchos\") --check-cfg=values(target_arch,\"asmjs\",\"spirv\",\"nvptx\",\"nvptx64\",\"le32\",\"xtensa\") --check-cfg=values(dont_compile_me) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-linux-gnu,
}
}
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=names,values,features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec -Zunstable-options -Wrustc::internal -Cprefer-dynamic",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=names,values,features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "--cfg=bootstrap -Csymbol-mangling-version=v0 -Zallow-features=binary-dep-depinfo,backtrace -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Zunstable-options -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-linux-gnu,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "--cfg=bootstrap -Csymbol-mangling-version=v0 --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-linux-gnu,
}
}
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=names,values,features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=names,values,features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=names,values,features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=names,values,features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=names,values,features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=names,values,features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=names,values,features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=names,values,features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=names,values,features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=names,values,features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=names,values,features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-illumos,
}
}
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "--cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Zunstable-options -Csplit-debuginfo=off -Cprefer-dynamic",
        x86_64-unknown-linux-gnu,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "--cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-linux-gnu,
}
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
   Compiling getopts v0.2.21
   Compiling test v0.0.0 (/checkout/library/test)
    Finished release [optimized + debuginfo] target(s) in 1m 03s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "--cfg=bootstrap -Csymbol-mangling-version=v0 -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Zunstable-options -Csplit-debuginfo=off -Ztls-model=initial-exec -Zunstable-options -Wrustc::internal -Cprefer-dynamic",
        x86_64-unknown-linux-gnu,
    ),
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "--cfg=bootstrap -Csymbol-mangling-version=v0 -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-linux-gnu,
}
running: "cmake" "/checkout/src/llvm-project/llvm" "-G" "Ninja" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_ENABLE_PLUGINS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;BPF;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR;M68k" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=16" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_INSTALL_UTILS=ON" "-DLLVM_ENABLE_ZLIB=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-1.63.0-nightly" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=cc" "-DCMAKE_CXX_COMPILER=c++" "-DCMAKE_ASM_COMPILER=cc" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_SHARED_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_MODULE_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_EXE_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_BUILD_TYPE=Release"
-- The C compiler identification is GNU 7.5.0
-- The CXX compiler identification is GNU 7.5.0
---
   Compiling rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
    Finished release [optimized] target(s) in 3m 19s
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-illumos)
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=names,values" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=legacy -Zunstable-options --check-cfg=values(bootstrap) --check-cfg=values(stdarch_intel_sde) --check-cfg=values(no_fp_fmt_parse) --check-cfg=values(no_global_oom_handling) --check-cfg=values(freebsd12) --check-cfg=values(backtrace_in_libstd) --check-cfg=values(target_env,\"libnx\") --check-cfg=values(target_os,\"watchos\") --check-cfg=values(target_arch,\"asmjs\",\"spirv\",\"nvptx\",\"nvptx64\",\"le32\",\"xtensa\") --check-cfg=values(dont_compile_me) -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Zsave-analysis -Cprefer-dynamic",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=legacy -Zunstable-options --check-cfg=values(bootstrap) --check-cfg=values(stdarch_intel_sde) --check-cfg=values(no_fp_fmt_parse) --check-cfg=values(no_global_oom_handling) --check-cfg=values(freebsd12) --check-cfg=values(backtrace_in_libstd) --check-cfg=values(target_env,\"libnx\") --check-cfg=values(target_os,\"watchos\") --check-cfg=values(target_arch,\"asmjs\",\"spirv\",\"nvptx\",\"nvptx64\",\"le32\",\"xtensa\") --check-cfg=values(dont_compile_me) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-illumos,
}
   Compiling cc v1.0.69
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.126
---
   Compiling getopts v0.2.21
   Compiling test v0.0.0 (/checkout/library/test)
    Finished release [optimized + debuginfo] target(s) in 1m 38s
Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-illumos)
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zcheck-cfg=names,values" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=legacy -Zunstable-options --check-cfg=values(bootstrap) --check-cfg=values(stdarch_intel_sde) --check-cfg=values(no_fp_fmt_parse) --check-cfg=values(no_global_oom_handling) --check-cfg=values(freebsd12) --check-cfg=values(backtrace_in_libstd) --check-cfg=values(target_env,\"libnx\") --check-cfg=values(target_os,\"watchos\") --check-cfg=values(target_arch,\"asmjs\",\"spirv\",\"nvptx\",\"nvptx64\",\"le32\",\"xtensa\") --check-cfg=values(dont_compile_me) -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Zunstable-options -Csplit-debuginfo=off -Zsave-analysis -Cprefer-dynamic",
        x86_64-unknown-linux-gnu,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=legacy -Zunstable-options --check-cfg=values(bootstrap) --check-cfg=values(stdarch_intel_sde) --check-cfg=values(no_fp_fmt_parse) --check-cfg=values(no_global_oom_handling) --check-cfg=values(freebsd12) --check-cfg=values(backtrace_in_libstd) --check-cfg=values(target_env,\"libnx\") --check-cfg=values(target_os,\"watchos\") --check-cfg=values(target_arch,\"asmjs\",\"spirv\",\"nvptx\",\"nvptx64\",\"le32\",\"xtensa\") --check-cfg=values(dont_compile_me) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-linux-gnu,
    ),
}
   Compiling core v0.0.0 (/checkout/library/core)
---
   Compiling getopts v0.2.21
   Compiling test v0.0.0 (/checkout/library/test)
    Finished release [optimized + debuginfo] target(s) in 1m 38s
Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=names,values,features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec -Zunstable-options -Wrustc::internal -Cprefer-dynamic",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
Building LLVM for x86_64-unknown-illumos
Building LLVM for x86_64-unknown-illumos
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-illumos,
    ),
}
-- The C compiler identification is GNU 8.4.0
---
   Compiling rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
    Finished release [optimized] target(s) in 4m 57s
Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-illumos)
Assembling stage2 compiler (x86_64-unknown-illumos)
[src/bootstrap/builder.rs:1979] Cargo { command: cargo, rustflags, rustdocflags } = Cargo {
    command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-illumos" "-Zcheck-cfg=names,values,features" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always",
    rustflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Csplit-debuginfo=off -Ztls-model=initial-exec",
        x86_64-unknown-illumos,
    rustdocflags: Rustflags(
    rustdocflags: Rustflags(
        "-Csymbol-mangling-version=v0 --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(release) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version 1.63.0-nightly\n(fa07fc14d\n2022-06-02)",
        x86_64-unknown-illumos,
}
Building rustdoc for stage2 (x86_64-unknown-illumos)
Building rustdoc for stage2 (x86_64-unknown-illumos)
error: failed to run `rustc` to learn about target-specific information
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names -Csymbol-mangling-version=v0 '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(release)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Csplit-debuginfo=off -Ztls-model=initial-exec --target x86_64-unknown-illumos --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=cfg` (exit status: 1)
  --- stderr
  error: the `-Z unstable-options` flag must also be passed to enable the flag `check-cfg`
Build completed unsuccessfully in 0:17:01
