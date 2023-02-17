CC=../../llvm-project/build/bin/clang
CFLAGS=--target=fara -nodefaultlibs
SIM=cargo run --manifest-path ../emulator/Cargo.toml --
SIMFLAGS=--loader elf

ifdef DEBUG
CFLAGS += -mllvm -debug
SIMFLAGS += -p
endif

SOURCES=$(wildcard *.asm)
EXECUTABLES=$(SOURCES:.asm=)

all: $(EXECUTABLES)

%: %.asm
	$(CC) $(CFLAGS) $< -o build/$@
	$(SIM) $(SIMFLAGS) build/$@

clean:
	rm -f build/*
