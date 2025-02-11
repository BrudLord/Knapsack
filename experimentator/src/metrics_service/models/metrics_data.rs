use serde::{Deserialize, Serialize};

/// Represents metrics data collected from algorithm execution
///
/// # Fields
/// * `result` - The calculated result of the algorithm execution
/// * `execution_time_ns` - Time taken for execution in nanoseconds
/// * `memory_usage` - Memory consumed during execution in bytes
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetricsData {
    /// The result value produced by the algorithm.
    /// `None` indicates execution failure or no result.
    pub result: Result<u64, String>,

    /// Execution time in nanoseconds.
    /// `None` if timing was not measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time_ns: Option<u128>,

    /// Memory usage in bytes.
    /// `None` if memory usage was not measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_usage: Option<usize>,
}

impl From<(Result<u64, String>, Option<u128>, Option<usize>)> for MetricsData {
    fn from(t: (Result<u64, String>, Option<u128>, Option<usize>)) -> MetricsData {
        MetricsData {
            result: t.0,
            execution_time_ns: t.1,
            memory_usage: t.2,
        }
    }
}