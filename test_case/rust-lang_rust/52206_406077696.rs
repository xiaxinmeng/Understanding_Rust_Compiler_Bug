plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0dc03944
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
travis_fold:end:dpl_3
travis_time:start:29d0267b
travis_fold:start:dpl.1
Installing deploy dependencies
ERROR:  Could not find a valid gem 'aws-sdk-core' (= 2.11.91) in any repository
ERROR:  Possible alternatives: aws-sdk-core
/home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/site_ruby/2.2.0/rubygems/core_ext/kernel_require.rb:59:in `require': cannot load such file -- dpl/provider/s3 (LoadError)
 from /home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/site_ruby/2.2.0/rubygems/core_ext/kernel_require.rb:59:in `require'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.9.8/lib/dpl/provider.rb:93:in `rescue in block in new'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.9.8/lib/dpl/provider.rb:68:in `block in new'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.9.8/lib/dpl/cli.rb:41:in `fold'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.9.8/lib/dpl/provider.rb:67:in `new'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.9.8/lib/dpl/cli.rb:31:in `run'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.9.8/lib/dpl/cli.rb:7:in `run'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.9.8/bin/dpl:5:in `<top (required)>'
 from /home/travis/.rvm/gems/ruby-2.2.7/bin/dpl:23:in `load'
 from /home/travis/.rvm/gems/ruby-2.2.7/bin/dpl:23:in `<main>'
failed to deploy
