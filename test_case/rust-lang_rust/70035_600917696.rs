
installing 6b561b4917e803c4be4ca44d8e552b680cb9e380

rustc 1.42.0-nightly (6b561b491 2019-12-20)

Running HashMap<(), ()> tests
Testing 'foo bar baz qux 1337 42\n' * 1_000_000      file=23MiB       ram=3,7GiB
Testing 'foo bar baz qux 1337 42 ' * 1_000_000       file=23MiB       ram=3,7GiB
Testing 'a' * 25*1024*1024                           file=25MiB       ram=3,9GiB
Testing 'a' * 10*1024*1024                           file=10MiB       ram=1,9GiB
Testing 'a' * 1*1024*1024                            file=1,0MiB      ram=251MiB

Running Vec<()> tests
Testing 'foo bar baz qux 1337 42\n' * 1_000_000      file=23MiB       ram=1,2GiB
Testing 'foo bar baz qux 1337 42 ' * 1_000_000       file=23MiB       ram=666MiB
Testing 'a' * 25*1024*1024                           file=25MiB       ram=1,2GiB
Testing 'a' * 10*1024*1024                           file=10MiB       ram=592MiB
Testing 'a' * 1*1024*1024                            file=1,0MiB      ram=163MiB

RESULT: 6b561b4917e803c4be4ca44d8e552b680cb9e380 ===> No

installing 01a46509a4c2dc430ebebf940a26232fdaeeba81

rustc 1.42.0-nightly (01a46509a 2019-12-20)

Running HashMap<(), ()> tests
Testing 'foo bar baz qux 1337 42\n' * 1_000_000      file=23MiB       ram=4,6GiB
Testing 'foo bar baz qux 1337 42 ' * 1_000_000       file=23MiB       ram=4,6GiB
Testing 'a' * 25*1024*1024                           file=25MiB       ram=4,8GiB
Testing 'a' * 10*1024*1024                           file=10MiB       ram=2,3GiB
Testing 'a' * 1*1024*1024                            file=1,0MiB      ram=282MiB

Running Vec<()> tests
Testing 'foo bar baz qux 1337 42\n' * 1_000_000      file=23MiB       ram=1,2GiB
Testing 'foo bar baz qux 1337 42 ' * 1_000_000       file=23MiB       ram=1,1GiB
Testing 'a' * 25*1024*1024                           file=25MiB       ram=1,2GiB
Testing 'a' * 10*1024*1024                           file=10MiB       ram=591MiB
Testing 'a' * 1*1024*1024                            file=1,0MiB      ram=164MiB

RESULT: 01a46509a4c2dc430ebebf940a26232fdaeeba81 ===> Yes

searched toolchains fc6b5d6efe163060bde31cc1c801086ed7ebc8f1 through 8a79d08fa57e1c257d647c9848e35defcb379c07
regression in 01a46509a4c2dc430ebebf940a26232fdaeeba81
