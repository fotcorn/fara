cp $1234, %i0
add 5, %i0
cp %i0, $1234

inc // a comment
main:
cp 15, $1234
cp 1234, %i0
ld %i0, %i1
// eof comment

cp 'a', %i1
jmp main
