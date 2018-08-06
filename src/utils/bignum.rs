use std::fmt;
use std::str::FromStr;
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::cmp::Ordering;

/// Zero.
pub trait Zero: Sized {
	/// Returns zero.
	fn zero() -> Self;

	/// Returns `true` if value is equal to zero.
	fn is_zero(&self) -> bool;
}

/// One.
pub trait One: Sized {
	/// Returns one.
	fn one() -> Self;

	/// Returns `true` if value is equal to one.
	fn is_one(&self) -> bool;
}

// TODO: write doc comment
///
fn lo(x: u64) -> u32 { (x & ((1u64 << 32) - 1)) as u32 }

// TODO: write doc comment
///
fn hi(x: u64) -> u32 { (x >> 32) as u32 }

// TODO write doc comment
///
fn item_or_zero(slice: &[u64], i:usize) -> u64 { if i < slice.len() { slice[i] } else { 0 } }

// TODO: write doc comment
///
fn mul_u64_carry(a: u64, b: u64) -> (u64, u64) {
	// Every `s` represents 32bit part of the resulting number
	let (s0, mut s1, mut s2, s3);

	let mut x = lo(a) as u64 * lo(b) as u64;
	s0 = lo(x);

	x = hi(a) as u64 * lo(b) as u64 + hi(x) as u64;
	s1 = lo(x);
	s2 = hi(x);

	x = lo(a) as u64 * hi(b) as u64 + s1 as u64;
	s1 = lo(x);

	x = hi(a) as u64 * hi(b) as u64 + s2 as u64 + hi(x) as u64;
	s2 = lo(x);
	s3 = hi(x);

	// result -> merge `s0` and `s1`
	let result = (s1 as u64) << 32 | s0 as u64;

	// carry -> merge `s2` and `s3`
	let carry = (s3 as u64) << 32 | s2 as u64;

	(result, carry)
}

/// Compares two big unsigned integers in raw big-endian format
fn bn_raw_cmp(a: &[u64], b: &[u64]) -> Ordering {
	for i in (0..::std::cmp::max(a.len(), b.len())).rev() {
		let r = item_or_zero(a, i).cmp(&item_or_zero(b, i));
		if r != Ordering::Equal {
			return r;
		}
	}

	Ordering::Equal
}

// Low level operations for the big numbers
macro_rules! bn_op {
	(__impl $op_method:ident, $a:expr, $b:expr, $res_size:expr) => {{
		let mut result = [0u64; $res_size];
		let mut overflow = 0u8;

		for i in 0..$res_size {
			let (mut r, o) = (item_or_zero($a, i)).$op_method(item_or_zero($b, i));

			if overflow > 0 {
				// Check for oveflow because `r` may contain max value of u64
				let (ro, oo) = r.$op_method(overflow as u64);
				r = ro;
				overflow = if oo { 1 } else { 0 };
			}

			result[i] = r;

			// Do not check for bounds cause it's checked by 'for' cycle
			if o { overflow += 1; }
		}

		result
	}};

	(add $a:expr, $b:expr, $res_size:expr) => {{
		bn_op!(__impl overflowing_add, $a, $b, $res_size)
	}};

	(sub $a:expr, $b:expr, $res_size:expr) => {{
		bn_op!(__impl overflowing_sub, $a, $b, $res_size)
	}};

	// Long multiplication algorithm
	(mul $a:expr, $b:expr, $res_size:expr) => {{
		fn add_carry(r: &mut [u64], i: usize, c: u64) {
			let mut overflow = true;
			let (mut i, mut c) = (i, c);

			while i < r.len() && overflow {
				let (rt, o) = r[i].overflowing_add(c);
				r[i] = rt;
				if o { c = 1; }
				overflow = o;
				i += 1;
			}
		}

		let mut result = [0u64; $res_size];

		for col in 0..$res_size {
			for row in 0..col + 1 {
				// Calculate a * b with `carry`
				let (r, c) = mul_u64_carry(item_or_zero($a, col - row), item_or_zero($b, row));
				add_carry(&mut result, col + 1, c);

				// Add multiply result to the result
				let (ar, ao) = result[col].overflowing_add(r);
				result[col] = ar;

				if ao {
					add_carry(&mut result, col + 1, 1);
				}
			}
		}

		result
	}};

	// Short division algorithm
	// Returns `(q, r)`, where q - division result, r - reminder
	(short_div $a:expr, $b:expr, $res_size:expr) => {{
		let mut q = [0u64; $res_size];	// result
		let mut r = 0u128;				// reminder
		let b = $b.clone() as u128;

		// i ∈ [n-1; 0]
		for i in (0..$res_size).rev() {
			// t = (rb + U_j)
			let t = ($a[i] as u128) + (r * (u64::max_value() as u128 + 1));

			// q_i = t / V
			q[i] = (t / b) as u64;

			// r = t mod V
			r = t % b;
		}

		(q, r as u64)
	}};
}

