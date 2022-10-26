SHELL=/bin/bash

CURDIR=$(shell pwd)

RELEASE_TYPE?=debug
INSTALL_DIR?=../artifacts/
CARGO_BUILD_FLAGS?=

WASM32_TARGET_DIR:=target/wasm32-unknown-unknown/${RELEASE_TYPE}/
LOCAL_TARGET_DIR:=target/${RELEASE_TYPE}/
RANDOM_EXAMPLES_ROOT:=$(CURDIR)/
RANDOM_EXAMPLES_WASM:=$(RANDOM_EXAMPLES_ROOT)/${WASM32_TARGET_DIR}/random_examples.wasm
TRY_CSL_ROOT:=$(CURDIR)/try-csl/
TRY_CSL_WASM:=$(TRY_CSL_ROOT)/${WASM32_TARGET_DIR}/try_csl.wasm

ifeq ($(RELEASE_TYPE),release)
	CARGO_BUILD_FLAGS+=--release
endif

.PHONY: run build install

$(INSTALL_DIR):
	@mkdir -p $@

$(TRY_CSL_WASM): $(TRY_CSL_ROOT)/src/* $(TRY_CSL_ROOT)/.cargo/ $(TRY_CSL_ROOT)/Cargo.toml
	@echo "Building try-csl (${RELEASE_TYPE})"
	@cd try-csl \
	&& cargo build ${CARGO_BUILD_FLAGS} \
	&& echo "Building done" \
    || echo "Error building try-csl"

$(RANDOM_EXAMPLES_WASM): $(TRY_CSL_WASM) $(RANDOM_EXAMPLES_ROOT)/src/* $(RANDOM_EXAMPLES_ROOT)/.cargo/ \
		$(RANDOM_EXAMPLES_ROOT)/Cargo.toml $(RANDOM_EXAMPLES_ROOT)/build.rs
	@echo "Building try-csl (${RELEASE_TYPE})"
	@cd try-csl \
	&& cargo build ${CARGO_BUILD_FLAGS} \
	&& echo "Building done" \
    || echo "Error building try-csl"
	@echo "Building random_examples (${RELEASE_TYPE})"
	@cargo build ${CARGO_BUILD_FLAGS} \
	&& echo "Building done" \
    || echo "Error building random_examples"

build: $(RANDOM_EXAMPLES_WASM)

install: $(TRY_CSL_WASM) $(RANDOM_EXAMPLES_WASM) $(INSTALL_DIR)
	@echo "Installing random_examples in ${INSTALL_DIR}"
	@ln -f $(TRY_CSL_WASM) "${INSTALL_DIR}/"
	@ln -f $(RANDOM_EXAMPLES_WASM) "${INSTALL_DIR}/"
