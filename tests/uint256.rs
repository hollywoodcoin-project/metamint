extern crate metamint;

use metamint::utils::bignum::{Uint512, Uint256, Zero, One};

#[test]
fn test_zero() {
	assert_eq!(Uint256::zero(), Uint256::from_raw([0; 4]));
	assert_ne!(Uint256::zero(), Uint256::from_raw([1; 4]));

	assert!(Uint256::from_raw([0; 4]).is_zero());
	assert!(!Uint256::from_raw([1, 2, 3, 4]).is_zero());
}

#[test]
fn test_one() {
	assert_eq!(Uint256::one(), Uint256::from_raw([1, 0, 0, 0]));
	assert_ne!(Uint256::one(), Uint256::from_raw([1; 4]));

	assert!(Uint256::from_raw([1, 0, 0, 0]).is_one());
	assert!(!Uint256::from_raw([1, 2, 3, 4]).is_one());
}

#[test]
fn test_max() {
	assert_eq!(Uint256::max(), Uint256::from_raw([0xffffffffffffffff; 4]));
}

#[test]
fn test_is_odd() {
	assert!(!Uint256::from(0).is_odd());
	assert!(Uint256::from(1).is_odd());
	assert!(!Uint256::from(2).is_odd());
	assert!(Uint256::from(467).is_odd());
	assert!(!Uint256::from(954).is_odd());
}

#[test]
fn test_is_even() {
	assert!(Uint256::from(0).is_even());
	assert!(!Uint256::from(1).is_even());
	assert!(Uint256::from(2).is_even());
	assert!(!Uint256::from(753).is_even());
	assert!(Uint256::from(1024).is_even());
}

#[test]
fn test_partial_eq() {
	// max = max
	assert!(Uint256::max() == Uint256::max());

	// Small
	let a = Uint256::from_u64(123456789u64);
	let b = Uint256::from_u64(123456789u64);
	let c = Uint256::from_u64(987654321u64);

	assert!(a == b);
	assert!(a != c);

	// Big
	let a = Uint256::from_raw([0xff98fdf13ceff45f, 0xf3bcc7f3f272feed, 0xff235feebdaedaaa, 0x7ffff3f6f1f600df]);
	let b = Uint256::from_raw([0xff98fdf13ceff45f, 0xf3bcc7f3f272feed, 0xff235feebdaedaaa, 0x7ffff3f6f1f600df]);
	let c = Uint256::from_raw([0x0b1fd4f71b3fb1ab, 0xe89d6c29e9ef895d, 0xcd3766c723dea6cc, 0]);

	assert!(a == b);
	assert!(a != c);
}

#[test]
fn test_partial_ord() {
	let a = Uint256::from_raw([1, 2, 3, 4]);
	let b = Uint256::from_raw([4, 5, 6, 1]);

	assert!(a > b);
	assert!(b < a);
}

#[test]
fn test_cast() {
	let from_u64 = Uint256::from(1000u64);
	let from_i64 = Uint256::from(1000i64);
	let from_u32 = Uint256::from(1000u32);
	let from_i32 = Uint256::from(1000i32);

	assert_eq!(from_u64, from_i64);
	assert_eq!(from_u64, from_u32);
	assert_eq!(from_u64, from_i32);
}

#[test]
fn test_from_uint512() {
	assert_eq!(Uint256::from(Uint512::from_raw([1, 2, 3, 4, 0, 0, 0, 0])), Uint256::from_raw([1, 2, 3, 4]));
}

#[test]
fn test_display() {} // TODO

#[test]
fn test_count_digits() {
	// Note that digits in the big-endian order

	// count_digits(0000) = 1
	assert_eq!(Uint256::zero().count_digits(), 1);

	// count_digits(1111) = 4
	assert_eq!(Uint256::from_raw([1, 1, 1, 1]).count_digits(), 4);

	// count_digits(0001) = 4
	assert_eq!(Uint256::from_raw([0, 0, 0, 1]).count_digits(), 4);

	// count_digits(1000) = 1
	assert_eq!(Uint256::from_raw([1, 0, 0, 0]).count_digits(), 1);

	// count_digits(1100) = 2
	assert_eq!(Uint256::from_raw([1, 1, 0, 0]).count_digits(), 2);

	// count_digits(0011) = 4
	assert_eq!(Uint256::from_raw([0, 0, 1, 1]).count_digits(), 4)
}

