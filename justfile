BIN := justfile_dir() + "/bin"
EXT := if OS == "linux" { "so" } else if OS == "darwin" { "dylib" } else { "dll" }

# Build the Rust library and copy to bin/
build:
    cd rust && cargo build
    mkdir -p {{BIN}}
    cp rust/target/debug/libosr_client.{{EXT}} {{BIN}}/libosr_client.{{EXT}}

# Run Godot with the project
run: build
    godot --path . --verbose

# Run all Rust tests
test:
    cd rust && cargo test

# Run E2E auth test (Godot subprocess)
test-e2e-auth: build
    OSR_TEST_USER=testuser OSR_TEST_PASS=testpass godot --path . --verbose --editor --quit --script scripts/e2e_auth.gd

# Run containerized networking tests with tc netem
test-networking-auth:
    docker-compose up --abort-on-container-exit
    docker-compose down

# Clean build artifacts
clean:
    cd rust && cargo clean
    rm -rf {{BIN}}
