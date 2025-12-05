ROOT := ../..
BUILD_DIR := $(ROOT)/target/release
UNAME_S := $(shell uname -s)
DC := dmd

ifeq ($(UNAME_S),Darwin)
    LIB_EXT := dylib
    RUN_ENV := DYLD_LIBRARY_PATH=$(BUILD_DIR)
    LDFLAGS := -L-L$(BUILD_DIR) -L-lbat_c -L-rpath -L$(BUILD_DIR)
else
    LIB_EXT := so
    RUN_ENV := LD_LIBRARY_PATH=$(BUILD_DIR)
    LDFLAGS := -L-L$(BUILD_DIR) -L-lbat_c -L-rpath=$(BUILD_DIR)
endif

LIB_TARGET := $(BUILD_DIR)/libbat_c.$(LIB_EXT)

all: basic self_print

$(LIB_TARGET):
	cargo build --manifest-path $(ROOT)/Cargo.toml --release

basic: $(LIB_TARGET) basic.d
	$(DC) basic.d -of=basic $(LDFLAGS)

self_print: $(LIB_TARGET) self_print.d
	$(DC) self_print.d -of=self_print $(LDFLAGS)

run-basic: basic
	$(RUN_ENV) ./basic

run-self_print: self_print
	$(RUN_ENV) ./self_print

clean:
	rm -f basic self_print basic.o self_print.o

.PHONY: all run-basic run-self_print clean
