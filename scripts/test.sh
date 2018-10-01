#!/bin/sh

cargo run --bin parse_csv -- --file test_100k.csv --schema "Int32, Float32, Float32, Int32"