#[test]
fn test_add() {
	// 0 + 0 = 0
	assert_eq!(Uint256::zero() + Uint256::zero(), Uint256::zero());

	// 1 + 0 = 1
	assert_eq!(Uint256::zero() + Uint256::from(1), Uint256::from(1));

	// a + b = b + a
	assert_eq!(Uint256::from_raw([0x0b1fd4f71b3fb1ab, 0xe89d6c29e9ef895d, 0xcd3766c723dea6cc, 0])
				   + Uint256::from_raw([0x9facd3f3eeff0071, 0, 0x71afd3f3feff0d71, 0]),
			   Uint256::from_raw([0x9facd3f3eeff0071, 0, 0x71afd3f3feff0d71, 0])
				   + Uint256::from_raw([0x0b1fd4f71b3fb1ab, 0xe89d6c29e9ef895d, 0xcd3766c723dea6cc, 0]));

	// Overflow checks

	// max + 1 = 0
	assert_eq!(Uint256::max() + Uint256::from(1), Uint256::zero());

	// max + 2 = 1
	assert_eq!(Uint256::max() + Uint256::from(2), Uint256::from(1));

	// Partial overflow check
	assert_eq!(
		Uint256::from_raw([0, 0xffffffffffffffff, 1, 0]) + Uint256([0, 1, 0, 0]),
		Uint256::from_raw([0, 0, 2, 0])
	);

	// 0x 162ebcd38c90b56f bff4b0210695afb4 71c944a6003cde34 bbf030a89c42b158
	// +
	// 0x 1089012beb484ddf 3612bdff9fcab867 6fed42c47bf00081 301220c079eedb8d
	// =
	// 0x 26b7bdff77d9034e f6076e20a660681b e1b6876a7c2cdeb5 ec02516916318ce5
	assert_eq!(Uint256::from_raw([0xbbf030a89c42b158, 0x71c944a6003cde34, 0xbff4b0210695afb4, 0x162ebcd38c90b56f])
				   + Uint256::from_raw([0x301220c079eedb8d, 0x6fed42c47bf00081, 0x3612bdff9fcab867, 0x1089012beb484ddf]),
			   Uint256::from_raw([0xec02516916318ce5, 0xe1b6876a7c2cdeb5, 0xf6076e20a660681b, 0x26b7bdff77d9034e]));
}

#[test]
fn test_sub() {
	// 0 - 0 = 0
	assert_eq!(Uint256::zero() - Uint256::zero(), Uint256::zero());

	// 1 - 0 = 0
	assert_eq!(Uint256::from(1) - Uint256::zero(), Uint256::from(1));

	// 1 - 1 = 0
	assert_eq!(Uint256::from(1) - Uint256::from(1), Uint256::zero());

	// max - max = 0
	assert_eq!(Uint256::max() - Uint256::max(), Uint256::zero());

	// Overflow checks

	// min - 1 = max
	assert_eq!(Uint256::zero() - Uint256::from(1), Uint256::max());

	//min - 2 = max - 1
	assert_eq!(Uint256::zero() - Uint256::from(2), Uint256::max() - Uint256::from(1));

	// Partial overflow check
	assert_eq!(Uint256::from_raw([0, 1, 1, 0]) - Uint256::from_raw([0, 2, 0, 0]),
			   Uint256::from_raw([0, 0xffffffffffffffff, 0, 0]));

	// (2^256 - 1) - 2^255 = 7fffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
	assert_eq!(Uint256::max() - Uint256::from_raw([0, 0, 0, 0x8000000000000000]),
			   Uint256([0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0x7fffffffffffffff]));

	// 0x 162ebcd38c90b56f bff4b0210695afb4 71c944a6003cde34 bbf030a89c42b158
	// -
	// 0x 1089012beb484ddf 3612bdff9fcab867 6fed42c47bf00081 301220c079eedb8d
	// =
	// 0x 05a5bba7a1486790 89e1f22166caf74d 01dc01e1844cddb3 8bde0fe82253d5cb
	assert_eq!(Uint256::from_raw([0xbbf030a89c42b158, 0x71c944a6003cde34, 0xbff4b0210695afb4, 0x162ebcd38c90b56f])
				   - Uint256::from_raw([0x301220c079eedb8d, 0x6fed42c47bf00081, 0x3612bdff9fcab867, 0x1089012beb484ddf]),
			   Uint256::from_raw([0x8bde0fe82253d5cb, 0x01dc01e1844cddb3, 0x89e1f22166caf74d, 0x05a5bba7a1486790]));
}

#[test]
fn test_mul() {
	// 0 * 0 = 0
	assert_eq!(Uint256::zero() * Uint256::zero(), Uint256::zero());

	// 0 * 1 = 0
	assert_eq!(Uint256::zero() * Uint256::from(1), Uint256::zero());

	// 1 * 1 = 1
	assert_eq!(Uint256::from(1) * Uint256::from(1), Uint256::from(1));

	// 1 * max = max
	assert_eq!(Uint256::from(1) * Uint256::max(), Uint256::max());

	// max(u64) * max(u64) = 0x fffffffffffffffe 0000000000000001
	assert_eq!(Uint256::from(u64::max_value()) * Uint256::from(u64::max_value()),
			   Uint256([1, 0xfffffffffffffffe, 0, 0]));

	// a * b = b * a
	assert_eq!(Uint256::from(u64::max_value() / 2) * Uint256::from(u64::max_value()),
			   Uint256::from(u64::max_value()) * Uint256::from(u64::max_value() / 2));

	// 0x 010695afb4 71c944a6003cde34 bbf030a89c42b158
	// *
	// 0x 81 301220c079eedb8d
	// =
	// 0x 8482bc32fd a9bd6aa2e336b156 226dd34476752d65 6be7d85e9641f578
	assert_eq!(Uint256::from_raw([0xbbf030a89c42b158, 0x71c944a6003cde34, 0x010695afb4, 0])
				   * Uint256::from_raw([0x301220c079eedb8d, 0x81, 0, 0]),
			   Uint256::from_raw([0x6be7d85e9641f578, 0x226dd34476752d65, 0xa9bd6aa2e336b156, 0x8482bc32fd]));
}

