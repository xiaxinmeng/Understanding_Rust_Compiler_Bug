
.wait status=Stopped(Pid(16438), SIGTRAP)                                                                                                                                                                    
breakpoint at rip=403861                                                                                                                                                                                     
%rip=value is 403861, at [403420] is ffe789cc18ec8348, rsp=7fffffffd018 rdi=7fffffffd028                                                                                                                        
writing fbb6e8243c8948cc>fbb6e8243c894850 to [403860]                                                                                                                                                         
wait status=PtraceEvent(Pid(16440), SIGTRAP, 6)                                                                                                                                                              
wait status=Stopped(Pid(16438), SIGTRAP)                                                                                                                                                                     
breakpoint at rip=403861                                                                                                                                                                                     
%rip=403861, at [403420] is ffe789cc18ec8348, rsp=7fffffffd010 rdi=7fffffffd028                                                                                                                                                                             
writing fbb6e8243c894850>fbb6e8243c894850 to [403860]
wait status=PtraceEvent(Pid(16442), SIGTRAP, 6)                                                                                                                                                              
wait status=Stopped(Pid(16438), SIGTRAP)                                                                                                                                                                     
breakpoint at rip=403425                                                                                                                                                                                     
%rip=403425, at [403420] is ffe789cc18ec8348, rsp=7fffffffcff0 rdi=7fffffffd028                                                                                          
writing ffe789cc18ec8348>ffe7894818ec8348 to [403420]                                                                                                                                                         
wait status=Stopped(Pid(16438), SIGTRAP)                                                                                                                                                                     
breakpoint at rip=403427
...
