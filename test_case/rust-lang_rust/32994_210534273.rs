 python
import lldb                                                          
debugger = lldb.SBDebugger.Create()                                  
target_error = lldb.SBError()                                        
target = debugger.CreateTarget('foo', None, None, True, target_error)
