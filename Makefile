APP=casewm

build:
	@echo "Building $(APP) in Release mode"
	@cargo build --release

run-embeded:
	@cargo build
	@./scripts/xephyr.sh

.PHONY: build run-embeded
