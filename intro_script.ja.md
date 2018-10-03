# 説明・自己紹介動画の脚本

こんばんは！コンはKobutaを書くを見に来てくれてありがとうございます！

僕の名前はコンティオ・プル、略してコンです。
これは私が私がライブで趣味のプログラミングを配信している
ライブプログラミングストリームです。何を作っているか
というと、Kobutaというテーブル型のデータ用のファイル形式を
開発していきます。

今話しているのは自己紹介的な部分で、これからこのストリームの
基本とKobuta形式についてを紹介したいと思います。

まず、自己紹介からいくと、私は東京に住んでる
プログラマーで、もともとフィンランドから来ました。
日本に来て一年半ぐらい立つところですかね。

仕事でも趣味でもプログラミングすることが好きだけど、
最近はなにか新しいことをやってみたくて、
ストリーム配信をやってみようかなと思いました。
結構ワクワクしながらも緊張してますが、
なんとかうまくいきますように！

このストリームを通ってなんか自分のモチベーションや
集中力をブーストできるんじゃないかと思うのも一つです。
他人に見ているとそういう効果がもしかしてあるかなと思いました。

I very much enjoy programming in Rust,
so that's going to be my main language in this stream too.

I'm trying to stream at least once a week in English
and once a week in Japanese, but I've got no
hard schedule. If you want to get notified
some hours before I start, follow me on Twitter!

This introduction is scripted, but normally I'm
planning to just hack away with the code,
verbalizing my thoughts and explaining my
thinking process while coding.
And hopefully interacting with you too!


So, that being said; about Kobuta. What's the thing I'm making here!?

Kobuta is a new file format, or a data format for
tabular data. Think of something like CSV,
comma separated values, or Excel files.
So there's going to be rows and columns.

Kobuta is meant for storing, exchanging and processing data.

The reason why I started developing a new format is
because I'm fed up with the major formats I use at work.
Those formats are CSV, JSON and Excel.

Unlike CSV and JSON, Kobuta is a binary format.
That means that you can't edit it with a text editor.
That's a bummer, but I think of it as a sacrifice I have to do
to enable many other things that CSV and JSON are incapable of.

To make up for it, I'm meaning it to be really easy and fast
to convert back and forth with existing formats.

The main features of Kobuta is that it's meant to be
very performant to read, write and edit.
It's designed modern hardware in mind.

Additionally, it's strongly typed,
and it carries it's own schema,
which means that you set the type of the each column of the data:
this is an integer,
this is a floating point value, a text string,
a boolean truth value and so on.
I think that's sorely missing
when moving data between databases using CSV.

You can read about the specifics of the format on the readme page
of the Github project of Kobuta. Be sure to check that out!

So, there we go. I hope you join me in developing
Kobuta - a new data format for tabular data.