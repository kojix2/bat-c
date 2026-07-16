ROOT := ../..
BUILD_DIR := $(ROOT)/target/release
UNAME_S := $(shell uname -s)
ODIN := odin

ifeq ($(UNAME_S),Darwin)
    LIB_EXT := dylib
    RUN_ENV := DYLD_LIBRARY_PATH=$(BUILD_DIR)
    RPATH_FLAG := -Wl,-rpath,$(BUILD_DIR)
else
    LIB_EXT := so
    RUN_ENV := LD_LIBRARY_PATH=$(BUILD_DIR)
    RPATH_FLAG := -Wl,-rpath=$(BUILD_DIR)
endif

LIB_TARGET := $(BUILD_DIR)/libbat_c.$(LIB_EXT)
ODIN_FLAGS := -extra-linker-flags:"-L$(BUILD_DIR) $(RPATH_FLAG)"

all: basic self_print

$(LIB_TARGET):
	cargo build --manifest-path $(ROOT)/Cargo.toml --release

basic: $(LIB_TARGET) basic.odin
	$(ODIN) build basic.odin -file -out:basic $(ODIN_FLAGS)

self_print: $(LIB_TARGET) self_print.odin
	$(ODIN) build self_print.odin -file -out:self_print $(ODIN_FLAGS)

run-basic: basic
	$(RUN_ENV) ./basic

run-self_print: self_print
	$(RUN_ENV) ./self_print

clean:
	rm -f basic self_print

.PHONY: all run-basic run-self_print clean
