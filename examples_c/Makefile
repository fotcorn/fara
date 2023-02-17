CC=../../llvm-project/build/bin/clang
CFLAGS=--target=fara -nodefaultlibs -I libc
SIM=cargo run --manifest-path ../emulator/Cargo.toml --
SIMFLAGS=--loader elf

ifdef DEBUG
CFLAGS += -mllvm -debug
SIMFLAGS += -p
endif

SOURCES=$(wildcard *.c)
EXECUTABLES=$(SOURCES:.c=)

all: $(EXECUTABLES)

%: %.c
	$(CC) $(CFLAGS) $< -o build/$@ libc/crt0.c libc/stdio.c
	$(SIM) $(SIMFLAGS) build/$@

%.S: %.c
	$(CC) $(CFLAGS) -S $< -o build/$@

%.ll: %.c
	$(CC) $(CFLAGS) -S -emit-llvm $< -o build/$@

clean:
	rm -f build/*
