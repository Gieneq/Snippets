# Compiler
CC = gcc
CFLAGS =-I. -Wall

# Rust project location
RUST_LIB_NAME = my_rust_lib
C_HEADER_NAME = $(RUST_LIB_NAME)_binding.h
RUST_DIR = ../$(RUST_LIB_NAME)
RUST_TARGET = $(RUST_DIR)/target/release
RUST_LIB = $(RUST_TARGET)/lib$(RUST_LIB_NAME).so

# Rust sources
RUST_SRCS = $(RUST_DIR)/src/lib.rs $(RUST_DIR)/Cargo.toml

# Output binary
TARGET = main

# 'all', 'clean', 'test' are not filenames but rules
.PHONY: all clean test

# Default rule will make lib and final executive
all: test $(RUST_LIB) $(TARGET)

# Build Rust library if not exist
$(RUST_LIB): $(RUST_SRCS)
	cargo build --release --manifest-path=$(RUST_DIR)/Cargo.toml

# Build C program and link against Rust .so
$(TARGET): main.c $(C_HEADER_NAME)
	$(CC) main.c $(CFLAGS) -L$(RUST_TARGET) -l$(RUST_LIB_NAME) -o $(TARGET)

# Run Rust unit tests
test:
	cargo test --manifest-path=$(RUST_DIR)/Cargo.toml --quiet

# Rule to run program: first build then run with dynamic library
run: all
	LD_LIBRARY_PATH=$(RUST_TARGET) ./$(TARGET)

clean:
	cargo clean --manifest-path=$(RUST_DIR)/Cargo.toml
	rm -f $(TARGET)