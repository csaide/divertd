# (c) Copyright 2020 Christian Saide <supernomad>
# SPDX-License-Identifier: GPL-3.0-only

.PHONY: dev setup-dev debug release docker fmt lint check clean

BUILD_TYPE := debug

dev: fmt check $(BUILD_TYPE)

setup-dev:
	@bash ../dist/bin/print.sh "SETTING UP LOCAL DEV"
	@rustup toolchain install nightly
	@rustup target add x86_64-unknown-linux-musl
	@rustup target add x86_64-unknown-linux-musl --toolchain nightly
	@rustup component add rustfmt --toolchain nightly

debug:
	@bash ../dist/bin/print.sh "BUILDING DEBUG"
	@cargo build --target x86_64-unknown-linux-musl

release:
	@bash ../dist/bin/print.sh "BUILDING RELEASE"
	@cargo build --target x86_64-unknown-linux-musl --release

docker-%:
	@bash ../dist/bin/print.sh "BUILDING DOCKER"
	@DOCKER_BUILDKIT=1 docker build \
		--build-arg BUILD=$(BUILD_TYPE) \
		--build-arg APP=$* \
		-t csaide/$*:$(BUILD_TYPE) \
		-f ../dist/docker/Dockerfile.rust .

docker: $(BUILD_TYPE) docker-divertd

fmt:
	@bash ../dist/bin/print.sh "FORMATTING CODE"
	@cargo +nightly fmt --all -- --emit=files

lint:
	@bash ../dist/bin/print.sh "LINTING"
	@cargo +nightly fmt --all -- --check

check:
	@bash ../dist/bin/print.sh "TESTING"
	@cargo test --all

clean:
	@bash ../dist/bin/print.sh "CLEANING"
	@rm -rf target/debug target/release target/x86_64-unknown-linux-musl
