
On 10/03/2012 06:16 PM, Adam wrote:
> I'm on Windows 8 64-bit RTM as well, however when I run rustc I get a
> prompted that "libgcc_s_dw2-1.dll is missing from your computer".

This sounds like a symptom of not running from the MinGW shell (an 
unfortunate limitation in our Windows build). There are some 
instructions for getting MinGW setup on the wiki.

* https://github.com/mozilla/rust/issues/1603
* https://github.com/mozilla/rust/wiki/Note-getting-started-developing-Rust
