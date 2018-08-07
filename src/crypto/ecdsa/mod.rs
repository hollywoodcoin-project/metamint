use utils::bignum::{Int512, Uint512, Uint256, Zero, One};
use std::ops::{Add, Mul};
use std::fmt;

/// Elliptic curve over finite field.
pub struct EllipticCurve<'a> {
	/// The modulo that defines finite field.
	modulo: Uint256,

	/// The `a` and `b` curve parameters.
	params: (Uint256, Uint256),

	/// Generation point.
	gen_point: ECPoint<'a>
}

impl<'a> EllipticCurve<'a> {
	pub fn modulo(&self) -> &Uint256 {
		&self.modulo
	}

	pub fn a(&self) -> &Uint256 {
		&self.params.0
	}

	pub fn b(&self) -> &Uint256 {
		&self.params.1
	}

	pub fn gen_point(&self) -> &ECPoint {
		&self.gen_point
	}

	/// Creates point on this curve.
	pub fn create_point(&self, x: Uint256, y: Uint256) -> ECPoint {
		ECPoint { x, y, curve: self }
	}

	pub fn try_create_point(&self, x: Uint256, y: Uint256) -> Option<ECPoint> {

		unimplemented!() // TODO: implement
	}
}

/// Secp256k1.
pub static SECP256K1: EllipticCurve<'static> = EllipticCurve {
	modulo: Uint256([0xfffffffefffffc2f, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff]),
	params: (Uint256([0; 4]), Uint256([7, 0, 0, 0])),
	gen_point: ECPoint {
		x: Uint256([0x59f2815b16f81798, 0x029bfcdb2dce28d9, 0x55a06295ce870b07, 0x79be667ef9dcbbac]),
		y: Uint256([0x9c47d08ffb10d4b8, 0xfd17b448a6855419, 0x5da4fbfc0e1108a8, 0x483ada7726a3c465]),
		curve: &SECP256K1
	}
};

fn clock_add(a: &Uint256, b: &Uint256, p: &Uint256) -> Uint256 {
	let mut res = Uint512::from(*a) + Uint512::from(*b);
	let p = Uint512::from(*p);
	if res > p { res = res - p; }
	Uint256::from(res)
}

fn clock_sub(a: &Uint256, b: &Uint256, p: &Uint256) -> Uint256 {
	if a < b {
		*p - (*b - *a)
	} else {
		*a - *b
	}
}

fn clock_mul(a: &Uint256, b: &Uint256, p: &Uint256) -> Uint256 {
	let mut res = Uint512::from(*a) * Uint512::from(*b);
	let p = Uint512::from(*p);

	if res > p {
		let div = res / p;
		res = res - div * p;
	}

	Uint256::from(res)
}

fn clock_square(a: &Uint256, p: &Uint256) -> Uint256 {
	clock_mul(a, a, p)
}

fn clock_div(a: &Uint256, b: &Uint256, p: &Uint256) -> Uint256 {
	// Cast `p`
	let p_i = Int512::from(Uint512::from(*p));

	// Compute multiplicative inverse of the b (b^(-1))

	let (mut t, mut nt, mut r, mut nr) = (Int512::zero(), Int512::from(1), p_i, Int512::from(Uint512::from(*b)));

	while nr != Int512::zero() {
		let q = r / nr;

		let bt = t;
		t = nt;
		nt = bt - q * nt;

		let br = r;
		r = nr;
		nr = br - q * nr;
	}

	if r > Int512::from(1) {
		panic!("Cannot invert 'a'.");
	}

	if t < Int512::zero() {
		t = t + p_i;
	}

	// Multiply: a * b^(-1) (mod p)
	clock_mul(a, &Uint256::from(Uint512::from(t)), p)
}

/// Point on the elliptic curve.
#[derive(Clone, Copy)]
pub struct ECPoint<'a> {
	/// The `x` coordinate of the point.
	x: Uint256,

	/// The `y` coordinate of the point.
	y: Uint256,

	/// The curve that associated to the point.
	curve: &'a EllipticCurve<'a>
}

