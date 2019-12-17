
mov 1234, %i0
push %i0
pop %i1
je %i0, %i1, ok

out 1, 'N'

ok:
out 1, 'O'
out 1, 'K'
out 1, '\n'

halt
