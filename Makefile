.PHONY: all build doc interactive tests clean
all: build interactive tests doc
build:
	mkdir -p target
	cd target && rustc ../src/lib.rs -L .
tests:
	rustc src/bin/tests.rs -L target -o target/tests
interactive:
	rustc src/bin/interactive.rs -L target -o target/interactive
doc:
	rustdoc src/lib.rs -o doc
clean:
	rm -rf target/*