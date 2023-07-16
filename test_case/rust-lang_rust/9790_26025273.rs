
$ cat foo.rs
pub use foo::bar;

mod foo {
    pub fn bar() {}
}
$ cat bar.rs
extern mod foo;

fn main() {
    foo::bar();
}
$ rustc foo.rs --lib
warning: missing crate link meta `name`, using `foo` as default
warning: missing crate link meta `vers`, using `0.0` as default
warning: no debug symbols in executable (-arch x86_64)
$ rustc bar.rs -L.
warning: no debug symbols in executable (-arch x86_64)                                                              
$ ./bar
dyld: lazy symbol binding failed: Symbol not found: __ZN3foo3bar19h15bec6257b8bff78ak4v0.0E                         
  Referenced from: /Users/alex/code/rust-opt/./bar                                                                  
  Expected in: flat namespace                                                                                       

dyld: Symbol not found: __ZN3foo3bar19h15bec6257b8bff78ak4v0.0E                                                     
  Referenced from: /Users/alex/code/rust-opt/./bar                                                                  
  Expected in: flat namespace                                                                                       

zsh: trace trap  ./bar                                                                                              

