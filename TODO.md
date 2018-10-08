Now
- Decide where to use u32 and where to use u16
- Code organisation!

For 0.1
- chunk level metadata
- codebase cleaning
- block/chunk size calculation module
- Define a proper error types
- check TODOs and unimplemented's in the code

For 0.2?
- 標準ライブラリのフォーマット機能とitoaやdtoaの
フォーマット機能をベンチマークする
- Why do we need to validate that the input CSV is UTF-8
if we could just parse an int directly from "ascii-like bytes"?
Maybe this needs to be made into an optimised library.
- CSV header inference

For 0.3?
- We should think about the public API