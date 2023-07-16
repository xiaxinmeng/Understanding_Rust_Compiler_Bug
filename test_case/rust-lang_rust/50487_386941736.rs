plain
[00:02:40]       Memory: 8 GB
[00:02:40]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:02:40]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:40]       SMC Version (system): 2.8f0
[00:02:40]       Serial Number (system): VMiSn9RNwuqG
[00:02:40] 
[00:02:40] hw.ncpu: 4
[00:02:40] hw.byteorder: 1234
[00:02:40] hw.memsize: 8589934592
---
Preparing deploy
travis_fold:end:dpl.2
travis_fold:start:dpl.3
Deploying application
uploading "3b225bb66cc921ab128ab06c127355412890d0e0/rustfmt-nightly-x86_64-apple-darwin.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "3b225bb66cc921ab128ab06c127355412890d0e0/rustfmt-nightly-x86_64-apple-darwin.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "3b225bb66cc921ab128ab06c127355412890d0e0/rustc-nightly-x86_64-apple-darwin.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "3b225bb66cc921ab128ab06c127355412890d0e0/rustc-nightly-x86_64-apple-darwin.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "3b225bb66cc921ab128ab06c127355412890d0e0/rust-std-nightly-x86_64-apple-darwin.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
/Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.43/lib/seahorse/client/plugins/raise_response_errors.rb:15:in `call': Encountered a `SocketError` while attempting to connect to: (Aws::Errors::NoSuchEndpointError)
  https://rust-lang-ci2.s3.us-west-1.amazonaws.com/rustc-builds-alt/3b225bb66cc921ab128ab06c127355412890d0e0/rustfmt-nightly-x86_64-apple-darwin.tar.gz
This is typically the result of an invalid `:region` option or a
poorly formatted `:endpoint` option.
* Avoid configuring the `:endpoint` option directly. Endpoints are constructed
  from the `:region`. The `:endpoint` option is reserved for connecting to
  non-standard test endpoints.
* Not every service is available in every region.
* Never suffix region names with availability zones.
  Use "us-east-1", not "us-east-1a"
Known AWS regions include (not specific to this service):
ap-northeast-1
ap-northeast-2
ap-south-1
ap-southeast-1
ap-southeast-2
ca-central-1
eu-central-1
eu-west-1
eu-west-2
eu-west-3
sa-east-1
us-east-1
us-east-2
us-west-2
us-west-2
cn-north-1
cn-northwest-1
us-gov-west-1
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.43/lib/aws-sdk-core/plugins/s3_sse_cpk.rb:19:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.43/lib/aws-sdk-core/plugins/s3_dualstack.rb:24:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.43/lib/aws-sdk-core/plugins/s3_accelerate.rb:34:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.43/lib/aws-sdk-core/plugins/jsonvalue_converter.rb:20:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.43/lib/aws-sdk-core/plugins/idempotency_token.rb:18:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.43/lib/aws-sdk-core/plugins/param_converter.rb:20:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.43/lib/seahorse/client/plugins/response_target.rb:21:in `call'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.43/lib/seahorse/client/request.rb:70:in `send_request'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-core-2.11.43/lib/seahorse/client/base.rb:207:in `block (2 levels) in define_operation_methods'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.43/lib/aws-sdk-resources/services/s3/file_uploader.rb:42:in `block in put_object'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.43/lib/aws-sdk-resources/services/s3/file_uploader.rb:49:in `open_file'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.43/lib/aws-sdk-resources/services/s3/file_uploader.rb:41:in `put_object'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.43/lib/aws-sdk-resources/services/s3/file_uploader.rb:34:in `upload'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.43/lib/aws-sdk-resources/services/s3/object.rb:252:in `upload_file'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/dpl-s3-1.9.6/lib/dpl/provider/s3.rb:99:in `block (2 levels) in upload_multithreaded'
failed to deploy
failed to deploy
No output has been received in the last 30m0s, this potentially indicates a stalled build or something wrong with the build itself.
Check the details on how to adjust your build configuration on: https://docs.travis-ci.com/user/common-build-problems/#Build-times-out-because-no-output-was-received
The build has been terminated
