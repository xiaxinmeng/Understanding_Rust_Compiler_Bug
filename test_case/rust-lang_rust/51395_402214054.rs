plain
[00:03:29]       Memory: 8 GB
[00:03:29]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:03:29]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:29]       SMC Version (system): 2.8f0
[00:03:29]       Serial Number (system): VM/1XdG2YgCq
[00:03:29] 
[00:03:29] hw.ncpu: 4
[00:03:29] hw.byteorder: 1234
[00:03:29] hw.memsize: 8589934592
---
[00:03:52] warning:                                                ^
[00:03:52] warning: 3 warnings generated.
[00:03:52] warning: /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/ranlib: file: /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/profiler_builtins-0997d9f1ee263c05/out/libprofiler-rt.a(InstrProfilingPlatformLinux.o) has no symbols
[00:03:52] warning: /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/ranlib: file: /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/profiler_builtins-0997d9f1ee263c05/out/libprofiler-rt.a(InstrProfilingPlatformOther.o) has no symbols
[00:04:01] warning: this feature has been stable since 1.28.0. Attribute no longer needed
[00:04:01]     |
[00:04:01]     |
[00:04:01] 125 | #![cfg_attr(stage0, feature(repr_transparent))]
[00:04:01]     |
[00:04:01]     |
[00:04:01]     = note: #[warn(stable_features)] on by default
[00:04:08] [RUSTC-TIMING] core test:false 35.348
[00:04:10] [RUSTC-TIMING] compiler_builtins test:false 2.255
[00:04:10]    Compiling libc v0.0.0 (file:///Users/travis/build/rust-lang/rust/src/rustc/libc_shim)
[00:04:10]    Compiling alloc v0.0.0 (file:///Users/travis/build/rust-lang/rust/src/liballoc)
---
[00:56:21]    |
[00:56:21] 22 | struct Range {
[00:56:21]    | - `range_map::Range` declared as private
[00:56:21] ...
[00:56:21] 63 |     ) -> impl Iterator<Item = (&'a Range, &'a T)> + 'a {
[00:56:21]    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't leak private type
[00:56:21] error: aborting due to previous error
[00:56:21] 
[00:56:21] For more information about this error, try `rustc --explain E0446`.
[00:56:21] [RUSTC-TIMING] miri test:false 2.590
---
uploading "c4a8bac476931e67f8bcf6199e7987d899fc3934/rust-std-nightly-x86_64-apple-darwin.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}uploading "c4a8bac476931e67f8bcf6199e7987d899fc3934/rust-std-nightly-x86_64-apple-darwin.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "c4a8bac476931e67f8bcf6199e7987d899fc3934/rustc-nightly-x86_64-apple-darwin.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "c4a8bac476931e67f8bcf6199e7987d899fc3934/rustc-nightly-x86_64-apple-darwin.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "c4a8bac476931e67f8bcf6199e7987d899fc3934/rust-src-nightly.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
/Users/travis/.rvm/rubies/ruby-2.4.2/lib/ruby/2.4.0/net/http.rb:906:in `rescue in block in connect': Failed to open TCP connection to rust-lang-ci2.s3.us-west-1.amazonaws.com:443 (Network is unreachable - connect(2) for "rust-lang-ci2.s3.us-west-1.amazonaws.com" port 443) (Errno::ENETUNREACH)
 from /Users/travis/.rvm/rubies/ruby-2.4.2/lib/ruby/2.4.0/net/http.rb:903:in `block in connect'
 from /Users/travis/.rvm/rubies/ruby-2.4.2/lib/ruby/2.4.0/timeout.rb:93:in `block in timeout'
 from /Users/travis/.rvm/rubies/ruby-2.4.2/lib/ruby/2.4.0/timeout.rb:103:in `timeout'
 from /Users/travis/.rvm/rubies/ruby-2.4.2/lib/ruby/2.4.0/net/http.rb:902:in `connect'
 from /Users/travis/.rvm/rubies/ruby-2.4.2/lib/ruby/2.4.0/net/http.rb:887:in `do_start'
 from /Users/travis/.rvm/rubies/ruby-2.4.2/lib/ruby/2.4.0/net/http.rb:882:in `start'
 from /Users/travis/.rvm/rubies/ruby-2.4.2/lib/ruby/2.4.0/delegate.rb:83:in `method_missing'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/seahorse/client/net_http/connection_pool.rb:285:in `start_session'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/seahorse/client/net_http/connection_pool.rb:92:in `session_for'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/seahorse/client/net_http/handler.rb:119:in `session'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/seahorse/client/net_http/handler.rb:71:in `transmit'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/seahorse/client/net_http/handler.rb:45:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/seahorse/client/plugins/content_length.rb:12:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/aws-sdk-core/plugins/s3_request_signer.rb:88:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/aws-sdk-core/plugins/s3_request_signer.rb:23:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/aws-sdk-core/plugins/s3_host_id.rb:14:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/aws-sdk-core/xml/error_handler.rb:8:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/aws-sdk-core/plugins/helpful_socket_errors.rb:10:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/aws-sdk-core/plugins/s3_request_signer.rb:65:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/aws-sdk-core/plugins/s3_redirects.rb:15:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/aws-sdk-core/plugins/retry_errors.rb:89:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/aws-sdk-core/plugins/s3_dualstack.rb:32:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/aws-sdk-core/plugins/s3_accelerate.rb:49:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/aws-sdk-core/plugins/s3_md5s.rb:31:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/aws-sdk-core/plugins/s3_expect_100_continue.rb:21:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/aws-sdk-core/plugins/s3_bucket_name_restrictions.rb:12:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/aws-sdk-core/plugins/s3_bucket_dns.rb:31:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/aws-sdk-core/rest/handler.rb:7:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/aws-sdk-core/plugins/user_agent.rb:12:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/seahorse/client/plugins/endpoint.rb:41:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/aws-sdk-core/plugins/param_validator.rb:21:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/seahorse/client/plugins/raise_response_errors.rb:14:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/aws-sdk-core/plugins/s3_sse_cpk.rb:19:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/aws-sdk-core/plugins/s3_dualstack.rb:24:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/aws-sdk-core/plugins/s3_accelerate.rb:34:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/aws-sdk-core/plugins/jsonvalue_converter.rb:20:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/aws-sdk-core/plugins/idempotency_token.rb:18:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/aws-sdk-core/plugins/param_converter.rb:20:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/seahorse/client/plugins/response_target.rb:21:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/seahorse/client/request.rb:70:in `send_request'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.80/lib/seahorse/client/base.rb:207:in `block (2 levels) in define_operation_methods'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.80/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:52:in `initiate_upload'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.80/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:43:in `upload'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.80/lib/aws-sdk-resources/services/s3/file_uploader.rb:32:in `upload'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.80/lib/aws-sdk-resources/services/s3/object.rb:252:in `upload_file'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/dpl-s3-1.9.7/lib/dpl/provider/s3.rb:99:in `block (2 levels) in upload_multithreaded'
failed to deploy
failed to deploy
No output has been received in the last 30m0s, this potentially indicates a stalled build or something wrong with the build itself.
Check the details on how to adjust your build configuration on: https://docs.travis-ci.com/user/common-build-problems/#Build-times-out-because-no-output-was-received
The build has been terminated
