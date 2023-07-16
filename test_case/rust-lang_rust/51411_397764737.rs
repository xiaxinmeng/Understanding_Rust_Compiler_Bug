plain
[00:02:54]       Memory: 8 GB
[00:02:54]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:02:54]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:54]       SMC Version (system): 2.8f0
[00:02:54]       Serial Number (system): VMlNH6iM8hLT
[00:02:54] 
[00:02:54] hw.ncpu: 4
[00:02:54] hw.byteorder: 1234
[00:02:54] hw.memsize: 8589934592
---
travis_fold:end:dpl_0
travis_time:start:141a70dc
travis_fold:start:dpl.1
Installing deploy dependencies
ERROR:  Could not find a valid gem 'aws-sdk-resources' (= 2.11.70) in any repository
ERROR:  Possible alternatives: aws-sdk-resources
/Users/travis/.rvm/rubies/ruby-2.4.2/lib/ruby/2.4.0/rubygems/core_ext/kernel_require.rb:55:in `require': cannot load such file -- dpl/provider/s3 (LoadError)
 from /Users/travis/.rvm/rubies/ruby-2.4.2/lib/ruby/2.4.0/rubygems/core_ext/kernel_require.rb:55:in `require'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/dpl-1.9.7/lib/dpl/provider.rb:93:in `rescue in block in new'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/dpl-1.9.7/lib/dpl/provider.rb:68:in `block in new'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/dpl-1.9.7/lib/dpl/cli.rb:41:in `fold'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/dpl-1.9.7/lib/dpl/provider.rb:67:in `new'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/dpl-1.9.7/lib/dpl/cli.rb:31:in `run'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/dpl-1.9.7/lib/dpl/cli.rb:7:in `run'
 from /Users/travis/.rvm/gems/ruby-2.4.2/gems/dpl-1.9.7/bin/dpl:5:in `<top (required)>'
 from /Users/travis/.rvm/gems/ruby-2.4.2/bin/dpl:23:in `load'
 from /Users/travis/.rvm/gems/ruby-2.4.2/bin/dpl:23:in `<main>'
failed to deploy
failed to deploy
No output has been received in the last 30m0s, this potentially indicates a stalled build or something wrong with the build itself.
Check the details on how to adjust your build configuration on: https://docs.travis-ci.com/user/common-build-problems/#Build-times-out-because-no-output-was-received
The build has been terminated
