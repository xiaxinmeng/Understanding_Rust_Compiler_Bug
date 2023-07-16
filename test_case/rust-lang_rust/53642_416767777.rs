plain
[00:05:17]       Memory: 8 GB
[00:05:17]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:05:17]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:05:17]       SMC Version (system): 2.8f0
[00:05:17]       Serial Number (system): VMwYzrXKAp8P
[00:05:17] 
[00:05:17] hw.ncpu: 4
[00:05:17] hw.byteorder: 1234
[00:05:17] hw.memsize: 8589934592
---
$ rvm $(travis_internal_ruby) --fuzzy do ruby -S gem install dpl
Warning, new version of rvm available '1.29.4', you are using older version '1.29.3'.
You can disable this warning with:    echo rvm_autoupdate_flag=0 >> ~/.rvmrc
You can enable  auto-update  with:    echo rvm_autoupdate_flag=2 >> ~/.rvmrc
ERROR:  Could not find a valid gem 'dpl' (>= 0), here is why:
          Unable to download data from https://rubygems.org/ - no such name (https://rubygems.org/specs.4.8.gz)


The command "rvm $(travis_internal_ruby) --fuzzy do ruby -S gem install dpl" failed and exited with 2 during .
Your build has been stopped.
