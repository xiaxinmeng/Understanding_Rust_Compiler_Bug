plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:26403f80
$ git clone --depth=2 --branch=master https://github.com/rust-lang/rust.git rust-lang/rust
---
  fi
travis_time:end:0002623e:start=1550398500132386472,finish=1550398500139270972,duration=6884500
travis_fold:end:before_script.3
travis_time:start:0ba89b50
$ MESSAGE_FILE=$(mktemp -t msg.XXXXXX); . src/ci/docker/x86_64-gnu-tools/repo.sh; commit_toolstate_change "$MESSAGE_FILE" "$TRAVIS_BUILD_DIR/src/tools/publish_toolstate.py" "$(git rev-parse HEAD)" "$(git log --format=%s -n1 HEAD)" "$MESSAGE_FILE" "$TOOLSTATE_REPO_ACCESS_TOKEN";
