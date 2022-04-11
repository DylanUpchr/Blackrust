prog :=dm

debug ?= true

$(info debug is $(debug))

ifdef debug
  release :=
  target :=debug
  extension :=debug
else
  release :=--release
  target :=release
  extension :=
endif

build:
	cd ./src/web; \
  npm install
	cd blackrust_lib; \
	cargo build $(release)
	cargo build $(release)

all: build
 
help:
	@echo "usage: make $(prog) [debug=1]"