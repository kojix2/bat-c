ROOT := ../..
BUILD_DIR := $(ROOT)/target/release
UNAME_S := $(shell uname -s)

ifeq ($(UNAME_S),Darwin)
    LIB_EXT := dylib
    RUN_ENV := DYLD_LIBRARY_PATH=$(BUILD_DIR)
else
    LIB_EXT := so
    RUN_ENV := LD_LIBRARY_PATH=$(BUILD_DIR)
endif

LIB_TARGET := $(BUILD_DIR)/libbat_c.$(LIB_EXT)

all: basic self_print

$(LIB_TARGET):
	cargo build --manifest-path $(ROOT)/Cargo.toml --release

basic: $(LIB_TARGET) basic.go
	CGO_LDFLAGS="-Wl,-rpath,$(BUILD_DIR)" go build -o basic basic.go

self_print: $(LIB_TARGET) self_print.go
	CGO_LDFLAGS="-Wl,-rpath,$(BUILD_DIR)" go build -o self_print self_print.go

run-basic: basic
	$(RUN_ENV) ./basic

run-self_print: self_print
	$(RUN_ENV) ./self_print

static-basic: $(BUILD_DIR)/libbat_c.a basic.go
	CGO_LDFLAGS="-L$(BUILD_DIR) -lbat_c" go build -o basic-static basic.go

static-self_print: $(BUILD_DIR)/libbat_c.a self_print.go
	CGO_LDFLAGS="-L$(BUILD_DIR) -lbat_c" go build -o self_print-static self_print.go

clean:
	rm -f basic self_print basic-static self_print-static

.PHONY: all run-basic run-self_print static-basic static-self_print clean
