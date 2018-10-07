# 9th Sep 2018 The test round
- Implemented the helper module create_test_data

# 1st Oct 2018 The first round
- Started implementing parse_csv command and the corresponding library functions

# 2nd Oct 2018 The second round
- Parsing CSV to KBT blocks seems to work now!
(But I'm not sure because we don't have tests or round-tripping yet.)

# 2018年10月3日 3回目 初めての日本語ストリーム！
- convert_to_csvというコマンドの実装をはじめました。
- なんとかkbtファイルを読んで一行のCSVを出せたという結果

## The third round - First time in Japanese!
- Started implementing convert_to_csv command
- Was somehow able to read a kbt file and write a single CSV row

# 7th Oct 2018 The fourth round
- Set some initial goals for the project (see goals.md file)
- Continued implementing convert_to_csv command
- Now able to output a whole block of Kobuta as CSV data
- We need to think about the input/output abstractions more
- The output is now capped only to the amount of data actually written
(before it would write out the whole fixed-size buffer)