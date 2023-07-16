 shell
os=unknown-linux-gnu                                                         
target=x86_64-$os                                                            
host=i686-$os                                                                
curl https://static.rust-lang.org/dist/rustc-nightly-$host.tar.gz | tar xzf -
./rustc-nightly-$host/install.sh --prefix=install           
curl https://static.rust-lang.org/dist/rust-std-nightly-$target.tar.gz | \   
  tar xzf - --strip-components=4 -C install/lib/rustlib \                    
  rust-std-nightly-$target/rust-std-$target/lib/rustlib                      

echo 'fn main() { std::thread::spawn(|| {}).join().unwrap(); }' |            
  PATH=`pwd`/install/bin:$PATH \                                             
  LD_LIBRARY_PATH=`pwd`/install/lib \                                        
  rustc - -o foo --target $target                                            
./foo                                                                        
