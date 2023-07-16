
        x_x = (x * x) / 200;
mov    %edi,%r13d           
imul   %edi,%r13d           
mov    %r13d,%eax           
sar    $0x1f,%r13d          
imul   %ebx                 
sar    $0x6,%edx            
mov    %edx,%ecx            
sub    %r13d,%ecx           
