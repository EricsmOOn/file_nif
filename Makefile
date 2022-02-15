all: clean compile

compile: 
	cargo build --release
	mkdir lib
	/bin/cp ./target/release/libfile_nif.so ./lib/file_nif.so
	erlc -o lib src/*.erl

clean:
	cargo clean
	/bin/rm -r lib
