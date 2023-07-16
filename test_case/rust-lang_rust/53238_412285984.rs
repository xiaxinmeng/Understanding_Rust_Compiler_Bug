plain
Preparing deploy
travis_fold:end:dpl.2
travis_fold:start:dpl.3
Deploying application
uploading "87e3cf5f4f72966ffb08ed516254d589281c6539/rust-analysis-nightly-arm-linux-androideabi.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "87e3cf5f4f72966ffb08ed516254d589281c6539/rust-analysis-nightly-armv7-linux-androideabi.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "87e3cf5f4f72966ffb08ed516254d589281c6539/rust-std-nightly-armv7-linux-androideabi.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "87e3cf5f4f72966ffb08ed516254d589281c6539/rust-std-nightly-aarch64-linux-android.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "87e3cf5f4f72966ffb08ed516254d589281c6539/rust-analysis-nightly-aarch64-linux-android.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "87e3cf5f4f72966ffb08ed516254d589281c6539/rust-std-nightly-aarch64-linux-android.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "87e3cf5f4f72966ffb08ed516254d589281c6539/rust-std-nightly-i686-linux-android.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "87e3cf5f4f72966ffb08ed516254d589281c6539/rust-std-nightly-x86_64-linux-android.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "87e3cf5f4f72966ffb08ed516254d589281c6539/rust-analysis-nightly-aarch64-linux-android.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "87e3cf5f4f72966ffb08ed516254d589281c6539/rust-analysis-nightly-arm-linux-androideabi.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "87e3cf5f4f72966ffb08ed516254d589281c6539/rust-std-nightly-i686-linux-android.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "87e3cf5f4f72966ffb08ed516254d589281c6539/rust-std-nightly-armv7-linux-androideabi.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "87e3cf5f4f72966ffb08ed516254d589281c6539/rust-std-nightly-arm-linux-androideabi.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "87e3cf5f4f72966ffb08ed516254d589281c6539/rust-analysis-nightly-i686-linux-android.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "87e3cf5f4f72966ffb08ed516254d589281c6539/rust-analysis-nightly-x86_64-linux-android.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "87e3cf5f4f72966ffb08ed516254d589281c6539/rust-std-nightly-arm-linux-androideabi.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "87e3cf5f4f72966ffb08ed516254d589281c6539/rust-std-nightly-x86_64-linux-android.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "87e3cf5f4f72966ffb08ed516254d589281c6539/rust-analysis-nightly-armv7-linux-androideabi.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "87e3cf5f4f72966ffb08ed516254d589281c6539/rust-analysis-nightly-x86_64-linux-android.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "87e3cf5f4f72966ffb08ed516254d589281c6539/rust-analysis-nightly-i686-linux-android.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
/home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.106/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:81:in `abort_upload': multipart upload failed: execution expired (Aws::S3::MultipartUploadError)
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.106/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:70:in `upload_parts'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.106/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:44:in `upload'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.106/lib/aws-sdk-resources/services/s3/file_uploader.rb:32:in `upload'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.106/lib/aws-sdk-resources/services/s3/object.rb:252:in `upload_file'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-s3-1.9.8/lib/dpl/provider/s3.rb:99:in `block (2 levels) in upload_multithreaded'
failed to deploy
