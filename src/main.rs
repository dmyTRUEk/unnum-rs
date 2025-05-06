//! unnum: ie 3.1415... -> pi

use std::f64::consts::PI;

use rand::{rng, rngs::ThreadRng, Rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

const X: float = 1.202056903159594285399738161511449990764986292;
const DIFF: float = 1e-14;

fn main() {
	// dbg!(2_f64.sqrt() - 1.41421356237309504880168872420969807856967187537694807317667973799073247846); // 0.0
	// dbg!(145_f64.sqrt() - 12.0415945787922954801282410303786080524253524050538339520724333245264931535); // 0.0
	// dbg!(PI.powf(2.)/6. - 1.64493406684822643647241516664602518921894990120679843773555822937000747040); // 0.0
	search_by_rand_and_min();
}

fn search_by_rand_and_min() {
	const CORES_N: u32 = 12;
	(0..CORES_N).into_par_iter().for_each(|_| {
		let mut rng = rng();
		loop {
			let args = Args {
				pipow_t: rng.random_range(-100 ..= 100),
				pipow_b: rng.random_range(1 ..= 100),
				numpow_t: rng.random_range(-100 ..= 100),
				numpow_b: rng.random_range(1 ..= 100),
				num  : rng.random_range(1_i32 ..= 10_000),
				denom: rng.random_range(1_i32 ..= 10_000_000),
			};
			try_by_discrete_grad_desc(args);
			// try_by_discrete_grad_dir_desc(args);
			// try_by_diff_evo(args);
		}
	});

	fn try_by_discrete_grad_desc(mut args: Args) {
		let mut diff = eval_diff(args);
		loop {
			let new_args = Args { pipow_t: args.pipow_t+1, ..args };
			let new_diff = eval_diff(new_args);
			if new_diff < diff {
				args = new_args;
				diff = new_diff;
				continue;
			}

			let new_args = Args { pipow_t: args.pipow_t-1, ..args };
			let new_diff = eval_diff(new_args);
			if new_diff < diff {
				args = new_args;
				diff = new_diff;
				continue;
			}

			let new_args = Args { pipow_b: args.pipow_b+1, ..args };
			let new_diff = eval_diff(new_args);
			if new_diff < diff {
				args = new_args;
				diff = new_diff;
				continue;
			}

			let new_args = Args { pipow_b: args.pipow_b-1, ..args };
			let new_diff = eval_diff(new_args);
			if new_diff < diff {
				args = new_args;
				diff = new_diff;
				continue;
			}

			let new_args = Args { numpow_t: args.numpow_t+1, ..args };
			let new_diff = eval_diff(new_args);
			if new_diff < diff {
				args = new_args;
				diff = new_diff;
				continue;
			}

			let new_args = Args { numpow_t: args.numpow_t-1, ..args };
			let new_diff = eval_diff(new_args);
			if new_diff < diff {
				args = new_args;
				diff = new_diff;
				continue;
			}

			let new_args = Args { numpow_b: args.numpow_b+1, ..args };
			let new_diff = eval_diff(new_args);
			if new_diff < diff {
				args = new_args;
				diff = new_diff;
				continue;
			}

			let new_args = Args { numpow_b: args.numpow_b-1, ..args };
			let new_diff = eval_diff(new_args);
			if new_diff < diff {
				args = new_args;
				diff = new_diff;
				continue;
			}

			let new_args = Args { num: args.num+1, ..args };
			let new_diff = eval_diff(new_args);
			if new_diff < diff {
				args = new_args;
				diff = new_diff;
				continue;
			}

			let new_args = Args { num: args.num-1, ..args };
			let new_diff = eval_diff(new_args);
			if new_diff < diff {
				args = new_args;
				diff = new_diff;
				continue;
			}

			let new_args = Args { denom: args.denom+1, ..args };
			let new_diff = eval_diff(new_args);
			if new_diff < diff {
				args = new_args;
				diff = new_diff;
				continue;
			}

			let new_args = Args { denom: args.denom-1, ..args };
			let new_diff = eval_diff(new_args);
			if new_diff < diff {
				args = new_args;
				diff = new_diff;
				continue;
			}

			if diff < DIFF {
				let Args { pipow_t, pipow_b, numpow_t, numpow_b, num, denom } = args;
				println!("pi^({pipow_t}/{pipow_b}) * {num}^({numpow_t}/{numpow_b}) / {denom}\t\tdiff = {diff:.3e}");
			}
			return;
		}
	}

	#[derive(Debug, Clone, Copy)]
	struct Args { pipow_t: i32, pipow_b: i32, numpow_t: i32, numpow_b: i32, num: i32, denom: i32 }
	fn eval_x(Args { pipow_t, pipow_b, numpow_t, numpow_b, num, denom }: Args) -> float {
		let pipow: float = (pipow_t as float) / (pipow_b as float);
		let numpow: float = (numpow_t as float) / (numpow_b as float);
		PI.powf(pipow) * (num as float).powf(numpow) / (denom as float)
	}

	fn eval_diff(args: Args) -> float {
		diff(X, eval_x(args))
	}

	fn check_and_maybe_print(Args { pipow_t, pipow_b, numpow_t, numpow_b, num, denom }: Args) {
		let diff = eval_diff(Args { pipow_t, pipow_b, numpow_t, numpow_b, num, denom });
		if diff < DIFF {
			println!("pi^({pipow_t}/{pipow_b}) * {num}^({numpow_t}/{numpow_b}) / {denom}\t\tdiff = {diff:.3e}");
		}
	}
}



fn search_by_rand() {
	const CORES_N: u32 = 11;
	(0..CORES_N).into_par_iter().for_each(|_| {
		let mut rng = rng();
		loop {
			let pipow_t = rng.random_range(1_u32 ..= 100);
			let pipow_b = rng.random_range(1_u32 ..= 100);
			let numpow_t = rng.random_range(1_u32 ..= 100);
			let numpow_b = rng.random_range(2_u32 ..= 1_000);
			let num   = rng.random_range(1_u32 ..= 10_000);
			let denom = rng.random_range(1_u32 ..= 10_000_000);
			if gcd(pipow_t, pipow_b) != 1 { continue }
			if gcd(numpow_t, numpow_b) != 1 { continue }
			let pipow: float = (pipow_t as float) / (pipow_b as float);
			let numpow: float = (numpow_t as float) / (numpow_b as float);
			let x = PI.powf(pipow) * (num as float).powf(numpow) / (denom as float);
			let diff = diff(X, x);
			if diff < DIFF {
				println!("pi^({pipow_t}/{pipow_b}) * {num}^({numpow_t}/{numpow_b}) / {denom}\t\tdiff = {diff:.3e}");
			}
		}
	});
}



fn search_by_bruteforce() {
	(1_u32 ..= 30).into_par_iter().for_each(|pipow_t| {
		for pipow_b in 2_u32 ..= 30 {
			if gcd(pipow_t, pipow_b) != 1 { continue }
			// if gcd
			println!("progress: pi ^ {pipow_t}/{pipow_b}");
			for numpow_t in 1_u32 ..= 50 {
				for numpow_b in 2_u32 ..= 50 {
					if gcd(numpow_t, numpow_b) != 1 { continue }
					for num in 1_u32 ..= 1_000 { /////////////// inc
						for denom in 1_u32 ..= 100_000 { /////// inc
							let pipow: float = (pipow_t as float) / (pipow_b as float);
							let numpow: float = (numpow_t as float) / (numpow_b as float);
							let x = PI.powf(pipow) * (num as float).powf(numpow) / (denom as float);
							let diff = diff(X, x);
							if diff < DIFF {
								println!("pi^({pipow_t}/{pipow_b}) * {num}^({numpow_t}/{numpow_b}) / {denom}\t\tdiff = {diff:.3e}");
							}
						}
					}
				}
			}
		}
	});
}



fn diff(x: float, y: float) -> float {
	(x-y).abs()
}

#[allow(non_camel_case_types)]
type float = f64;



#[inline]
fn gcd(mut a: u32, mut b: u32) -> u32 {
	// src: https://docs.rs/num-bigint/0.4.6/src/num_bigint/biguint.rs.html#221
	#[inline]
	fn twos(x: u32) -> u32 {
		x.trailing_zeros()
	}

	// Stein's algorithm
	// if a.is_zero() { return b }
	// if b.is_zero() { return a }
	// assert_ne!(0, a);
	// assert_ne!(0, b);

	// find common factors of 2
	let shift = twos(a).min(twos(b));

	// divide `a` and `b` by 2 until odd
	// `a` inside loop
	b >>= twos(b);
	while a != 0 {
		a >>= twos(a);
		if b > a {
			std::mem::swap(&mut b, &mut a)
		}
		a -= &b;
	}
	b << shift
}

#[cfg(test)]
mod gcd_ {
	use super::*;
	#[test]
	fn symetricy() {
		for a in 1..=100 {
			for b in 1..=100 {
				assert_eq!(gcd(a, b), gcd(b, a));
			}
		}
	}
	#[test]
	fn _9_6() {
		assert_eq!(3, gcd(6, 9))
	}
	#[test]
	fn _802496890_82893460() {
		assert_eq!(130, gcd(802496890, 82893460))
	}
}

