plain
   Compiling cc v1.0.67
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.93
   Compiling std v0.0.0 (/checkout/library/std)
error: integer literal is too large
    |
    |
452 |         f32::from_bits(self.to_bits() & 0x7fff_ffff)


error: integer literal is too large
    |
    |
567 |         const EXP_MASK: u32 = 0x7f800000;


error: integer literal is too large
    |
    |
568 |         const MAN_MASK: u32 = 0x007fffff;


error: integer literal is too large
    |
    |
451 |         f64::from_bits(self.to_bits() & 0x7fff_ffff_ffff_ffff)


error: integer literal is too large
    |
    |
566 |         const EXP_MASK: u64 = 0x7ff0000000000000;


error: integer literal is too large
    |
    |
567 |         const MAN_MASK: u64 = 0x000fffffffffffff;


error: invalid character in numeric character escape: `C`
    |
    |
628 |         matches!(*self, b'\t' | b'\n' | b'\x0C' | b'\r' | b' ')
    |                                              ^ invalid character in numeric character escape

error: invalid character in numeric character escape: `F`
    |
    |
663 |         matches!(*self, b'\0'..=b'\x1F' | b'\x7F')
    |                                      ^ invalid character in numeric character escape

error: invalid character in numeric character escape: `F`
    |
    |
663 |         matches!(*self, b'\0'..=b'\x1F' | b'\x7F')
    |                                                ^ invalid character in numeric character escape

error: integer literal is too large
  --> library/core/src/num/dec2flt/algorithm.rs:97:22
   |
97 |         set_cw((cw & 0xFCFF) | cw_precision);


error: integer literal is too large
   --> library/core/src/num/dec2flt/algorithm.rs:198:30
    |
198 |         let m_digits = [(m & 0xFF_FF_FF_FF) as u32, (m >> 32) as u32];


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:10:9
10 |         0xe0b62e2929aba83c,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:11:9
11 |         0x8c71dcd9ba0b4926,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:12:9
12 |         0xaf8e5410288e1b6f,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:13:9
13 |         0xdb71e91432b1a24b,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:14:9
14 |         0x892731ac9faf056f,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:15:9
15 |         0xab70fe17c79ac6ca,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:16:9
16 |         0xd64d3d9db981787d,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:17:9
17 |         0x85f0468293f0eb4e,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:18:9
18 |         0xa76c582338ed2622,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:19:9
19 |         0xd1476e2c07286faa,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:20:9
20 |         0x82cca4db847945ca,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:21:9
21 |         0xa37fce126597973d,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:22:9
22 |         0xcc5fc196fefd7d0c,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:23:9
23 |         0xff77b1fcbebcdc4f,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:24:9
24 |         0x9faacf3df73609b1,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:25:9
25 |         0xc795830d75038c1e,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:26:9
26 |         0xf97ae3d0d2446f25,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:27:9
27 |         0x9becce62836ac577,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:28:9
28 |         0xc2e801fb244576d5,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:29:9
29 |         0xf3a20279ed56d48a,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:30:9
30 |         0x9845418c345644d7,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:31:9
31 |         0xbe5691ef416bd60c,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:32:9
32 |         0xedec366b11c6cb8f,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:33:9
33 |         0x94b3a202eb1c3f39,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:34:9
34 |         0xb9e08a83a5e34f08,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:35:9
35 |         0xe858ad248f5c22ca,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:36:9
36 |         0x91376c36d99995be,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:37:9
37 |         0xb58547448ffffb2e,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:38:9
38 |         0xe2e69915b3fff9f9,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:39:9
39 |         0x8dd01fad907ffc3c,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:40:9
40 |         0xb1442798f49ffb4b,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:41:9
41 |         0xdd95317f31c7fa1d,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:42:9
42 |         0x8a7d3eef7f1cfc52,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:43:9
43 |         0xad1c8eab5ee43b67,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:44:9
44 |         0xd863b256369d4a41,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:45:9
45 |         0x873e4f75e2224e68,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:46:9
46 |         0xa90de3535aaae202,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:47:9
47 |         0xd3515c2831559a83,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:48:9
48 |         0x8412d9991ed58092,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:49:9
49 |         0xa5178fff668ae0b6,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:50:9
50 |         0xce5d73ff402d98e4,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:51:9
51 |         0x80fa687f881c7f8e,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:52:9
52 |         0xa139029f6a239f72,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:53:9
53 |         0xc987434744ac874f,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:54:9
54 |         0xfbe9141915d7a922,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:55:9
55 |         0x9d71ac8fada6c9b5,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:56:9
56 |         0xc4ce17b399107c23,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:57:9
57 |         0xf6019da07f549b2b,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:58:9
58 |         0x99c102844f94e0fb,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:59:9
59 |         0xc0314325637a193a,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:60:9
60 |         0xf03d93eebc589f88,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:61:9
61 |         0x96267c7535b763b5,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:62:9
62 |         0xbbb01b9283253ca3,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:63:9
63 |         0xea9c227723ee8bcb,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:64:9
64 |         0x92a1958a7675175f,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:65:9
65 |         0xb749faed14125d37,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:66:9
66 |         0xe51c79a85916f485,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:67:9
67 |         0x8f31cc0937ae58d3,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:68:9
68 |         0xb2fe3f0b8599ef08,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:69:9
69 |         0xdfbdcece67006ac9,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:70:9
70 |         0x8bd6a141006042be,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:71:9
71 |         0xaecc49914078536d,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:72:9
72 |         0xda7f5bf590966849,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:73:9
73 |         0x888f99797a5e012d,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:74:9
74 |         0xaab37fd7d8f58179,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:75:9
75 |         0xd5605fcdcf32e1d7,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:76:9
76 |         0x855c3be0a17fcd26,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:77:9
77 |         0xa6b34ad8c9dfc070,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:78:9
78 |         0xd0601d8efc57b08c,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:79:9
79 |         0x823c12795db6ce57,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:80:9
80 |         0xa2cb1717b52481ed,
   |         ^^^^^^^^^^^^^^^^^^


error: integer literal is too large
  --> library/core/src/num/dec2flt/table.rs:81:9
81 |         0xcb7ddcdda26da269,
   |         ^^^^^^^^^^^^^^^^^^
