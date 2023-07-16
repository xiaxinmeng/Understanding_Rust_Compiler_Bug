plain
uploading "e745cf17cdfc02f86c2ce268ecac129e7f3b3e63/rust-analysis-nightly-i686-unknown-freebsd.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "e745cf17cdfc02f86c2ce268ecac129e7f3b3e63/rustc-nightly-i686-unknown-freebsd.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "e745cf17cdfc02f86c2ce268ecac129e7f3b3e63/cargo-nightly-i686-unknown-freebsd.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "e745cf17cdfc02f86c2ce268ecac129e7f3b3e63/rust-nightly-i686-unknown-freebsd.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
/home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.63/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:81:in `abort_upload': multipart upload failed: execution expired (Aws::S3::MultipartUploadError)
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.63/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:70:in `upload_parts'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.63/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:44:in `upload'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.63/lib/aws-sdk-resources/services/s3/file_uploader.rb:32:in `upload'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.63/lib/aws-sdk-resources/services/s3/object.rb:252:in `upload_file'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-s3-1.9.7/lib/dpl/provider/s3.rb:99:in `block (2 levels) in upload_multithreaded'
failed to deploy
