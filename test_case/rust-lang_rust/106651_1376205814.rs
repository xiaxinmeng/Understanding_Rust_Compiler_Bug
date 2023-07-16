
$ git remote --verbose
[custom-name]	git@github.com:[github-user]/rust (fetch)
[custom-name]	git@github.com:[github-user]/rust (push)

$ git remote remove [custom-name]

$ git remote add origin git@github.com:[github-user]/rust

$ git remote add [custom-name] git@github.com:[github-user]/rust

$ git remote --verbose
[custom-name]	git@github.com:[github-user]/rust (fetch)
[custom-name]	git@github.com:[github-user]/rust (push)
origin	git@github.com:[github-user]/rust (fetch)
origin	git@github.com:[github-user]/rust (push)

$ python x.py build
