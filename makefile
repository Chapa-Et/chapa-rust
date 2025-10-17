# ======================================================================
# Chapa-rust : Project Automation Makefile
# Use 'make <target>' to run common development tasks.
# ? For future maybe we can consider using `just` instead of makefile
# ======================================================================

CLIPPY_CMD := clippy --all-targets --all-features -- -D warnings
FMT_CMD := fmt --all

.PHONY: all fmt lint test check clean

# Default target: runs the comprehensive check
all: check

#* CODE QUALITY TARGETS

# Formats all Rust code files in the project and checks for discrepancies
fmt:
	@echo "Running rustfmt across all files"
	cargo $(FMT_CMD)

# Runs the linter (Clippy) and enforces a strict, zero-warning policy
lint:
	@echo "Running cargo clippy with zero-warning policy..."
	cargo $(CLIPPY_CMD)

# Runs all tests (unit, integration, and doc tests, including doc-comment examples)
test:
	@echo "Running all tests..."
	cargo test --all-targets --all-features

# Comprehensive check: runs linting and testing
check: lint test
	@echo "✅ All code quality checks passed."

#* UTILITY TARGETS

# Cleans build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean