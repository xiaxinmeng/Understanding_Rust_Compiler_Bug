plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
Rustbook (x86_64-unknown-linux-gnu) - edition-guide
Rustbook (x86_64-unknown-linux-gnu) - style-guide
Building tool linkchecker (stage0)
    Finished release [optimized] target(s) in 0.19s
std/primitive.slice.html:2377: broken intra-doc link - [<code>into_vec&lt;A&gt;(self)</code>]
std/primitive.slice.html:2378: broken intra-doc link - [<code>into_vec::&lt;A&gt;(self)</code>]
std/primitive.slice.html:2379: broken intra-doc link - [<code>into_vec&lt;A&gt;(Box&lt;Self, A&gt;)</code>]
std/primitive.slice.html:2380: broken intra-doc link - [<code>into_vec::&lt;A&gt;(Box&lt;Self, A&gt;)</code>]
std/collections/struct.VecDeque.html:851: broken intra-doc link - [<code>&lt;VecDeque::&lt;T&gt;&gt;::from_iter(I)</code>]
std/collections/struct.VecDeque.html:852: broken intra-doc link - [<code>VecDeque::&lt;T&gt;::from_iter(I)</code>]
std/collections/struct.VecDeque.html:853: broken intra-doc link - [<code>&lt;VecDeque::&lt;T&gt;&gt;::from_iter(I: IntoIterator&lt;Item = T&gt;)</code>]
std/collections/struct.VecDeque.html:854: broken intra-doc link - [<code>VecDeque::&lt;T&gt;::from_iter(I: IntoIterator&lt;Item = T&gt;)</code>]
std/collections/vec_deque/struct.VecDeque.html:851: broken intra-doc link - [<code>&lt;VecDeque::&lt;T&gt;&gt;::from_iter(I)</code>]
std/collections/vec_deque/struct.VecDeque.html:852: broken intra-doc link - [<code>VecDeque::&lt;T&gt;::from_iter(I)</code>]
std/collections/vec_deque/struct.VecDeque.html:853: broken intra-doc link - [<code>&lt;VecDeque::&lt;T&gt;&gt;::from_iter(I: IntoIterator&lt;Item = T&gt;)</code>]
std/collections/vec_deque/struct.VecDeque.html:854: broken intra-doc link - [<code>VecDeque::&lt;T&gt;::from_iter(I: IntoIterator&lt;Item = T&gt;)</code>]
alloc/collections/vec_deque/struct.VecDeque.html:848: broken intra-doc link - [<code>&lt;VecDeque::&lt;T&gt;&gt;::from_iter(I)</code>]
alloc/collections/vec_deque/struct.VecDeque.html:849: broken intra-doc link - [<code>VecDeque::&lt;T&gt;::from_iter(I)</code>]
alloc/collections/vec_deque/struct.VecDeque.html:850: broken intra-doc link - [<code>&lt;VecDeque::&lt;T&gt;&gt;::from_iter(I: IntoIterator&lt;Item = T&gt;)</code>]
alloc/collections/vec_deque/struct.VecDeque.html:851: broken intra-doc link - [<code>VecDeque::&lt;T&gt;::from_iter(I: IntoIterator&lt;Item = T&gt;)</code>]
number of HTML files scanned: 33580
number of HTML redirects found: 10265
number of links checked: 2976656
number of links ignored due to external: 96905
