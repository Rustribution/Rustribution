current_dir := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

test:
	cargo test --all -- --nocapture --color always
