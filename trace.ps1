# Set ENV Variables
$env:RUST_LOG = "trace";

# Run Program
cargo run

# Reset Env Variables
if ($env:RUST_LOG -ne $null) {
    Remove-Item -Path "Env:RUST_LOG"
}