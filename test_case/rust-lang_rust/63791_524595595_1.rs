asm
1.33.0
vpunpckhwd ymm4, ymm3, ymm1
vpunpckhwd ymm0, ymm0, ymm3
vpsrlvd ymm0, ymm4, ymm0
vpsrld ymm0, ymm0, 16
vpunpcklwd ymm1, ymm3, ymm1
vpsrld ymm1, ymm1, 1
vpsrld ymm1, ymm1, 16
vpackusdw ymm0, ymm1, ymm0

1.34.0
vpunpckhwd ymm1, ymm3, ymm1
vpunpckhwd ymm0, ymm0, ymm3
vpsrlvd ymm0, ymm1, ymm0
