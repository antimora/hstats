# Hstats: Online Data Stream Statistics & Histograms

`hstats` is a high-performance, lightweight library designed for real-time statistical analysis and
histogram generation from data streams. With a focus on multi-threaded environments, `hstats`
enables the creation and merging of multiple `Hstats` instances, providing a statistical overview
and histogram of the entire data stream.

When creating a histogram, the number and width of bins are pre-determined. The bin width is
determined as (end - start)/nbins. Values within the range [start, end) are counted within the bins,
while values outside this range are included in underflow and overflow bins, allowing for histogram
range adjustments.

`hstats` leverages Welford's algorithm to calculate mean and standard deviation statistics,
implemented through the [rolling-stats](https://github.com/ryankurte/rust-rolling-stats) library. It
is compatible with [no_std environments](https://docs.rust-embedded.org/book/intro/no-std.html)
supporting alloc.

For simple printing of statistics and histograms, the library implements `Display` for `Hstats`.
Users can specify the floating-point precision (default precision is 2) of statistics to print and
the character used for histogram bars (default is `░`).

## Getting Started

Add the following to your `Cargo.toml`:

```toml
[dependencies]
hstats = "0.1.0"
```

## Examples

1. Single thread example: See [examples/single-thread.rs](examples/single-thread.rs) Run the example
   with:
   ```shell
   time cargo run --example single-thread --release
   ```
2. Multi-thread example: See [examples/multi-thread.rs](examples/multi-thread.rs) Run the example
   with:
   ```shell
   time cargo run --example multi-thread --release
   ```

Here is a sample output from the multi-thread example:

```
Thread count: 20
Chunk size: 2500000
Number of hstats to merge: 20
Start |  End
------|-------
 -inf | -8.00 |  21553 (0.04%)
-8.00 | -7.40 |  21752 (0.04%)
-7.40 | -6.80 |  40523 (0.08%)
-6.80 | -6.20 | ░ 73078 (0.15%)
-6.20 | -5.60 | ░ 125206 (0.25%)
-5.60 | -5.00 | ░░░ 207593 (0.42%)
-5.00 | -4.40 | ░░░░ 331470 (0.66%)
-4.40 | -3.80 | ░░░░░░░ 508330 (1.02%)
-3.80 | -3.20 | ░░░░░░░░░░░ 745836 (1.49%)
-3.20 | -2.60 | ░░░░░░░░░░░░░░░ 1054228 (2.11%)
-2.60 | -2.00 | ░░░░░░░░░░░░░░░░░░░░░ 1433304 (2.87%)
-2.00 | -1.40 | ░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 1868758 (3.74%)
-1.40 | -0.80 | ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 2339425 (4.68%)
-0.80 | -0.20 | ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 2819448 (5.64%)
-0.20 |  0.40 | ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 3261165 (6.52%)
 0.40 |  1.00 | ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 3623683 (7.25%)
 1.00 |  1.60 | ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 3875841 (7.75%)
 1.60 |  2.20 | ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 3980760 (7.96%)
 2.20 |  2.80 | ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 3928130 (7.86%)
 2.80 |  3.40 | ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 3725868 (7.45%)
 3.40 |  4.00 | ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 3393196 (6.79%)
 4.00 |  4.60 | ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 2971132 (5.94%)
 4.60 |  5.20 | ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 2497847 (5.00%)
 5.20 |  5.80 | ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 2022084 (4.04%)
 5.80 |  6.40 | ░░░░░░░░░░░░░░░░░░░░░░░ 1569143 (3.14%)
 6.40 |  7.00 | ░░░░░░░░░░░░░░░░░ 1171523 (2.34%)
 7.00 |  7.60 | ░░░░░░░░░░░░ 841536 (1.68%)
 7.60 |  8.20 | ░░░░░░░░ 579675 (1.16%)
 8.20 |  8.80 | ░░░░░ 383658 (0.77%)
 8.80 |  9.40 | ░░░ 243884 (0.49%)
 9.40 | 10.00 | ░░ 148977 (0.30%)
10.00 |   inf | ░░ 191394 (0.38%)

Total Count: 50000000 Min: -14.19 Max: 18.04 Mean: 2.00 Std Dev: 3.00


real    0m1.836s
user    0m9.756s
sys     0m0.099s
```

## License

`hstats` is licensed under your choice of either the Apache License, Version 2.0, or the MIT
license.
