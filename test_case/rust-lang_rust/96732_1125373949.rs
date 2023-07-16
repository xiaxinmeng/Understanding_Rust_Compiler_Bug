plain
Updating files:  98% (32355/33015)
Updating files:  99% (32685/33015)
Updating files: 100% (33015/33015)
Updating files: 100% (33015/33015), done.
Switched to a new branch 'try'
branch 'try' set up to track 'origin/try'.
[command]/usr/local/bin/git log -1 --format='%H'
'771ef42acff5fb9e0bc90317b84c8c28917b696f'
##[group]Run src/ci/scripts/setup-environment.sh
src/ci/scripts/setup-environment.sh
---
      Memory: 14 GB
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMUVfC0dTrjr
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---
hw.physicalcpu_max: 3
hw.serialdebugmode: 0
hw.tbfrequency: 1000000000
hw.use_kernelmanagerd: 1
+ python src/ci/pgo-dist.py --host=x86_64-apple-darwin --target=x86_64-apple-darwin
  File "src/ci/pgo-dist.py", line 27
    *sys.argv[1:]
SyntaxError: invalid syntax