macro_rules! impl_bignum {
	($name:ident, $size:expr) => {
		// TODO: make this constructor private when `const fn` feature will be stable
		#[derive(Clone, Copy)]
		pub struct $name(pub [u64; $size]);

		impl $name {
			pub fn from_u64(num: u64) -> Self {
				let mut arr = [0; $size];
				arr[0] = num;
				$name(arr)
			}

			pub fn from_i64(num: i64) -> Self {
				let mut arr = [0; $size];
				arr[0] = if num < 0 { -num } else { num } as u64;
				$name(arr)
			}

			/// Returns raw `u64` digits in the big-endian format.
			pub fn raw(&self) -> &[u64; $size] {
				&self.0
			}

			pub fn from_raw(arr: [u64; $size]) -> Self {
				$name(arr)
			}

			/// Returns maximal value.
			pub fn max() -> Self {
				$name([0xFFFFFFFFFFFFFFFF; $size])
			}

			/// Returns minimal value.
			pub fn min() -> Self {
				$name([0; $size])
			}

			/// Counts digits in the number.
			pub fn count_digits(&self) -> usize {
				for i in (0..self.0.len()).rev() {
					if self.0[i] > 0 {
						return i + 1;
					}
				}

				1
			}

			/// Returns `true` if number is odd.
			pub fn is_odd(&self) -> bool {
				self.0[0] & 1 == 1
			}

			/// Returns `true` if number is even.
			pub fn is_even(&self) -> bool {
				self.0[0] & 1 == 0
			}
		}

		impl Default for $name {
			fn default() -> Self {
				$name([0; $size])
			}
		}

		impl fmt::Debug for $name {
			/// Ouptut number in the hex format
			fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
				let &$name(ref data) = self; // TODO: change
				write!(f, "0x")?;

				for ch in data.iter().rev() {
					write!(f, "{:016x}", ch)?;
				}

				Ok(())
			}
		}

		impl fmt::Display for $name {
			// TODO: works wrong, need to fix this
			fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
				let &$name(ref data) = self; // TODO: change

				for num in data.iter().rev() {
					if *num != 0u64 {
						write!(f, "{}", num);
					}
				}

				Ok(())
			}
		}

		impl From<u64> for $name {
			fn from(num: u64) -> Self {
				Self::from_u64(num)
			}
		}

		impl From<i64> for $name {
			fn from(num: i64) -> Self { Self::from_i64(num) }
		}

		impl From<u32> for $name {
			fn from(num: u32) -> Self { Self::from(num as u64) }
		}

		impl From<i32> for $name {
			fn from(num: i32) -> Self { Self::from(num as i64) }
		}

		impl FromStr for $name {
			type Err = ();
			fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
				unimplemented!()
			}
		}

		impl PartialEq for $name {
			fn eq(&self, other: &Self) -> bool {
				for i in 0..$size {
					if self.0[i] != other.0[i] {
						return false;
					}
				}

				true
			}
		}

		impl Eq for $name { }

		impl PartialOrd for $name {
			fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(bn_raw_cmp(&self.0, &other.0)) }
		}

		impl Ord for $name {
			fn cmp(&self, other: &Self) -> Ordering { bn_raw_cmp(&self.0, &other.0) }
		}

		impl Add for $name {
			type Output = Self;
			fn add(self, other: Self) -> Self { $name(bn_op!(add &self.0, &other.0, $size)) }
		}

		impl Sub for $name {
			type Output = Self;
			fn sub(self, other: Self) -> Self { $name(bn_op!(sub &self.0, &other.0, $size)) }
		}

		impl Mul for $name {
			type Output = Self;
			fn mul(self, other: Self) -> Self { $name(bn_op!(mul &self.0, &other.0, $size)) }
		}

		impl Div for $name {
			type Output = Self;

			// Division by using `Algorithm D` from the book "The Art of Computer Programming" (4.3.1)
			fn div(self, other: Self) -> Self {
				if other > self {
					return Self::zero();
				}

				if other == Self::zero() {
					panic!("The divisor should not be zero.");
				}

				let n = other.count_digits();
				let m = self.count_digits() - n;

				// Use `short division` algorithm if divisor is one-digit
				if n == 1 {
					let (r, _) = bn_op!(short_div &self.0, other.0[0], $size);
					return $name(r);
				}

				let mut result = [0u64; $size];

				// D1 (Normalize)

				let b = [0, 1];

				// d = floor((b-1) / V_(n-1))
				let halfb = u64::max_value() / 2 + 1;
				let d = if other.0[n - 1] > halfb {
					u64::max_value() / other.0[n - 1]
				} else {
					halfb / other.0[n - 1]
				};

				// (U_(m+n) U_(m+n-1) ... U_1 U_0) = (U_(m+n-1) ... U_1 U_0) * d
				let mut u = bn_op!(mul &self.0, &[d], $size + 1);
				// (V_(n-1) ... V_1 V_0) = (V_(n-1) ... V_1 V_0) * d
				let v = bn_op!(mul &other.0, &[d], $size);

				// D2 (Initialize i)
				// i ∈ [m; 0]
				for i in (0..m + 1).rev() {
					// D3 (Calculate qs)

					// qs = floor((U_(i+n)b + U_(i+n-1)) / V_(n-1))
					// rs = (U_(i+n)b + U_(i+n-1)) mod V_(n-1)
					// qs consist of only two digits
					let (mut qs, mut rs) = bn_op!(short_div &u[n + i - 1 .. n + i + 1], v[n - 1], 2);

					// if qs = b or qs * V_(n-2) > b * rs + U_(n+i-2)
					while bn_raw_cmp(&qs, &b) == Ordering::Equal || bn_raw_cmp(&bn_op!(mul &qs, &[v[n - 2]], $size), &[u[n + i - 2], rs]) == Ordering::Greater {
						// qs = qs - 1
						qs = bn_op!(sub &qs, &[1], 2);
						// rs = rs + V_(n-1)
						let (trs, o) = rs.overflowing_add(v[n - 1]);
						// test rs < b
						if o { break; }
						rs = trs;
					}

					// D4 (Multiply and subtract)

					// qs * (V_(n-1) ... V_1 V_0)
					let mul = bn_op!(mul &v, &qs, $size + 1);
					// (U_(i+n) U_(i+n-1) ... U_i)
					let ui = &mut u[i..n + i + 1];
					// Is result of ui - mul negative
					let negative = bn_raw_cmp(ui, &mul) == Ordering::Less;
					// (U_(i+n) U_(i+n - 1) ... U_i) - qs * (V_(n-1) ... V_1 V_0)
					let repl = &bn_op!(sub ui, &mul, $size)[0..n];

					// Replace digits
					for j in 0..n {
						ui[j] = repl[j];
					}

					// D5. q_i = qs
					result[i] = qs[0];

					// TODO: need test this branch!
					// D6 (Add back)
					if negative {
						// q_i = q_i - 1
						result[i] -= 1;
						// (U_(i+n) U_(i+n - 1) ... U_i) + (0 V_(n-1) ... V_1 V_0)
						let sum = &bn_op!(add ui, &v, $size)[0..n];
						// Replace digits
						for j in 0..n {
							ui[j] = sum[j];
						}
					}
				}

				$name(result)
			}
		}

		impl Div<u64> for $name {
			type Output = Self;
			fn div(self, other: u64) -> Self {
				if other == 0 {
					panic!("The divisor should not be zero.");
				}

				let (q, _) = bn_op!(short_div &self.0, &other, $size);
				$name(q)
			}
		}

		impl Zero for $name {
			fn zero() -> Self {
				$name([0; $size])
			}

			fn is_zero(&self) -> bool {
				for part in self.0.iter() {
					if *part != 0 {
						return false;
					}
				}

				true
			}
		}

		impl One for $name {
			fn one() -> Self {
				let mut r = $name::zero();
				r.0[0] = 1;
				r
			}

			fn is_one(&self) -> bool {
				if self.0[0] == 1 {
					for i in 1..$size {
						if self.0[i] != 0 {
							return false;
						}
					}
				}

				true
			}
		}
	}
}

