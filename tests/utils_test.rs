extern crate metamint;

use metamint::utils::uint256::{Uint256, Zero};




#[test]
fn test() {
//	let val = Uint256::from_u64(12);
//	println!("hex: {:?}", val);
//	println!("dec: {}", val);
//
//	println!("b: {:?}", Uint256::from(u64::max_value()) + Uint256::from(1));
//	println!("b: {:?}", Uint256([0, 1, 0, 0]));
//
//	assert_eq!(Uint256::from(u64::max_value()) + Uint256::from(1), Uint256([0, 1, 0, 0]));
}

#[test]
fn test_partial_eq() {
	// Small
	let mut a = Uint256::from_u64(123456789u64);
	let mut b = Uint256::from_u64(123456789u64);
	let c = Uint256::from_u64(987654321u64);

	assert_eq!(a, b);
	assert_ne!(a, c);

	// Big
	a = a * c;
	b = b * c;

	assert_eq!(a, b);
	assert_ne!(a, c);
}

#[test]
fn test_partial_ord() {
	let a = Uint256([1, 2, 3, 4]);
	let b = Uint256([4, 5, 6, 1]);

	assert!(a > b);
	assert!(b < a);
}

#[test]
fn test_cast() {
	let from_u64 = Uint256::from(1000u64);
	let from_i64 = Uint256::from(1000i64);

	assert_eq!(from_u64, from_i64);
}

#[test]
fn test_display() {}

#[test]
fn test_count_digits() {
	// Note that digits in the big-endian order

	// count_digits(0000) = 1
	assert_eq!(Uint256::zero().count_digits(), 1);

	// count_digits(1111) = 4
	assert_eq!(Uint256([1, 1, 1, 1]).count_digits(), 4);

	// count_digits(0001) = 4
	assert_eq!(Uint256([0, 0, 0, 1]).count_digits(), 4);

	// count_digits(1000) = 1
	assert_eq!(Uint256([1, 0, 0, 0]).count_digits(), 1);

	// count_digits(1100) = 2
	assert_eq!(Uint256([1, 1, 0, 0]).count_digits(), 2);

	// count_digits(0011) = 4
	assert_eq!(Uint256([0, 0, 1, 1]).count_digits(), 4)
}

#[test]
fn test_add() {
	// 0 + 0 = 0
	assert_eq!(Uint256::zero() + Uint256::zero(), Uint256::zero());

	// 1 + 0 = 1
	assert_eq!(Uint256::zero() + Uint256::from(1), Uint256::from(1));

	// Overflow checks

	// max + 1 = 0
	assert_eq!(Uint256::max() + Uint256::from(1), Uint256::zero());

	// max + 2 = 1
	assert_eq!(Uint256::max() + Uint256::from(2), Uint256::from(1));

	// Partial overflow check
	assert_eq!(
		Uint256([0, 0xFFFFFFFFFFFFFFFF, 1, 0]) + Uint256([0, 1, 0, 0]),
		Uint256([0, 0, 2, 0])
	);
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
	assert_eq!(Uint256([0, 1, 1, 0]) - Uint256([0, 2, 0, 0]), Uint256([0, u64::max_value(), 0, 0]));

	// (2^256 - 1) - 2^255 = 7fffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
	assert_eq!(
		Uint256::max() - Uint256([0, 0, 0, 0x8000000000000000]),
		Uint256([0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0x7fffffffffffffff])
	);
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

	// max(u64) * max(u64) = 0xFFFFFFFFFFFFFFFE 0000000000000001
	assert_eq!(
		Uint256::from(u64::max_value()) * Uint256::from(u64::max_value()),
		Uint256([0x0000000000000001, 0xFFFFFFFFFFFFFFFE, 0u64, 0u64])
	);

	// a * b = b * a
	assert_eq!(
		Uint256::from(u64::max_value() / 2) * Uint256::from(u64::max_value()),
		Uint256::from(u64::max_value()) * Uint256::from(u64::max_value() / 2)
	);
}

#[test]
fn test_short_div() {
	// 0 / 1 = 0
	assert_eq!(Uint256::zero() / 1, Uint256::zero());

	// a / 1 = a
	assert_eq!(Uint256([0x0000000000000001, 0xFFFFFFFFFFFFFFFE, 0u64, 0u64]) / 1,
			   Uint256([0x0000000000000001, 0xFFFFFFFFFFFFFFFE, 0u64, 0u64]));

	// a * 2 / 2 = a
	assert_eq!(Uint256::from(u64::max_value()) * Uint256::from(2) / 2, Uint256::from(u64::max_value()));

	// if a < b then a / b = 0
	// a: 9223372036854775807
	// b: 17233868258737669948
	assert_eq!(Uint256::from(9223372036854775807u64) / 17233868258737669948u64, Uint256::zero());

	assert_eq!(Uint256([18446744073709551615u64, 9223372036854775807u64, 0u64, 0u64]) / 17233868258737669948u64, Uint256::from(9872489501839302786u64));

}

#[test]
fn test_long_div() {
	// Should panic if division by 0
//	let p = panic::catch_unwind(|| {
//		Uint256::from(5) / Uint256::zero();
//	});
//	assert!(p.is_err());

	// If divisor is one-digit, short division should be used
	assert_eq!(Uint256([0x0000000000000001, 0xFFFFFFFFFFFFFFFE, 0u64, 0u64]) / Uint256::from(1),
			   Uint256([0x0000000000000001, 0xFFFFFFFFFFFFFFFE, 0u64, 0u64]));

	// a / a = 1
	assert_eq!(Uint256([0x0000000000000001, 0xFFFFFFFFFFFFFFFE, 0u64, 0u64])
				   / Uint256([0x0000000000000001, 0xFFFFFFFFFFFFFFFE, 0u64, 0u64]),
			   Uint256::from(1));

	// max / max = 1
	assert_eq!(Uint256::max() / Uint256::max(), Uint256::from(1));

	// a / max = 0, where a != max
	assert_eq!(Uint256([0x0000000000000001, 0xFFFFFFFFFFFFFFFE, 0u64, 0u64]) / Uint256::max(), Uint256::zero());

	// a / b = c

	// 0x 7fffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
	// divide by
	// 0x 0000000000000000 ef2affdf53f0473c ebdcf3dfd3572173 113cf3ffdfff0071
	// equal to
	// 0x 890220e52de7a082
	assert_eq!(Uint256([0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0x7fffffffffffffff])
		/ Uint256([0x113cf3ffdfff0071, 0xebdcf3dfd3572173, 0xef2affdf53f0473c, 0x0000000000000000]),
		Uint256::from(0x890220e52de7a082u64));

	// 0x 7ffff3f6f1f600df ff235feebdaedaaa f3bcc7f3f272feed ff98fdf13ceff45f
	// divide by
	// 0x 9facd3f3eeff0071
	// equal to
	// 0x cd3766c723dea6cc e89d6c29e9ef895d 0b1fd4f71b3fb1ab
	assert_eq!(Uint256([0xff98fdf13ceff45f, 0xf3bcc7f3f272feed, 0xff235feebdaedaaa, 0x7ffff3f6f1f600df])
		/ Uint256([0x9facd3f3eeff0071, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000]),
		Uint256([0x0b1fd4f71b3fb1ab, 0xe89d6c29e9ef895d, 0xcd3766c723dea6cc, 0x0000000000000000]));
}
