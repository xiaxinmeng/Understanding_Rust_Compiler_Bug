
set PATH="C:\Program Files (x86)\Inno Setup 5";%PATH%
Invoke-WebRequest -Uri https://download.sysinternals.com/files/Handle.zip -OutFile handle.zip
Invoke-WebRequest : The operation has timed out.
At line:1 char:1
+ Invoke-WebRequest -Uri https://download.sysinternals.com/files/Handle ...
+ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : InvalidOperation: (System.Net.HttpWebRequest:HttpWebRequest) [Invoke-WebRequest], WebException
    + FullyQualifiedErrorId : WebCmdletWebResponseException,Microsoft.PowerShell.Commands.InvokeWebRequestCommand
 
Command executed with exception: The operation has timed out.
cat %CD%\sccache.log || exit 0

