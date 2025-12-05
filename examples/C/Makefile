# Simple build helpers for the C examples.
# Usage:
#   cd examples/C
#   make            # build both examples
#   make run-basic  # run with runtime library path set
#   make clean

ROOT       := ../..
BUILD_DIR  := $(ROOT)/target/release
INCLUDE    := $(ROOT)
CC        ?= cc
CFLAGS    ?= -O2 -Wall -Wextra
LDFLAGS    := -L$(BUILD_DIR) -lbat_c
UNAME_S    := $(shell uname -s)

# Pick the platform-specific shared library extension and run-path env var.
ifeq ($(UNAME_S),Darwin)
    LIB_EXT    := dylib
    RUN_ENV    := DYLD_LIBRARY_PATH=$(BUILD_DIR)
else
    LIB_EXT    := so
    RUN_ENV    := LD_LIBRARY_PATH=$(BUILD_DIR)
endif

LIB_TARGET := $(BUILD_DIR)/libbat_c.$(LIB_EXT)
TARGETS    := basic self_print

all: $(TARGETS)

$(LIB_TARGET):
	cargo build --manifest-path $(ROOT)/Cargo.toml --release

basic: $(LIB_TARGET) basic.c
	$(CC) $(CFLAGS) -I$(INCLUDE) basic.c -o $@ $(LDFLAGS) -Wl,-rpath,$(BUILD_DIR)

self_print: $(LIB_TARGET) self_print.c
	$(CC) $(CFLAGS) -I$(INCLUDE) self_print.c -o $@ $(LDFLAGS) -Wl,-rpath,$(BUILD_DIR)

run-basic: basic
	$(RUN_ENV) ./basic

run-self_print: self_print
	$(RUN_ENV) ./self_print

clean:
	rm -f $(TARGETS)

.PHONY: all run-basic run-self_print clean
