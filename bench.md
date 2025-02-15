# Simple Benchmark

## macOS

input file: enwiki-20231101-pages-articles-multistream-index.txt

| tool            | #keywords | rate     | ratio |
|:---------------:|:---------:|:--------:|:-----:|
| rs-find-keyword | 1         | 738 MB/s | 2.5x  |
| fgrep           | 1         | 295 MB/s | (1x)  |

| tool            | #keywords | rate     | ratio |
|:---------------:|:---------:|:--------:|:-----:|
| rs-find-keyword | 2         | 655 MB/s | 4.0x  |
| fgrep           | 2         | 162 MB/s | (1x)  |

| tool            | #keywords | rate     | ratio |
|:---------------:|:---------:|:--------:|:-----:|
| rs-find-keyword | 3         | 639 MB/s | 5.5x  |
| fgrep           | 3         | 117 MB/s | (1x)  |
