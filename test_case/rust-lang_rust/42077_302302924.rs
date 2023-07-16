sh
test/ui/resolve/issue-18252.stderr:1:error[E0423]: expected function, found struct variant `Foo::Variant`
test/ui/resolve/issue-19452.stderr:1:error[E0423]: expected value, found struct variant `Homura::Madoka`
test/ui/resolve/issue-19452.stderr:7:error[E0423]: expected value, found struct variant `issue_19452_aux::Homura::Madoka`
test/ui/resolve/issue-33876.stderr:1:error[E0423]: expected value, found trait `Bar`
test/ui/resolve/issue-39226.stderr:1:error[E0423]: expected value, found struct `Handle`
test/ui/resolve/issue-6702.stderr:1:error[E0423]: expected function, found struct `Monster`
test/ui/resolve/privacy-struct-ctor.stderr:1:error[E0423]: expected value, found struct `Z`
test/ui/resolve/privacy-struct-ctor.stderr:14:error[E0423]: expected value, found struct `S`
test/ui/resolve/privacy-struct-ctor.stderr:26:error[E0423]: expected value, found struct `xcrate::S`
test/ui/resolve/resolve-hint-macro.stderr:1:error[E0423]: expected function, found macro `assert`
test/ui/resolve/suggest-path-instead-of-mod-dot-item.stderr:1:error[E0423]: expected value, found module `a`
test/ui/resolve/suggest-path-instead-of-mod-dot-item.stderr:9:error[E0423]: expected value, found module `a`
test/ui/resolve/suggest-path-instead-of-mod-dot-item.stderr:17:error[E0423]: expected value, found module `a`
test/ui/resolve/suggest-path-instead-of-mod-dot-item.stderr:25:error[E0423]: expected value, found module `a::b`
test/ui/resolve/suggest-path-instead-of-mod-dot-item.stderr:34:error[E0423]: expected value, found module `a`
test/ui/resolve/suggest-path-instead-of-mod-dot-item.stderr:42:error[E0423]: expected value, found module `a::b`
test/ui/resolve/suggest-path-instead-of-mod-dot-item.stderr:50:error[E0423]: expected value, found module `a::b`
test/ui/resolve/suggest-path-instead-of-mod-dot-item.stderr:59:error[E0423]: expected value, found module `a::b`
test/ui/resolve/suggest-path-instead-of-mod-dot-item.stderr:67:error[E0423]: expected function, found module `a::b`
test/ui/resolve/tuple-struct-alias.stderr:1:error[E0423]: expected function, found self type `Self`
test/ui/resolve/tuple-struct-alias.stderr:13:error[E0423]: expected function, found type alias `A`
