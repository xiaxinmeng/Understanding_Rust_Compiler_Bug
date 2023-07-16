
	.text
	.file	"test339.ll"
	.section	.text.test,"",@
	.globl	test                            # -- Begin function test
	.type	test,@function
test:                                   # @test
	.functype	test (i32, i32) -> (i32)
	.local  	i32, i32, i32, i32
# %bb.0:
	local.get	0
	i32.load8_u	0
	local.set	2
	local.get	2
	i32.eqz
	drop
	local.get	1
	local.get	2
	i32.store8	0
# %bb.1:                                # %bb2
	block   	
	local.get	3
	i32.eqz
	br_if   	0                               # 0: down to label0
# %bb.2:                                # %bb4
	i32.const	0
	local.set	4
	local.get	4
	return
.LBB0_3:                                # %bb3
	end_block                               # label0:
	i32.const	1
	local.set	5
	local.get	5
	return
	end_function
.Lfunc_end0:
	.size	test, .Lfunc_end0-test
                                        # -- End function
