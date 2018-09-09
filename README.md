[![Build Status](https://travis-ci.org/golddranks/kobuta.svg?branch=master)](https://travis-ci.org/golddranks/kobuta)

**Note:** I am live streaming the development of this project!
Check out the [details below](#live-stream).

# Kobuta
## Summary

A binary data serialization format for tabular data
that is fast for common use cases,
strongly-typed and (reasonably) simple.
It is meant to be easy and fast to convert from and to
other data formats such as CSV and JSON and fast to
map/filter/fold over as an in-memory format.

This repo will provide a Rust-based library with a C API
that provides support for all kinds of operations for Kobuta format.

## Motivation

The foremost motivation is: because I want to do it.
Developing this thing is my hobby.
I also want to try streaming the development process
as live-coding sessions. Never tried that before,
so I'm quite excited about that.

However, there are very valid questions about why develop a new
data format. Here's the features I'm after. I am not aware of any
data format that provides this combination:

- Must be especially suitable for tabular data
- Must be a suitable format for storing, exchanging and processing data
- Must be suitable for in-memory format
- Must be high-performance (read speed/modify speed/memory footprint)
- Must be accessible in a streaming way
- Must be accessible in random-access way
- Must be designed in modern hardware in mind (cache layers, SIMD)
- Must be relatively simple
- Must be strongly typed
- Must carry the schema with the data
- Must be easily convertible from/to popular data formats

### Why not CSV/JSON/XML/Excel/ProtoBuf/FlatBuffers/Cap'n Proto/MessagePack?

The most common data format
used for the purpose I'm imagining this format is CSV.
However, CSV doesn't fulfill even half of the above requirements.
Another popular format is JSON,
although it isn't actually meant for tabular data.
Both are text formats,
which means that they are easily editable in text editor.
Kobuta is not, and that is the main thing it gives away
in order to achieve it's other goals.

There are also multiple existing binary formats such as
ProtoBuf, Cap'n Proto and FlatBuffers.
However, they are meant for exchanging messages
with a pre-defined, separate schema.
Additionally, they aren't optimized for tabular data.

### The name?

Kon's Binary Tables → KoBuTa
(the u is an artefact of me wanting a name
that means something in Japanese.
子豚, kobuta, means a piglet. Cute, huh?)
I'm imagining the file extension to be `something.kbt`

## Overview

TODO: This section will contain a rough description of the file format

## Live stream

TODO: This section will contain links and details of the development live stream

## Goals / TODO list

### Kickstarting the project
- Prepares sample data in CSV
- Prepare a project skeleton (error handling, logging, testing, simple command line interface)
- Prepare a rough overview of the file format
- Take a 20-minute overview video where I explain the project motivation and the file format overview
    - In English
    - In Japanese (?)

### Getting started with the library
- Implement parsing CSV using the schema
- Implement writing Float32 and Int32 to file
- Implement writing & reading metadata
	- Reading the magic string
	- Reading the binary start
	- Parsing the binary
	- Validating
	- CRC32 
	- Implement printing metadata
- Implement strings
- TODO

### Advancing the library

- TODO

### Something fun & ambitious for the future
- A JIT for optimizing data serialization (CSV/JSON)
- Implement a plugin for MySQL and/or Postgres that
reads and writes Kobuta directly.
- Implement an interface for kernels that map/filter/fold
over the data

## Specs / Documentation

TODO