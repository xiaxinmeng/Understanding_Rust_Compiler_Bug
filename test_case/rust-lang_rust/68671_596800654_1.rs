
- bash: src/ci/scripts/install-clang.sh
  displayName: Install clang
  condition: and(succeeded(), not(variables.SKIP_JOB))
