BIN := justfile_dir() + "/bin"
EXT := if OS == "linux" { "so" } else if OS == "darwin" { "dylib" } else { "dll" }

build:
    cd rust && cargo build
    mkdir -p {{BIN}}
    cp rust/target/debug/libosr-client.{{EXT}} {{BIN}}/libosr-client.{{EXT}}

run: build
    godot --path . --verbose

clean:
    cd rust && cargo clean
    rm -rf {{BIN}}
