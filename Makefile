NAME=$(shell grep name Cargo.toml | head -n1 | cut -d' ' -f 3 | sed 's/"\(.*\)"/\1/')
CARGO_LOCATION=$(shell which cargo | grep .asdf || echo cargo-not-found)
DOCKER_LOCATION=$(shell which docker-compose || echo docker-not-found)
GVERSION=$(shell git describe --match "[0-9]*\.[0-9]*\.[0-9]*")

.PHONY :test build

${CARGO_LOCATION}:
	asdf install
	touch `which cargo`

test: ${CARGO_LOCATION}
	cargo test

build: ${DOCKER_LOCATION}
	docker build -t ${NAME} . --build-arg APP_VSN=${GVERSION}

up: ${DOCKER_LOCATION} build
	docker-compose -f ./test/docker-compose.yml up -d --remove-\orphans --build --scale test=0

down: ${DOCKER_LOCATION}
	docker-compose -f ./test/docker-compose.yml down
