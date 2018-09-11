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
- Must be suitable as an in-memory format
- Must perform well (read speed/modify speed/memory footprint)
- Must support streaming access patterns
- Must support random access patterns
- Must be designed as modern hardware in mind (cache layers, SIMD)
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
in order to achieve its other goals.

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

### Glossary
**Record** is a like a horizontal row in a CSV files
or like a single flat JSON object.
It consists of fields of data,
each of which belong to a column.

**Column** is like a vertical column in a CSV file.
Column has a name and a data type.

**Schema** is piece of metadata that specifies the column names and datatypes.

**File** is the largest unit of Kobuta format.
It doesn't need to be an actual file on the disk –
it might also be an in-memory representation or a stream.
It consists of three sections:
metadata section that contains the schema and some other metadata.
After that, there is the chunk index section that contains an index of chunks.
Finally there is the data section that consists of chunks.

**Chunk** is a big unit of data.
It may contain variable number of records.
A typical chunk size is around one megabyte.
Chunks may be individually compressed.
A chunk starts with a block index section,
and after that, a block data section.

**Block** is a small unit of data.
It's designed to fit in L1 cache of modern processors.
It contains always 64 records,
except when it's the last block of a chunk when it may contain less.
It's always aligned to a 64-byte boundary.
A record data inside a block is organized in an sized section and unsized section.
The sized section contains all the data that has statically know sizes,
such as integers, floats, bools etc.
The unsized section contains all the data that is variable-sized,
such as strings and bytestrings.
The sized section contains offsets of the unsized data fields.

**Stripe** is how sized data is organized inside blocks.
A stripe is a continuous memory slice of 64 values of the same type,
beloning to the same column.
The sized section of a block therefore contains a stripe
that has the data of the first column of the 64 records stored in that block,
followed by a stripe that has the data of the next column of the 64 records
and so on.

### About metadata

A Kobuta file starts with metadata.
The start of the metadata consists of an ASCII magic string that
provides human-readable info that this file is in Kobuta format.
Following that is a UTF-8 comment section
that contains an user-definable file comment comment,
the amount of records in the file
and a human-readable version of the schema.

This design allows for peeking the file
with the `head` command or a text editor.

After the UTF-8 section,
there is a binary section that contains the actual metadata.


### About schema

Kobuta is strictly typed
and carries its schema in its metadata section.
Basically, each column has a name and a type.
The types have well-defined binary representation.
Little-endian is used thoroughly.
Each type is aligned to it's size.
Here a byte means an octet.
The basic types are:

**Float128, Float64, Float32, Float16** are
IEEE 754 floating point formats of type
binary128, binary64, binary32 and binary16, respectively.

**Int128, Int64, Int32, Int16, Int8** are
two's complement integers.

**Bool** is identical to Int8
but with the invariant that that it may only have values
0 (False) or 1 (True).

**Text64, Text32 and Text16** are
Int64, Int32 and Int16-backed
offsets to the unsized section of a block
that contains all the text data.
All text must be encoded in valid UTF-8.
Each of the types have limitations how long a text field they can represent.
(262144 TiB, 64 MiB, 1024 bytes, respectively.)
Each text field in the unsized section must end with a zero byte.

**Bin64, Bin32 and Bin16** are
similar to the Text types,
but they are not required to be valid UTF-8.
They can be an arbitrary stream of octets.

**TODO:** Inline (sized) binary types?

**TODO:** Defining categories/enums?

**TODO:** About sparse representations?

**TODO:** Unums?

Any column can be set to be **Nullable**,
although this isn't the default.
In case of nullable data,
a separate metadata is kept
whether a field is null or not. (**TODO:** Representation? A bit buffer?)

## Live stream

I will stream the development of this project on Twitch:
https://www.twitch.tv/golddranks

I have no predefined schedule at the moment,
but I'm trying to stream twice a week at least;
one day in English and one day in Japanese.

If you want to get streaming announcements
(I'll try to post them a few hours in advance),
follow me in Twitter:
https://twitter.com/GolDDranks

## Goals / TODO list

### Kickstarting the project
- <s>Prepare sample data in CSV</s>
- <s>Prepare a project skeleton</s>
    - <s>error handling</s>
    - <s>logging</s>
    - <s>testing</s>
    - simple command line interface
- Prepare a rough overview of the file format
    - <s>glossary</s>
    - about schema
- Take a 20-minute overview video
where I explain the project motivation
and the file format overview
    - In English
    - In Japanese

### Getting started with the library
- Implement parsing CSV and storing in-memory block using the schema
- Implement writing Float32 and Int32 to file
- Implement writing & reading metadata
	- Reading the magic string
	- Reading the binary start
	- Parsing the binary
	- Validating
	- CRC32 
	- Implement printing metadata
- Implement strings
- Implement block index
- Do some performance testing

### Advancing the library

- Implement conversion to CSV
- Implement conversion to JSON
- Implement nullable types
- Implement chunk index
- Test compression formats for compressed chunks


### Something fun & ambitious for the future
- A JIT for optimizing data serialization (CSV/JSON)
- Implement plugins for MySQL, Postgres and SQLite that
import and export Kobuta directly.
- Implement an interface for kernels that map/filter/fold
over the data
- Implement wrappers for C, C++, Java, Go, Python,
JavaScript, Ruby, R, Julia
- Write a full spec