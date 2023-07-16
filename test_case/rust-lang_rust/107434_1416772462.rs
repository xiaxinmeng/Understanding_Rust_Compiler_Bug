plain
drwxrwxrwt 14 root root 4.0K Feb  3 22:01 ..
-rw-r--r--  1 gha  gha  125K Feb  3 22:01 cpu-aarch64-gnu.csv
-rw-r--r--  1 gha  gha   99K Feb  3 22:01 metrics-aarch64-gnu.json

Attempting with retry: aws s3 cp --storage-class INTELLIGENT_TIERING --no-progress --recursive --acl public-read /tmp/tmp.GVaKeMDjnt s3://rust-lang-ci2/rustc-builds/90c1e6c689773c8d55031328f01ca276da9d157d
upload failed: ../../../../tmp/tmp.GVaKeMDjnt/cpu-aarch64-gnu.csv to s3://rust-lang-ci2/rustc-builds/90c1e6c689773c8d55031328f01ca276da9d157d/cpu-aarch64-gnu.csv Could not connect to the endpoint URL: "https://rust-lang-ci2.s3.amazonaws.com/rustc-builds/90c1e6c689773c8d55031328f01ca276da9d157d/cpu-aarch64-gnu.csv"
upload failed: ../../../../tmp/tmp.GVaKeMDjnt/metrics-aarch64-gnu.json to s3://rust-lang-ci2/rustc-builds/90c1e6c689773c8d55031328f01ca276da9d157d/metrics-aarch64-gnu.json Could not connect to the endpoint URL: "https://rust-lang-ci2.s3.amazonaws.com/rustc-builds/90c1e6c689773c8d55031328f01ca276da9d157d/metrics-aarch64-gnu.json"
Command failed. Attempt 2/5:
upload failed: ../../../../tmp/tmp.GVaKeMDjnt/cpu-aarch64-gnu.csv to s3://rust-lang-ci2/rustc-builds/90c1e6c689773c8d55031328f01ca276da9d157d/cpu-aarch64-gnu.csv Could not connect to the endpoint URL: "https://rust-lang-ci2.s3.amazonaws.com/rustc-builds/90c1e6c689773c8d55031328f01ca276da9d157d/cpu-aarch64-gnu.csv"
upload failed: ../../../../tmp/tmp.GVaKeMDjnt/metrics-aarch64-gnu.json to s3://rust-lang-ci2/rustc-builds/90c1e6c689773c8d55031328f01ca276da9d157d/metrics-aarch64-gnu.json Could not connect to the endpoint URL: "https://rust-lang-ci2.s3.amazonaws.com/rustc-builds/90c1e6c689773c8d55031328f01ca276da9d157d/metrics-aarch64-gnu.json"
Command failed. Attempt 3/5:
upload failed: ../../../../tmp/tmp.GVaKeMDjnt/cpu-aarch64-gnu.csv to s3://rust-lang-ci2/rustc-builds/90c1e6c689773c8d55031328f01ca276da9d157d/cpu-aarch64-gnu.csv Could not connect to the endpoint URL: "https://rust-lang-ci2.s3.us-west-1.amazonaws.com/rustc-builds/90c1e6c689773c8d55031328f01ca276da9d157d/cpu-aarch64-gnu.csv"
upload failed: ../../../../tmp/tmp.GVaKeMDjnt/metrics-aarch64-gnu.json to s3://rust-lang-ci2/rustc-builds/90c1e6c689773c8d55031328f01ca276da9d157d/metrics-aarch64-gnu.json Could not connect to the endpoint URL: "https://rust-lang-ci2.s3.us-west-1.amazonaws.com/rustc-builds/90c1e6c689773c8d55031328f01ca276da9d157d/metrics-aarch64-gnu.json"
Command failed. Attempt 4/5:
upload failed: ../../../../tmp/tmp.GVaKeMDjnt/cpu-aarch64-gnu.csv to s3://rust-lang-ci2/rustc-builds/90c1e6c689773c8d55031328f01ca276da9d157d/cpu-aarch64-gnu.csv Could not connect to the endpoint URL: "https://rust-lang-ci2.s3.us-west-1.amazonaws.com/rustc-builds/90c1e6c689773c8d55031328f01ca276da9d157d/cpu-aarch64-gnu.csv"
upload failed: ../../../../tmp/tmp.GVaKeMDjnt/metrics-aarch64-gnu.json to s3://rust-lang-ci2/rustc-builds/90c1e6c689773c8d55031328f01ca276da9d157d/metrics-aarch64-gnu.json Could not connect to the endpoint URL: "https://rust-lang-ci2.s3.us-west-1.amazonaws.com/rustc-builds/90c1e6c689773c8d55031328f01ca276da9d157d/metrics-aarch64-gnu.json"
Command failed. Attempt 5/5:
upload failed: ../../../../tmp/tmp.GVaKeMDjnt/cpu-aarch64-gnu.csv to s3://rust-lang-ci2/rustc-builds/90c1e6c689773c8d55031328f01ca276da9d157d/cpu-aarch64-gnu.csv Could not connect to the endpoint URL: "https://rust-lang-ci2.s3.us-west-1.amazonaws.com/rustc-builds/90c1e6c689773c8d55031328f01ca276da9d157d/cpu-aarch64-gnu.csv"
upload failed: ../../../../tmp/tmp.GVaKeMDjnt/metrics-aarch64-gnu.json to s3://rust-lang-ci2/rustc-builds/90c1e6c689773c8d55031328f01ca276da9d157d/metrics-aarch64-gnu.json Could not connect to the endpoint URL: "https://rust-lang-ci2.s3.us-west-1.amazonaws.com/rustc-builds/90c1e6c689773c8d55031328f01ca276da9d157d/metrics-aarch64-gnu.json"
##[error]Process completed with exit code 1.
Post job cleanup.
