# regs

cp $1234, i0
add 5, i0
cp i0, $1234

cp 15, $1234
cp 1234, i0
ld i0, i1

# code
* 16 bit instruction
* 8 bit call type

# display
80x20 console @ 0x10000

# io
in instruction: ready value from port
0x01: keyboard
