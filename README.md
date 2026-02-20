# Hstats: Online Statistics and Histograms for Data Streams

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/antimora/hstats/rust.yml)](https://github.com/antimora/hstats/actions)
[![Crates.io](https://img.shields.io/crates/v/hstats.svg)](https://crates.io/crates/hstats)
[![Docs.rs](https://docs.rs/hstats/badge.svg)](https://docs.rs/hstats)

A Rust library for computing histograms and statistics from data streams without loading entire
datasets into memory. Designed for parallel workloads where independent histograms can be merged
into a single result.

## Features

- **Online computation** - processes values one at a time, constant memory usage
- **Parallel-friendly** - build histograms per-thread, then `merge()` them
- **Underflow/overflow tracking** - values outside `[start, end)` are counted separately
- **Quantile queries** - `bins_at_centiles`, `bins_at_quartiles`, `bins_at_myriatiles`, or custom scales via `bins_at_quantiles`
- **Statistics** - min, max, mean, and standard deviation via Welford's algorithm
  ([rolling-stats](https://github.com/ryankurte/rust-rolling-stats))
- **`Display` trait** - configurable text-based histogram output with custom precision and bar characters
- **`no_std` compatible** - works in `no_std` environments that support `alloc`

## Getting Started

Add the following to your `Cargo.toml`:

```toml
[dependencies]
hstats = "0.2.0"
```

## Usage

```rust
use hstats::Hstats;

// Create a histogram with 10 bins over the range [0.0, 100.0)
let mut hist = Hstats::new(0.0, 100.0, 10);

// Add values
for value in &[15.0, 25.0, 35.5, 50.0, 72.0, 91.0] {
    hist.add(*value);
}

// Query statistics
println!("count: {}, mean: {:.2}, std_dev: {:.2}", hist.count(), hist.mean(), hist.std_dev());
println!("min: {:.2}, max: {:.2}", hist.min(), hist.max());

// Print the histogram
println!("{}", hist.with_precision(1));

// Query percentile bins (p50, p90, p99)
let percentiles = hist.bins_at_centiles(&[50, 90, 99]);
for (lower, upper, cumul_count) in &percentiles {
    println!("[{lower}, {upper}) cumulative: {cumul_count}");
}
```

### Parallel usage

Build histograms independently on each thread, then merge:

```rust
// On each thread:
let mut local = Hstats::new(0.0, 100.0, 10);
for value in chunk {
    local.add(*value);
}

// After all threads finish, merge results:
let combined = histograms.into_iter()
    .reduce(|a, b| a.merge(&b))
    .unwrap();
```

See [examples/single-thread.rs](examples/single-thread.rs) and
[examples/multi-thread.rs](examples/multi-thread.rs) for complete runnable examples.

## Examples

Run the examples with:

```shell
cargo run --example single-thread --release
cargo run --example multi-thread --release
```

Sample output from the multi-thread example:

```
Number of random samples: 50000000
Number of bins: 30
Start: -8
End: 10
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
Percentiles:
  p25: ~0.10
  p50: ~1.90
  p75: ~4.30
  p90: ~6.10
  p99: ~9.10


real    0m1.905s
user    0m9.727s
sys     0m0.127s
```

## License

`hstats` is licensed under your choice of either the Apache License, Version 2.0, or the MIT
license.
