powershell
(Get-CimInstance Win32_Process -Filter "ParentProcessId=$PID").ProcessId
