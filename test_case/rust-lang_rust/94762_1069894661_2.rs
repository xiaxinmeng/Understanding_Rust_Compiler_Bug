asm
000000000315c300 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj>:
 315c300:	55                   	push   %rbp
 315c301:	41 57                	push   %r15
 315c303:	41 56                	push   %r14
 315c305:	41 55                	push   %r13
 315c307:	41 54                	push   %r12
 315c309:	53                   	push   %rbx
 315c30a:	48 81 ec a8 06 00 00 	sub    $0x6a8,%rsp
 315c311:	48 89 54 24 18       	mov    %rdx,0x18(%rsp)
 315c316:	48 89 74 24 10       	mov    %rsi,0x10(%rsp)
 315c31b:	41 bc 01 00 00 00    	mov    $0x1,%r12d
 315c321:	83 7f 78 00          	cmpl   $0x0,0x78(%rdi)
 315c325:	0f 85 c8 06 00 00    	jne    315c9f3 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x6f3>
 315c32b:	48 8b 87 60 02 00 00 	mov    0x260(%rdi),%rax
 315c332:	48 8b 40 38          	mov    0x38(%rax),%rax
 315c336:	83 78 28 ff          	cmpl   $0xffffffff,0x28(%rax)
 315c33a:	0f 85 b3 06 00 00    	jne    315c9f3 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x6f3>
 315c340:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
 315c345:	48 c1 e8 20          	shr    $0x20,%rax
 315c349:	48 89 44 24 40       	mov    %rax,0x40(%rsp)
 315c34e:	48 8b 9f 48 02 00 00 	mov    0x248(%rdi),%rbx
 315c355:	48 89 7c 24 08       	mov    %rdi,0x8(%rsp)
 315c35a:	48 8b 87 50 02 00 00 	mov    0x250(%rdi),%rax
 315c361:	48 8b 68 58          	mov    0x58(%rax),%rbp
 315c365:	48 89 ef             	mov    %rbp,%rdi
 315c368:	48 89 de             	mov    %rbx,%rsi
 315c36b:	e8 30 e1 fe ff       	call   314a4a0 <_ZN4llvm15ScalarEvolution25getSmallConstantTripCountEPKNS_4LoopE>
 315c370:	85 c0                	test   %eax,%eax
 315c372:	0f 85 bb 07 00 00    	jne    315cb33 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x833>
 315c378:	80 3d 91 41 f8 01 00 	cmpb   $0x0,0x1f84191(%rip)        # 50e0510 <_ZL31LoopVectorizeWithBlockFrequency+0x80>
 315c37f:	41 bf 00 00 00 00    	mov    $0x0,%r15d
 315c385:	74 1c                	je     315c3a3 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0xa3>
 315c387:	48 89 df             	mov    %rbx,%rdi
 315c38a:	31 f6                	xor    %esi,%esi
 315c38c:	e8 0f e3 fe ff       	call   314a6a0 <_ZN4llvm25getLoopEstimatedTripCountEPNS_4LoopEPj>
 315c391:	41 89 c7             	mov    %eax,%r15d
 315c394:	41 c1 ef 08          	shr    $0x8,%r15d
 315c398:	48 89 c2             	mov    %rax,%rdx
 315c39b:	48 c1 ea 20          	shr    $0x20,%rdx
 315c39f:	84 d2                	test   %dl,%dl
 315c3a1:	75 61                	jne    315c404 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x104>
 315c3a3:	48 89 ef             	mov    %rbp,%rdi
 315c3a6:	48 89 de             	mov    %rbx,%rsi
 315c3a9:	e8 42 0f d5 ff       	call   2ead2f0 <_ZN4llvm15ScalarEvolution20getBackedgeTakenInfoEPKNS_4LoopE>
 315c3ae:	48 89 c7             	mov    %rax,%rdi
 315c3b1:	48 89 ee             	mov    %rbp,%rsi
 315c3b4:	e8 a7 8c 24 ff       	call   23a5060 <_ZNK4llvm15ScalarEvolution17BackedgeTakenInfo14getConstantMaxEPS0_>
 315c3b9:	66 83 78 18 00       	cmpw   $0x0,0x18(%rax)
 315c3be:	75 40                	jne    315c400 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x100>
 315c3c0:	48 85 c0             	test   %rax,%rax
 315c3c3:	74 3b                	je     315c400 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x100>
 315c3c5:	48 8b 68 20          	mov    0x20(%rax),%rbp
 315c3c9:	8b 5d 20             	mov    0x20(%rbp),%ebx
 315c3cc:	48 83 c5 18          	add    $0x18,%rbp
 315c3d0:	83 fb 40             	cmp    $0x40,%ebx
 315c3d3:	0f 87 42 08 00 00    	ja     315cc1b <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x91b>
 315c3d9:	8d 43 c0             	lea    -0x40(%rbx),%eax
 315c3dc:	48 8b 4d 00          	mov    0x0(%rbp),%rcx
 315c3e0:	48 85 c9             	test   %rcx,%rcx
 315c3e3:	0f 84 1f 06 00 00    	je     315ca08 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x708>
 315c3e9:	48 0f bd c9          	bsr    %rcx,%rcx
 315c3ed:	48 83 f1 3f          	xor    $0x3f,%rcx
 315c3f1:	01 c8                	add    %ecx,%eax
 315c3f3:	89 d9                	mov    %ebx,%ecx
 315c3f5:	29 c1                	sub    %eax,%ecx
 315c3f7:	83 f9 20             	cmp    $0x20,%ecx
 315c3fa:	0f 86 1f 07 00 00    	jbe    315cb1f <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x81f>
 315c400:	31 d2                	xor    %edx,%edx
 315c402:	31 c0                	xor    %eax,%eax
 315c404:	41 c1 e7 08          	shl    $0x8,%r15d
 315c408:	0f b6 c0             	movzbl %al,%eax
 315c40b:	44 09 f8             	or     %r15d,%eax
 315c40e:	89 44 24 24          	mov    %eax,0x24(%rsp)
 315c412:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
 315c417:	48 8b 80 60 02 00 00 	mov    0x260(%rax),%rax
 315c41e:	48 8b 48 68          	mov    0x68(%rax),%rcx
 315c422:	48 8b 40 70          	mov    0x70(%rax),%rax
 315c426:	48 89 4c 24 38       	mov    %rcx,0x38(%rsp)
 315c42b:	48 89 44 24 30       	mov    %rax,0x30(%rsp)
 315c430:	48 39 c1             	cmp    %rax,%rcx
 315c433:	0f 95 44 24 07       	setne  0x7(%rsp)
 315c438:	48 89 54 24 60       	mov    %rdx,0x60(%rsp)
 315c43d:	84 d2                	test   %dl,%dl
 315c43f:	74 10                	je     315c451 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x151>
 315c441:	8b 44 24 24          	mov    0x24(%rsp),%eax
 315c445:	39 05 c5 3a f8 01    	cmp    %eax,0x1f83ac5(%rip)        # 50dff10 <_ZL32TinyTripCountInterleaveThreshold+0x80>
 315c44b:	0f 87 78 06 00 00    	ja     315cac9 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x7c9>
 315c451:	48 8d 54 24 28       	lea    0x28(%rsp),%rdx
 315c456:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
 315c45b:	48 89 02             	mov    %rax,(%rdx)
 315c45e:	48 8d 9c 24 18 01 00 	lea    0x118(%rsp),%rbx
 315c465:	00 
 315c466:	b9 01 00 00 00       	mov    $0x1,%ecx
 315c46b:	48 89 df             	mov    %rbx,%rdi
 315c46e:	48 8b 74 24 08       	mov    0x8(%rsp),%rsi
 315c473:	e8 18 a6 01 00       	call   3176a90 <_ZN4llvm26LoopVectorizationCostModel22calculateRegisterUsageENS_8ArrayRefINS_12ElementCountEEE>
 315c478:	48 8b 2b             	mov    (%rbx),%rbp
 315c47b:	48 8d 7c 24 68       	lea    0x68(%rsp),%rdi
 315c480:	48 c7 07 01 00 00 00 	movq   $0x1,(%rdi)
 315c487:	bb ff ff ff ff       	mov    $0xffffffff,%ebx
 315c48c:	89 5f 08             	mov    %ebx,0x8(%rdi)
 315c48f:	89 5f 10             	mov    %ebx,0x10(%rdi)
 315c492:	89 5f 18             	mov    %ebx,0x18(%rdi)
 315c495:	89 5f 20             	mov    %ebx,0x20(%rdi)
 315c498:	48 89 ee             	mov    %rbp,%rsi
 315c49b:	e8 70 ce 01 00       	call   3179310 <_ZN4llvm13SmallDenseMapIjjLj4ENS_12DenseMapInfoIjvEENS_6detail12DenseMapPairIjjEEE8copyFromERKS6_>
 315c4a0:	48 8d 84 24 a0 00 00 	lea    0xa0(%rsp),%rax
 315c4a7:	00 
 315c4a8:	48 89 40 f0          	mov    %rax,-0x10(%rax)
 315c4ac:	49 be 00 00 00 00 04 	movabs $0x400000000,%r14
 315c4b3:	00 00 00 
 315c4b6:	4c 89 70 f8          	mov    %r14,-0x8(%rax)
 315c4ba:	83 7d 30 00          	cmpl   $0x0,0x30(%rbp)
 315c4be:	74 11                	je     315c4d1 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x1d1>
 315c4c0:	48 8d 75 28          	lea    0x28(%rbp),%rsi
 315c4c4:	48 8d bc 24 90 00 00 	lea    0x90(%rsp),%rdi
 315c4cb:	00 
 315c4cc:	e8 7f 08 00 00       	call   315cd50 <_ZN4llvm15SmallVectorImplISt4pairIjjEEaSERKS3_>
 315c4d1:	48 8d bc 24 c0 00 00 	lea    0xc0(%rsp),%rdi
 315c4d8:	00 
 315c4d9:	48 c7 07 01 00 00 00 	movq   $0x1,(%rdi)
 315c4e0:	89 5f 08             	mov    %ebx,0x8(%rdi)
 315c4e3:	89 5f 10             	mov    %ebx,0x10(%rdi)
 315c4e6:	89 5f 18             	mov    %ebx,0x18(%rdi)
 315c4e9:	89 5f 20             	mov    %ebx,0x20(%rdi)
 315c4ec:	48 8d 75 58          	lea    0x58(%rbp),%rsi
 315c4f0:	e8 1b ce 01 00       	call   3179310 <_ZN4llvm13SmallDenseMapIjjLj4ENS_12DenseMapInfoIjvEENS_6detail12DenseMapPairIjjEEE8copyFromERKS6_>
 315c4f5:	48 8d 84 24 f8 00 00 	lea    0xf8(%rsp),%rax
 315c4fc:	00 
 315c4fd:	48 89 40 f0          	mov    %rax,-0x10(%rax)
 315c501:	4c 89 70 f8          	mov    %r14,-0x8(%rax)
 315c505:	83 bd 88 00 00 00 00 	cmpl   $0x0,0x88(%rbp)
 315c50c:	74 14                	je     315c522 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x222>
 315c50e:	48 83 ed 80          	sub    $0xffffffffffffff80,%rbp
 315c512:	48 8d bc 24 e8 00 00 	lea    0xe8(%rsp),%rdi
 315c519:	00 
 315c51a:	48 89 ee             	mov    %rbp,%rsi
 315c51d:	e8 2e 08 00 00       	call   315cd50 <_ZN4llvm15SmallVectorImplISt4pairIjjEEaSERKS3_>
 315c522:	48 8b ac 24 18 01 00 	mov    0x118(%rsp),%rbp
 315c529:	00 
 315c52a:	8b 84 24 20 01 00 00 	mov    0x120(%rsp),%eax
 315c531:	48 85 c0             	test   %rax,%rax
 315c534:	74 63                	je     315c599 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x299>
 315c536:	48 69 d8 b0 00 00 00 	imul   $0xb0,%rax,%rbx
 315c53d:	4c 8d 7d 88          	lea    -0x78(%rbp),%r15
 315c541:	49 89 ec             	mov    %rbp,%r12
 315c544:	49 83 c4 e0          	add    $0xffffffffffffffe0,%r12
 315c548:	48 8b 7c 1d d0       	mov    -0x30(%rbp,%rbx,1),%rdi
 315c54d:	49 8d 04 1c          	lea    (%r12,%rbx,1),%rax
 315c551:	48 39 f8             	cmp    %rdi,%rax
 315c554:	0f 85 ce 06 00 00    	jne    315cc28 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x928>
 315c55a:	f6 44 1d a8 01       	testb  $0x1,-0x58(%rbp,%rbx,1)
 315c55f:	0f 84 74 06 00 00    	je     315cbd9 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x8d9>
 315c565:	48 8b bc 1d 78 ff ff 	mov    -0x88(%rbp,%rbx,1),%rdi
 315c56c:	ff 
 315c56d:	49 8d 04 1f          	lea    (%r15,%rbx,1),%rax
 315c571:	48 39 f8             	cmp    %rdi,%rax
 315c574:	0f 85 b8 06 00 00    	jne    315cc32 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x932>
 315c57a:	f6 84 1d 50 ff ff ff 	testb  $0x1,-0xb0(%rbp,%rbx,1)
 315c581:	01 
 315c582:	0f 84 60 06 00 00    	je     315cbe8 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x8e8>
 315c588:	48 81 c3 50 ff ff ff 	add    $0xffffffffffffff50,%rbx
 315c58f:	75 b7                	jne    315c548 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x248>
 315c591:	48 8b ac 24 18 01 00 	mov    0x118(%rsp),%rbp
 315c598:	00 
 315c599:	48 8d 84 24 28 01 00 	lea    0x128(%rsp),%rax
 315c5a0:	00 
 315c5a1:	48 39 c5             	cmp    %rax,%rbp
 315c5a4:	0f 85 92 06 00 00    	jne    315cc3c <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x93c>
 315c5aa:	4c 8b ac 24 e8 00 00 	mov    0xe8(%rsp),%r13
 315c5b1:	00 
 315c5b2:	8b 84 24 f0 00 00 00 	mov    0xf0(%rsp),%eax
 315c5b9:	48 85 c0             	test   %rax,%rax
 315c5bc:	74 27                	je     315c5e5 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x2e5>
 315c5be:	48 8d 0c c5 00 00 00 	lea    0x0(,%rax,8),%rcx
 315c5c5:	00 
 315c5c6:	31 d2                	xor    %edx,%edx
 315c5c8:	be 01 00 00 00       	mov    $0x1,%esi
 315c5cd:	41 8b 7c 15 04       	mov    0x4(%r13,%rdx,1),%edi
 315c5d2:	85 ff                	test   %edi,%edi
 315c5d4:	0f 44 fe             	cmove  %esi,%edi
 315c5d7:	41 89 7c 15 04       	mov    %edi,0x4(%r13,%rdx,1)
 315c5dc:	48 83 c2 08          	add    $0x8,%rdx
 315c5e0:	48 39 d1             	cmp    %rdx,%rcx
 315c5e3:	75 e8                	jne    315c5cd <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x2cd>
 315c5e5:	83 7c 24 40 01       	cmpl   $0x1,0x40(%rsp)
 315c5ea:	0f 95 c1             	setne  %cl
 315c5ed:	83 7c 24 10 01       	cmpl   $0x1,0x10(%rsp)
 315c5f2:	0f 94 c2             	sete   %dl
 315c5f5:	20 ca                	and    %cl,%dl
 315c5f7:	88 54 24 06          	mov    %dl,0x6(%rsp)
 315c5fb:	41 bc ff ff ff ff    	mov    $0xffffffff,%r12d
 315c601:	48 85 c0             	test   %rax,%rax
 315c604:	0f 84 e7 01 00 00    	je     315c7f1 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x4f1>
 315c60a:	48 8d 04 c5 00 00 00 	lea    0x0(,%rax,8),%rax
 315c611:	00 
 315c612:	4c 01 e8             	add    %r13,%rax
 315c615:	48 89 44 24 58       	mov    %rax,0x58(%rsp)
 315c61a:	41 bc ff ff ff ff    	mov    $0xffffffff,%r12d
 315c620:	48 8d 0d 29 39 f8 01 	lea    0x1f83929(%rip),%rcx        # 50dff50 <_ZL24ForceTargetNumScalarRegs>
 315c627:	48 8d 15 e2 39 f8 01 	lea    0x1f839e2(%rip),%rdx        # 50e0010 <_ZL24ForceTargetNumVectorRegs>
 315c62e:	b8 80 00 00 00       	mov    $0x80,%eax
 315c633:	48 01 c1             	add    %rax,%rcx
 315c636:	48 89 4c 24 50       	mov    %rcx,0x50(%rsp)
 315c63b:	48 01 c2             	add    %rax,%rdx
 315c63e:	48 89 54 24 48       	mov    %rdx,0x48(%rsp)
 315c643:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
 315c648:	48 8b 80 68 02 00 00 	mov    0x268(%rax),%rax
 315c64f:	41 8b 75 00          	mov    0x0(%r13),%esi
 315c653:	48 8b 38             	mov    (%rax),%rdi
 315c656:	48 8b 07             	mov    (%rdi),%rax
 315c659:	ff 90 90 02 00 00    	call   *0x290(%rax)
 315c65f:	89 c5                	mov    %eax,%ebp
 315c661:	80 7c 24 06 00       	cmpb   $0x0,0x6(%rsp)
 315c666:	48 8d 0d ab 39 f8 01 	lea    0x1f839ab(%rip),%rcx        # 50e0018 <_ZL24ForceTargetNumVectorRegs+0x8>
 315c66d:	48 8d 05 e4 38 f8 01 	lea    0x1f838e4(%rip),%rax        # 50dff58 <_ZL24ForceTargetNumScalarRegs+0x8>
 315c674:	48 0f 45 c8          	cmovne %rax,%rcx
 315c678:	48 8b 44 24 48       	mov    0x48(%rsp),%rax
 315c67d:	48 0f 45 44 24 50    	cmovne 0x50(%rsp),%rax
 315c683:	66 83 39 00          	cmpw   $0x0,(%rcx)
 315c687:	74 02                	je     315c68b <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x38b>
 315c689:	8b 28                	mov    (%rax),%ebp
 315c68b:	45 8b 7d 04          	mov    0x4(%r13),%r15d
 315c68f:	f6 44 24 68 01       	testb  $0x1,0x68(%rsp)
 315c694:	48 8b 44 24 70       	mov    0x70(%rsp),%rax
 315c699:	48 8d 4c 24 70       	lea    0x70(%rsp),%rcx
 315c69e:	48 0f 45 c1          	cmovne %rcx,%rax
 315c6a2:	8b 54 24 78          	mov    0x78(%rsp),%edx
 315c6a6:	b9 04 00 00 00       	mov    $0x4,%ecx
 315c6ab:	0f 45 d1             	cmovne %ecx,%edx
 315c6ae:	41 89 d1             	mov    %edx,%r9d
 315c6b1:	85 d2                	test   %edx,%edx
 315c6b3:	74 28                	je     315c6dd <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x3dd>
 315c6b5:	41 8b 75 00          	mov    0x0(%r13),%esi
 315c6b9:	8d 0c f6             	lea    (%rsi,%rsi,8),%ecx
 315c6bc:	8d 3c 8e             	lea    (%rsi,%rcx,4),%edi
 315c6bf:	ff ca                	dec    %edx
 315c6c1:	21 d7                	and    %edx,%edi
 315c6c3:	8b 0c f8             	mov    (%rax,%rdi,8),%ecx
 315c6c6:	39 ce                	cmp    %ecx,%esi
 315c6c8:	0f 84 76 04 00 00    	je     315cb44 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x844>
 315c6ce:	41 b8 01 00 00 00    	mov    $0x1,%r8d
 315c6d4:	83 f9 ff             	cmp    $0xffffffff,%ecx
 315c6d7:	0f 85 e1 04 00 00    	jne    315cbbe <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x8be>
 315c6dd:	4a 8d 1c c8          	lea    (%rax,%r9,8),%rbx
 315c6e1:	4a 8d 0c c8          	lea    (%rax,%r9,8),%rcx
 315c6e5:	31 c0                	xor    %eax,%eax
 315c6e7:	48 39 cb             	cmp    %rcx,%rbx
 315c6ea:	74 5c                	je     315c748 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x448>
 315c6ec:	8b 4b 04             	mov    0x4(%rbx),%ecx
 315c6ef:	3b 8c 24 98 00 00 00 	cmp    0x98(%rsp),%ecx
 315c6f6:	74 50                	je     315c748 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x448>
 315c6f8:	41 8b 45 00          	mov    0x0(%r13),%eax
 315c6fc:	89 44 24 28          	mov    %eax,0x28(%rsp)
 315c700:	c7 44 24 2c 00 00 00 	movl   $0x0,0x2c(%rsp)
 315c707:	00 
 315c708:	48 8d bc 24 18 01 00 	lea    0x118(%rsp),%rdi
 315c70f:	00 
 315c710:	48 8d 74 24 68       	lea    0x68(%rsp),%rsi
 315c715:	48 8d 54 24 28       	lea    0x28(%rsp),%rdx
 315c71a:	48 8d 4c 24 2c       	lea    0x2c(%rsp),%rcx
 315c71f:	e8 bc cc 01 00       	call   31793e0 <_ZN4llvm12DenseMapBaseINS_13SmallDenseMapIjjLj4ENS_12DenseMapInfoIjvEENS_6detail12DenseMapPairIjjEEEEjjS3_S6_E11try_emplaceIJRKjEEESt4pairINS_16DenseMapIteratorIjjS3_S6_Lb0EEEbESB_DpOT_>
 315c724:	4c 8b b4 24 18 01 00 	mov    0x118(%rsp),%r14
 315c72b:	00 
 315c72c:	80 bc 24 28 01 00 00 	cmpb   $0x0,0x128(%rsp)
 315c733:	00 
 315c734:	75 7b                	jne    315c7b1 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x4b1>
 315c736:	41 8b 46 04          	mov    0x4(%r14),%eax
 315c73a:	89 c0                	mov    %eax,%eax
 315c73c:	48 8b 8c 24 90 00 00 	mov    0x90(%rsp),%rcx
 315c743:	00 
 315c744:	8b 44 c1 04          	mov    0x4(%rcx,%rax,8),%eax
 315c748:	29 c5                	sub    %eax,%ebp
 315c74a:	41 39 ef             	cmp    %ebp,%r15d
 315c74d:	0f 87 fa 03 00 00    	ja     315cb4d <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x84d>
 315c753:	89 e8                	mov    %ebp,%eax
 315c755:	31 d2                	xor    %edx,%edx
 315c757:	41 f7 f7             	div    %r15d
 315c75a:	48 0f bd c8          	bsr    %rax,%rcx
 315c75e:	b8 01 00 00 00       	mov    $0x1,%eax
 315c763:	48 d3 e0             	shl    %cl,%rax
 315c766:	80 3d a3 40 f8 01 00 	cmpb   $0x0,0x1f840a3(%rip)        # 50e0810 <_ZL24EnableIndVarRegisterHeur+0x80>
 315c76d:	74 2a                	je     315c799 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x499>
 315c76f:	ff cd                	dec    %ebp
 315c771:	41 ff cf             	dec    %r15d
 315c774:	b8 01 00 00 00       	mov    $0x1,%eax
 315c779:	44 0f 44 f8          	cmove  %eax,%r15d
 315c77d:	41 39 ef             	cmp    %ebp,%r15d
 315c780:	0f 87 0d 04 00 00    	ja     315cb93 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x893>
 315c786:	89 e8                	mov    %ebp,%eax
 315c788:	31 d2                	xor    %edx,%edx
 315c78a:	41 f7 f7             	div    %r15d
 315c78d:	48 0f bd c8          	bsr    %rax,%rcx
 315c791:	b8 01 00 00 00       	mov    $0x1,%eax
 315c796:	48 d3 e0             	shl    %cl,%rax
 315c799:	41 39 c4             	cmp    %eax,%r12d
 315c79c:	44 0f 47 e0          	cmova  %eax,%r12d
 315c7a0:	49 83 c5 08          	add    $0x8,%r13
 315c7a4:	4c 3b 6c 24 58       	cmp    0x58(%rsp),%r13
 315c7a9:	0f 85 94 fe ff ff    	jne    315c643 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x343>
 315c7af:	eb 40                	jmp    315c7f1 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x4f1>
 315c7b1:	41 8b 5d 00          	mov    0x0(%r13),%ebx
 315c7b5:	8b 94 24 98 00 00 00 	mov    0x98(%rsp),%edx
 315c7bc:	3b 94 24 9c 00 00 00 	cmp    0x9c(%rsp),%edx
 315c7c3:	0f 83 a5 04 00 00    	jae    315cc6e <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x96e>
 315c7c9:	48 8b 84 24 90 00 00 	mov    0x90(%rsp),%rax
 315c7d0:	00 
 315c7d1:	89 d1                	mov    %edx,%ecx
 315c7d3:	48 89 1c c8          	mov    %rbx,(%rax,%rcx,8)
 315c7d7:	8b 84 24 98 00 00 00 	mov    0x98(%rsp),%eax
 315c7de:	8d 48 01             	lea    0x1(%rax),%ecx
 315c7e1:	89 8c 24 98 00 00 00 	mov    %ecx,0x98(%rsp)
 315c7e8:	41 89 46 04          	mov    %eax,0x4(%r14)
 315c7ec:	e9 49 ff ff ff       	jmp    315c73a <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x43a>
 315c7f1:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
 315c7f6:	48 8b 80 68 02 00 00 	mov    0x268(%rax),%rax
 315c7fd:	48 8b 38             	mov    (%rax),%rdi
 315c800:	48 8b 07             	mov    (%rdi),%rax
 315c803:	48 8b 74 24 10       	mov    0x10(%rsp),%rsi
 315c808:	ff 90 20 03 00 00    	call   *0x320(%rax)
 315c80e:	89 c1                	mov    %eax,%ecx
 315c810:	48 8d 15 c1 38 f8 01 	lea    0x1f838c1(%rip),%rdx        # 50e00d8 <_ZL36ForceTargetMaxScalarInterleaveFactor+0x8>
 315c817:	48 8d 35 b2 38 f8 01 	lea    0x1f838b2(%rip),%rsi        # 50e00d0 <_ZL36ForceTargetMaxScalarInterleaveFactor>
 315c81e:	48 8d 3d 73 39 f8 01 	lea    0x1f83973(%rip),%rdi        # 50e0198 <_ZL36ForceTargetMaxVectorInterleaveFactor+0x8>
 315c825:	48 8d 05 64 39 f8 01 	lea    0x1f83964(%rip),%rax        # 50e0190 <_ZL36ForceTargetMaxVectorInterleaveFactor>
 315c82c:	bd 80 00 00 00       	mov    $0x80,%ebp
 315c831:	48 01 ee             	add    %rbp,%rsi
 315c834:	48 01 e8             	add    %rbp,%rax
 315c837:	80 7c 24 06 00       	cmpb   $0x0,0x6(%rsp)
 315c83c:	48 0f 45 fa          	cmovne %rdx,%rdi
 315c840:	48 0f 45 c6          	cmovne %rsi,%rax
 315c844:	66 83 3f 00          	cmpw   $0x0,(%rdi)
 315c848:	74 02                	je     315c84c <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x54c>
 315c84a:	8b 08                	mov    (%rax),%ecx
 315c84c:	80 7c 24 60 00       	cmpb   $0x0,0x60(%rsp)
 315c851:	0f 85 aa 02 00 00    	jne    315cb01 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x801>
 315c857:	45 85 e4             	test   %r12d,%r12d
 315c85a:	bd 01 00 00 00       	mov    $0x1,%ebp
 315c85f:	41 0f 45 ec          	cmovne %r12d,%ebp
 315c863:	41 39 cc             	cmp    %ecx,%r12d
 315c866:	0f 47 e9             	cmova  %ecx,%ebp
 315c869:	83 7c 24 18 00       	cmpl   $0x0,0x18(%rsp)
 315c86e:	0f 84 e0 02 00 00    	je     315cb54 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x854>
 315c874:	83 7c 24 40 01       	cmpl   $0x1,0x40(%rsp)
 315c879:	0f 95 c0             	setne  %al
 315c87c:	48 8b 54 24 10       	mov    0x10(%rsp),%rdx
 315c881:	85 d2                	test   %edx,%edx
 315c883:	0f 94 c1             	sete   %cl
 315c886:	08 c1                	or     %al,%cl
 315c888:	83 fa 02             	cmp    $0x2,%edx
 315c88b:	0f 92 c0             	setb   %al
 315c88e:	84 c1                	test   %al,%cl
 315c890:	75 10                	jne    315c8a2 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x5a2>
 315c892:	48 8b 44 24 30       	mov    0x30(%rsp),%rax
 315c897:	48 39 44 24 38       	cmp    %rax,0x38(%rsp)
 315c89c:	0f 85 c4 03 00 00    	jne    315cc66 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x966>
 315c8a2:	31 f6                	xor    %esi,%esi
 315c8a4:	8a 44 24 07          	mov    0x7(%rsp),%al
 315c8a8:	40 88 c6             	mov    %al,%sil
 315c8ab:	80 7c 24 06 00       	cmpb   $0x0,0x6(%rsp)
 315c8b0:	74 35                	je     315c8e7 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x5e7>
 315c8b2:	48 8b 4c 24 08       	mov    0x8(%rsp),%rcx
 315c8b7:	48 8b 81 60 02 00 00 	mov    0x260(%rcx),%rax
 315c8be:	48 8b 89 68 02 00 00 	mov    0x268(%rcx),%rcx
 315c8c5:	48 8b 40 38          	mov    0x38(%rax),%rax
 315c8c9:	48 8b 40 08          	mov    0x8(%rax),%rax
 315c8cd:	8a 18                	mov    (%rax),%bl
 315c8cf:	48 8b 39             	mov    (%rcx),%rdi
 315c8d2:	48 8b 07             	mov    (%rdi),%rax
 315c8d5:	ff 90 20 02 00 00    	call   *0x220(%rax)
 315c8db:	41 89 c6             	mov    %eax,%r14d
 315c8de:	84 db                	test   %bl,%bl
 315c8e0:	74 20                	je     315c902 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x602>
 315c8e2:	e9 d1 01 00 00       	jmp    315cab8 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x7b8>
 315c8e7:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
 315c8ec:	48 8b 80 68 02 00 00 	mov    0x268(%rax),%rax
 315c8f3:	48 8b 38             	mov    (%rax),%rdi
 315c8f6:	48 8b 07             	mov    (%rdi),%rax
 315c8f9:	ff 90 20 02 00 00    	call   *0x220(%rax)
 315c8ff:	45 31 f6             	xor    %r14d,%r14d
 315c902:	8b 0d 48 3b f8 01    	mov    0x1f83b48(%rip),%ecx        # 50e0450 <_ZL13SmallLoopCost+0x80>
 315c908:	39 4c 24 18          	cmp    %ecx,0x18(%rsp)
 315c90c:	0f 83 a6 01 00 00    	jae    315cab8 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x7b8>
 315c912:	89 c8                	mov    %ecx,%eax
 315c914:	31 d2                	xor    %edx,%edx
 315c916:	f7 74 24 18          	divl   0x18(%rsp)
 315c91a:	48 0f bd c8          	bsr    %rax,%rcx
 315c91e:	41 bf 01 00 00 00    	mov    $0x1,%r15d
 315c924:	49 d3 e7             	shl    %cl,%r15
 315c927:	44 39 fd             	cmp    %r15d,%ebp
 315c92a:	44 0f 46 fd          	cmovbe %ebp,%r15d
 315c92e:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
 315c933:	48 8b 88 60 02 00 00 	mov    0x260(%rax),%rcx
 315c93a:	48 8b 41 38          	mov    0x38(%rcx),%rax
 315c93e:	8b 70 20             	mov    0x20(%rax),%esi
 315c941:	8b 78 24             	mov    0x24(%rax),%edi
 315c944:	85 ff                	test   %edi,%edi
 315c946:	bb 01 00 00 00       	mov    $0x1,%ebx
 315c94b:	0f 44 fb             	cmove  %ebx,%edi
 315c94e:	89 e8                	mov    %ebp,%eax
 315c950:	31 d2                	xor    %edx,%edx
 315c952:	f7 f7                	div    %edi
 315c954:	41 89 c4             	mov    %eax,%r12d
 315c957:	85 f6                	test   %esi,%esi
 315c959:	0f 44 f3             	cmove  %ebx,%esi
 315c95c:	89 e8                	mov    %ebp,%eax
 315c95e:	31 d2                	xor    %edx,%edx
 315c960:	f7 f6                	div    %esi
 315c962:	41 89 c5             	mov    %eax,%r13d
 315c965:	48 8b 44 24 30       	mov    0x30(%rsp),%rax
 315c96a:	48 39 44 24 38       	cmp    %rax,0x38(%rsp)
 315c96f:	0f 85 9d 00 00 00    	jne    315ca12 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x712>
 315c975:	80 3d 54 3c f8 01 00 	cmpb   $0x0,0x1f83c54(%rip)        # 50e05d0 <_ZL32EnableLoadStoreRuntimeInterleave+0x80>
 315c97c:	74 0c                	je     315c98a <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x68a>
 315c97e:	45 39 ec             	cmp    %r13d,%r12d
 315c981:	45 0f 42 e5          	cmovb  %r13d,%r12d
 315c985:	45 39 fc             	cmp    %r15d,%r12d
 315c988:	77 16                	ja     315c9a0 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x6a0>
 315c98a:	80 3d ff 3c f8 01 00 	cmpb   $0x0,0x1f83cff(%rip)        # 50e0690 <_ZL34InterleaveSmallLoopScalarReduction+0x80>
 315c991:	74 0a                	je     315c99d <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x69d>
 315c993:	41 80 f6 01          	xor    $0x1,%r14b
 315c997:	0f 84 c0 02 00 00    	je     315cc5d <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x95d>
 315c99d:	45 89 fc             	mov    %r15d,%r12d
 315c9a0:	48 8b bc 24 e8 00 00 	mov    0xe8(%rsp),%rdi
 315c9a7:	00 
 315c9a8:	48 8d 84 24 e8 00 00 	lea    0xe8(%rsp),%rax
 315c9af:	00 
 315c9b0:	48 83 c0 10          	add    $0x10,%rax
 315c9b4:	48 39 c7             	cmp    %rax,%rdi
 315c9b7:	0f 85 8c 02 00 00    	jne    315cc49 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x949>
 315c9bd:	f6 84 24 c0 00 00 00 	testb  $0x1,0xc0(%rsp)
 315c9c4:	01 
 315c9c5:	0f 84 2f 02 00 00    	je     315cbfa <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x8fa>
 315c9cb:	48 8b bc 24 90 00 00 	mov    0x90(%rsp),%rdi
 315c9d2:	00 
 315c9d3:	48 8d 84 24 90 00 00 	lea    0x90(%rsp),%rax
 315c9da:	00 
 315c9db:	48 83 c0 10          	add    $0x10,%rax
 315c9df:	48 39 c7             	cmp    %rax,%rdi
 315c9e2:	0f 85 6b 02 00 00    	jne    315cc53 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x953>
 315c9e8:	f6 44 24 68 01       	testb  $0x1,0x68(%rsp)
 315c9ed:	0f 84 19 02 00 00    	je     315cc0c <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x90c>
 315c9f3:	44 89 e0             	mov    %r12d,%eax
 315c9f6:	48 81 c4 a8 06 00 00 	add    $0x6a8,%rsp
 315c9fd:	5b                   	pop    %rbx
 315c9fe:	41 5c                	pop    %r12
 315ca00:	41 5d                	pop    %r13
 315ca02:	41 5e                	pop    %r14
 315ca04:	41 5f                	pop    %r15
 315ca06:	5d                   	pop    %rbp
 315ca07:	c3                   	ret    
 315ca08:	b9 40 00 00 00       	mov    $0x40,%ecx
 315ca0d:	e9 df f9 ff ff       	jmp    315c3f1 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0xf1>
 315ca12:	48 8b 79 68          	mov    0x68(%rcx),%rdi
 315ca16:	48 8b 71 70          	mov    0x70(%rcx),%rsi
 315ca1a:	48 89 f1             	mov    %rsi,%rcx
 315ca1d:	48 29 f9             	sub    %rdi,%rcx
 315ca20:	49 b8 a3 8b 2e ba e8 	movabs $0x2e8ba2e8ba2e8ba3,%r8
 315ca27:	a2 8b 2e 
 315ca2a:	48 81 f9 11 02 00 00 	cmp    $0x211,%rcx
 315ca31:	0f 8d 60 02 00 00    	jge    315cc97 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x997>
 315ca37:	48 89 f8             	mov    %rdi,%rax
 315ca3a:	48 c1 f9 04          	sar    $0x4,%rcx
 315ca3e:	49 0f af c8          	imul   %r8,%rcx
 315ca42:	48 83 f9 01          	cmp    $0x1,%rcx
 315ca46:	0f 85 4e 01 00 00    	jne    315cb9a <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x89a>
 315ca4c:	8b 48 28             	mov    0x28(%rax),%ecx
 315ca4f:	83 c1 f1             	add    $0xfffffff1,%ecx
 315ca52:	83 f9 02             	cmp    $0x2,%ecx
 315ca55:	0f 82 b4 02 00 00    	jb     315cd0f <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0xa0f>
 315ca5b:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
 315ca60:	48 8b 80 48 02 00 00 	mov    0x248(%rax),%rax
 315ca67:	48 8b 00             	mov    (%rax),%rax
 315ca6a:	48 85 c0             	test   %rax,%rax
 315ca6d:	0f 84 02 ff ff ff    	je     315c975 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x675>
 315ca73:	b9 01 00 00 00       	mov    $0x1,%ecx
 315ca78:	ff c1                	inc    %ecx
 315ca7a:	48 8b 00             	mov    (%rax),%rax
 315ca7d:	48 85 c0             	test   %rax,%rax
 315ca80:	75 f6                	jne    315ca78 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x778>
 315ca82:	83 f9 02             	cmp    $0x2,%ecx
 315ca85:	0f 82 ea fe ff ff    	jb     315c975 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x675>
 315ca8b:	e8 40 3d 03 00       	call   31907d0 <_ZN4llvm6any_ofIRKNS_9MapVectorIPNS_7PHINodeENS_20RecurrenceDescriptorENS_8DenseMapIS3_jNS_12DenseMapInfoIS3_vEENS_6detail12DenseMapPairIS3_jEEEESt6vectorISt4pairIS3_S4_ESaISE_EEEEZNS_26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEjE4$_34EEbOT_T0_>
 315ca90:	84 c0                	test   %al,%al
 315ca92:	0f 85 80 02 00 00    	jne    315cd18 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0xa18>
 315ca98:	8b 05 f2 3e f8 01    	mov    0x1f83ef2(%rip),%eax        # 50e0990 <_ZL26MaxNestedScalarReductionIC+0x80>
 315ca9e:	44 39 f8             	cmp    %r15d,%eax
 315caa1:	44 0f 42 f8          	cmovb  %eax,%r15d
 315caa5:	44 39 e0             	cmp    %r12d,%eax
 315caa8:	44 0f 42 e0          	cmovb  %eax,%r12d
 315caac:	44 39 e8             	cmp    %r13d,%eax
 315caaf:	44 0f 42 e8          	cmovb  %eax,%r13d
 315cab3:	e9 bd fe ff ff       	jmp    315c975 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x675>
 315cab8:	84 c0                	test   %al,%al
 315caba:	41 bc 01 00 00 00    	mov    $0x1,%r12d
 315cac0:	44 0f 45 e5          	cmovne %ebp,%r12d
 315cac4:	e9 d7 fe ff ff       	jmp    315c9a0 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x6a0>
 315cac9:	80 3d c0 3b f8 01 00 	cmpb   $0x0,0x1f83bc0(%rip)        # 50e0690 <_ZL34InterleaveSmallLoopScalarReduction+0x80>
 315cad0:	0f 84 1d ff ff ff    	je     315c9f3 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x6f3>
 315cad6:	48 8b 44 24 30       	mov    0x30(%rsp),%rax
 315cadb:	48 39 44 24 38       	cmp    %rax,0x38(%rsp)
 315cae0:	0f 84 0d ff ff ff    	je     315c9f3 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x6f3>
 315cae6:	83 7c 24 40 01       	cmpl   $0x1,0x40(%rsp)
 315caeb:	0f 84 02 ff ff ff    	je     315c9f3 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x6f3>
 315caf1:	83 7c 24 10 01       	cmpl   $0x1,0x10(%rsp)
 315caf6:	0f 84 55 f9 ff ff    	je     315c451 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x151>
 315cafc:	e9 f2 fe ff ff       	jmp    315c9f3 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x6f3>
 315cb01:	8b 44 24 24          	mov    0x24(%rsp),%eax
 315cb05:	31 d2                	xor    %edx,%edx
 315cb07:	f7 74 24 10          	divl   0x10(%rsp)
 315cb0b:	39 c1                	cmp    %eax,%ecx
 315cb0d:	0f 42 c1             	cmovb  %ecx,%eax
 315cb10:	85 c0                	test   %eax,%eax
 315cb12:	b9 01 00 00 00       	mov    $0x1,%ecx
 315cb17:	0f 45 c8             	cmovne %eax,%ecx
 315cb1a:	e9 38 fd ff ff       	jmp    315c857 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x557>
 315cb1f:	83 fb 41             	cmp    $0x41,%ebx
 315cb22:	72 04                	jb     315cb28 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x828>
 315cb24:	48 8b 6d 00          	mov    0x0(%rbp),%rbp
 315cb28:	8b 45 00             	mov    0x0(%rbp),%eax
 315cb2b:	ff c0                	inc    %eax
 315cb2d:	0f 84 cd f8 ff ff    	je     315c400 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x100>
 315cb33:	41 89 c7             	mov    %eax,%r15d
 315cb36:	41 c1 ef 08          	shr    $0x8,%r15d
 315cb3a:	ba 01 00 00 00       	mov    $0x1,%edx
 315cb3f:	e9 c0 f8 ff ff       	jmp    315c404 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x104>
 315cb44:	48 8d 1c f8          	lea    (%rax,%rdi,8),%rbx
 315cb48:	e9 94 fb ff ff       	jmp    315c6e1 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x3e1>
 315cb4d:	31 c0                	xor    %eax,%eax
 315cb4f:	e9 12 fc ff ff       	jmp    315c766 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x466>
 315cb54:	31 c0                	xor    %eax,%eax
 315cb56:	48 89 44 24 18       	mov    %rax,0x18(%rsp)
 315cb5b:	4c 8d b4 24 18 01 00 	lea    0x118(%rsp),%r14
 315cb62:	00 
 315cb63:	4c 89 f7             	mov    %r14,%rdi
 315cb66:	48 8b 74 24 08       	mov    0x8(%rsp),%rsi
 315cb6b:	48 8b 54 24 10       	mov    0x10(%rsp),%rdx
 315cb70:	31 c9                	xor    %ecx,%ecx
 315cb72:	e8 99 ac ff ff       	call   3157810 <_ZN4llvm26LoopVectorizationCostModel12expectedCostENS_12ElementCountEPNS_15SmallVectorImplISt4pairIPNS_11InstructionES1_EEE>
 315cb77:	41 83 7e 08 00       	cmpl   $0x0,0x8(%r14)
 315cb7c:	0f 85 f2 fc ff ff    	jne    315c874 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x574>
 315cb82:	8b 84 24 18 01 00 00 	mov    0x118(%rsp),%eax
 315cb89:	48 89 44 24 18       	mov    %rax,0x18(%rsp)
 315cb8e:	e9 e1 fc ff ff       	jmp    315c874 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x574>
 315cb93:	31 c0                	xor    %eax,%eax
 315cb95:	e9 ff fb ff ff       	jmp    315c799 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x499>
 315cb9a:	48 83 f9 02          	cmp    $0x2,%rcx
 315cb9e:	0f 85 87 01 00 00    	jne    315cd2b <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0xa2b>
 315cba4:	8b 48 28             	mov    0x28(%rax),%ecx
 315cba7:	83 c1 f1             	add    $0xfffffff1,%ecx
 315cbaa:	83 f9 02             	cmp    $0x2,%ecx
 315cbad:	0f 82 5c 01 00 00    	jb     315cd0f <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0xa0f>
 315cbb3:	48 05 b0 00 00 00    	add    $0xb0,%rax
 315cbb9:	e9 8e fe ff ff       	jmp    315ca4c <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x74c>
 315cbbe:	44 01 c7             	add    %r8d,%edi
 315cbc1:	41 ff c0             	inc    %r8d
 315cbc4:	21 d7                	and    %edx,%edi
 315cbc6:	48 8d 1c f8          	lea    (%rax,%rdi,8),%rbx
 315cbca:	8b 0b                	mov    (%rbx),%ecx
 315cbcc:	39 ce                	cmp    %ecx,%esi
 315cbce:	0f 85 00 fb ff ff    	jne    315c6d4 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x3d4>
 315cbd4:	e9 08 fb ff ff       	jmp    315c6e1 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x3e1>
 315cbd9:	48 8b 7c 1d b0       	mov    -0x50(%rbp,%rbx,1),%rdi
 315cbde:	e8 4d fb 1d ff       	call   233c730 <_ZdlPv>
 315cbe3:	e9 7d f9 ff ff       	jmp    315c565 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x265>
 315cbe8:	48 8b bc 1d 58 ff ff 	mov    -0xa8(%rbp,%rbx,1),%rdi
 315cbef:	ff 
 315cbf0:	e8 3b fb 1d ff       	call   233c730 <_ZdlPv>
 315cbf5:	e9 8e f9 ff ff       	jmp    315c588 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x288>
 315cbfa:	48 8b bc 24 c8 00 00 	mov    0xc8(%rsp),%rdi
 315cc01:	00 
 315cc02:	e8 29 fb 1d ff       	call   233c730 <_ZdlPv>
 315cc07:	e9 bf fd ff ff       	jmp    315c9cb <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x6cb>
 315cc0c:	48 8b 7c 24 70       	mov    0x70(%rsp),%rdi
 315cc11:	e8 1a fb 1d ff       	call   233c730 <_ZdlPv>
 315cc16:	e9 d8 fd ff ff       	jmp    315c9f3 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x6f3>
 315cc1b:	48 89 ef             	mov    %rbp,%rdi
 315cc1e:	e8 9d 65 e9 ff       	call   2ff31c0 <_ZNK4llvm5APInt25countLeadingZerosSlowCaseEv>
 315cc23:	e9 cb f7 ff ff       	jmp    315c3f3 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0xf3>
 315cc28:	e8 53 08 b1 01       	call   4c6d480 <free@plt>
 315cc2d:	e9 28 f9 ff ff       	jmp    315c55a <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x25a>
 315cc32:	e8 49 08 b1 01       	call   4c6d480 <free@plt>
 315cc37:	e9 3e f9 ff ff       	jmp    315c57a <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x27a>
 315cc3c:	48 89 ef             	mov    %rbp,%rdi
 315cc3f:	e8 3c 08 b1 01       	call   4c6d480 <free@plt>
 315cc44:	e9 61 f9 ff ff       	jmp    315c5aa <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x2aa>
 315cc49:	e8 32 08 b1 01       	call   4c6d480 <free@plt>
 315cc4e:	e9 6a fd ff ff       	jmp    315c9bd <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x6bd>
 315cc53:	e8 28 08 b1 01       	call   4c6d480 <free@plt>
 315cc58:	e9 8b fd ff ff       	jmp    315c9e8 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x6e8>
 315cc5d:	d1 ed                	shr    %ebp
 315cc5f:	44 39 fd             	cmp    %r15d,%ebp
 315cc62:	41 0f 42 ef          	cmovb  %r15d,%ebp
 315cc66:	41 89 ec             	mov    %ebp,%r12d
 315cc69:	e9 32 fd ff ff       	jmp    315c9a0 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x6a0>
 315cc6e:	48 ff c2             	inc    %rdx
 315cc71:	b9 08 00 00 00       	mov    $0x8,%ecx
 315cc76:	48 8d bc 24 90 00 00 	lea    0x90(%rsp),%rdi
 315cc7d:	00 
 315cc7e:	48 8d b4 24 a0 00 00 	lea    0xa0(%rsp),%rsi
 315cc85:	00 
 315cc86:	e8 a5 fb 1d ff       	call   233c830 <_ZN4llvm15SmallVectorBaseIjE8grow_podEPvmm>
 315cc8b:	8b 94 24 98 00 00 00 	mov    0x98(%rsp),%edx
 315cc92:	e9 32 fb ff ff       	jmp    315c7c9 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x4c9>
 315cc97:	48 89 c8             	mov    %rcx,%rax
 315cc9a:	49 f7 e0             	mul    %r8
 315cc9d:	48 c1 ea 07          	shr    $0x7,%rdx
 315cca1:	48 ff c2             	inc    %rdx
 315cca4:	48 8d 87 60 01 00 00 	lea    0x160(%rdi),%rax
 315ccab:	8b 98 c8 fe ff ff    	mov    -0x138(%rax),%ebx
 315ccb1:	83 c3 f1             	add    $0xfffffff1,%ebx
 315ccb4:	83 fb 02             	cmp    $0x2,%ebx
 315ccb7:	72 48                	jb     315cd01 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0xa01>
 315ccb9:	8b 98 78 ff ff ff    	mov    -0x88(%rax),%ebx
 315ccbf:	83 c3 f1             	add    $0xfffffff1,%ebx
 315ccc2:	83 fb 02             	cmp    $0x2,%ebx
 315ccc5:	72 42                	jb     315cd09 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0xa09>
 315ccc7:	8b 58 28             	mov    0x28(%rax),%ebx
 315ccca:	83 c3 f1             	add    $0xfffffff1,%ebx
 315cccd:	83 fb 02             	cmp    $0x2,%ebx
 315ccd0:	72 3d                	jb     315cd0f <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0xa0f>
 315ccd2:	8b 98 d8 00 00 00    	mov    0xd8(%rax),%ebx
 315ccd8:	83 c3 f1             	add    $0xfffffff1,%ebx
 315ccdb:	83 fb 02             	cmp    $0x2,%ebx
 315ccde:	72 43                	jb     315cd23 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0xa23>
 315cce0:	48 ff ca             	dec    %rdx
 315cce3:	48 05 c0 02 00 00    	add    $0x2c0,%rax
 315cce9:	48 81 c1 40 fd ff ff 	add    $0xfffffffffffffd40,%rcx
 315ccf0:	48 83 fa 01          	cmp    $0x1,%rdx
 315ccf4:	7f b5                	jg     315ccab <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x9ab>
 315ccf6:	48 05 a0 fe ff ff    	add    $0xfffffffffffffea0,%rax
 315ccfc:	e9 39 fd ff ff       	jmp    315ca3a <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x73a>
 315cd01:	48 05 a0 fe ff ff    	add    $0xfffffffffffffea0,%rax
 315cd07:	eb 06                	jmp    315cd0f <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0xa0f>
 315cd09:	48 05 50 ff ff ff    	add    $0xffffffffffffff50,%rax
 315cd0f:	48 39 f0             	cmp    %rsi,%rax
 315cd12:	0f 84 43 fd ff ff    	je     315ca5b <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x75b>
 315cd18:	41 bc 01 00 00 00    	mov    $0x1,%r12d
 315cd1e:	e9 7d fc ff ff       	jmp    315c9a0 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x6a0>
 315cd23:	48 05 b0 00 00 00    	add    $0xb0,%rax
 315cd29:	eb e4                	jmp    315cd0f <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0xa0f>
 315cd2b:	48 83 f9 03          	cmp    $0x3,%rcx
 315cd2f:	0f 85 26 fd ff ff    	jne    315ca5b <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x75b>
 315cd35:	8b 48 28             	mov    0x28(%rax),%ecx
 315cd38:	83 c1 f1             	add    $0xfffffff1,%ecx
 315cd3b:	83 f9 02             	cmp    $0x2,%ecx
 315cd3e:	72 cf                	jb     315cd0f <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0xa0f>
 315cd40:	48 05 b0 00 00 00    	add    $0xb0,%rax
 315cd46:	e9 59 fe ff ff       	jmp    315cba4 <_ZN4llvm26LoopVectorizationCostModel21selectInterleaveCountENS_12ElementCountEj+0x8a4>
 315cd4b:	cc                   	int3   
 315cd4c:	cc                   	int3   
 315cd4d:	cc                   	int3   
 315cd4e:	cc                   	int3   
 315cd4f:	cc                   	int3   
