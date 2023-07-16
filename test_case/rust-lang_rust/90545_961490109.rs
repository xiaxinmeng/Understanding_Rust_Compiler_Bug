
git remote add upstream git@github.com:rust-lang/rust.git # only if you haven't added it already
git fetch upstream
git rebase upstream/master # this will prompt you to resolve conflicts if needed
git push -f
