plain
uploading "a433cc8cf3b2b2adc38800eac2276bc670598b91/rust-docs-nightly-i386-apple-ios.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "a433cc8cf3b2b2adc38800eac2276bc670598b91/rust-docs-nightly-armv7s-apple-ios.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "a433cc8cf3b2b2adc38800eac2276bc670598b91/rust-docs-nightly-armv7s-apple-ios.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "a433cc8cf3b2b2adc38800eac2276bc670598b91/rust-docs-nightly-armv7-apple-ios.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
/Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.121/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:81:in `abort_upload': multipart upload failed: execution expired (Aws::S3::MultipartUploadError)
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.121/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:70:in `upload_parts'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.121/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:44:in `upload'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.121/lib/aws-sdk-resources/services/s3/file_uploader.rb:32:in `upload'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.121/lib/aws-sdk-resources/services/s3/object.rb:252:in `upload_file'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/dpl-s3-1.10.0/lib/dpl/provider/s3.rb:99:in `block (2 levels) in upload_multithreaded'
failed to deploy
failed to deploy
No output has been received in the last 30m0s, this potentially indicates a stalled build or something wrong with the build itself.
Check the details on how to adjust your build configuration on: https://docs.travis-ci.com/user/common-build-problems/#Build-times-out-because-no-output-was-received
The build has been terminated
