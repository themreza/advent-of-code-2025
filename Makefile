run:
	@echo "Running debug"
	@cargo run
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