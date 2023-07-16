
$file = Get-Item .\target\debug\libstate_manipulation.dll
$file.LastWriteTime = (Get-Date)
