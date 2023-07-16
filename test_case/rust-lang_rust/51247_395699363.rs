plain
[00:08:16] sha256:5e8b97a2a0820b10338bd91674249a94679e4568fd1183ea46acff63b9883e9c
[00:08:16] Attempting with retry: docker build --rm -t rust-ci -f /home/travis/build/rust-lang/rust/src/ci/docker/dist-mips64el-linux/Dockerfile /home/travis/build/rust-lang/rust/src/ci/docker
[00:08:16] Sending build context to Docker daemon  501.2kB
[00:08:16] Step 1/7 : FROM ubuntu:16.04
[00:08:31] Get https://registry-1.docker.io/v2/: net/http: request canceled while waiting for connection (Client.Timeout exceeded while awaiting headers)
[00:08:32] Sending build context to Docker daemon  501.2kB
[00:08:32] Step 1/7 : FROM ubuntu:16.04
[00:08:36] 16.04: Pulling from library/ubuntu
[00:08:40] Digest: sha256:b050c1822d37a4463c01ceda24d0fc4c679b0dd3c43e742730e2884d3c582e3a
---
uploading "8806d6d9e2fe1ba0e0c917334d1918fca21c7ed0/cargo-nightly-mips64el-unknown-linux-gnuabi64.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "8806d6d9e2fe1ba0e0c917334d1918fca21c7ed0/rust-src-nightly.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "8806d6d9e2fe1ba0e0c917334d1918fca21c7ed0/rls-nightly-mips64el-unknown-linux-gnuabi64.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "8806d6d9e2fe1ba0e0c917334d1918fca21c7ed0/rust-std-nightly-mips64el-unknown-linux-gnuabi64.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
/home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.64/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:81:in `abort_upload': multipart upload failed: execution expired (Aws::S3::MultipartUploadError)
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.64/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:70:in `upload_parts'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.64/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:44:in `upload'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.64/lib/aws-sdk-resources/services/s3/file_uploader.rb:32:in `upload'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.64/lib/aws-sdk-resources/services/s3/object.rb:252:in `upload_file'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-s3-1.9.7/lib/dpl/provider/s3.rb:99:in `block (2 levels) in upload_multithreaded'
failed to deploy
