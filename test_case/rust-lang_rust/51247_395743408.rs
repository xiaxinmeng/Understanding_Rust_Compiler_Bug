plain
##################################################################        92.4%
######################################################################## 100.0%
[00:08:53] extracting /checkout/obj/build/cache/2018-05-10/rustc-beta-x86_64-unknown-linux-gnu.tar.gz
[00:11:01] downloading https://static.rust-lang.org/dist/2018-05-10/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:11:29] Warning: Transient problem: timeout Will retry in 1 seconds. 3 retries left.
[00:12:09] Warning: Transient problem: timeout Will retry in 2 seconds. 2 retries left.
########################################                                  56.8%
######################################################################## 100.0%
[00:12:12] extracting /checkout/obj/build/cache/2018-05-10/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:12:12]     Updating registry `https://github.com/rust-lang/crates.io-index`
---
uploading "cf1868eb5551f209adc10fb964d7f7712dc6937b/rust-src-nightly.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "cf1868eb5551f209adc10fb964d7f7712dc6937b/rustc-nightly-armv7-unknown-linux-gnueabihf.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "cf1868eb5551f209adc10fb964d7f7712dc6937b/rustfmt-nightly-armv7-unknown-linux-gnueabihf.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "cf1868eb5551f209adc10fb964d7f7712dc6937b/rust-nightly-armv7-unknown-linux-gnueabihf.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
/home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.64/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:81:in `abort_upload': multipart upload failed: execution expired (Aws::S3::MultipartUploadError)
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.64/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:70:in `upload_parts'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.64/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:44:in `upload'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.64/lib/aws-sdk-resources/services/s3/file_uploader.rb:32:in `upload'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.64/lib/aws-sdk-resources/services/s3/object.rb:252:in `upload_file'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-s3-1.9.7/lib/dpl/provider/s3.rb:99:in `block (2 levels) in upload_multithreaded'
failed to deploy
