diff
@@ -82417,9 +82417,10 @@ Disassembly of section .text:
 
 000000000008ea80 <<libc::unix::notbsd::timezone as core::clone::Clone>::clone>:
    8ea80:	0f 0b                	ud2    
-   8ea82:	66 2e 0f 1f 84 00 00 	nopw   %cs:0x0(%rax,%rax,1)
-   8ea89:	00 00 00 
-   8ea8c:	0f 1f 40 00          	nopl   0x0(%rax)
+   8ea82:	0f 0b                	ud2    
+   8ea84:	66 2e 0f 1f 84 00 00 	nopw   %cs:0x0(%rax,%rax,1)
+   8ea8b:	00 00 00 
+   8ea8e:	66 90                	xchg   %ax,%ax
 
 000000000008ea90 <libc::unix::notbsd::CMSG_ALIGN>:
    8ea90:	48 8d 47 07          	lea    0x7(%rdi),%rax
@@ -83112,9 +83113,10 @@ Disassembly of section .text:
 
 000000000008f280 <<libc::unix::notbsd::linux::fpos64_t as core::clone::Clone>::clone>:
    8f280:	0f 0b                	ud2    
-   8f282:	66 2e 0f 1f 84 00 00 	nopw   %cs:0x0(%rax,%rax,1)
-   8f289:	00 00 00 
-   8f28c:	0f 1f 40 00          	nopl   0x0(%rax)
+   8f282:	0f 0b                	ud2    
+   8f284:	66 2e 0f 1f 84 00 00 	nopw   %cs:0x0(%rax,%rax,1)
+   8f28b:	00 00 00 
+   8f28e:	66 90                	xchg   %ax,%ax
 
 000000000008f290 <<libc::unix::notbsd::linux::rlimit64 as core::clone::Clone>::clone>:
    8f290:	48 8b 07             	mov    (%rdi),%rax
@@ -83517,9 +83519,10 @@ Disassembly of section .text:
 
 000000000008f6f0 <<libc::unix::DIR as core::clone::Clone>::clone>:
    8f6f0:	0f 0b                	ud2    
-   8f6f2:	66 2e 0f 1f 84 00 00 	nopw   %cs:0x0(%rax,%rax,1)
-   8f6f9:	00 00 00 
-   8f6fc:	0f 1f 40 00          	nopl   0x0(%rax)
+   8f6f2:	0f 0b                	ud2    
+   8f6f4:	66 2e 0f 1f 84 00 00 	nopw   %cs:0x0(%rax,%rax,1)
+   8f6fb:	00 00 00 
+   8f6fe:	66 90                	xchg   %ax,%ax
 
 000000000008f700 <<libc::unix::rusage as core::clone::Clone>::clone>:
    8f700:	53                   	push   %rbx
