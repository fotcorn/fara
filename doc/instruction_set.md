# instructions

For opcodes, see emulator/src/instruction_set.rs.

## memory

- mov: Copy value from parameter 1 into parameter 2
- ld: Load value from address in parameter 1 into parameter 2
- str: Store value pointed at by address in parameter 1 into parameter 2

## arithmetic

- add
- sub
- inc
- dec
- div
- mul
- mod

## binary

- and
- or
- xor
- not

## compare & jumps

- jmp: Unconditional jump
- je: Jump if equal
- jne: Jump if not equal

### signed

- jls: Jump if lower (signed compare)
- jles: Jump if lower or equal (signed compare)
- jgs: Jump if greater (signed compare)
- jges: Jump if greater or equal (signed compare)

### unsigned

- jlu: Jump if lower (unsigned compare)
- jleu: Jump if lower or equal (unsigned compare)
- jgu: Jump if greater (unsigned compare)
- jgeu: Jump if greater or equal (unsigned compare)

## stack

- push
- pop
- call
- ret

## io

- in
- out

## syscalls

- syscall
- sysret

## other

- halt
