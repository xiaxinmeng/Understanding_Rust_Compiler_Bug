
testcase.rs:14:15: 14:25 error: in type `&mut std::vec::Vec<&'a Node<'b>>`, pointer has a longer lifetime than the data it references
testcase.rs:14               self.queue.push( child )
                             ^~~~~~~~~~
testcase.rs:8:48: 23:4 note: the pointer is valid for the lifetime &'a  as defined on the block at 8:47
testcase.rs:8   fn next( &mut self ) -> Option<&'a Node<'b>> {
testcase.rs:9     match self.queue.pop() {
testcase.rs:10       ex @ Some( node ) => {
testcase.rs:11         match node.contents {
testcase.rs:12           Children( ref x ) => {
testcase.rs:13             for child in x.as_slice().rev_iter() {
               ...
testcase.rs:8:48: 23:4 note: but the referenced data is only valid for the lifetime &'b  as defined on the block at 8:47
testcase.rs:8   fn next( &mut self ) -> Option<&'a Node<'b>> {
testcase.rs:9     match self.queue.pop() {
testcase.rs:10       ex @ Some( node ) => {
testcase.rs:11         match node.contents {
testcase.rs:12           Children( ref x ) => {
testcase.rs:13             for child in x.as_slice().rev_iter() {
               ...
testcase.rs:8:3: 23:4 note: consider using an explicit lifetime parameter as shown: fn next(&mut self) -> Option<&'a Node<'a>>
testcase.rs:8   fn next( &mut self ) -> Option<&'a Node<'b>> {
testcase.rs:9     match self.queue.pop() {
testcase.rs:10       ex @ Some( node ) => {
testcase.rs:11         match node.contents {
testcase.rs:12           Children( ref x ) => {
testcase.rs:13             for child in x.as_slice().rev_iter() {
               ...
testcase.rs:12:21: 12:26 error: cannot infer an appropriate lifetime for pattern due to conflicting requirements
testcase.rs:12           Children( ref x ) => {
                                   ^~~~~
error: aborting due to 2 previous errors
