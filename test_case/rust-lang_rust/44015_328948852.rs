
ERROR:  Could not find a valid gem 'aws-sdk-resources' (= 2.10.44) in any repository
ERROR:  Possible alternatives: aws-sdk-resources
/home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/site_ruby/2.2.0/rubygems/core_ext/kernel_require.rb:55:in `require': cannot load such file -- aws-sdk (LoadError)
	from /home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/site_ruby/2.2.0/rubygems/core_ext/kernel_require.rb:55:in `require'
	from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.8.43/lib/dpl/provider.rb:85:in `requires'
	from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.8.43/lib/dpl/provider/s3.rb:6:in `<class:S3>'
	from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.8.43/lib/dpl/provider/s3.rb:5:in `<class:Provider>'
	from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.8.43/lib/dpl/provider/s3.rb:4:in `<module:DPL>'
	from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.8.43/lib/dpl/provider/s3.rb:3:in `<top (required)>'
	from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.8.43/lib/dpl/provider.rb:59:in `const_get'
	from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.8.43/lib/dpl/provider.rb:59:in `block in new'
	from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.8.43/lib/dpl/cli.rb:41:in `fold'
	from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.8.43/lib/dpl/provider.rb:56:in `new'
	from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.8.43/lib/dpl/cli.rb:31:in `run'
	from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.8.43/lib/dpl/cli.rb:7:in `run'
	from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-1.8.43/bin/dpl:5:in `<top (required)>'
	from /home/travis/.rvm/gems/ruby-2.2.7/bin/dpl:23:in `load'
	from /home/travis/.rvm/gems/ruby-2.2.7/bin/dpl:23:in `<main>'

travis_fold:end:dpl.1
failed to deploy
