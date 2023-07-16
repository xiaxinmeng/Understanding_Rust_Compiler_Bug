plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0c66cf34
$ git clone --depth=2 --branch=master https://github.com/rust-lang/rust.git rust-lang/rust
---
  fi
travis_time:end:03eb835e:start=1550257596557322948,finish=1550257596561747618,duration=4424670
travis_fold:end:before_script.3
travis_time:start:0bd2db50
$ MESSAGE_FILE=$(mktemp -t msg.XXXXXX); . src/ci/docker/x86_64-gnu-tools/repo.sh; commit_toolstate_change "$MESSAGE_FILE" "$TRAVIS_BUILD_DIR/src/tools/publish_toolstate.py" "$(git rev-parse HEAD)" "$(git log --format=%s -n1 HEAD)" "$MESSAGE_FILE" "$TOOLSTATE_REPO_ACCESS_TOKEN";
Traceback (most recent call last):
Traceback (most recent call last):
  File "/home/travis/build/rust-lang/rust/src/tools/publish_toolstate.py", line 196, in <module>
    cur_datetime
  File "/home/travis/build/rust-lang/rust/src/tools/publish_toolstate.py", line 143, in update_latest
    except IOError as (errno, strerror):
ValueError: need more than 0 values to unpack
