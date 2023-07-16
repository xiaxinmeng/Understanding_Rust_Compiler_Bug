
>>> commandinterp = debugger.GetCommandInterpreter()
>>> returnobj = lldb.SBCommandReturnObject()
>>> print commandinterp.HandleCommand ("p foo", returnobj)
2
>>> print returnobj
Status:  Success
Output Message:
(int) $0 = 5
>>> print returnobj.Succeeded()
True
>>> print returnobj.GetOutput()
(int) $0 = 5
>>>
