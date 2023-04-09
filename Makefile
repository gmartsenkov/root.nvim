
.PHONY: build
build:
	cargo build --release
	rm -f ./lua/root_backend.so
	[[ -e ./target/release/libroot.dylib ]] && cp ./target/release/libroot.dylib ./lua/root_backend.so || cp ./target/release/libroot.so ./lua/root_backend.so
	# if your Rust project has dependencies,
	# you'll need to do this as well
	mkdir -p ./lua/deps/
	cp ./target/release/deps/*.rlib ./lua/deps/
