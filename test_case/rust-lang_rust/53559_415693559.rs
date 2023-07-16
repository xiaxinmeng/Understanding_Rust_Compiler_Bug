plain
Preparing deploy
travis_fold:end:dpl.2
travis_fold:start:dpl.3
Deploying application
uploading "b6884ac50ec71ffc9fa72fb91b9c383c1d0d0d60/rust-analysis-nightly-arm-unknown-linux-gnueabihf.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "b6884ac50ec71ffc9fa72fb91b9c383c1d0d0d60/rust-nightly-arm-unknown-linux-gnueabihf.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "b6884ac50ec71ffc9fa72fb91b9c383c1d0d0d60/llvm-tools-nightly-arm-unknown-linux-gnueabihf.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "b6884ac50ec71ffc9fa72fb91b9c383c1d0d0d60/rust-std-nightly-arm-unknown-linux-gnueabihf.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "b6884ac50ec71ffc9fa72fb91b9c383c1d0d0d60/rust-src-nightly.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "b6884ac50ec71ffc9fa72fb91b9c383c1d0d0d60/rustc-nightly-arm-unknown-linux-gnueabihf.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "b6884ac50ec71ffc9fa72fb91b9c383c1d0d0d60/rustc-nightly-arm-unknown-linux-gnueabihf.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "b6884ac50ec71ffc9fa72fb91b9c383c1d0d0d60/rustfmt-nightly-arm-unknown-linux-gnueabihf.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "b6884ac50ec71ffc9fa72fb91b9c383c1d0d0d60/rust-std-nightly-arm-unknown-linux-gnueabihf.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "b6884ac50ec71ffc9fa72fb91b9c383c1d0d0d60/rust-src-nightly.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "b6884ac50ec71ffc9fa72fb91b9c383c1d0d0d60/cargo-nightly-arm-unknown-linux-gnueabihf.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "b6884ac50ec71ffc9fa72fb91b9c383c1d0d0d60/rust-nightly-arm-unknown-linux-gnueabihf.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "b6884ac50ec71ffc9fa72fb91b9c383c1d0d0d60/rust-analysis-nightly-arm-unknown-linux-gnueabihf.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "b6884ac50ec71ffc9fa72fb91b9c383c1d0d0d60/rustfmt-nightly-arm-unknown-linux-gnueabihf.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "b6884ac50ec71ffc9fa72fb91b9c383c1d0d0d60/cargo-nightly-arm-unknown-linux-gnueabihf.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "b6884ac50ec71ffc9fa72fb91b9c383c1d0d0d60/llvm-tools-nightly-arm-unknown-linux-gnueabihf.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
/home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/2.2.0/net/http.rb:879:in `initialize': execution expired (Seahorse::Client::NetworkingError)
 from /home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/2.2.0/net/http.rb:879:in `open'
 from /home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/2.2.0/net/http.rb:879:in `block in connect'
 from /home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/2.2.0/timeout.rb:88:in `block in timeout'
 from /home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/2.2.0/timeout.rb:98:in `call'
 from /home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/2.2.0/timeout.rb:98:in `timeout'
 from /home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/2.2.0/net/http.rb:878:in `connect'
 from /home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/2.2.0/net/http.rb:1457:in `begin_transport'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/seahorse/client/net_http/patches.rb:25:in `new_transport_request'
 from /home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/2.2.0/net/http.rb:1384:in `request'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/seahorse/client/net_http/connection_pool.rb:330:in `request'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/seahorse/client/net_http/handler.rb:72:in `block in transmit'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/seahorse/client/net_http/handler.rb:121:in `block in session'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/seahorse/client/net_http/connection_pool.rb:96:in `session_for'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/seahorse/client/net_http/handler.rb:119:in `session'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/seahorse/client/net_http/handler.rb:71:in `transmit'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/seahorse/client/net_http/handler.rb:45:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/seahorse/client/plugins/content_length.rb:12:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/s3_request_signer.rb:88:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/s3_request_signer.rb:23:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/s3_host_id.rb:14:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/xml/error_handler.rb:8:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/helpful_socket_errors.rb:10:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/s3_request_signer.rb:65:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/s3_redirects.rb:15:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/retry_errors.rb:89:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/retry_errors.rb:120:in `retry_request'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/retry_errors.rb:103:in `retry_if_possible'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/retry_errors.rb:91:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/retry_errors.rb:120:in `retry_request'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/retry_errors.rb:103:in `retry_if_possible'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/retry_errors.rb:91:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/retry_errors.rb:120:in `retry_request'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/retry_errors.rb:103:in `retry_if_possible'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/retry_errors.rb:91:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/s3_dualstack.rb:32:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/s3_accelerate.rb:49:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/s3_md5s.rb:31:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/s3_expect_100_continue.rb:21:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/s3_bucket_name_restrictions.rb:12:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/s3_bucket_dns.rb:31:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/rest/handler.rb:7:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/user_agent.rb:12:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/seahorse/client/plugins/endpoint.rb:41:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/param_validator.rb:21:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/seahorse/client/plugins/raise_response_errors.rb:14:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/s3_sse_cpk.rb:19:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/s3_dualstack.rb:24:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/s3_accelerate.rb:34:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/jsonvalue_converter.rb:20:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/idempotency_token.rb:18:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/aws-sdk-core/plugins/param_converter.rb:20:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/seahorse/client/plugins/response_target.rb:21:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/seahorse/client/request.rb:70:in `send_request'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.115/lib/seahorse/client/base.rb:207:in `block (2 levels) in define_operation_methods'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.115/lib/aws-sdk-resources/services/s3/file_uploader.rb:42:in `block in put_object'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.115/lib/aws-sdk-resources/services/s3/file_uploader.rb:49:in `open_file'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.115/lib/aws-sdk-resources/services/s3/file_uploader.rb:41:in `put_object'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.115/lib/aws-sdk-resources/services/s3/file_uploader.rb:34:in `upload'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.115/lib/aws-sdk-resources/services/s3/object.rb:252:in `upload_file'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-s3-1.10.0/lib/dpl/provider/s3.rb:99:in `block (2 levels) in upload_multithreaded'
failed to deploy
