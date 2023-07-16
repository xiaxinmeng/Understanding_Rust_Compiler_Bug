
% cat >a.c
int main () { }
% clang a.c
% setenv PYTHONPATH /Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python/
% python
Python 2.7.10 (default, Oct 23 2015, 19:19:21) 
[GCC 4.2.1 Compatible Apple LLVM 7.0.0 (clang-700.0.59.5)] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> import lldb
>>> debugger = lldb.SBDebugger.Create()
>>> target_error = lldb.SBError()
>>> target = debugger.CreateTarget('a.out', None, None, True, target_error)
>>> print target.IsValid()
True
>>> print debugger.GetVersionString()
lldb-350.0.21.3
>>> 
