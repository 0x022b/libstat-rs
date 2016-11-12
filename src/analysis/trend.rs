//! Trend contains technical analysis indicators that try to predict the
//! direction in which values are moving towards.
use analysis::{AnalysisError, Result};

/// Exponential moving average (EMA) is a filter that applies weighting factors
/// which decrease exponentially. The weighting for each older datum decreases
/// exponentially, never reaching zero.
///
/// The number of periods depends on the analytical objectives. Typical number
/// of periods for short, medium and long term trends are 5-20, 20-60 and
/// 100-200 respectively.
///
/// Function calculates the weighting factor from the slice length and
/// calculates the exponential moving average using that factor. If EMA is
/// calculated for the first time previous argument needs to be 0. When previous
/// is 0 function returns simple moving average of the datum points.
///
/// # Arguments
///
/// * `slice` - array of values
/// * `old` - previous EMA if any
///
/// # Example
///
/// ```
/// use stat::analysis::trend;
///
/// let array = [3.5, 3.4, 3.3, 3.6, 3.7];
/// let value = trend::exponential_moving_average(&array, Some(3.4));
/// assert_eq!(value.ok(), Some(3.5));
/// ```
pub fn exponential_moving_average(slice: &[f64], old: Option<f64>) -> Result<f64> {
	let length = slice.len();
	if length == 0 {
		return Err(AnalysisError::SliceIsEmpty);
	}
	Ok(match old {
		Some(ema) => (slice[length-1] - ema) * 2. / (1. + length as f64) + ema,
		None => try!(simple_moving_average(slice)),
	})
}

/// Simple moving average (SMA) is the unweighted mean of the datum points.
///
/// The number of periods depends on the analytical objectives. Typical number
/// of periods for short, medium and long term trends are 5-20, 20-60 and
/// 100-200 respectively.
///
/// # Arguments
///
/// * `slice` - array of values
///
/// # Example
///
/// ```
/// use stat::analysis::trend;
///
/// let array = [3.5, 3.4, 3.3, 3.6, 3.7];
/// let value = trend::simple_moving_average(&array);
/// assert_eq!(value.ok(), Some(3.5));
/// ```
pub fn simple_moving_average(slice: &[f64]) -> Result<f64> {
	let length = slice.len();
	if length == 0 {
		return Err(AnalysisError::SliceIsEmpty);
	}
	Ok(slice.iter().fold(0., |sum, x| sum + x) / length as f64)
}

#[cfg(test)]
mod tests {
	extern crate math;
	use analysis::{AnalysisError, Result};
	use self::math::round::half_up;
	use std::error::Error;

	#[test]
	fn exponential_moving_average() {
		let values: [f64; 30] = [
			22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24,
			22.29, 22.15, 22.39, 22.38, 22.61, 23.36, 24.05, 23.75, 23.83,
			23.95, 23.63, 23.82, 23.87, 23.65, 23.19, 23.10, 23.33, 22.68,
			23.10, 22.40, 22.17,
		];
		let results: [f64; 21] = [
			22.221000, 22.208091, 22.241165, 22.266408, 22.328879, 22.516356,
			22.795200, 22.968800, 23.125382, 23.275313, 23.339802, 23.427111,
			23.507636, 23.533520, 23.471062, 23.403596, 23.390215, 23.261085,
			23.231797, 23.080561, 22.915004,
		];
		let tests: [(&[f64], Option<f64>, Result<f64>); 22] = [
			(&values[0..0], None, Err(AnalysisError::SliceIsEmpty)),
			(&values[0..10], None, Ok(results[0])),
			(&values[1..11], Some(results[0]), Ok(results[1])),
			(&values[2..12], Some(results[1]), Ok(results[2])),
			(&values[3..13], Some(results[2]), Ok(results[3])),
			(&values[4..14], Some(results[3]), Ok(results[4])),
			(&values[5..15], Some(results[4]), Ok(results[5])),
			(&values[6..16], Some(results[5]), Ok(results[6])),
			(&values[7..17], Some(results[6]), Ok(results[7])),
			(&values[8..18], Some(results[7]), Ok(results[8])),
			(&values[9..19], Some(results[8]), Ok(results[9])),
			(&values[10..20], Some(results[9]), Ok(results[10])),
			(&values[11..21], Some(results[10]), Ok(results[11])),
			(&values[12..22], Some(results[11]), Ok(results[12])),
			(&values[13..23], Some(results[12]), Ok(results[13])),
			(&values[14..24], Some(results[13]), Ok(results[14])),
			(&values[15..25], Some(results[14]), Ok(results[15])),
			(&values[16..26], Some(results[15]), Ok(results[16])),
			(&values[17..27], Some(results[16]), Ok(results[17])),
			(&values[18..28], Some(results[17]), Ok(results[18])),
			(&values[19..29], Some(results[18]), Ok(results[19])),
			(&values[20..30], Some(results[19]), Ok(results[20])),
		];

		for test in &tests {
			let result = super::exponential_moving_average(test.0, test.1);
			match (result, test.2.as_ref()) {
				(Ok(val), Ok(exp))
					=> assert_eq!(half_up(val, 6), *exp),
				(Err(err), Err(exp))
					=> assert_eq!(err.description(), exp.description()),
				_ => panic!("return type mismatch"),
			}
		}
	}

	#[test]
	fn simple_moving_average() {
		let values: [f64; 30] = [
			22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24,
			22.29, 22.15, 22.39, 22.38, 22.61, 23.36, 24.05, 23.75, 23.83,
			23.95, 23.63, 23.82, 23.87, 23.65, 23.19, 23.10, 23.33, 22.68,
			23.10, 22.40, 22.17,
		];
		let results: [f64; 21] = [
			22.221, 22.209, 22.229, 22.259, 22.303, 22.421, 22.613, 22.765,
			22.905, 23.076, 23.210, 23.377, 23.525, 23.652, 23.710, 23.684,
			23.612, 23.505, 23.432, 23.277, 23.131,
		];
		let tests: [(&[f64], Result<f64>); 22] = [
			(&values[0..0], Err(AnalysisError::SliceIsEmpty)),
			(&values[0..10], Ok(results[0])),
			(&values[1..11], Ok(results[1])),
			(&values[2..12], Ok(results[2])),
			(&values[3..13], Ok(results[3])),
			(&values[4..14], Ok(results[4])),
			(&values[5..15], Ok(results[5])),
			(&values[6..16], Ok(results[6])),
			(&values[7..17], Ok(results[7])),
			(&values[8..18], Ok(results[8])),
			(&values[9..19], Ok(results[9])),
			(&values[10..20], Ok(results[10])),
			(&values[11..21], Ok(results[11])),
			(&values[12..22], Ok(results[12])),
			(&values[13..23], Ok(results[13])),
			(&values[14..24], Ok(results[14])),
			(&values[15..25], Ok(results[15])),
			(&values[16..26], Ok(results[16])),
			(&values[17..27], Ok(results[17])),
			(&values[18..28], Ok(results[18])),
			(&values[19..29], Ok(results[19])),
			(&values[20..30], Ok(results[20])),
		];

		for test in &tests {
			let result = super::simple_moving_average(test.0);
			match (result, test.1.as_ref()) {
				(Ok(val), Ok(exp))
					=> assert_eq!(half_up(val, 3), *exp),
				(Err(err), Err(exp))
					=> assert_eq!(err.description(), exp.description()),
				_ => panic!("return type mismatch"),
			}
		}
	}
}
