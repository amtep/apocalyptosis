flamegraph:
	RUSTFLAGS='-Cforce-frame-pointers=y -Clink-arg=-Wl,--no-rosegment' \
	CARGO_PROFILE_RELEASE_DEBUG=true \
	cargo flamegraph
