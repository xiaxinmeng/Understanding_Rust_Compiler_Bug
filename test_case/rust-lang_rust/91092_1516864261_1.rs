
set -euo pipefail                                                                                                                                                                          
                                                                                                                                                                                           
rm -fr target                                                                                                                                                                              
mkdir -p target/coverage                                                                                                                                                                   
                                                                                                                                                                                           
export RUSTFLAGS="-Cinstrument-coverage"                                                                                                                                                   
cargo build                                                                                                                                                                                
export LLVM_PROFILE_FILE="target/coverage/%p-%m.profraw"                                                                                                                                   
                                                                                                                                                                                           
for i in $(seq 1 15000); do                                                                                                                                                                
    rm -f target/coverage/*.profraw target/coverage/cobertura.xml                                                                                                                          
    cargo test > /dev/null 2>&1                                                                                                                                                            
    grcov target/coverage --binary-path target/debug -s . -o target/coverage --keep-only 'src/*' --output-types cobertura                                                                  
    if [ ! -z "$(cat target/coverage/cobertura.xml | grep -E 'number="(8|9|10)' | grep -v 'hits="4"')" ]; then                                                                             
        echo "Found BUG at step $i"                                                                                                                                                        
        cat target/coverage/cobertura.xml                                                                                                                                                  
        exit 1                                                                                                                                                                             
    fi                                                                                                                                                                                     
done                                                                                                                                                                                       
