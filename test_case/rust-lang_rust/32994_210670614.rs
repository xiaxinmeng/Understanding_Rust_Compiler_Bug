
% cat >a.c
int main ()
{  
  int foo = 5;
  foo++; // break here
}
% clang -g a.c
% setenv PYTHONPATH /Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python/
% python
>>> import lldb
>>> debugger = lldb.SBDebugger.Create()
>>> target_error = lldb.SBError()
>>> target = debugger.CreateTarget('a.out', None, None, True, target_error)
>>> srcfile = lldb.SBFileSpec("a.c")
>>> bp = target.BreakpointCreateBySourceRegex("break here", srcfile)
>>> debugger.SetAsync(False)
>>> process = target.LaunchSimple ([], [], ".")
>>> print process.IsValid()
True

>>> frame = process.GetSelectedThread().GetFrameAtIndex(0)
>>> print frame
frame #0: 0x0000000100973fad a.out`main + 13 at a.c:4
>>> print bp
SBBreakpoint: id = 1, source regex = "break here", exact_match = 0, locations = 1
>>> print bp.GetHitCount()
1
>>> 
