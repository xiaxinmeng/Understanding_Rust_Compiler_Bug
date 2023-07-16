
$ git checkout origin/master
$ ./x.py
info: Downloading and building bootstrap before processing --help
      command. See src/bootstrap/README.md for help with common
      commands.
Updating only changed submodules
Updating submodule src/llvm-project
<snip>
$ git checkout origin/pr/80585
$ ./x.py
Failed to execute process './x.py'. Reason:
exec: unknown error (errno was 8)
The file './x.py' is marked as an executable but could not be run by the operating system.
