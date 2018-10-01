#!/bin/sh

cargo run --bin create_test_data -- --records 100000 --schema "Int32, Float32, Float32, Int32" > test_100k.csv