#[test]
fn test_short_div() {
	// 0 / 1 = 0
	assert_eq!(Uint256::zero() / 1, Uint256::zero());

	// a / 1 = a
	assert_eq!(Uint256::from_raw([1, 0xfffffffffffffffe, 0, 0]) / 1, Uint256::from_raw([1, 0xfffffffffffffffe, 0, 0]));

	// a * 2 / 2 = a
	assert_eq!(Uint256::from(u64::max_value()) * Uint256::from(2) / 2, Uint256::from(u64::max_value()));

	// if a < b then a / b = 0
	// a: 9223372036854775807
	// b: 17233868258737669948
	assert_eq!(Uint256::from(9223372036854775807u64) / 17233868258737669948u64, Uint256::zero());

	assert_eq!(Uint256([18446744073709551615u64, 9223372036854775807u64, 0, 0]) / 17233868258737669948u64,
			   Uint256::from(9872489501839302786u64));

	// 0x 7fffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
	// /
	// 0x 301220c079eedb8d
	// =
	// 0x 2 a9a93a16a8441992 ff729502429ee73d d7736c4d797ceaf5
	assert_eq!(Uint256::from_raw([0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0x7fffffffffffffff])
				   / 0x301220c079eedb8du64,
			   Uint256::from_raw([0xd7736c4d797ceaf5, 0xff729502429ee73d, 0xa9a93a16a8441992, 0x2]));
}

#[test]
fn test_long_div() {
	// If divisor is one-digit, short division should be used
	assert_eq!(Uint256::from_raw([1, 0xfffffffffffffffe, 0, 0]) / Uint256::from(1),
			   Uint256::from_raw([1, 0xfffffffffffffffe, 0, 0]));

	// a / a = 1
	assert_eq!(
		Uint256::from_raw([0x0000000000000001, 0xFFFFFFFFFFFFFFFE, 0u64, 0u64])
			/ Uint256::from_raw([0x0000000000000001, 0xFFFFFFFFFFFFFFFE, 0u64, 0u64]),
		Uint256::from(1));

	// max / max = 1
	assert_eq!(Uint256::max() / Uint256::max(), Uint256::from(1));

	// a / max = 0, where a != max
	assert_eq!(Uint256::from_raw([1, 0xfffffffffffffffe, 0, 0]) / Uint256::max(), Uint256::zero());

	// a / b = c

	// 0x 7fffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
	// divide by
	// 0x 0000000000000000 ef2affdf53f0473c ebdcf3dfd3572173 113cf3ffdfff0071
	// equal to
	// 0x 890220e52de7a082
	assert_eq!(Uint256::from_raw([0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0x7fffffffffffffff])
				   / Uint256::from_raw([0x113cf3ffdfff0071, 0xebdcf3dfd3572173, 0xef2affdf53f0473c, 0]),
			   Uint256::from(0x890220e52de7a082u64));

	// 0x 7ffff3f6f1f600df ff235feebdaedaaa f3bcc7f3f272feed ff98fdf13ceff45f
	// divide by
	// 0x 9facd3f3eeff0071
	// equal to
	// 0x cd3766c723dea6cc e89d6c29e9ef895d 0b1fd4f71b3fb1ab
	assert_eq!(Uint256::from_raw([0xff98fdf13ceff45f, 0xf3bcc7f3f272feed, 0xff235feebdaedaaa, 0x7ffff3f6f1f600df])
				   / Uint256::from_raw([0x9facd3f3eeff0071, 0, 0, 0]),
			   Uint256::from_raw([0x0b1fd4f71b3fb1ab, 0xe89d6c29e9ef895d, 0xcd3766c723dea6cc, 0]));

	// 0x 7ffff3f6f1f600df ff235feebdaedaaa f3bcc7f3f272feed ff98fdf13ceff45f
	// /
	// 0x 9facd3f3eeff0071 0dfff235feebdaed aaa2723f60dffff1
	// =
	// 0x cd3766c723dea6cc
	assert_eq!(
		Uint256::from_raw([0xff98fdf13ceff45f, 0xf3bcc7f3f272feed, 0xff235feebdaedaaa, 0x7ffff3f6f1f600df])
		/ Uint256::from_raw([0xaaa2723f60dffff1, 0x0dfff235feebdaed, 0x9facd3f3eeff0071, 0]),
		Uint256::from(0xcd3766c723dea6ccu64)
	);
}