SHELL=/bin/bash

CURDIR=$(shell pwd)

RELEASE_TYPE?=release
ARTIFACTS_INSTALL_DIR:=$(CURDIR)/../artifacts/
CARGO_BUILD_FLAGS?=--no-default-features --features csl_static

WASM32_TARGET_DIR:=target/wasm32-unknown-unknown/${RELEASE_TYPE}/
LOCAL_TARGET_DIR:=target/${RELEASE_TYPE}/
RANDOM_EXAMPLES_ROOT:=$(CURDIR)/
RANDOM_EXAMPLES_WASM:=$(RANDOM_EXAMPLES_ROOT)/${WASM32_TARGET_DIR}/random_examples.wasm

ifeq ($(RELEASE_TYPE),release)
	override CARGO_BUILD_FLAGS += --release
endif

# The reason as to why `$(RANDOM_EXAMPLES_WASM)` phony targets are being used is: we need to force recompile stuff each time
# This will slow down compilation, but it's a necessary evil :(, we compile the same target again and again with different features
# A solution will be found soon
# using the rust feature of conditional compilation, requiring multiple compilations.
# THOUGH, it can be solved using workspaces???
.PHONY: run build bond $(RANDOM_EXAMPLES_WASM)

$(ARTIFACTS_INSTALL_DIR):
	@mkdir -p $@

$(RANDOM_EXAMPLES_WASM): $(RANDOM_EXAMPLES_ROOT)/src/* $(RANDOM_EXAMPLES_ROOT)/.cargo/ \
		$(RANDOM_EXAMPLES_ROOT)/Cargo.toml $(RANDOM_EXAMPLES_ROOT)/build.rs
	@echo "Building random_examples (${RELEASE_TYPE})"
	cargo build ${CARGO_BUILD_FLAGS}
	@echo "Building done"

build: $(RANDOM_EXAMPLES_WASM)

bond: $(ARTIFACTS_INSTALL_DIR)
	@echo "-----------Packaging random-examples-csl-static---------------"
	bond --app-config ${CURDIR}/conpkg_config_files/1_thread__csl_static.conifer.ron \
	--install-dir ${ARTIFACTS_INSTALL_DIR}
