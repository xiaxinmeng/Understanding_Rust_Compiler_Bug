bash
for file in `find -type f -name '*.rs'` ; do
    sed -i -re '/\/\/ Copyright [0-9-]+ The Rust Project Developers. See the COPYRIGHT/d' $file
    sed -i -re '/\/\/ file at the top-level directory of this distribution and at/d' $file
    sed -i -re 'N;/\/\/ http:\/\/rust-lang.org\/COPYRIGHT.\n\/\//d' $file
    sed -i -re '/\/\/ Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or/d' $file
    sed -i -re '/\/\/ http:\/\/www.apache.org\/licenses\/LICENSE-2.0> or the MIT license/d' $file
    sed -i -re '/\/\/ <LICENSE-MIT or http:\/\/opensource.org\/licenses\/MIT>, at your/d' $file
    sed -i -re '/\/\/ option. This file may not be copied, modified, or distributed/d' $file
    sed -i -re '/\/\/ except according to those terms./d' $file
    sed -i '/./,$!d' $file
done
