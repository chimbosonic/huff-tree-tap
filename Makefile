watch:
	cargo watch -x fmt -x check -x clippy -x test

flamegraph:
	cargo flamegraph --profile flamegraph --test tests

bench:
	cargo bench