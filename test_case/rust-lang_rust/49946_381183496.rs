
pietro@january: ~/tmp/polylabel_cmd$ rustc +beta --version                               
rustc 1.26.0-beta.3 (8a75d2b50 2018-04-08)                                               
pietro@january: ~/tmp/polylabel_cmd$ cargo +beta build                                   
error: failed to parse manifest at `/home/pietro/tmp/polylabel_cmd/Cargo.toml`           
                                                                                         
Caused by:                                                                               
  the target `polylabel` is a binary and can't have any crate-types set (currently "bin")
pietro@january: ~/tmp/polylabel_cmd$                                                     