impl<'a> ECPoint<'a> {
	pub fn new(x: Uint256, y: Uint256, curve: &'a EllipticCurve) -> Self {
		ECPoint { x, y, curve }
	}

	/// Returns `x` component of the elliptic curve point.
	pub fn x(&self) -> &Uint256 { &self.x }

	/// Returns `y` component of the elliptic curve point.
	pub fn y(&self) -> &Uint256 { &self.y }

	/// Returns associated elliptic `curve`.
	pub fn curve(&self) -> &EllipticCurve { &self.curve }

	/// Doubles the point.
	pub fn double(&self) -> Self {
		let mut res = ECPoint { x: Uint256::zero(), y: Uint256::zero(), curve: self.curve };
		let p = self.curve.modulo();
		let a = self.curve.params.0;
		let (x, y) = (self.x, self.y);

		// Calc `X`
		let xp2 = clock_square(&x, p);						// x^2
		let xp2m3 = clock_mul(&xp2, &Uint256::from(3), p);	// 3x^2
		let xaa = clock_add(&xp2m3, &a, p);					// 3x^2 + a
		let ym2 = clock_mul(&y, &Uint256::from(2), p);		// 2y
		let div = clock_div(&xaa, &ym2, p);					// (3x^2 + a) / 2y
		let divp2 = clock_square(&div, p);					// ((3x^2 + a) / 2y)^2
		let xm2 = clock_mul(&x, &Uint256::from(2), p);		// 2x
		res.x = clock_sub(&divp2, &xm2, p);					// ((3x^2 + a) / 2y)^2 - 2x

		// Calc `Y`
		let x1sx3 = clock_sub(&x, &res.x, p);				// x_1 - x_3
		let dmx1s3 = clock_mul(&div, &x1sx3, p);			// ((3x^2 + a) / 2y) * (x_1 - x_3)
		res.y = clock_sub(&dmx1s3, &y, p);					// ((3x^2 + a) / 2y) * (x_1 - x_3) - y

		res
	}
}

impl<'a> fmt::Debug for ECPoint<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "({:?}, {:?})", self.x, self.y)?;
		Ok(())
	}
}

impl<'a> PartialEq for ECPoint<'a> {
	fn eq(&self, other: &ECPoint) -> bool {
		self.x == other.x && self.y == other.y && ::std::ptr::eq(self.curve, other.curve)
	}
}

impl<'a> Add for ECPoint<'a> {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		let mut res = ECPoint { x: Uint256::zero(), y: Uint256::zero(), curve: self.curve };
		let p = self.curve.modulo();
		let (x1, y1) = (self.x, self.y);
		let (x2, y2) = (other.x, other.y);

		// Calc `X`
		let y2sy1 = clock_sub(&y2, &y1, p);			// y_2 - y_1
		let x2sx1 = clock_sub(&x2, &x1, p); 		// x_2 - x_1
		let div = clock_div(&y2sy1, &x2sx1, p);		// (y_2 - y_1) / (x_2 - x_1)
		let dp2 = clock_square(&div, p);			// ((y_2 - y_1) / (x_2 - x_1))^2
		let dp2sx1 = clock_sub(&dp2, &x1, p);		// ((y_2 - y_1) / (x_2 - x_1))^2 - x_1
		res.x = clock_sub(&dp2sx1, &x2, p);			// ((y_2 - y_1) / (x_2 - x_1))^2 - x_1 - x_2

		// Calc `Y`
		let x1sx3 = clock_sub(&x1, &res.x, p);		// x_1 - x_3
		let dmx1sx3 = clock_mul(&div, &x1sx3, p);	// (y_2 - y_1) / (x_2 - x_1) * (x_1 - x_3)
		res.y = clock_sub(&dmx1sx3, &y1, p);		// (y_2 - y_1) / (x_2 - x_1) * (x_1 - x_3) - y_1

		res
	}
}

impl<'a> Mul<Uint256> for ECPoint<'a> {
	type Output = Self;

