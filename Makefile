SHELL=/bin/bash

CURDIR=$(shell pwd)

RELEASE_TYPE?=debug
INSTALL_DIR?=../artifacts/
CARGO_BUILD_FLAGS?=

WASM32_TARGET_DIR:=target/wasm32-unknown-unknown/${RELEASE_TYPE}/
LOCAL_TARGET_DIR:=target/${RELEASE_TYPE}/
RANDOM_EXAMPLES_ROOT:=$(CURDIR)/
RANDOM_EXAMPLES_WASM:=$(RANDOM_EXAMPLES_ROOT)/${WASM32_TARGET_DIR}/random_examples.wasm

ifeq ($(RELEASE_TYPE),release)
	CARGO_BUILD_FLAGS+=--release
endif

.PHONY: run build install

$(INSTALL_DIR):
	@mkdir -p $@

$(RANDOM_EXAMPLES_WASM): $(RANDOM_EXAMPLES_ROOT)/src/* $(RANDOM_EXAMPLES_ROOT)/.cargo/ $(RANDOM_EXAMPLES_ROOT)/Cargo.toml \
$(RANDOM_EXAMPLES_ROOT)/try-csl/src/* $(RANDOM_EXAMPLES_ROOT)/try-csl/.cargo/ $(RANDOM_EXAMPLES_ROOT)/try-csl/Cargo.toml
	@echo "Building random_examples (${RELEASE_TYPE})"
	@cargo build ${CARGO_BUILD_FLAGS} \
	&& echo "Building done" \
    || echo "Error building random_examples"

build: $(RANDOM_EXAMPLES_WASM)

run: $(RANDOM_EXAMPLES_WASM)
	@#cargo run \
	#&& 	echo "Running done" \
	#|| echo "Error running random_examples"
	@echo "You can't run this program as it generates a cdylib"

install: $(RANDOM_EXAMPLES_WASM) $(INSTALL_DIR)
	@echo "Installing random_examples in ${INSTALL_DIR}"
	@ln -f $< "${INSTALL_DIR}/"
