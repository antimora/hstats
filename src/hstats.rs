use core::fmt::{Error, Formatter};
use core::{
    fmt::{Debug, Display},
    ops::AddAssign,
};

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use num_traits::{Float, FromPrimitive};
use rolling_stats::Stats;

const DEFAULT_BAR_CHAR: &str = "░";
const DEFAULT_PRECISION: usize = 2;

/// `Hstats` is a struct for creating and managing histograms of data.
///
/// The generic `T` refers to the type of the data this histogram manages.
/// `T` must be a float type that implements Float, AddAssign, FromPrimitive,
///  Debug , Display
///
/// The struct includes fields for managing the histogram bins, underflow,
/// overflow, and other statistics.
#[derive(Debug, Default, Clone)]
pub struct Hstats<T>
where
    T: Float + AddAssign + FromPrimitive + Debug + Display,
{
    start: T,
    end: T,
    bin_count: usize,
    bin_width: T,
    bins: Vec<u64>,
    underflow: u64,
    overflow: u64,
    stats: Stats<T>,
    precision: usize,
    bar_char: String,
}

impl<T> Hstats<T>
where
    T: Float + AddAssign + FromPrimitive + Debug + Display,
{
    /// Constructs a new `Hstats` instance with specified start and end points and bin count.
    ///
    /// # Arguments
    ///
    /// * `start`: Lower bound of the range for the histogram bins.
    /// * `end`: Upper bound of the range for the histogram bins.
    /// * `bin_count`: Number of bins in the histogram.
    ///
    /// # Panics
    ///
    /// Panics if `start` >= `end` or if `bin_count` <= 0.
    pub fn new(start: T, end: T, bin_count: usize) -> Self {
        assert!(start < end, "start ({start}) must be less than end ({end})");
        assert!(
            bin_count > 0,
            "bin_count ({bin_count}) must be greater than 0"
        );

        let bin_width = (end - start) / T::from(bin_count).unwrap();

        Self {
            start,
            end,
            bin_count,
            bin_width,
            bins: vec![0; bin_count],
            underflow: 0,
            overflow: 0,
            stats: Stats::new(),
            precision: DEFAULT_PRECISION,
            bar_char: DEFAULT_BAR_CHAR.to_string(),
        }
    }

    /// Adds a value to the histogram and updates the statistics.
    ///
    /// # Arguments
    ///
    /// * `value`: Value to be added to the histogram.
    pub fn add(&mut self, value: T) {
        self.stats.update(value);

        if value < self.start {
            self.underflow += 1;
        } else if value >= self.end {
            self.overflow += 1;
        } else {
            let index = ((value - self.start) / self.bin_width)
                .floor()
                .to_usize()
                .unwrap();
            self.bins[index] += 1;
        }
    }

    /// Merges this histogram with another histogram.
    ///
    /// # Arguments
    ///
    /// * `other`: Another `Hstats` instance to merge with this histogram.
    ///
    /// # Returns
    ///
    /// A new `Hstats` instance resulting from the merge.
    ///
    /// # Panics
    ///
    /// Panics if the `start`, `end`, and `bin_count` of the two histograms aren't equal.
    pub fn merge(&self, other: &Self) -> Self {
        assert_eq!(self.start, other.start, "Starts must be equal");
        assert_eq!(self.end, other.end, "Ends must be equal");
        assert_eq!(self.bin_count, other.bin_count, "Bin counts must be equal");

        let mut merged = Hstats::new(self.start, self.end, self.bin_count);

        // Add the underflow and overflow together
        merged.underflow = self.underflow + other.underflow;
        merged.overflow = self.overflow + other.overflow;

        // Add the bins together
        for (i, (left, right)) in (self.bins.iter().zip(other.bins.iter())).enumerate() {
            merged.bins[i] = *left + *right;
        }

        // Merge the stats
        merged.stats = self.stats.merge(&other.stats);

        merged
    }

    /// Returns the number of bins in the histogram.
    /// Same as the `bin_count` argument passed to `new()`.
    pub fn bin_count(&self) -> usize {
        self.bin_count
    }

    /// Returns the width of each bin in the histogram.
    /// Same as `(end - start) / bin_count`.
    pub fn bin_width(&self) -> T {
        self.bin_width
    }

    /// Returns the start of the range for the histogram bins.
    ///
    /// Values < `start` are counted in the underflow. Values >= `start` are counted in the bins.
    pub fn start(&self) -> T {
        self.start
    }

    /// Returns the end of the range for the histogram bins.
    ///
    /// Values < `end` are counted in the bins. Values >= `end` are counted in the overflow.
    pub fn end(&self) -> T {
        self.end
    }

    /// Returns the ranges and counts for the histogram bins.
    ///
    /// # Returns
    ///
    /// A vector of tuples. Each tuple has lower bound, upper bound, and count for each bin.
    pub fn bins(&self) -> Vec<(T, T, u64)> {
        let mut bins = Vec::with_capacity(self.bin_count + 2);

        // From negative infinity to the start of the first bin
        bins.push((T::neg_infinity(), self.start, self.underflow));

        // From the start of the first bin to the end of the last bin
        let mut lower = self.start;
        let mut upper = self.start + self.bin_width;

        for count in &self.bins {
            bins.push((lower, upper, *count));
            lower = upper;
            upper += self.bin_width;
        }

        // From the end of the last bin to positive infinity
        bins.push((self.end, T::infinity(), self.overflow));

        bins
    }

    /// Maximum value seen so far.
    pub fn max(&self) -> T {
        self.stats.max
    }

    /// Minimum value seen so far.
    pub fn min(&self) -> T {
        self.stats.min
    }

    /// Mean value calculated so far.
    pub fn mean(&self) -> T {
        self.stats.mean
    }

    /// Standard deviation calculated so far.
    pub fn std_dev(&self) -> T {
        self.stats.std_dev
    }

    /// Number of values seen so far.
    pub fn count(&self) -> usize {
        self.stats.count
    }

    /// Modifies the precision of the histogram.
    pub fn with_precision(mut self, precision: usize) -> Self {
        self.precision = precision;
        self
    }

    /// Modifies the character used to display the histogram bars.
    pub fn with_bar_char(mut self, bar_char: &str) -> Self {
        self.bar_char = bar_char.to_string();
        self
    }
}

