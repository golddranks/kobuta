# 9th Sep 2018 The test round
- Implemented the helper module create_test_data

# 1st Oct 2018 The first round (hoc: 874)
- Started implementing parse_csv command and the corresponding library functions

# 2nd Oct 2018 The second round (hoc: 1023)
- Parsing CSV to KBT blocks seems to work now!
(But I'm not sure because we don't have tests or round-tripping yet.)

# 2018年10月3日 3回目 初めての日本語ストリーム！ (hoc: 1121)
- convert_to_csvというコマンドの実装をはじめました。
- なんとかkbtファイルを読んで一行のCSVを出せたという結果

## The third round - First time in Japanese!
- Started implementing convert_to_csv command
- Was somehow able to read a kbt file and write a single CSV row

# 7th Oct 2018 The fourth round (hoc: 1252)
- Set some initial goals for the project (see goals.md file)
- Continued implementing convert_to_csv command
- Now able to output a whole block of Kobuta as CSV data
- We need to think about the input/output abstractions more
- The output is now capped only to the amount of data actually written
(before it would write out the whole fixed-size buffer)

# 2018年10月7日 5回目 (hoc: 1490)
- 何ブロックでもKBTからCSVに変換できるようになった！
- そのおかげでCSVからKBTに変換している局面で最後のブロックに
むだな0.0があるのが明らかになっている
    - ここはチャンクレベルのメタデータがあればなおせる

## The fifth round
- Now able to convert multi-block KBT to CSV
- Because of that, it became clear that the CSV -> KBT
  conversion leaves the final block partially full
  of zeros
    - We can fix that once we have support for chunk-level metadata

# 8th Oct 2018 Refactoring (no stream; hoc: 2287)
- Cargo clean
- A huge refactoring & code cleanup
- Module split-up
- Thinking about APIs and abstractions
- Introduced a trait for parsing/printing values for different external formats

# 8th Oct 2018 Refactoring (no stream; hoc: 2381)
- Updated TODO to reflect the goals and prioritise the items
- Refactored the binary commands