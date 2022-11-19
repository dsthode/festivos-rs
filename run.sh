docker run --rm -it --user "$(id -u):$(id -g)" -p 8000:8000 -v $(pwd):/app -e CARGO_HOME=/app/.cargo -e RUST_BACKTRACE=1 -w /app rust-dev $*
