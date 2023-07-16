shell
; git clone git@github.com:rust-lang/rust.git git-bug
; cd git-bug
; git remote add fork git@github.com:WaffleLapkin/rust.git # replace with your own fork
; x setup compiler
; x c compiler # (optionally) cancel after all submodules are fetched
; git checkout HEAD~5000
; git switch -c new_test_branch_so_maybe_the_bug_will_show_up
; git commit --allow-empty -m "it's a commit, nothing more, nothing less"
; git push -u fork
; x c compiler
Updating only changed submodules
Updating submodule src/tools/rust-installer
fatal: 'fork' does not appear to be a git repository
fatal: Could not read from remote repository.

Please make sure you have the correct access rights
and the repository exists.
fatal: Fetched in submodule path 'src/tools/rust-installer', but it did not contain d66f476b4d5e7fdf1ec215c9ac16c923dc292324. Direct fetching of that commit failed.
Traceback (most recent call last):
  File "/home/waffle/projects/repos/git-bug/x.py", line 11, in <module>
    bootstrap.main()
  File "/home/waffle/projects/repos/git-bug/src/bootstrap/bootstrap.py", line 1009, in main
    bootstrap(help_triggered)
  File "/home/waffle/projects/repos/git-bug/src/bootstrap/bootstrap.py", line 969, in bootstrap
    build.update_submodules()
  File "/home/waffle/projects/repos/git-bug/src/bootstrap/bootstrap.py", line 845, in update_submodules
    self.update_submodule(module[0], module[1], recorded_submodules)
  File "/home/waffle/projects/repos/git-bug/src/bootstrap/bootstrap.py", line 797, in update_submodule
    run(update_args, cwd=self.rust_root, verbose=self.verbose, exception=True)
  File "/home/waffle/projects/repos/git-bug/src/bootstrap/bootstrap.py", line 143, in run
    raise RuntimeError(err)
RuntimeError: failed to run: git submodule update --init --recursive --progress src/tools/rust-installer

