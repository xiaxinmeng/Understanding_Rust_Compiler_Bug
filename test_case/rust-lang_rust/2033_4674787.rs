
LogSpec ::== 'help' | LogLevel (',' Module)*
LogLevel ::== Error | Warning | Info | Debug
Error ::== 'e' | 'er' | 'err' | 'erro' | 'error' | 'errors'
Warning ::== 'w' | 'wa' | 'war' | 'warn' | 'warni' | 'warnin' | 'warning' | 'warnings'
Info ::== 'i' | 'in' | 'inf' | 'info'
Debug ::== 'd' | 'de' | 'deb' | 'debu' | 'debug'
Module ::== AbsoluteModule | ModuleFragment
AbsoluteModule ::== ('::' Identifier)+
ModuleFragment ::== Identifier ('::' Identifier)*
Identifier ::== [a-zA-Z_][a-zA-Z0-9_]*
