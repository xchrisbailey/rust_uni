program_name :=taskie

debug ?=

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
	cargo build $(release)

install:
	cp target/$(target)/$(program_name) ~/bin/$(program_name)

all: test build install

test:
	cargo test
 
help:
	@echo "usage: make $(program_name) [debug=1]"
