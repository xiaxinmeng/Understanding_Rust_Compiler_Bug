plain
2019-11-05T23:45:42.5786020Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-11-05T23:45:42.5786150Z ==============================================================================
2019-11-05T23:45:42.7849300Z Generating script.
2019-11-05T23:45:42.7877570Z Script contents:
2019-11-05T23:45:42.7878450Z src/ci/scripts/run-build-from-ci.sh
2019-11-05T23:45:42.7932790Z [command]/bin/bash --noprofile --norc /Users/runner/runners/2.160.0/work/_temp/0fe79892-05c8-4f50-88e9-8ed4c1857db7.sh
2019-11-05T23:45:42.7932790Z [command]/bin/bash --noprofile --norc /Users/runner/runners/2.160.0/work/_temp/0fe79892-05c8-4f50-88e9-8ed4c1857db7.sh
2019-11-05T23:45:42.8249060Z src/ci/scripts/run-build-from-ci.sh: line 16: rustup: command not found
2019-11-05T23:45:42.8256490Z src/ci/scripts/run-build-from-ci.sh: line 17: IMAGE: unbound variable
2019-11-05T23:45:42.8289350Z 
2019-11-05T23:45:42.8447220Z ##[error]Bash exited with code '1'.
2019-11-05T23:45:42.8498760Z ##[section]Starting: Checkout
2019-11-05T23:45:42.8501760Z ==============================================================================
2019-11-05T23:45:42.8501970Z Task         : Get sources
2019-11-05T23:45:42.8502070Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
