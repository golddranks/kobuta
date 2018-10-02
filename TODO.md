- We should think about the public API
- Pass a Reader to parse_csv instead of a slice
- Return a proper error from parse_csv instead of Box<Error>
- Define a proper error type from schema::parse_data
- What to do with CSV headers in parse_csv?
- Why do we need to validate that the input CSV is UTF-8
if we could just parse an int directly from "ascii-like bytes"?
Maybe this needs to be made into an optimised library.
- Code organisation!
- Decide where to use u32 and where to use u16
- Add size checks