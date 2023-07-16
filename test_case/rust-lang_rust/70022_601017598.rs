asm
#[repr(align(0x10000))]
struct Aligned(u8);

static X: Aligned = Aligned(0);

fn main() {
00007FF6EC990000  push        rbp  
00007FF6EC990001  mov         eax,4FFF0h  
00007FF6EC990006  call        __chkstk (07FF6EC9A9D20h)  ; this checks the stack
00007FF6EC99000B  sub         rsp,rax  
00007FF6EC99000E  lea         rbp,[rsp+80h]  
; this rounds down the stack by well over the size of a page
; causing the stack to extend past the guard page
00007FF6EC990016  and         rsp,0FFFFFFFFFFFF0000h 
00007FF6EC99001D  lea         rax,[rsp+10000h]  
    let x = Aligned(0);
00007FF6EC990025  mov         byte ptr [rsp+10000h],0  
    println!("{:#x}", &x as *const _ as usize);
00007FF6EC99002D  mov         qword ptr [rsp+2FF48h],rax  
00007FF6EC990035  lea         rax,[rsp+2FF48h]  
00007FF6EC99003D  mov         qword ptr [rsp+2FF40h],rax  
00007FF6EC990045  mov         rax,qword ptr [rsp+2FF40h]  
00007FF6EC99004D  mov         qword ptr [rsp+2FF50h],rax  
00007FF6EC990055  mov         rcx,qword ptr [rsp+2FF50h]  
00007FF6EC99005D  lea         rdx,[core::fmt::num::{{impl}}::fmt (07FF6EC9A93A0h)]  
00007FF6EC990064  call        core::fmt::ArgumentV1::new<usize> (07FF6EC990450h)  
00007FF6EC990069  mov         qword ptr [rsp+0FFF8h],rax  
00007FF6EC990071  mov         qword ptr [rsp+0FFF0h],rdx  
00007FF6EC990079  lea         rax,[X+10008h (07FF6EC9C2008h)]  
00007FF6EC990080  lea         rcx,[X+10028h (07FF6EC9C2028h)]  
00007FF6EC990087  mov         rdx,qword ptr [rsp+0FFF8h]  
00007FF6EC99008F  mov         qword ptr [rsp+2FF30h],rdx  
00007FF6EC990097  mov         r8,qword ptr [rsp+0FFF0h]  
00007FF6EC99009F  mov         qword ptr [rsp+2FF38h],r8  
00007FF6EC9900A7  lea         r9,[rsp+2FF30h]  
00007FF6EC9900AF  lea         r10,[rsp+2FF00h]  
00007FF6EC9900B7  mov         qword ptr [rsp+0FFE8h],rcx  
00007FF6EC9900BF  mov         rcx,r10  
00007FF6EC9900C2  mov         rdx,rax  
00007FF6EC9900C5  mov         r8d,2  
00007FF6EC9900CB  mov         qword ptr [rsp+20h],1  
00007FF6EC9900D4  mov         rax,qword ptr [rsp+0FFE8h]  
00007FF6EC9900DC  mov         qword ptr [rsp+28h],rax  
00007FF6EC9900E1  mov         qword ptr [rsp+30h],1  
00007FF6EC9900EA  call        core::fmt::Arguments::new_v1_formatted (07FF6EC9904B0h)  
00007FF6EC9900EF  lea         rcx,[rsp+2FF00h]  
00007FF6EC9900F7  call        std::io::stdio::_print (07FF6EC995440h)  
    println!("{:#x}", &X as *const _ as usize);
; notice here that the static is NOT aligned correctly
00007FF6EC9900FC  lea         rax,[X (07FF6EC9B2000h)]  
00007FF6EC990103  mov         qword ptr [rsp+2FFA0h],rax  
00007FF6EC99010B  lea         rax,[rsp+2FFA0h]  
00007FF6EC990113  mov         qword ptr [rsp+2FF98h],rax  
00007FF6EC99011B  mov         rax,qword ptr [rsp+2FF98h]  
00007FF6EC990123  mov         qword ptr [rsp+2FFA8h],rax  
00007FF6EC99012B  mov         rcx,qword ptr [rsp+2FFA8h]  
00007FF6EC990133  lea         rdx,[core::fmt::num::{{impl}}::fmt (07FF6EC9A93A0h)]  
00007FF6EC99013A  call        core::fmt::ArgumentV1::new<usize> (07FF6EC990450h)  
00007FF6EC99013F  mov         qword ptr [rsp+0FFE0h],rax  
00007FF6EC990147  mov         qword ptr [rsp+0FFD8h],rdx  
00007FF6EC99014F  lea         rax,[X+10008h (07FF6EC9C2008h)]  
00007FF6EC990156  lea         rcx,[X+10028h (07FF6EC9C2028h)]  
00007FF6EC99015D  mov         rdx,qword ptr [rsp+0FFE0h]  
00007FF6EC990165  mov         qword ptr [rsp+2FF88h],rdx  
00007FF6EC99016D  mov         r8,qword ptr [rsp+0FFD8h]  
00007FF6EC990175  mov         qword ptr [rsp+2FF90h],r8  
00007FF6EC99017D  lea         r9,[rsp+2FF88h]  
00007FF6EC990185  lea         r10,[rsp+2FF58h]  
00007FF6EC99018D  mov         qword ptr [rsp+0FFD0h],rcx  
00007FF6EC990195  mov         rcx,r10  
00007FF6EC990198  mov         rdx,rax  
00007FF6EC99019B  mov         r8d,2  
00007FF6EC9901A1  mov         qword ptr [rsp+20h],1  
00007FF6EC9901AA  mov         rax,qword ptr [rsp+0FFD0h]  
00007FF6EC9901B2  mov         qword ptr [rsp+28h],rax  
00007FF6EC9901B7  mov         qword ptr [rsp+30h],1  
00007FF6EC9901C0  call        core::fmt::Arguments::new_v1_formatted (07FF6EC9904B0h)  
00007FF6EC9901C5  lea         rcx,[rsp+2FF58h]  
00007FF6EC9901CD  call        std::io::stdio::_print (07FF6EC995440h)  
    println!("{:#x}", Box::into_raw(Box::new(Aligned(0))) as usize);
00007FF6EC9901D2  mov         byte ptr [rsp+30000h],0  
00007FF6EC9901DA  mov         eax,10000h  
00007FF6EC9901DF  mov         rcx,rax  
00007FF6EC9901E2  mov         rdx,rax  
00007FF6EC9901E5  call        alloc::alloc::exchange_malloc (07FF6EC9905F0h)  
00007FF6EC9901EA  mov         rcx,rax  
00007FF6EC9901ED  lea         rdx,[rsp+30000h]  
00007FF6EC9901F5  mov         qword ptr [rsp+0FFC8h],rcx  
00007FF6EC9901FD  mov         rcx,rax  
00007FF6EC990200  mov         r8d,10000h  
00007FF6EC990206  call        memcpy (07FF6EC9AA8D5h)  
00007FF6EC99020B  mov         rcx,qword ptr [rsp+0FFC8h]  
00007FF6EC990213  call        alloc::boxed::{{impl}}::into_raw<aligntest::Aligned> (07FF6EC990860h)  
    println!("{:#x}", Box::into_raw(Box::new(Aligned(0))) as usize);
00007FF6EC990218  mov         qword ptr [rsp+0FFC0h],rax  
00007FF6EC990220  mov         rax,qword ptr [rsp+0FFC0h]  
00007FF6EC990228  mov         qword ptr [rsp+2FFF8h],rax  
00007FF6EC990230  lea         rcx,[rsp+2FFF8h]  
00007FF6EC990238  mov         qword ptr [rsp+2FFF0h],rcx  
00007FF6EC990240  mov         rcx,qword ptr [rsp+2FFF0h]  
00007FF6EC990248  mov         qword ptr [rsp+4FFE8h],rcx  
00007FF6EC990250  mov         rcx,qword ptr [rsp+4FFE8h]  
00007FF6EC990258  lea         rdx,[core::fmt::num::{{impl}}::fmt (07FF6EC9A93A0h)]  
00007FF6EC99025F  call        core::fmt::ArgumentV1::new<usize> (07FF6EC990450h)  
00007FF6EC990264  mov         qword ptr [rsp+0FFB8h],rax  
00007FF6EC99026C  mov         qword ptr [rsp+0FFB0h],rdx  
00007FF6EC990274  lea         rax,[X+10008h (07FF6EC9C2008h)]  
00007FF6EC99027B  lea         rcx,[X+10028h (07FF6EC9C2028h)]  
00007FF6EC990282  mov         rdx,qword ptr [rsp+0FFB8h]  
00007FF6EC99028A  mov         qword ptr [rsp+2FFE0h],rdx  
00007FF6EC990292  mov         r8,qword ptr [rsp+0FFB0h]  
00007FF6EC99029A  mov         qword ptr [rsp+2FFE8h],r8  
00007FF6EC9902A2  lea         r9,[rsp+2FFE0h]  
00007FF6EC9902AA  lea         r10,[rsp+2FFB0h]  
00007FF6EC9902B2  mov         qword ptr [rsp+0FFA8h],rcx  
00007FF6EC9902BA  mov         rcx,r10  
00007FF6EC9902BD  mov         rdx,rax  
00007FF6EC9902C0  mov         r8d,2  
00007FF6EC9902C6  mov         qword ptr [rsp+20h],1  
00007FF6EC9902CF  mov         rax,qword ptr [rsp+0FFA8h]  
00007FF6EC9902D7  mov         qword ptr [rsp+28h],rax  
00007FF6EC9902DC  mov         qword ptr [rsp+30h],1  
00007FF6EC9902E5  call        core::fmt::Arguments::new_v1_formatted (07FF6EC9904B0h)  
00007FF6EC9902EA  lea         rcx,[rsp+2FFB0h]  
00007FF6EC9902F2  call        std::io::stdio::_print (07FF6EC995440h)  
}
00007FF6EC9902F7  nop  
00007FF6EC9902F8  lea         rsp,[rbp+4FF70h]  
00007FF6EC9902FF  pop         rbp  
00007FF6EC990300  ret  
