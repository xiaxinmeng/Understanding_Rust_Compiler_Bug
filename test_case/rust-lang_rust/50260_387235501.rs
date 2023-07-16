plain
[00:02:39]       Memory: 8 GB
[00:02:39]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:02:39]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:39]       SMC Version (system): 2.8f0
[00:02:39]       Serial Number (system): VMzSmZvXsph4
[00:02:39] 
[00:02:39] hw.ncpu: 4
[00:02:39] hw.byteorder: 1234
[00:02:39] hw.memsize: 8589934592
---
Preparing deploy
travis_fold:end:dpl.2
travis_fold:start:dpl.3
Deploying application
uploading "7264c79a1b43751df9f59954dc7092d35447e399/rustc-nightly-x86_64-apple-darwin.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "7264c79a1b43751df9f59954dc7092d35447e399/rustc-nightly-x86_64-apple-darwin.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}uploading "7264c79a1b43751df9f59954dc7092d35447e399/rustfmt-nightly-x86_64-apple-darwin.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "7264c79a1b43751df9f59954dc7092d35447e399/rust-std-nightly-x86_64-apple-darwin.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}uploading "7264c79a1b43751df9f59954dc7092d35447e399/rustfmt-nightly-x86_64-apple-darwin.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "7264c79a1b43751df9f59954dc7092d35447e399/rust-std-nightly-x86_64-apple-darwin.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "7264c79a1b43751df9f59954dc7092d35447e399/rust-src-nightly.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "7264c79a1b43751df9f59954dc7092d35447e399/rust-src-nightly.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "7264c79a1b43751df9f59954dc7092d35447e399/rust-nightly-x86_64-apple-darwin.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "7264c79a1b43751df9f59954dc7092d35447e399/rust-nightly-x86_64-apple-darwin.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "7264c79a1b43751df9f59954dc7092d35447e399/rust-nightly-x86_64-apple-darwin.pkg" with {:content_type=>"application/octet-stream", :acl=>"public-read"}
uploading "7264c79a1b43751df9f59954dc7092d35447e399/rust-docs-nightly-x86_64-apple-darwin.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "7264c79a1b43751df9f59954dc7092d35447e399/rust-docs-nightly-x86_64-apple-darwin.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "7264c79a1b43751df9f59954dc7092d35447e399/rust-analysis-nightly-x86_64-apple-darwin.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "7264c79a1b43751df9f59954dc7092d35447e399/rust-analysis-nightly-x86_64-apple-darwin.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "7264c79a1b43751df9f59954dc7092d35447e399/rls-nightly-x86_64-apple-darwin.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "7264c79a1b43751df9f59954dc7092d35447e399/rls-nightly-x86_64-apple-darwin.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "7264c79a1b43751df9f59954dc7092d35447e399/cargo-nightly-x86_64-apple-darwin.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "7264c79a1b43751df9f59954dc7092d35447e399/cargo-nightly-x86_64-apple-darwin.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
/Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.43/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:81:in `abort_upload': multipart upload failed: Encountered a `SocketError` while attempting to connect to: (Aws::S3::MultipartUploadError)
  https://rust-lang-ci2.s3.us-west-1.amazonaws.com/rustc-builds-alt/7264c79a1b43751df9f59954dc7092d35447e399/rust-docs-nightly-x86_64-apple-darwin.tar.gz?partNumber=1&uploadId=MrlFz57KdI4FgxkMWdusiZwScTak3ncY1krE8k02o9WJsYmbBQt92qKI3201l2rrnU5ggDIM3tfvS5XSIJJY7uNqMkQ5lBKncI5tAz.qz0GX_3cv.Qx0Sc9.lq5oMzVg
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
; Encountered a `SocketError` while attempting to connect to:
  https://rust-lang-ci2.s3.us-west-1.amazonaws.com/rustc-builds-alt/7264c79a1b43751df9f59954dc7092d35447e399/rust-docs-nightly-x86_64-apple-darwin.tar.gz?partNumber=3&uploadId=MrlFz57KdI4FgxkMWdusiZwScTak3ncY1krE8k02o9WJsYmbBQt92qKI3201l2rrnU5ggDIM3tfvS5XSIJJY7uNqMkQ5lBKncI5tAz.qz0GX_3cv.Qx0Sc9.lq5oMzVg
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
; Encountered a `SocketError` while attempting to connect to:
  https://rust-lang-ci2.s3.us-west-1.amazonaws.com/rustc-builds-alt/7264c79a1b43751df9f59954dc7092d35447e399/rust-docs-nightly-x86_64-apple-darwin.tar.gz?partNumber=4&uploadId=MrlFz57KdI4FgxkMWdusiZwScTak3ncY1krE8k02o9WJsYmbBQt92qKI3201l2rrnU5ggDIM3tfvS5XSIJJY7uNqMkQ5lBKncI5tAz.qz0GX_3cv.Qx0Sc9.lq5oMzVg
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
; Encountered a `SocketError` while attempting to connect to:
  https://rust-lang-ci2.s3.us-west-1.amazonaws.com/rustc-builds-alt/7264c79a1b43751df9f59954dc7092d35447e399/rust-docs-nightly-x86_64-apple-darwin.tar.gz?partNumber=5&uploadId=MrlFz57KdI4FgxkMWdusiZwScTak3ncY1krE8k02o9WJsYmbBQt92qKI3201l2rrnU5ggDIM3tfvS5XSIJJY7uNqMkQ5lBKncI5tAz.qz0GX_3cv.Qx0Sc9.lq5oMzVg
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
; Encountered a `SocketError` while attempting to connect to:
  https://rust-lang-ci2.s3.us-west-1.amazonaws.com/rustc-builds-alt/7264c79a1b43751df9f59954dc7092d35447e399/rust-docs-nightly-x86_64-apple-darwin.tar.gz?partNumber=6&uploadId=MrlFz57KdI4FgxkMWdusiZwScTak3ncY1krE8k02o9WJsYmbBQt92qKI3201l2rrnU5ggDIM3tfvS5XSIJJY7uNqMkQ5lBKncI5tAz.qz0GX_3cv.Qx0Sc9.lq5oMzVg
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
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.43/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:70:in `upload_parts'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.43/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:44:in `upload'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.43/lib/aws-sdk-resources/services/s3/file_uploader.rb:32:in `upload'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.43/lib/aws-sdk-resources/services/s3/object.rb:252:in `upload_file'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/dpl-s3-1.9.6/lib/dpl/provider/s3.rb:99:in `block (2 levels) in upload_multithreaded'
failed to deploy
failed to deploy
No output has been received in the last 30m0s, this potentially indicates a stalled build or something wrong with the build itself.
Check the details on how to adjust your build configuration on: https://docs.travis-ci.com/user/common-build-problems/#Build-times-out-because-no-output-was-received
The build has been terminated
