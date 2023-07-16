plain
travis_fold:end:dpl_0
travis_time:start:01bdebe5
travis_fold:start:dpl.1
Installing deploy dependencies
ERROR:  Could not find a valid gem 'aws-sdk-resources' (= 2.11.152) in any repository
ERROR:  Possible alternatives: aws-sdk-resources
travis_fold:end:dpl.1
/home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/site_ruby/2.2.0/rubygems/core_ext/kernel_require.rb:59:in `require': cannot load such file -- dpl/provider/s3 (LoadError)
 from /home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/site_ruby/2.2.0/rubygems/core_ext/kernel_require.rb:59:in `require'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.10.3/lib/dpl/provider.rb:94:in `rescue in block in new'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.10.3/lib/dpl/provider.rb:69:in `block in new'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.10.3/lib/dpl/cli.rb:41:in `fold'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.10.3/lib/dpl/provider.rb:68:in `new'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.10.3/lib/dpl/cli.rb:31:in `run'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.10.3/lib/dpl/cli.rb:7:in `run'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.10.3/bin/dpl:5:in `<top (required)>'
 from /home/travis/.rvm/gems/ruby-2.2.7/bin/dpl:23:in `load'
 from /home/travis/.rvm/gems/ruby-2.2.7/bin/dpl:23:in `<main>'
failed to deploy
