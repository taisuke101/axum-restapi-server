watch-server:
	cargo watch -q -c -w src/ -w .cargo/ -x run

watch-client:
	cargo watch -q -c -w examples/ -x "run --example quick_dev"