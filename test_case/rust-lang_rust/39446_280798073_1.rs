
        x_x = (x * x) / 200;
mov    %eax,%edx            
imul   %edx,%edx            
movslq %edx,%rdx            
imul   $0x51eb851f,%rdx,%rbp
mov    %rbp,%rdx            
shr    $0x3f,%rdx           
sar    $0x26,%rbp           
add    %edx,%ebp            
