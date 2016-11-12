//! Momentum contains simple technical analysis indicators showing the
//! difference between period's closing price and the closing price N
//! periods ago.
use analysis::{AnalysisError, Result};

/// Relative strength index (RSI) is a momentum oscillator developed by J.
/// Welles Wilder Jr. that measures the speed and change of movements. The
/// output of the function oscillates between 0 and 100.
///
/// Wilder suggested in his book that RSI calculation should be based on 14
/// periods. gain and loss arguments are averages for the used period.
///
/// # Arguments
///
/// * `gain` - averege gain for the period
/// * `loss` - averege loss for the period
///
/// # Example
///
/// ```
/// use stat::analysis::momentum;
///
/// let value = momentum::relative_strength_index(60., 20.);
/// assert_eq!(value.ok(), Some(75.));
/// ```
pub fn relative_strength_index(gain: f64, loss: f64) -> Result<f64> {
	if gain < 0. {
		return Err(AnalysisError::GainLessThanZero);
	}
	if loss < 0. {
		return Err(AnalysisError::LossLessThanZero);
	}
	Ok(match loss {
		0. => 100.,
		_ => 100. - 100. / (1. + gain / loss),
	})
}

/// Stochastic oscillator is a momentum indicator developed by Dr. George Lane
/// that shows the location of the value relative to the high-low range. The
/// output of the function oscillates between 0 and 100.
///
/// Typical number of periods for stochastic oscillator calculations is 5, 9,
/// or 14. Stochastic oscillator is used together with 1 or 2 signal lines.
/// The 1st signal line is the exponential moving averege of the stochastic
/// oscillator value and the 2nd signal line is the exponential moving average
/// of the 1st signal line value. Signal lines are typically calculated from
/// 3 previous periods.
///
/// `high` and `low` arguments are the highest high and the lowest low
/// respectively for the used period.
///
/// The formula is modified to return 50 if the highest high equals the
/// lowest low.
///
/// # Arguments
///
/// * `close` - closing price for the period
/// * `high` - highest price for the period
/// * `low` - lowest price for the period
///
/// # Example
///
/// ```
/// use stat::analysis::momentum;
///
/// let value = momentum::stochastic_oscillator(80., 90., 50.);
/// assert_eq!(value.ok(), Some(75.));
/// ```
pub fn stochastic_oscillator(close: f64, high: f64, low: f64) -> Result<f64> {
	if high < low {
		return Err(AnalysisError::HighLessThanLow);
	}
	if close > high {
		return Err(AnalysisError::CloseGreaterThanHigh);
	}
	if close < low {
		return Err(AnalysisError::CloseLessThanLow);
	}
	Ok(match high == low {
		true => 50.,
		false => 100. * (close - low) / (high - low),
	})
}

/// Williams %R is a momentum indicator developed by Larry R. Williams that
/// is the inverse of the stochastic oscillator. It reflects the level of the
/// value relative to the high. The output of the function oscillates between
/// -100 and 0.
///
/// Typical number of periods and signal lines used with the stochastic
/// oscillator are also applicable to Williams %R.
///
/// `high` and `low` arguments are the highest high and the lowest low
/// respectively for the used period.
///
/// The formula is modified to return -50 if the highest high equals the
/// lowest low.
///
/// # Arguments
///
/// * `close` - closing price for the period
/// * `high` - highest price for the period
/// * `low` - lowest price for the period
///
/// # Example
///
/// ```
/// use stat::analysis::momentum;
///
/// let value = momentum::williams_percent_r(80., 90., 50.);
/// assert_eq!(value.ok(), Some(-25.));
/// ```
pub fn williams_percent_r(close: f64, high: f64, low: f64) -> Result<f64> {
	if high < low {
		return Err(AnalysisError::HighLessThanLow);
	}
	if close > high {
		return Err(AnalysisError::CloseGreaterThanHigh);
	}
	if close < low {
		return Err(AnalysisError::CloseLessThanLow);
	}
	Ok(match high == low {
		true => -50.,
		false => -100. * (high - close) / (high - low),
	})
}

#[cfg(test)]
mod tests {
	extern crate math;
	use analysis::{AnalysisError, Result};
	use self::math::round::half_up;
	use std::error::Error;

