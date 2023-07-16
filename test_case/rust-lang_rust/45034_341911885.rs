asm
push ecx
push 0e51f8h ; the address to string
call std::path::{{impl}}::as_ref ; comes from libstd, not inlined.
  push ebp
  mov ebp, esp
  mov eax, dword ptr [ebp + 8] ; eax = e51f8h
  mov edx, dword ptr [ebp + 0ch] ; edx = 1
  pop ebp
  ret
pop ecx ; ecx = 0e51f8h
pop esi ; esi = 1
mov dword ptr [esi + 8], eax ; access violation
