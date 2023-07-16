plain
[00:04:00]       Memory: 8 GB
[00:04:00]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:04:00]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:04:00]       SMC Version (system): 2.8f0
[00:04:00]       Serial Number (system): VMSUw/Gm0wVB
[00:04:00] 
[00:04:00] hw.ncpu: 4
[00:04:00] hw.byteorder: 1234
[00:04:00] hw.memsize: 8589934592
---
uploading "cd6812e1d19fa95ac27c8936158ed3a447367579/clippy-nightly-i686-apple-darwin.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "cd6812e1d19fa95ac27c8936158ed3a447367579/clippy-nightly-i686-apple-darwin.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "cd6812e1d19fa95ac27c8936158ed3a447367579/cargo-nightly-i686-apple-darwin.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "cd6812e1d19fa95ac27c8936158ed3a447367579/cargo-nightly-i686-apple-darwin.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
/Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.136/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:81:in `abort_upload': multipart upload failed: Encountered a `SocketError` while attempting to connect to: (Aws::S3::MultipartUploadError)
  https://rust-lang-ci2.s3.us-west-1.amazonaws.com/rustc-builds/cd6812e1d19fa95ac27c8936158ed3a447367579/rust-std-nightly-i686-apple-darwin.tar.xz?partNumber=2&uploadId=1gWMYLcN5MIIKc.1dEXzTeFfCK2gGZ0MzdtyCGCBw3KAIybK3Q_xqUMRGdkyIbMkN.64ZjIlkmFnpLkWnCyTBIGuk_JSNi11RphiUyTQ6b7HJpqbRAR.7CNH880JEZms
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
cn-north-1
cn-north-1
cn-northwest-1
us-gov-west-1
; Encountered a `SocketError` while attempting to connect to:
  https://rust-lang-ci2.s3.us-west-1.amazonaws.com/rustc-builds/cd6812e1d19fa95ac27c8936158ed3a447367579/rust-std-nightly-i686-apple-darwin.tar.xz?partNumber=7&uploadId=1gWMYLcN5MIIKc.1dEXzTeFfCK2gGZ0MzdtyCGCBw3KAIybK3Q_xqUMRGdkyIbMkN.64ZjIlkmFnpLkWnCyTBIGuk_JSNi11RphiUyTQ6b7HJpqbRAR.7CNH880JEZms
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
cn-north-1
cn-north-1
cn-northwest-1
us-gov-west-1
; Encountered a `SocketError` while attempting to connect to:
  https://rust-lang-ci2.s3.us-west-1.amazonaws.com/rustc-builds/cd6812e1d19fa95ac27c8936158ed3a447367579/rust-std-nightly-i686-apple-darwin.tar.xz?partNumber=4&uploadId=1gWMYLcN5MIIKc.1dEXzTeFfCK2gGZ0MzdtyCGCBw3KAIybK3Q_xqUMRGdkyIbMkN.64ZjIlkmFnpLkWnCyTBIGuk_JSNi11RphiUyTQ6b7HJpqbRAR.7CNH880JEZms
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
cn-north-1
cn-north-1
cn-northwest-1
us-gov-west-1
; Encountered a `SocketError` while attempting to connect to:
  https://rust-lang-ci2.s3.us-west-1.amazonaws.com/rustc-builds/cd6812e1d19fa95ac27c8936158ed3a447367579/rust-std-nightly-i686-apple-darwin.tar.xz?partNumber=8&uploadId=1gWMYLcN5MIIKc.1dEXzTeFfCK2gGZ0MzdtyCGCBw3KAIybK3Q_xqUMRGdkyIbMkN.64ZjIlkmFnpLkWnCyTBIGuk_JSNi11RphiUyTQ6b7HJpqbRAR.7CNH880JEZms
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
cn-north-1
cn-north-1
cn-northwest-1
us-gov-west-1
; Encountered a `SocketError` while attempting to connect to:
  https://rust-lang-ci2.s3.us-west-1.amazonaws.com/rustc-builds/cd6812e1d19fa95ac27c8936158ed3a447367579/rust-std-nightly-i686-apple-darwin.tar.xz?partNumber=5&uploadId=1gWMYLcN5MIIKc.1dEXzTeFfCK2gGZ0MzdtyCGCBw3KAIybK3Q_xqUMRGdkyIbMkN.64ZjIlkmFnpLkWnCyTBIGuk_JSNi11RphiUyTQ6b7HJpqbRAR.7CNH880JEZms
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
cn-north-1
cn-north-1
cn-northwest-1
us-gov-west-1
; Encountered a `SocketError` while attempting to connect to:
  https://rust-lang-ci2.s3.us-west-1.amazonaws.com/rustc-builds/cd6812e1d19fa95ac27c8936158ed3a447367579/rust-std-nightly-i686-apple-darwin.tar.xz?partNumber=6&uploadId=1gWMYLcN5MIIKc.1dEXzTeFfCK2gGZ0MzdtyCGCBw3KAIybK3Q_xqUMRGdkyIbMkN.64ZjIlkmFnpLkWnCyTBIGuk_JSNi11RphiUyTQ6b7HJpqbRAR.7CNH880JEZms
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
cn-north-1
cn-north-1
cn-northwest-1
us-gov-west-1
; Encountered a `SocketError` while attempting to connect to:
  https://rust-lang-ci2.s3.us-west-1.amazonaws.com/rustc-builds/cd6812e1d19fa95ac27c8936158ed3a447367579/rust-std-nightly-i686-apple-darwin.tar.xz?partNumber=9&uploadId=1gWMYLcN5MIIKc.1dEXzTeFfCK2gGZ0MzdtyCGCBw3KAIybK3Q_xqUMRGdkyIbMkN.64ZjIlkmFnpLkWnCyTBIGuk_JSNi11RphiUyTQ6b7HJpqbRAR.7CNH880JEZms
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
cn-north-1
cn-north-1
cn-northwest-1
us-gov-west-1
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.136/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:70:in `upload_parts'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.136/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:44:in `upload'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.136/lib/aws-sdk-resources/services/s3/file_uploader.rb:32:in `upload'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/aws-sdk-resources-2.11.136/lib/aws-sdk-resources/services/s3/object.rb:252:in `upload_file'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/dpl-s3-1.10.1/lib/dpl/provider/s3.rb:106:in `block (2 levels) in upload_multithreaded'
failed to deploy
failed to deploy
No output has been received in the last 30m0s, this potentially indicates a stalled build or something wrong with the build itself.
Check the details on how to adjust your build configuration on: https://docs.travis-ci.com/user/common-build-problems/#Build-times-out-because-no-output-was-received
The build has been terminated
