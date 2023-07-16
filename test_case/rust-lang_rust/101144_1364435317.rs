
git clone https://github.com/rust-lang/rust rust-not-a-worktree
cd rust-not-a-worktree
git remote add personal git@github.com:jyn514/rust
git checkout -b example-branch
git push -u personal example-branch --no-verify
x test tidy
