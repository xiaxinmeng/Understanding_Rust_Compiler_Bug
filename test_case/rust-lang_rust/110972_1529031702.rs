plain
drwxrwxrwt 16 root   root   4.0K Apr 30 13:56 ..
-rw-r--r--  1 runner runner  60K Apr 30 13:56 cpu-mingw-check.csv
-rw-r--r--  1 runner runner 982K Apr 30 13:56 metrics-mingw-check.json

Attempting with retry: aws s3 cp --storage-class INTELLIGENT_TIERING --no-progress --recursive --acl public-read /tmp/tmp.ltaCUsUpS7 s3://rust-lang-ci2/rustc-builds/200be0aeacba7e41d86347224b22163f6cb8b696

<botocore.awsrequest.AWSRequest object at 0x7ff2a3a94210>


<botocore.awsrequest.AWSRequest object at 0x7f1168064090>


<botocore.awsrequest.AWSRequest object at 0x7f22be3602d0>


<botocore.awsrequest.AWSRequest object at 0x7f7ef9d802d0>


<botocore.awsrequest.AWSRequest object at 0x7f4fc91f0110>
##[error]Process completed with exit code 1.
Post job cleanup.
