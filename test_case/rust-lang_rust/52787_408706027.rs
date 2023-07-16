 console
$ # these are nameless targets
$ arm-none-eabi-gcc -print-multi-lib | grep v7e
thumb/v7e-m/nofp;@mthumb@march=armv7e-m@mfloat-abi=soft # this one is like our `thumbv7em-none-eabi` target
thumb/v7e-m+fp/softfp;@mthumb@march=armv7e-m+fp@mfloat-abi=softfp
thumb/v7e-m+fp/hard;@mthumb@march=armv7e-m+fp@mfloat-abi=hard
thumb/v7e-m+dp/softfp;@mthumb@march=armv7e-m+fp.dp@mfloat-abi=softfp
thumb/v7e-m+dp/hard;@mthumb@march=armv7e-m+fp.dp@mfloat-abi=hard