	fn mul(self, other: Uint256) -> Self {
		if other == Uint256::zero() {
			panic!("Cannot multiply EC point by zero.");
		} else if other == Uint256::one() {
			self
		} else if other == Uint256::from(2) {
			self.double()
		} else if other.is_odd() {
			self * (other - Uint256::from(1)) + self
		} else { // even
			(self * (other / Uint256::from(2))).double()
		}
	}
}

#[test]
fn test_clock_add() {
	let p = SECP256K1.modulo;

	// 0x ffffffffffffffff ffffffffffffffff ffffffffffffffff fffffffeaffffc2f
	// +
	// 0x 162ebcd38c90b56f bdb4b0390695afb4 71c944a6003cb334 bbf030a89c42b584
	// =
	// 0x 162ebcd38c90b56f bdb4b0390695afb4 71c944a6003cb334 bbf030a84c42b584
	// mod p

	assert_eq!(
		clock_add(&Uint256::from_raw([0xfffffffeaffffc2f, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff]),
				  &Uint256::from_raw([0xbbf030a89c42b584, 0x71c944a6003cb334, 0xbdb4b0390695afb4, 0x162ebcd38c90b56f]), &p),
		Uint256::from_raw([0xbbf030a84c42b584, 0x71c944a6003cb334, 0xbdb4b0390695afb4, 0x162ebcd38c90b56f])
	);
}

#[test]
fn test_clock_sub() {
	let p = SECP256K1.modulo;

	// 0x 162ebcd38c90b56f bdb4b0390695afb4 71c944a6003cb334 bbf030a89c42b584
	// -
	// 0x ffffffffffffffff ffffffffffffffff ffffffffffffffff fffffffeaffffc2f
	// =
	// 0x 162ebcd38c90b56f bdb4b0390695afb4 71c944a6003cb334 bbf030a8ec42b584
	// mod p

	assert_eq!(
		clock_sub(&Uint256::from_raw([0xbbf030a89c42b584, 0x71c944a6003cb334, 0xbdb4b0390695afb4, 0x162ebcd38c90b56f]),
				  &Uint256::from_raw([0xfffffffeaffffc2f, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff]), &p),
		Uint256::from_raw([0xbbf030a8ec42b584, 0x71c944a6003cb334, 0xbdb4b0390695afb4, 0x162ebcd38c90b56f])
	);
}

#[test]
fn test_clock_mul() {
	let p = SECP256K1.modulo;

	// 0x 162ebcd38c90b56f bdb4b0390695afb4 71c944a6003cb334 bbf030a89c42b584
	// -
	// 0x ffffffffffffffff ffffffffffffffff ffffffffffffffff fffffffeaffffc2f
	// =
	// 0x e412c74d14b788ee 2df139179c711a8c 1fed07ff8544f0cb 483c8c294b62698d
	// mod p

	assert_eq!(
		clock_mul(&Uint256::from_raw([0xbbf030a89c42b584, 0x71c944a6003cb334, 0xbdb4b0390695afb4, 0x162ebcd38c90b56f]),
				  &Uint256::from_raw([0xfffffffeaffffc2f, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff]), &p),
		Uint256::from_raw([0x483c8c294b62698d, 0x1fed07ff8544f0cb, 0x2df139179c711a8c, 0xe412c74d14b788ee])
	);
}

#[test]
fn test_clock_div() {
	let p = SECP256K1.modulo;

	let div = clock_div(
		&Uint256::from_raw([0xaa7f067e28fef8ac, 0xaa2f64be71462131, 0x42fd096d2f1f7fd9, 0x8ff2b776aaf6d919]),
		&Uint256::from_raw([0x388fa11ff621a970, 0xfa2f68914d0aa833, 0xbb49f7f81c221151, 0x9075b4ee4d4788ca]),
		&p
	);

	assert_eq!(div, Uint256::from_raw([0x36d3aebbeddcd1b1, 0xf58857c2f631ee69, 0x3eb9d1235992ac63, 0xcb35b28428101a30]));
}