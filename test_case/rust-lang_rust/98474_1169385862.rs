
D:\git\rust-lang\rust>winget list python
Name            Id                                     Version     Available   Source
-------------------------------------------------------------------------------------
Python 3        Python.Python.3                        3.10.4150.0 3.10.5150.0 winget
Python Launcher {691AAAA1-FE86-4973-8DA2-6AA2B3327562} 3.10.7751.0

D:\git\rust-lang\rust>ftype | rg py
Python.ArchiveFile="C:\Windows\py.exe" "%L" %*
Python.CompiledFile="C:\Windows\py.exe" "%L" %*
Python.File="C:\Windows\py.exe" "%L" %*
Python.NoConArchiveFile="C:\Windows\pyw.exe" "%L" %*
Python.NoConFile="C:\Windows\pyw.exe" "%L" %*

D:\git\rust-lang\rust>py ./x.py --help
Unable to create process using '/usr/bin/env bash ./x.py --help'

D:\git\rust-lang\rust>x.py --help
Unable to create process using '/usr/bin/env bash "D:\git\rust-lang\rust\x.py"  --help'

D:\git\rust-lang\rust>py -3 ./x.py --help
info: Downloading and building bootstrap before processing --help
      command. See src/bootstrap/README.md for help with common
      commands.
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.06s
Usage: x.py <subcommand> [options] [<paths>...]
