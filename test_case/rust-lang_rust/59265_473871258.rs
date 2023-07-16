
xor    %ebp,%ebp
and    $0xfffffff0,%esp
; perhaps initialize the other registers to something as well
call     <actual_entry_point>
