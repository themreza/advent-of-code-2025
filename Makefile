# From https://stackoverflow.com/a/14061796
ifneq (,$(filter run profile,$(firstword $(MAKECMDGOALS))))
  # use the rest as arguments
  RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  # ...and turn them into do-nothing targets
  $(eval $(RUN_ARGS):;@:)
endif

run:
	@echo "Running debug"
	@cargo run --bin $(RUN_ARGS)
clean:
	@echo "Cleaning build dir"
	@rm -rf target/*
	@echo "Cleaning using cargo"
	@cargo clean
check:
	@echo "Checking"
	@cargo check
tests:
	@echo "Running tests"
	@cargo test
docs:
	@echo "Generating documentation"
	@cargo doc --open
fmt:
	@echo "Formatting code"
	@cargo fmt
lint:
	@echo "Linting code"
	@cargo clippy
fix:
	@echo "Linting and fixing code"
	@cargo clippy --fix
profile:
	@echo "Running performance profiling"
	@CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --bin $(RUN_ARGS)
