# New chunk design

After refactoring the codebase a bit and giving quite a lot of thought about the design itself,
I've come to think that the original design is suboptimal.

It de-emphasises the streaming use case. Streaming is very, very important.

Besides of actually sending data over a stream, it's how we convert other file formats to kobuta.
It's how we convert kobuta to other file formats. We don't want to "jump around" while writing or reading.

Thinking about relationships between data and meta-data lead me to an important realization.

The meta-data should, generally be placed AFTER the data.

Schema is different. It should come first. Because data depends on it. But index-like metadata,
data _about_ data, depends on the data itself. It should be after.

After having written 50 gigabytes of data, we don't want to jump back, write down the metadata and then
see that we're out of space and we must move those 50 gigabytes. Sure, it helps the reader to have the metadata first.
However, in the random-access setting it doesn't make difference. We can read metadata from the end of the file or from the start.

In streaming setting it absolutely makes a difference. It _allows_ streaming in the first place.

The new design in nutshell is:

File:

```
SCHEMA DATA + SIZED INDEX DATA
CHUNKS
INDEX
```

Chunk:

```
SIZED HEADER
COMPRESSED BLOCKS
INDEX
```
This allows us not to "tweak" the sizes of metadata after writing it.
It still allows editing sized metadata (for example, to speed up access to the index etc.)
But it doesn't mandate it.

# Compression

Another thing I've been thinking about is compression.

I thought a single chunk as a single compressed unit. The problem with this is that
a compressed file is a stream. That hinders random access; we must decompress all the data that
preceeds the data we actually care about.
Additionally, that forces us to decompress a lot of data we won't need,
if we need only access a few columns.

A new idea is that we should try per-column compression. The compressed "streams" would be striped.
This allows us to skip de- and re-compressing columns we don't care about, plus keeping the
columns in smaller size in case we need to relocate them.

I still need to test these ideas.
