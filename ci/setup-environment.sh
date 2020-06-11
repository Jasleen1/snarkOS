export CIRCLE_WORKING_DIRECTORY=/home/circleci/project
export SCCACHE_CACHE_SIZE=200M
export SCCACHE_DIR="$CIRCLE_WORKING_DIRECTORY/.cache/sccache"
export PATH="$PATH:$CIRCLE_WORKING_DIRECTORY/.bin"
export RUSTC_WRAPPER="sccache"
export RUST_MIN_STACK=8388608
export RUSTFLAGS="-Clink-dead-code"