/// Display the histogram as a text-based histogram.
impl<T> Display for Hstats<T>
where
    T: Float + AddAssign + FromPrimitive + Debug + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        const MAX_BAR_SIZE: usize = 60; // Maximum size of the histogram bar

        // Find the bin with maximum count
        let max_count = *self.bins.iter().max().unwrap_or(&0);

        // let col1_width = self.bins.iter().max_by_key(|(start, _, _)| *start).unwrap();

        let col1 = self
            .bins()
            .iter()
            .map(|(start, _, _)| format!("{:.*}", self.precision, start).len())
            .max()
            .unwrap();

        let col2 = self
            .bins()
            .iter()
            .map(|(_, end, _)| format!("{:.*}", self.precision, end).len())
            .max()
            .unwrap();

        let precision = self.precision;

        writeln!(f, "{:^col1$} | {:^col2$}", "Start", "End")?;
        writeln!(f, "{:-^col1$}-|-{:-^col2$}-", "", "")?;
        for (range_start, range_end, count) in self.bins() {
            // Calculate the length of the bar
            let bar_length = ((count as f64 / max_count as f64) * MAX_BAR_SIZE as f64) as usize;

            let percent = count as f64 / self.count() as f64 * 100.0;

            // Create the bar string with '#' characters
            let bar = self.bar_char.repeat(bar_length);

            writeln!(
                f,
                "{range_start:>col1$.precision$} | {range_end:>col2$.precision$} | {bar} {count} ({percent:.2}%)",
            )?;
        }
        writeln!(f)?;
        write!(f, "Total Count: {}", self.count())?;
        write!(f, " Min: {:.*}", self.precision, self.min())?;
        write!(f, " Max: {:.*}", self.precision, self.max())?;
        write!(f, " Mean: {:.*}", self.precision, self.mean())?;
        writeln!(f, " Std Dev: {:.*}", self.precision, self.std_dev())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::ApproxEq;
    use rand::SeedableRng;
    use rand_distr::{Distribution, Normal};
    use rayon::{prelude::ParallelIterator, slice::ParallelSlice};

    use super::*;

    // Tests for Hstats::new
    #[test]
    fn test_new() {
        let hstats = Hstats::new(0.0, 10.0, 10);

        assert_eq!(hstats.start, 0.0);
        assert_eq!(hstats.end, 10.0);
        assert_eq!(hstats.bin_count, 10);
        assert_eq!(hstats.bin_width, 1.0);
        assert_eq!(hstats.bins.len(), 10);
    }

    #[test]
    #[should_panic(expected = "start (0) must be less than end (0)")]
    fn test_new_start_equal_end() {
        let _ = Hstats::new(0.0, 0.0, 10);
    }

    #[test]
    #[should_panic(expected = "bin_count (0) must be greater than 0")]
    fn test_new_bin_count_zero() {
        let _ = Hstats::new(0.0, 10.0, 0);
    }

    // Tests for Hstats::add
    #[test]
    fn test_add() {
        let mut hstats = Hstats::new(0.0, 10.0, 10);
        hstats.add(5.0);

        assert_eq!(hstats.bins[5], 1);
        assert_eq!(hstats.count(), 1);
    }

    #[test]
    fn test_add_underflow() {
        let mut hstats = Hstats::new(0.0, 10.0, 10);
        hstats.add(-1.0);
        hstats.add(0.0);

        assert_eq!(hstats.underflow, 1);
        assert_eq!(hstats.count(), 2);
    }

    #[test]
    fn test_add_overflow() {
        let mut hstats = Hstats::new(0.0, 10.0, 10);
        hstats.add(11.0);
        hstats.add(10.0);

        assert_eq!(hstats.overflow, 2);
        assert_eq!(hstats.count(), 2);
    }

    // Test for Hstats::merge
    #[test]
    fn test_merge() {
        let mut hstats1 = Hstats::new(0.0, 10.0, 10);
        hstats1.add(5.0);
        let mut hstats2 = Hstats::new(0.0, 10.0, 10);
        hstats2.add(6.0);

        let merged = hstats1.merge(&hstats2);
        assert_eq!(merged.bins[5], 1);
        assert_eq!(merged.bins[6], 1);
        assert_eq!(merged.count(), 2);
    }

    #[test]
    #[should_panic(expected = "Starts must be equal")]
    fn test_merge_different_start() {
        let hstats1 = Hstats::new(0.0, 10.0, 10);
        let hstats2 = Hstats::new(1.0, 10.0, 10);

        let _ = hstats1.merge(&hstats2);
    }

    #[test]
    #[should_panic(expected = "Ends must be equal")]
    fn test_merge_different_end() {
        let hstats1 = Hstats::new(0.0, 10.0, 10);
        let hstats2 = Hstats::new(0.0, 11.0, 10);

        let _ = hstats1.merge(&hstats2);
    }

    #[test]
    #[should_panic(expected = "Bin counts must be equal")]
    fn test_merge_different_bin_count() {
        let hstats1 = Hstats::new(0.0, 10.0, 10);
        let hstats2 = Hstats::new(0.0, 10.0, 11);

        let _ = hstats1.merge(&hstats2);
    }

    #[test]
    fn stats_for_large_random_data() {
        type T = f64;

        // Define some constants
        const MEAN: T = 2.0;
        const STD_DEV: T = 3.0;
        const SEED: u64 = 42;
        const NUM_SAMPLES: usize = 10_000;
        const NUM_BINS: usize = 100;
        const START: T = -10.0;
        const END: T = 10.0;

        let mut hstats = Hstats::new(START, END, NUM_BINS);

        let mut rng = rand::rngs::StdRng::seed_from_u64(SEED);

        let normal = Normal::<T>::new(MEAN, STD_DEV).unwrap();

        // Generate some random data
        let random_data: Vec<T> = (0..NUM_SAMPLES).map(|_x| normal.sample(&mut rng)).collect();

        // Update the stats
        random_data.iter().for_each(|v| hstats.add(*v));

        // Check the standard deviation against the stats' standard deviation
        assert!(hstats.std_dev().approx_eq(STD_DEV, (1.0e-2, 2)));

        // Check the mean against the stats' mean
        assert!(hstats.mean().approx_eq(MEAN, (1.0e-1, 2)));

        // Check the counts
        assert_eq!(hstats.count(), random_data.len());

        let count_from_bins: u64 =
            hstats.bins.iter().copied().sum::<u64>() + hstats.underflow + hstats.overflow;

        assert_eq!(count_from_bins as usize, random_data.len());

        // Min, Max, Mean, and StdDev are tested by rolling-stats tests, so we don't need to test them here
    }

    #[test]
    fn stats_parallel() {
        type T = f64;

        // Define some constants
        const MEAN: T = 2.0;
        const STD_DEV: T = 3.0;
        const SEED: u64 = 42;
        const NUM_SAMPLES: usize = 10_000;
        const NUM_BINS: usize = 100;
        const START: T = -10.0;
        const END: T = 10.0;

        // let mut hstats = Hstats::new(START, END, NUM_BINS);

        let mut rng = rand::rngs::StdRng::seed_from_u64(SEED);

        let normal = Normal::<T>::new(MEAN, STD_DEV).unwrap();

        // Generate some random data
        let random_data: Vec<T> = (0..NUM_SAMPLES).map(|_x| normal.sample(&mut rng)).collect();

        // Update the stats

        let thread_count = rayon::current_num_threads() * 2;
        let chunk_size = random_data.len() / thread_count;

        // Update the stats for each chunk in parallel
        let hstats_list: Vec<Hstats<T>> = random_data
            .par_chunks(chunk_size)
            .map(|chunk| {
                let mut hstats = Hstats::new(START, END, NUM_BINS);
                chunk.iter().for_each(|v| hstats.add(*v));
                hstats
            })
            .collect();

        // Merge the stats from each chunk
        let merged = hstats_list
            .into_iter()
            .reduce(|hstats1, hstats2| hstats1.merge(&hstats2))
            .unwrap();

        // Check the standard deviation against the stats' standard deviation
        assert!(merged.std_dev().approx_eq(STD_DEV, (1.0e-2, 2)));

        // Check the mean against the stats' mean
        assert!(merged.mean().approx_eq(MEAN, (1.0e-1, 2)));

        // Check the count
        assert_eq!(merged.count(), random_data.len());
    }
}
