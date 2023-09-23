use chrono::{DateTime, Datelike, TimeZone, Utc};
use napi_derive::napi;

const MAX_ITERATIONS: usize = 100;
const REFINEMENT_LIMIT: f64 = 2.0;

#[napi]
pub fn get_xirr(amounts: Vec<f64>, timestamps: Vec<i64>) -> f64 {
  let dates: Vec<DateTime<Utc>> = timestamps
    .into_iter()
    .map(|t| Utc.timestamp_millis_opt(t).unwrap())
    .collect();

  let interval_start = &dates[0];
  let last_transaction_date = &dates[dates.len() - 1];

  if is_same_day(interval_start, last_transaction_date) {
    return 0.0;
  }

  let year_diff =
    diff_in_years(interval_start, last_transaction_date).expect("Dates ordered descending!");

  let mut amount_invested: f64 = 0.0;
  let mut amount_gained: f64 = 0.0;

  for amount in &amounts {
    if *amount > 0.0 {
      amount_gained += amount.abs();
    } else {
      amount_invested += amount.abs();
    }
  }

  let durations: Vec<f64> = dates
    .iter()
    .enumerate()
    .map(|(idx, datetime)| {
      if idx == 0 {
        0.0
      } else {
        diff_in_years_f64(interval_start, datetime)
      }
    })
    .collect();

  let first_guess = get_return(amount_gained, amount_invested) / (100.0 * year_diff as f64);
  let mut rate: f64 = if first_guess.abs() > 1.0 {
    0.0
  } else {
    first_guess
  };
  let mut npv = f64::MAX;
  let mut iterations = 0;

  while iterations < MAX_ITERATIONS {
    if is_close_to_zero(npv) {
      break;
    }

    let mut sum: f64 = 0.0;
    let mut sum_fdx: f64 = 0.0;
    let sign: f64 = if rate < -1.0 { -1.0 } else { 1.0 };

    for (idx, amount) in amounts.iter().enumerate() {
      let dur = durations[idx];
      let base = (1.0 + rate).abs();

      sum += amount / base.powf(dur) * sign;
      sum_fdx += (-amount) * durations[idx] * sign * (1.0 + rate).abs().powf(-1.0 - durations[idx]);
    }

    npv = sum;
    rate -= npv / sum_fdx;
    iterations += 1;
  }

  if iterations >= MAX_ITERATIONS {
    0.0
  } else {
    rate * 100.0
  }
}

fn get_return(gain: f64, base: f64) -> f64 {
  let denominator = base.abs();

  if denominator == 0.0 {
    0.0
  } else {
    (gain / base) * 100.0
  }
}

fn is_same_day(d1: &DateTime<Utc>, d2: &DateTime<Utc>) -> bool {
  d1.ordinal() == d2.ordinal() && d1.year() == d2.year()
}

fn diff_in_years(previous_date: &DateTime<Utc>, later_date: &DateTime<Utc>) -> Option<u32> {
  later_date.years_since(*previous_date)
}

fn diff_in_years_f64(previous_date: &DateTime<Utc>, later_date: &DateTime<Utc>) -> f64 {
  let diff_in_days = (*later_date - previous_date).num_days() as f64;

  diff_in_days / 365.25
}

fn is_close_to_zero(npv: f64) -> bool {
  (npv * REFINEMENT_LIMIT * 10.0).round() / (REFINEMENT_LIMIT * 10.0) == 0.0
}
