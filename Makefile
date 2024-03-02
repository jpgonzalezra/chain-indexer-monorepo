.PHONY: doc test fmt upgrade

doc:
	@cargo doc --no-deps --all-features

test:
	@cargo test --release

fmt:
	@cargo fmt --all

upgrade:
	@cargo upgrade --workspace