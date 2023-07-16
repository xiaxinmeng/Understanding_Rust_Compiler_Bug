
[09:18:08] <simukis> nikomatsakis: tbu- argues that your fix doesn’t apply for, uhm, generic code
[09:18:39] <simukis> something like fn `magic(a : T) where T: Shr { a << 10i8 }`
[09:18:59] <simukis> nmatsakis: ^
[09:19:15] <simukis> nmatsakis: this is re issue you just closed
[09:20:10] <simukis> though I have no idea how that would be implemented.
[09:21:03] <nmatsakis> simukis: ah, well, he's not wrong. we should probably add more impls.
