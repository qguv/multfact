CARGO = cargo

build:
	@$(CARGO) build

build_release: doc
	@$(CARGO) build --release

doc:
	@$(CARGO) doc

doc_release:
	@$(CARGO) doc --release

release: build_release doc_release

check: build test

run:
	@$(CARGO) run

test:
	@$(CARGO) test

bench:
	@$(CARGO) bench

clean:
	@$(CARGO) clean

.PHONY: build build_release doc doc_release release check run test bench clean
