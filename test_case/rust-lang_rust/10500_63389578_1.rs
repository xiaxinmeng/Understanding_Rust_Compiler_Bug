 asm
s:                                      # @s
# BB#0:
    movl    4(%esp), %eax
    movl    $1073741824, 4(%eax)    # imm = 0x40000000
    movl    $1065353216, (%eax)     # imm = 0x3F800000
    retl    $4

c:                                      # @c
# BB#0:
    movl    $1065353216, %eax       # imm = 0x3F800000
    movl    $1073741824, %edx       # imm = 0x40000000
    retl
