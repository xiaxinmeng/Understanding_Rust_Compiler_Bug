plain
   Compiling time v0.1.43
   Compiling sha2 v0.9.5
   Compiling parking_lot_core v0.8.3
   Compiling tracing v0.1.26
error[E0793]: reference to packed field is unaligned
    --> C:\Users\runneradmin\.cargo\registry\src\index.crates.io-6f17d22bba15001f\ntapi-0.3.6\src\ntexapi.rs:2785:52
     |
2785 |         *tick_count.QuadPart_mut() = read_volatile(&(*USER_SHARED_DATA).u.TickCountQuad);
     |
     = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
     = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)


error[E0793]: reference to packed field is unaligned
    --> C:\Users\runneradmin\.cargo\registry\src\index.crates.io-6f17d22bba15001f\ntapi-0.3.6\src\ntexapi.rs:2809:25
     |
2809 |         ((read_volatile(&(*USER_SHARED_DATA).u.TickCountQuad)
     |
     = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
     = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)

---
Total duration:                        0s
-----------------------------------------
root INFO: Free disk space: 45.21 GiB out of total 56.00 GiB (19.27% used)
Traceback (most recent call last):
  File "D:\a\rust\rust\src\ci\stage-build.py", line 839, in <module>
    raise e
  File "D:\a\rust\rust\src\ci\stage-build.py", line 836, in <module>
    execute_build_pipeline(timer, pipeline, build_args)
  File "D:\a\rust\rust\src\ci\stage-build.py", line 752, in execute_build_pipeline
    pipeline.build_rustc_perf()
  File "D:\a\rust\rust\src\ci\stage-build.py", line 195, in build_rustc_perf
    cmd([self.cargo_stage_0(), "build", "-p", "collector"], env=dict(
  File "D:\a\rust\rust\src\ci\stage-build.py", line 452, in cmd
    return subprocess.run(args, env=environment, check=True)
  File "C:\hostedtoolcache\windows\Python\3.11.2\x64\Lib\subprocess.py", line 571, in run
  File "C:\hostedtoolcache\windows\Python\3.11.2\x64\Lib\subprocess.py", line 571, in run
    raise CalledProcessError(retcode, process.args,
subprocess.CalledProcessError: Command '['D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe', 'build', '-p', 'collector']' returned non-zero exit status 101.