macro_rules! sign_wrap {
	($name:ident, $base:ident) => {
		#[derive(Eq, Clone, Copy)]
		pub struct $name {
			num: $base,
			positive: bool
		}

		impl $name {
			pub fn positive(&self) -> bool {
				self.positive
			}

			pub fn negative(&self) -> bool {
				!self.positive
			}

			pub fn from_raw(num: $base, positive: bool) -> Self {
				$name {num, positive}
			}
		}

		impl fmt::Debug for $name {
			fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
				write!(f, "{}{:?}", if self.positive() { "" } else { "-" }, self.num)?;
				Ok(())
			}
		}

		impl From<$base> for $name {
			fn from(n: $base) -> Self {
				Self::from_raw(n, true)
			}
		}

		impl From<$name> for $base {
			fn from(n: $name) -> $base {
				if n.negative() {
					panic!("Value should be positive.");
				}

				n.num
			}
		}

		impl From<u64> for $name {
			fn from(num: u64) -> Self {
				Self::from_raw($base::from(num), true)
			}
		}

		impl From<i64> for $name {
			fn from(num: i64) -> Self {
				Self::from_raw($base::from(num.abs() as u32), num >= 0)
			}
		}

		impl From<u32> for $name {
			fn from(num: u32) -> Self { Self::from(num as u64) }
		}

		impl From<i32> for $name {
			fn from(num: i32) -> Self { Self::from(num as i64) }
		}

		impl Neg for $name {
			type Output = Self;
			fn neg(self) -> Self {
				Self::from_raw(self.num, !self.positive)
			}
		}

		impl PartialEq for $name {
			fn eq(&self, other: &Self) -> bool {
				self.num.is_zero() && other.num.is_zero() || self.positive == other.positive && self.num.eq(&other.num)
			}
		}

		impl PartialOrd for $name {
			fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
		}

		impl Ord for $name {
			fn cmp(&self, other: &Self) -> Ordering {
				use self::Ordering::*;

				let a_is_zero = self.num.is_zero();

				// a != 0 and sign(a) != sign(b)
				// We do this step first because we dont need to compare actual numbers, only signs
				if !a_is_zero && self.positive() != other.positive() {
					return if self.positive() { Greater } else { Less }
				}

				let num_ord = self.num.cmp(&other.num);

				// a = 0
				if a_is_zero {
					return if num_ord == Equal { num_ord } else { if other.positive() { Less } else { Greater } };
				}

				// sign(a) == sign(b)
				if num_ord == Equal || self.positive() { num_ord } else { num_ord.reverse() }
			}
		}

		impl Add for $name {
			type Output = Self;
			fn add(self, other: Self) -> Self {
				// a = 0
				if self.num.is_zero() {
					return if other.num.is_zero() { Self::zero() } else { other };
				}

				// sign(a) = sign(b)
				if self.positive() == other.positive() {
					return Self::from_raw(self.num + other.num, self.positive());
				}

				// sign(a) != sign(b)
				if self.num < other.num {
					Self::from_raw(other.num - self.num, other.positive())
				} else {
					Self::from_raw(self.num - other.num, self.positive())
				}
			}
		}

		impl Sub for $name {
			type Output = Self;
			fn sub(self, other: Self) -> Self {
				self + -other
			}
		}

		impl Mul for $name {
			type Output = Self;
			fn mul(self, other: Self) -> Self {
				Self::from_raw(self.num * other.num, self.positive() == other.positive())
			}
		}

		impl Div for $name {
			type Output = Self;
			fn div(self, other: Self) -> Self {
				Self::from_raw(self.num / other.num, self.positive() == other.positive())
			}
		}

		impl Zero for $name {
			fn zero() -> Self {
				Self::from_raw($base::zero(), true)
			}

			fn is_zero(&self) -> bool {
				self.num.is_zero()
			}
		}
	}
}

/// Number of the `u64` digits of the `Uint256`.
const U256_SIZE: usize = 4;

/// Number of the `u64` digits of the `Uint512`.
const U512_SIZE: usize = 8;

impl_bignum!(Uint256, U256_SIZE);
impl_bignum!(Uint512, U512_SIZE);
sign_wrap!(Int512, Uint512);

impl From<Uint256> for Uint512 {
	fn from(n: Uint256) -> Self {
		let mut arr = [0u64; U512_SIZE];

		for (i, num) in n.0.iter().enumerate() {
			arr[i] = *num;
		}

		Uint512(arr)
	}
}

impl From<Uint512> for Uint256 {
	fn from(n: Uint512) -> Self {
		if n.count_digits() > U256_SIZE {
			panic!("Passed number is bigged than maximum value of the Uint256.");
		}

		let mut arr = [0u64; U256_SIZE];

		for i in 0..U256_SIZE {
			arr[i] = n.0[i];
		}

		Uint256(arr)
	}
}