//! Analysis contains technical indicators that try to predict the direction of
//! future values using the past data.
use std::error::Error;
use std::fmt;
use std::result;

pub mod momentum;
pub mod trend;

/// A specialised `Result` type for analysis operations.
///
/// This is used across `stat::analysis` for any operation
/// which may cause an error.
pub type Result<T> = result::Result<T, AnalysisError>;

/// A list specifying types of analysis errors.
#[derive(Debug)]
pub enum AnalysisError {
	/// Gain must be positive.
	GainLessThanZero,
	/// Loss must be positive.
	LossLessThanZero,
	/// Close must be less than or equal to high.
	CloseGreaterThanHigh,
	/// Close must be greater than or equal to low.
	CloseLessThanLow,
	/// High must be greater than or equal to low.
	HighLessThanLow,
	/// Slice must not be empty.
	SliceIsEmpty,
}

impl fmt::Display for AnalysisError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "error: {}", self.description())
	}
}

impl Error for AnalysisError {
	fn description(&self) -> &str {
		match *self {
			AnalysisError::GainLessThanZero => "gain < 0",
			AnalysisError::LossLessThanZero => "loss < 0",
			AnalysisError::CloseGreaterThanHigh => "close > high",
			AnalysisError::CloseLessThanLow => "close < low",
			AnalysisError::HighLessThanLow => "high < low",
			AnalysisError::SliceIsEmpty => "slice is empty",
		}
	}

	fn cause(&self) -> Option<&Error> {
		None
	}
}
