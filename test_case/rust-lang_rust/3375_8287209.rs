
burg: 'impl foo of bar' got changed to 'impl foo: bar', right?
nmatsakis: burg: not...exactly
nmatsakis: more like `impl of foo for bar` got changed to `impl bar: foo`
nmatsakis: that is, `impl rcvr_type: trait_type`
