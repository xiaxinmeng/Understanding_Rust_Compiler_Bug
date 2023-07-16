
rust/src$ git status
On branch master
Your branch is up-to-date with 'upstream/master'.
Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git checkout -- <file>..." to discard changes in working directory)

	modified:   librustdoc/html/static/favicon.ico
	modified:   librustdoc/html/static/rust-logo.png

no changes added to commit (use "git add" and/or "git commit -a")
rust/src$ rm librustdoc/html/static/favicon.ico librustdoc/html/static/rust-logo.png
rust/src$ git checkout .
rust/src$ git status
On branch master
Your branch is up-to-date with 'upstream/master'.
Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git checkout -- <file>..." to discard changes in working directory)

	modified:   librustdoc/html/static/favicon.ico
	modified:   librustdoc/html/static/rust-logo.png

no changes added to commit (use "git add" and/or "git commit -a")
