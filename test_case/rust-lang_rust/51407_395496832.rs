plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/50/09/c53398e0005b11f7ffb27b7aa720c617aba53be4fb4f4f3f06b9b5c60f28/docutils-0.14-py2-none-any.whl (543kB)
Collecting botocore==1.10.33 (from awscli)
  Retrying (Retry(total=4, connect=None, read=None, redirect=None)) after connection broken by 'ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7ff553ac71d0>, 'Connection to pypi.python.org timed out. (connect timeout=15)')': /simple/botocore/
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/79/91/aa2731dc3d2ed94d8fa3ff934fb0d2fd1d03171f57be95c464dcfeb2d623/botocore-1.10.33-py2.py3-none-any.whl (4.3MB)
Collecting pyasn1>=0.1.3 (from rsa<=3.5.0,>=3.1.2->awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
---
travis_fold:start:before_deploy.3
travis_time:start:076c2f94
$ travis_retry gem update --system
ERROR:  Error installing rubygems-update:
 Unable to resolve dependency: user requested 'rubygems-update (= 2.7.7)'
ERROR:  While executing gem ... (NoMethodError)
    undefined method `version' for nil:NilClass
The command "gem update --system" failed. Retrying, 2 of 3.
Updating rubygems-update
Successfully installed rubygems-update-2.7.7
Installing RubyGems 2.7.7
---
uploading "9ee195925ca16f3a6615027e234216c9b7888bdc/rust-std-nightly-sparc64-unknown-linux-gnu.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "9ee195925ca16f3a6615027e234216c9b7888bdc/rust-std-nightly-wasm32-unknown-emscripten.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "9ee195925ca16f3a6615027e234216c9b7888bdc/rust-std-nightly-arm-unknown-linux-musleabihf.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "9ee195925ca16f3a6615027e234216c9b7888bdc/rust-std-nightly-armv5te-unknown-linux-gnueabi.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
/home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.63/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:81:in `abort_upload': multipart upload failed: execution expired (Aws::S3::MultipartUploadError)
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.63/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:70:in `upload_parts'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.63/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:44:in `upload'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.63/lib/aws-sdk-resources/services/s3/file_uploader.rb:32:in `upload'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.63/lib/aws-sdk-resources/services/s3/object.rb:252:in `upload_file'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-s3-1.9.7/lib/dpl/provider/s3.rb:99:in `block (2 levels) in upload_multithreaded'
failed to deploy
