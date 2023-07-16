
error: reached the type-length limit while instantiating `<linkerd_app::linkerd_app_core::...:stack::{{closure}}#5]>>>::layer`
  --> /home/ver/.cargo/registry/src/github.com-1ecc6299db9ec823/tower-layer-0.3.1/src/layer_fn.rs:83:5
   |
83 | /     fn layer(&self, inner: S) -> Self::Service {
84 | |         (self.f)(inner)
85 | |     }
   | |_____^
   |
   = note: consider adding a `#![type_length_limit="16653749"]` attribute to your crate

error: aborting due to previous error

error: could not compile `linkerd2-proxy`.

To learn more, run the command again with --verbose.
Command exited with non-zero status 101
	Command being timed: "cargo +1.46.0 build -p linkerd2-proxy"
	User time (seconds): 1304.77
	System time (seconds): 0.52
	Percent of CPU this job got: 100%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 21:41.26
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 619476
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 0
	Minor (reclaiming a frame) page faults: 303077
	Voluntary context switches: 4502
	Involuntary context switches: 5358
	Swaps: 0
	File system inputs: 0
	File system outputs: 134816
	Socket messages sent: 0
	Socket messages received: 0
	Signals delivered: 0
	Page size (bytes): 4096
	Exit status: 101
