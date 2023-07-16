bash
echo 'fn main() {' > hw.rs
for i in {0..16384}
do
echo '    dbg!("Hello, world!");' >> hw.rs
done
echo '}' >> hw.rs
/usr/bin/time -f "%Uuser %Ssystem %Eelapsed %PCPU (%Xtext+%Ddata %Mmax)k\n%Iinputs+%Ooutputs (%Fmajor+%Rminor)pagefaults %Wswaps\nUsed %MKbytes Max Resident MEM" rustc hw.rs

