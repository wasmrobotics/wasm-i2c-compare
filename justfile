# Use `just work day-01 part1` to work on the specific binary for a specific day's problems
work day part:
    cargo watch -w {{day}} -x "check -p {{day}}" -s "just test {{day}} {{part}}" -s "just lint {{day}}" -s "just bench {{day}} {{part}}" 
lint day:
    cargo clippy -p {{day}}
test day part:
    cargo nextest run -p {{day}} {{part}}
bench-all:
    cargo bench -q > benchmarks.txt
bench package name:
    cargo bench --package {{package}} --bench  {{name}} >> {{package}}.bench.txt
flamegraph package name:
    cargo flamegraph --profile flamegraph --root --package {{package}} --bin {{name}} --features flamegraph -o flamegraphs/{{package}}--{{name}}.svg
dhat package name:
    cargo run --profile dhat --features dhat-heap --example {{name}}
dhat-build package name:
    cargo build --profile dhat --features dhat-heap --example {{name}}

