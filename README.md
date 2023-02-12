Compressing the [canterbury corpus](https://corpus.canterbury.ac.nz/) files
with [brotli](https://www.rfc-editor.org/rfc/rfc7932) compression in Rust using
the [rust-brotli](https://github.com/dropbox/rust-brotli) library.

```
$ cargo run -- --directory canterbury-corpus/
Compressed canterbury-corpus/calgary/paper6 by 70.78% in 454ms
Compressed canterbury-corpus/calgary/paper1 by 70.92% in 613ms
Compressed canterbury-corpus/calgary/obj1 by 56.56% in 265ms
Compressed canterbury-corpus/calgary/geo by 48.15% in 1977ms
Compressed canterbury-corpus/calgary/progc by 70.67% in 420ms
Compressed canterbury-corpus/calgary/progl by 80.45% in 1012ms
Compressed canterbury-corpus/calgary/book1 by 66.65% in 10468ms
Compressed canterbury-corpus/calgary/progp by 79.99% in 656ms
Compressed canterbury-corpus/calgary/pic by 92.04% in 14593ms
Compressed canterbury-corpus/calgary/news by 70.03% in 4979ms
Compressed canterbury-corpus/calgary/paper4 by 67.76% in 177ms
Compressed canterbury-corpus/calgary/bib by 74.51% in 1145ms
Compressed canterbury-corpus/calgary/paper3 by 68.54% in 525ms
Compressed canterbury-corpus/calgary/paper2 by 69.77% in 941ms
Compressed canterbury-corpus/calgary/paper5 by 65.93% in 156ms
Compressed canterbury-corpus/calgary/obj2 by 73.57% in 3478ms
Compressed canterbury-corpus/calgary/trans by 83.55% in 1074ms
Compressed canterbury-corpus/calgary/book2 by 72.95% in 8769ms
Compressed canterbury-corpus/misc/pi.txt by 57.51% in 10696ms
Compressed canterbury-corpus/large/world192.txt by 80.79% in 35587ms
Compressed canterbury-corpus/large/E.coli by 75.47% in 120477ms
Compressed canterbury-corpus/large/bible.txt by 78.02% in 62254ms
Compressed canterbury-corpus/artificial/a.txt by -400.00% in 7ms
Compressed canterbury-corpus/artificial/aaa.txt by 99.99% in 115ms
Compressed canterbury-corpus/artificial/alphabet.txt by 99.97% in 74ms
Compressed canterbury-corpus/artificial/random.txt by 24.98% in 949ms
Compressed canterbury-corpus/canterbury/xargs.1 by 65.37% in 73ms
Compressed canterbury-corpus/canterbury/grammar.lsp by 69.77% in 63ms
Compressed canterbury-corpus/canterbury/kennedy.xls by 94.03% in 30720ms
Compressed canterbury-corpus/canterbury/sum by 73.37% in 504ms
Compressed canterbury-corpus/canterbury/asyoulik.txt by 65.88% in 1390ms
Compressed canterbury-corpus/canterbury/ptt5 by 92.04% in 14538ms
Compressed canterbury-corpus/canterbury/lcet10.txt by 73.42% in 5590ms
Compressed canterbury-corpus/canterbury/plrabn12.txt by 66.14% in 5842ms
Compressed canterbury-corpus/canterbury/alice29.txt by 69.43% in 1829ms
Compressed canterbury-corpus/canterbury/cp.html by 71.97% in 256ms
Compressed canterbury-corpus/canterbury/fields.c by 75.63% in 144ms
```
