# Lint everything.
export def lint [] {
	# XXX: move this to [lints] in Cargo.toml when Rust 1.74 lands
	(
		cargo clippy --workspace
		--
		# Panics
		-W clippy::unwrap_used
		-W clippy::expect_used
		-W clippy::arithmetic_side_effects
		-W clippy::cast_abs_to_unsigned
		-W clippy::cast_possible_truncation
		-W clippy::fallible_impl_from
		-W clippy::get_last_with_len
		-W clippy::index_refutable_slice
		-W clippy::indexing_slicing
		-W clippy::match_on_vec_items
		-W clippy::missing_panics_doc
		-W clippy::modulo_one
		-W clippy::panic
	)
}

# Format all of the project in-place.
export def format [
	--check	# Instead of formatting, run a check.
] {
	if $check {
		cargo fmt --all -- --check
	} else {
		cargo fmt --all
	}
}

# Build the entire project, using at most 5 jobs.
export def build [] {
	cargo build --workspace --jobs 5
}

# Run all tests.
export def test [] {
	cargo test --workspace --no-fail-fast -- --test-threads 5
}

# Run all checks, including formatting, linting, and tests.
export def check [] {
	format --check
	lint
	cargo check --workspace
	test
}
