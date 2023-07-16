plain
[00:03:43]       Memory: 8 GB
[00:03:43]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:03:43]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:43]       SMC Version (system): 2.8f0
[00:03:43]       Serial Number (system): VMu3TxZLawDz
[00:03:43] 
[00:03:43] hw.ncpu: 4
[00:03:43] hw.byteorder: 1234
[00:03:43] hw.memsize: 8589934592
---
travis_fold:end:before_deploy.3
travis_fold:start:dpl_0
travis_time:start:069acacc
$ rvm $(travis_internal_ruby) --fuzzy do ruby -S gem install dpl
ERROR:  While executing gem ... (Gem::RemoteFetcher::FetchError)
    Errno::EHOSTUNREACH: Failed to open TCP connection to api.rubygems.org:443 (No route to host - connect(2) for "api.rubygems.org" port 443) (https://api.rubygems.org/quick/Marshal.4.8/dpl-1.10.0.gemspec.rz)


The command "rvm $(travis_internal_ruby) --fuzzy do ruby -S gem install dpl" failed and exited with 1 during .
Your build has been stopped.
