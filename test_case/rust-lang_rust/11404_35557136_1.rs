 ruby
"abc\n".chomp # => "abc"
"abc\r\n".chomp # => "abc"
"abc".chomp # => "abc"
"abc\n\n".chomp # => "abc\n" -- Only one newline is chomped off!
