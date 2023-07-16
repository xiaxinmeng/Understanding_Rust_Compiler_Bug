 json
{
  "cmd": "make",
  "name": "rust",
  "args": [ "check", "TESTNAME=test_process_mask"],
  "sh": true,
  "cwd": "{PROJECT_PATH}",
  "env": {
    "RUST_BACKTRACE": "1",
  },
}