	#[test]
	fn relative_strength_index() {
		let tests: [(f64, f64, Result<f64>); 24] = [
			(0.238386, 0.099593, Ok(70.532785)),
			(0.221358, 0.112422, Ok(66.318533)),
			(0.207690, 0.104392, Ok(66.549817)),
			(0.219912, 0.096935, Ok(69.406370)),
			(0.204204, 0.103540, Ok(66.355152)),
			(0.189618, 0.137451, Ok(57.974923)),
			(0.216667, 0.127633, Ok(62.929712)),
			(0.204040, 0.118517, Ok(63.257037)),
			(0.189466, 0.148508, Ok(56.059342)),
			(0.228633, 0.137901, Ok(62.377024)),
			(0.212302, 0.175765, Ok(54.707563)),
			(0.197137, 0.193832, Ok(50.422668)),
			(0.183056, 0.274701, Ok(39.989776)),
			(0.180659, 0.255079, Ok(41.460465)),
			(0.170598, 0.236859, Ok(41.868958)),
			(0.183348, 0.219941, Ok(45.463179)),
			(0.170252, 0.286138, Ok(37.304060)),
			(0.158091, 0.319821, Ok(33.079521)),
			(0.180270, 0.296977, Ok(37.772893)),

			(1., 0., Ok(100.)),
			(0., 1., Ok(0.)),
			(1., 1., Ok(50.)),
			(-1., 0., Err(AnalysisError::GainLessThanZero)),
			(0., -1., Err(AnalysisError::LossLessThanZero)),
		];

		for test in &tests {
			let result = super::relative_strength_index(test.0, test.1);
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
	fn stochastic_oscillator() {
		let tests: [(f64, f64, f64, Result<f64>); 23] = [
			(127.2876, 128.4317, 124.5615, Ok(70.438220)),
			(127.1781, 128.4317, 124.5615, Ok(67.608909)),
			(128.0138, 128.4317, 124.5615, Ok(89.202108)),
			(127.1085, 128.4317, 124.5615, Ok(65.810552)),
			(127.7253, 128.4317, 124.5615, Ok(81.747713)),
			(127.0587, 128.4317, 124.5615, Ok(64.523797)),
			(127.3273, 128.2725, 124.5615, Ok(74.529776)),
			(128.7103, 128.7700, 124.5615, Ok(98.581442)),
			(127.8745, 129.2873, 124.5615, Ok(70.104533)),
			(128.5809, 130.0633, 124.5615, Ok(73.056091)),
			(128.6008, 130.0633, 124.5615, Ok(73.417791)),
			(127.9342, 130.0633, 124.5715, Ok(61.231290)),
			(128.1133, 130.0633, 125.0689, Ok(60.956271)),
			(127.5960, 130.0633, 125.9245, Ok(40.386102)),
			(127.5960, 130.0633, 125.9245, Ok(40.386102)),
			(128.6904, 130.0633, 125.9245, Ok(66.828549)),
			(128.2725, 130.0633, 125.9245, Ok(56.731420)),

			(100., 100., 100., Ok(50.)),
			(100., 100., 0., Ok(100.)),
			(0., 100., 0., Ok(0.)),
			(100., 0., 100., Err(AnalysisError::HighLessThanLow)),
			(100., 0., 0., Err(AnalysisError::CloseGreaterThanHigh)),
			(0., 100., 100., Err(AnalysisError::CloseLessThanLow)),
		];

		for test in &tests {
			let result = super::stochastic_oscillator(test.0, test.1, test.2);
			match (result, test.3.as_ref()) {
				(Ok(val), Ok(exp))
					=> assert_eq!(half_up(val, 6), *exp),
				(Err(err), Err(exp))
					=> assert_eq!(err.description(), exp.description()),
				_ => panic!("return type mismatch"),
			}
		}
	}

	#[test]
	fn williams_percent_r() {
		let tests: [(f64, f64, f64, Result<f64>); 23] = [
			(127.2876, 128.4317, 124.5615, Ok(-29.561780)),
			(127.1781, 128.4317, 124.5615, Ok(-32.391091)),
			(128.0138, 128.4317, 124.5615, Ok(-10.797891)),
			(127.1085, 128.4317, 124.5615, Ok(-34.189447)),
			(127.7253, 128.4317, 124.5615, Ok(-18.252287)),
			(127.0587, 128.4317, 124.5615, Ok(-35.476203)),
			(127.3273, 128.2725, 124.5615, Ok(-25.470224)),
			(128.7103, 128.7700, 124.5615, Ok(-1.418558)),
			(127.8745, 129.2873, 124.5615, Ok(-29.895467)),
			(128.5809, 130.0633, 124.5615, Ok(-26.943909)),
			(128.6008, 130.0633, 124.5615, Ok(-26.582209)),
			(127.9342, 130.0633, 124.5715, Ok(-38.768710)),
			(128.1133, 130.0633, 125.0689, Ok(-39.043729)),
			(127.5960, 130.0633, 125.9245, Ok(-59.613898)),
			(127.5960, 130.0633, 125.9245, Ok(-59.613898)),
			(128.6904, 130.0633, 125.9245, Ok(-33.171451)),
			(128.2725, 130.0633, 125.9245, Ok(-43.268580)),

			(100., 100., 100., Ok(-50.)),
			(100., 100., 0., Ok(0.)),
			(0., 100., 0., Ok(-100.)),
			(100., 0., 100., Err(AnalysisError::HighLessThanLow)),
			(100., 0., 0., Err(AnalysisError::CloseGreaterThanHigh)),
			(0., 100., 100., Err(AnalysisError::CloseLessThanLow)),
		];

		for test in &tests {
			let result = super::williams_percent_r(test.0, test.1, test.2);
			match (result, test.3.as_ref()) {
				(Ok(val), Ok(exp))
					=> assert_eq!(half_up(val, 6), *exp),
				(Err(err), Err(exp))
					=> assert_eq!(err.description(), exp.description()),
				_ => panic!("return type mismatch"),
			}
		}
	}
}
