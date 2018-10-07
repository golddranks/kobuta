#!/bin/sh

cargo run --bin create_test_data -- --records 100000 --schema "Int32, Float32, Float32, Int32" > test_100k.csv
cargo run --bin parse_csv -- --in test_100k.csv --schema "Int32, Float32, Float32, Int32" --out "test_100k.kbt"
cargo run --bin convert_to_csv -- --in test_100k.kbt --schema "Int32, Float32, Float32, Int32" --out "test_100k_roundtripped.csv"
diff test_100k_roundtripped.csv test_100k.csv
