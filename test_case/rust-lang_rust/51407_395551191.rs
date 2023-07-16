plain
travis_fold:end:before_deploy.2
travis_fold:start:before_deploy.3
travis_time:start:1509a480
$ travis_retry gem update --system
ERROR:  While executing gem ... (Gem::RemoteFetcher::FetchError)
    IOError: HTTP session not yet started (https://api.rubygems.org/quick/Marshal.4.8/rubygems-update-2.7.7.gemspec.rz)
The command "gem update --system" failed. Retrying, 2 of 3.
Updating rubygems-update
Successfully installed rubygems-update-2.7.7
Installing RubyGems 2.7.7
---
uploading "f4a7602125318b7f107c3177abb12371e97e8e43/rust-src-nightly.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "f4a7602125318b7f107c3177abb12371e97e8e43/rustc-nightly-armv7-unknown-linux-gnueabihf.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "f4a7602125318b7f107c3177abb12371e97e8e43/rustfmt-nightly-armv7-unknown-linux-gnueabihf.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "f4a7602125318b7f107c3177abb12371e97e8e43/rust-nightly-armv7-unknown-linux-gnueabihf.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
/home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.63/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:81:in `abort_upload': multipart upload failed: execution expired (Aws::S3::MultipartUploadError)
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.63/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:70:in `upload_parts'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.63/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:44:in `upload'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.63/lib/aws-sdk-resources/services/s3/file_uploader.rb:32:in `upload'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.63/lib/aws-sdk-resources/services/s3/object.rb:252:in `upload_file'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-s3-1.9.7/lib/dpl/provider/s3.rb:99:in `block (2 levels) in upload_multithreaded'
failed to deploy
