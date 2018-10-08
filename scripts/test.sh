#!/bin/sh

cargo run --bin create_test_data -- --records 100000 --schema "Int32, Float32, Float32, Int32" > "test_100k.csv"
cargo run --bin kobuta -- convert --in "test_100k.csv" --schema "Int32, Float32, Float32, Int32" --out "test_100k.kbt" --has_headers false
cargo run --bin kobuta -- convert --in "test_100k.kbt" --schema "Int32, Float32, Float32, Int32" --out "test_100k_roundtripped.csv"
diff "test_100k_roundtripped.csv" "test_100k.csv"