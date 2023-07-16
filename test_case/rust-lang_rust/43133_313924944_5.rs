asm
__1::reverse2::hbb951fc559ba0b9c:
	and	w8, w0, #0xff
	tst	 w0, #0xff
	csetm	 w9, ne
	cmp		w8, #255
	csinc	w0, w9, wzr, ne
	ret

__1::reverse3::hf325c37bc61a71c6:
	neg	 w0, w0
	ret
