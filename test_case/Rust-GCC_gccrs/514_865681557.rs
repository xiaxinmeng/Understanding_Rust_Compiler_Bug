
$ git clang-format -h
usage: git clang-format [OPTIONS] [<commit>] [<commit>] [--] [<file>...]

If zero or one commits are given, run clang-format on all lines that differ
between the working directory and <commit>, which defaults to HEAD.  Changes are
only applied to the working directory.

If two commits are given (requires --diff), run clang-format on all lines in the
second <commit> that differ from the first <commit>.

The following git-config settings set the default of the corresponding option:
  clangFormat.binary
  clangFormat.commit
  clangFormat.extensions
  clangFormat.style

positional arguments:
  <commit>              revision from which to compute the diff
  <file>...             if specified, only consider differences in these files

optional arguments:
  -h, --help            show this help message and exit
  --binary BINARY       path to clang-format
  --commit COMMIT       default commit to use if none is specified
  --diff                print a diff instead of applying the changes
  --extensions EXTENSIONS
                        comma-separated list of file extensions to format, excluding the period and case-insensitive
  -f, --force           allow changes to unstaged files
  -p, --patch           select hunks interactively
  -q, --quiet           print less information
  --style STYLE         passed to clang-format
  -v, --verbose         print extra information
