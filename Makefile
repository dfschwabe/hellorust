CARGO_LOCATION=$(shell which cargo | grep .asdf || echo node-not-found)

.PHONY :test build 

${CARGO_LOCATION}:
	asdf install
	touch `which cargo`

test: ${CARGO_LOCATION}
	cargo test

build: ${CARGO_LOCATION}
	cargo build