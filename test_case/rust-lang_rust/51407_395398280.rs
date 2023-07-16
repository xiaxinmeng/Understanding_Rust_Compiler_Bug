plain
travis_fold:start:before_install.1
travis_time:start:00b188bc
$ if [ "$TRAVIS_OS_NAME" = linux ]; then pip install --user awscli; export PATH=$PATH:$HOME/.local/bin; fi
Collecting awscli
  Retrying (Retry(total=4, connect=None, read=None, redirect=None)) after connection broken by 'ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7f4ca230f1d0>, 'Connection to pypi.python.org timed out. (connect timeout=15)')': /simple/awscli/
  SNIMissingWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
---
[01:57:02]    Compiling bitflags v0.7.0
[01:57:02] [RUSTC-TIMING] bitflags test:false 0.108
[01:57:02]    Compiling rustc-ap-syntax v149.0.0
[01:57:04] [RUSTC-TIMING] term test:false 4.881
[01:57:04]    Compiling rustfmt-nightly v0.8.2 (https://github.com/rust-lang-nursery/rustfmt?rev=f3906267#f3906267)
[01:57:06] [RUSTC-TIMING] nibble_vec test:false 1.005
[01:57:06]    Compiling endian-type v0.1.2
[01:57:06]    Compiling rls v0.128.0 (file:///checkout/src/tools/rls)
[01:57:06] [RUSTC-TIMING] endian_type test:false 0.327
---
Preparing deploy
travis_fold:end:dpl.2
travis_fold:start:dpl.3
Deploying application
uploading "39df8d6634394e9e1e03be54da0a1ceae616d6d9/cargo-nightly-mips64el-unknown-linux-gnuabi64.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "39df8d6634394e9e1e03be54da0a1ceae616d6d9/rust-analysis-nightly-mips64el-unknown-linux-gnuabi64.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}uploading "39df8d6634394e9e1e03be54da0a1ceae616d6d9/rust-nightly-mips64el-unknown-linux-gnuabi64.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "39df8d6634394e9e1e03be54da0a1ceae616d6d9/rls-nightly-mips64el-unknown-linux-gnuabi64.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "39df8d6634394e9e1e03be54da0a1ceae616d6d9/rust-src-nightly.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "39df8d6634394e9e1e03be54da0a1ceae616d6d9/rust-analysis-nightly-mips64el-unknown-linux-gnuabi64.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "39df8d6634394e9e1e03be54da0a1ceae616d6d9/rustc-nightly-mips64el-unknown-linux-gnuabi64.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "39df8d6634394e9e1e03be54da0a1ceae616d6d9/rustfmt-nightly-mips64el-unknown-linux-gnuabi64.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "39df8d6634394e9e1e03be54da0a1ceae616d6d9/rustc-nightly-mips64el-unknown-linux-gnuabi64.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "39df8d6634394e9e1e03be54da0a1ceae616d6d9/rustfmt-nightly-mips64el-unknown-linux-gnuabi64.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "39df8d6634394e9e1e03be54da0a1ceae616d6d9/rust-nightly-mips64el-unknown-linux-gnuabi64.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "39df8d6634394e9e1e03be54da0a1ceae616d6d9/rust-std-nightly-mips64el-unknown-linux-gnuabi64.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "39df8d6634394e9e1e03be54da0a1ceae616d6d9/cargo-nightly-mips64el-unknown-linux-gnuabi64.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "39df8d6634394e9e1e03be54da0a1ceae616d6d9/rust-src-nightly.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "39df8d6634394e9e1e03be54da0a1ceae616d6d9/rls-nightly-mips64el-unknown-linux-gnuabi64.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "39df8d6634394e9e1e03be54da0a1ceae616d6d9/rust-std-nightly-mips64el-unknown-linux-gnuabi64.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
/home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.63/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:81:in `abort_upload': multipart upload failed: execution expired (Aws::S3::MultipartUploadError)
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.63/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:70:in `upload_parts'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.63/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:44:in `upload'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.63/lib/aws-sdk-resources/services/s3/file_uploader.rb:32:in `upload'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.63/lib/aws-sdk-resources/services/s3/object.rb:252:in `upload_file'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-s3-1.9.7/lib/dpl/provider/s3.rb:99:in `block (2 levels) in upload_multithreaded'
failed to deploy
