
[00:04:12]       Memory: 8 GB
[00:04:12]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:04:12]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:04:12]       SMC Version (system): 2.8f0
[00:04:12]       Serial Number (system): VMzcdHi9sZvn
[00:04:12] 
[00:04:12] hw.ncpu: 4
[00:04:12] hw.byteorder: 1234
[00:04:12] hw.memsize: 8589934592
---
$ rvm $(travis_internal_ruby) --fuzzy do ruby -S gem install dpl
Warning, new version of rvm available '1.29.4', you are using older version '1.29.3'.
You can disable this warning with:    echo rvm_autoupdate_flag=0 >> ~/.rvmrc
You can enable  auto-update  with:    echo rvm_autoupdate_flag=2 >> ~/.rvmrc
ERROR:  While executing gem ... (Gem::RemoteFetcher::FetchError)
    Errno::EHOSTUNREACH: Failed to open TCP connection to api.rubygems.org:443 (No route to host - connect(2) for "api.rubygems.org" port 443) (https://api.rubygems.org/quick/Marshal.4.8/dpl-1.9.7.gemspec.rz)

