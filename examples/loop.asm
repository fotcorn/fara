cp 0, %i0
loop:
cp %i0, %i1
add 48, %i1
out 1, %i1
jne %i0, 5, loop
halt
