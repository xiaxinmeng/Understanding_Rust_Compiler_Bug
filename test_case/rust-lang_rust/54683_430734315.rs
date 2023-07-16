plain
casperjs                   mimetic                    sary
ecj                        mimms                      sickbeard
ghc@8.0                    monotone                   sonarlint
gjstest                    nazghul                    submarine
gnupg@2.0                  nesemu2                    tomcat@8.0
go@1.7                     onepass                    voltdb
hachoir-metadata           open-vcdiff                wry
i3                         opensyobon
==> Downloading https://homebrew.bintray.com/bottles/xz-5.2.4.high_sierra.bottle
---
[00:04:05]       Memory: 8 GB
[00:04:05]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:04:05]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:04:05]       SMC Version (system): 2.8f0
[00:04:05]       Serial Number (system): VMokd/oNuDtt
[00:04:05] 
[00:04:05] hw.ncpu: 4
[00:04:05] hw.byteorder: 1234
[00:04:05] hw.memsize: 8589934592
---
travis_fold:end:dpl_0
travis_time:start:1c2698d0
travis_fold:start:dpl.1
Installing deploy dependencies
ERROR:  Could not find a valid gem 'dpl-s3' (= 1.10.3), here is why:
          Unable to download data from https://rubygems.org/ - no such name (https://rubygems.org/specs.4.8.gz)
travis_fold:end:dpl.1
/Users/travis/.rvm/rubies/ruby-2.4.2/lib/ruby/2.4.0/rubygems/core_ext/kernel_require.rb:55:in `require': cannot load such file -- dpl/provider/s3 (LoadError)
 from /Users/travis/.rvm/rubies/ruby-2.4.2/lib/ruby/2.4.0/rubygems/core_ext/kernel_require.rb:55:in `require'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/dpl-1.10.3/lib/dpl/provider.rb:94:in `rescue in block in new'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/dpl-1.10.3/lib/dpl/provider.rb:69:in `block in new'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/dpl-1.10.3/lib/dpl/cli.rb:41:in `fold'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/dpl-1.10.3/lib/dpl/provider.rb:68:in `new'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/dpl-1.10.3/lib/dpl/cli.rb:31:in `run'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/dpl-1.10.3/lib/dpl/cli.rb:7:in `run'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/dpl-1.10.3/bin/dpl:5:in `<top (required)>'
 from /Users/travis/.rvm/gems/ruby-2.4.2/bin/dpl:23:in `load'
 from /Users/travis/.rvm/gems/ruby-2.4.2/bin/dpl:23:in `<main>'
failed to deploy
