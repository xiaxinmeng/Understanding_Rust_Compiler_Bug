
<anon>:14:9: 14:14 error: type `Foo` does not implement any method in scope named `bar`
<anon>:14     Foo.bar();
                  ^~~~~
<anon>:14:14: 14:14 note: found defined static methods, maybe a `self` is missing?
<anon>:3:5: 3:16 note: candidate #1 is `Foo::bar`
<anon>:3     fn bar() {}
             ^~~~~~~~~~~
<anon>:16:9: 16:14 error: type `Foo` does not implement any method in scope named `qux`
<anon>:16     Foo.qux();
                  ^~~~~
<anon>:16:14: 16:14 note: found defined static methods, maybe a `self` is missing?
<anon>:7:5: 7:21 note: candidate #1 is `Baz::qux`
<anon>:7     fn qux() -> Self;
             ^~~~~~~~~~~~~~~~
error: aborting due to 2 previous errors
