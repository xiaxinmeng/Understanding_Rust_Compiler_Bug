plain
uploading "d753a8c4926ac2a5277409fc1240a45fe32992c9/rustc-nightly-x86_64-unknown-linux-gnu.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "d753a8c4926ac2a5277409fc1240a45fe32992c9/cargo-nightly-x86_64-unknown-linux-gnu.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "d753a8c4926ac2a5277409fc1240a45fe32992c9/rustc-docs-nightly-x86_64-unknown-linux-gnu.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "d753a8c4926ac2a5277409fc1240a45fe32992c9/rust-std-nightly-x86_64-unknown-linux-gnu.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
/home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.106/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:81:in `abort_upload': multipart upload failed: execution expired (Aws::S3::MultipartUploadError)
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.106/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:70:in `upload_parts'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.106/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:44:in `upload'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.106/lib/aws-sdk-resources/services/s3/file_uploader.rb:32:in `upload'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.106/lib/aws-sdk-resources/services/s3/object.rb:252:in `upload_file'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-s3-1.9.8/lib/dpl/provider/s3.rb:99:in `block (2 levels) in upload_multithreaded'
failed to deploy
