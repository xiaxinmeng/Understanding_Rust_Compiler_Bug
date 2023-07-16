
error: could not compile `linkerd2-proxy`

Caused by:
  process didn't exit successfully: `rustc --crate-name linkerd2_proxy --edition=2018 linkerd2-proxy/src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -Z time-passes --cfg 'feature="default"' --cfg 'feature="multicore"' --cfg 'feature="num_cpus"' -C metadata=4b636c9abf0080e5 -C extra-filename=-4b636c9abf0080e5 --out-dir /home/ver/b/linkerd2-proxy/target/debug/deps -C incremental=/home/ver/b/linkerd2-proxy/target/debug/incremental -L dependency=/home/ver/b/linkerd2-proxy/target/debug/deps --extern futures=/home/ver/b/linkerd2-proxy/target/debug/deps/libfutures-ba905e7fd290ac0d.rlib --extern linkerd_app=/home/ver/b/linkerd2-proxy/target/debug/deps/liblinkerd_app-413b736e1b748ce1.rlib --extern linkerd_signal=/home/ver/b/linkerd2-proxy/target/debug/deps/liblinkerd_signal-e33579c6c7162bd5.rlib --extern num_cpus=/home/ver/b/linkerd2-proxy/target/debug/deps/libnum_cpus-372d7745cb8ee5a2.rlib --extern tokio=/home/ver/b/linkerd2-proxy/target/debug/deps/libtokio-6439a3cdc075b537.rlib --extern tracing=/home/ver/b/linkerd2-proxy/target/debug/deps/libtracing-9cda1e0af1393c64.rlib -L native=/home/ver/b/linkerd2-proxy/target/debug/build/ring-de36c98917ce89f3/out` (signal: 9, SIGKILL: kill)
Command exited with non-zero status 101
	Command being timed: "cargo +nightly rustc -- -Z time-passes"
	User time (seconds): 1210.51
	System time (seconds): 102.65
	Percent of CPU this job got: 128%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 16:58.61
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 62990612
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 5127
	Minor (reclaiming a frame) page faults: 99760499
	Voluntary context switches: 38121
	Involuntary context switches: 30180
	Swaps: 0
	File system inputs: 595200
	File system outputs: 2482104
	Socket messages sent: 0
	Socket messages received: 0
	Signals delivered: 0
	Page size (bytes): 4096
	Exit status: 101
