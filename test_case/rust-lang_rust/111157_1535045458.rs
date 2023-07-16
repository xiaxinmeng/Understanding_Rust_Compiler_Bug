
$ emits="asm llvm-bc llvm-ir obj metadata link dep-info mir"
$ for emit in $emits; do
> rustc -o /dev/null --emit "$emit" main.rs && echo "Successfully emitted $emit" || echo "Failed to emit $emit"
> done
error: couldn't create a temp dir: Operation not permitted (os error 1) at path "/dev/rmeta9vkboa"

error: aborting due to previous error

Failed to emit asm
error: couldn't create a temp dir: Operation not permitted (os error 1) at path "/dev/rmetayrfxZD"

error: aborting due to previous error

Failed to emit llvm-bc
error: couldn't create a temp dir: Operation not permitted (os error 1) at path "/dev/rmetaZ9nLU5"

error: aborting due to previous error

Failed to emit llvm-ir
error: couldn't create a temp dir: Operation not permitted (os error 1) at path "/dev/rmetaksTXms"

error: aborting due to previous error

Failed to emit obj
error: couldn't create a temp dir: Operation not permitted (os error 1) at path "/dev/rmetaM31JYS"

error: aborting due to previous error

Failed to emit metadata
error: couldn't create a temp dir: Operation not permitted (os error 1) at path "/dev/rmeta1PPnSR"

error: aborting due to previous error

Failed to emit link
Successfully emitted dep-info
error: couldn't create a temp dir: Operation not permitted (os error 1) at path "/dev/rmetajaS5Hc"

error: aborting due to previous error

Failed to emit mir
