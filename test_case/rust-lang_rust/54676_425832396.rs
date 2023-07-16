plain
Warning, new version of rvm available '1.29.4', you are using older version '1.29.3'.
You can disable this warning with:    echo rvm_autoupdate_flag=0 >> ~/.rvmrc
You can enable  auto-update  with:    echo rvm_autoupdate_flag=2 >> ~/.rvmrc
ERROR:  While executing gem ... (Gem::RemoteFetcher::UnknownHostError)
    no such name (https://api.rubygems.org/quick/Marshal.4.8/dpl-1.10.2.gemspec.rz)


The command "rvm $(travis_internal_ruby) --fuzzy do ruby -S gem install dpl" failed and exited with 1 during .
Your build has been stopped.
