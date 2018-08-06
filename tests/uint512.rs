extern crate metamint;

use metamint::utils::bignum::{Uint512, Uint256, Zero, One};

#[test]
fn test_zero() {
	assert_eq!(Uint512::zero(), Uint512::from_raw([0; 8]));
	assert_ne!(Uint512::zero(), Uint512::from_raw([1; 8]));

	assert!(!Uint512::from(1).is_zero());
	assert!(Uint512::from_raw([0; 8]).is_zero());
	assert!(!Uint512::from_raw([1, 2, 3, 4, 5, 6, 7, 8]).is_zero());
}

#[test]
fn test_one() {
	assert_eq!(Uint512::one(), Uint512::from_raw([1, 0, 0, 0, 0, 0, 0, 0]));
	assert_ne!(Uint512::one(), Uint512::from_raw([1; 8]));

	assert!(Uint512::from_raw([1, 0, 0, 0, 0, 0, 0, 0]).is_one());
	assert!(!Uint512::from_raw([1, 2, 3, 4, 5, 6, 7, 8]).is_one());
}

#[test]
fn test_max() {
	assert_eq!(Uint512::max(), Uint512::from_raw([0xFFFFFFFFFFFFFFFF; 8]));
}

#[test]
fn test_partial_eq() {
	// max = max
	assert!(Uint512::max() == Uint512::max());

	// Big
	let a = Uint512::from_raw([0xff98fdf13ceff45f, 0xf3bcc7f3f272feed, 0xff235feebdaedaaa, 0x7ffff3f6f1f600df, 0x1f6f33f0f1abcdea, 0x9c458ded283c2719, 0x17757a4a83e3aa71, 0x1673d0df54bb18f6]);
	let b = Uint512::from_raw([0xff98fdf13ceff45f, 0xf3bcc7f3f272feed, 0xff235feebdaedaaa, 0x7ffff3f6f1f600df, 0x1f6f33f0f1abcdea, 0x9c458ded283c2719, 0x17757a4a83e3aa71, 0x1673d0df54bb18f6]);
	let c = Uint512::from_raw([0x1670c87c5acc13a, 0x60f8ee7a94c858f, 0xd346181a4410469a, 0, 0x845d8098427b4e4d, 0xe5996fe4ab94e93f, 0, 0]);

	assert!(a == b);
	assert!(a != c);
}

#[test]
fn test_partial_ord() {
	let a = Uint512::from_raw([1, 2, 3, 4, 5, 6, 7, 8]);
	let b = Uint512::from_raw([4, 5, 6, 1, 2, 6, 8, 3]);

	assert!(a > b);
	assert!(b < a);

	assert!(Uint512::from(1) > Uint512::zero());
	assert!(!(Uint512::from(1) < Uint512::zero()));
}

#[test]
fn test_cast() {
	let from_u64 = Uint512::from(1000u64);
	let from_i64 = Uint512::from(1000i64);
	let from_u32 = Uint512::from(1000u32);
	let from_i32 = Uint512::from(1000i32);

	assert_eq!(from_u64, from_i64);
	assert_eq!(from_u64, from_u32);
	assert_eq!(from_u64, from_i32);
}

#[test]
fn test_from_uint256() {
	assert_eq!(Uint512::from(Uint256::from_raw([1, 2, 3, 4])), Uint512::from_raw([1, 2, 3, 4, 0, 0, 0, 0]));
}

#[test]
fn test_count_digits() {
	// Note that digits in the big-endian order

	// count_digits(0000) = 1
	assert_eq!(Uint512::zero().count_digits(), 1);

	// count_digits(11111111) = 8
	assert_eq!(Uint512::from_raw([1, 1, 1, 1, 1, 1, 1, 1]).count_digits(), 8);

	// count_digits(00000001) = 8
	assert_eq!(Uint512::from_raw([0, 0, 0, 0, 0, 0, 0, 1]).count_digits(), 8);

	// count_digits(10000000) = 1
	assert_eq!(Uint512::from_raw([1, 0, 0, 0, 0, 0, 0, 0]).count_digits(), 1);

	// count_digits(11110000) = 4
	assert_eq!(Uint512::from_raw([1, 1, 1, 1, 0, 0, 0, 0]).count_digits(), 4);

	// count_digits(00001011) = 8
	assert_eq!(Uint512::from_raw([0, 0, 0, 0, 1, 0, 1, 1]).count_digits(), 8)
}

#[test]
fn test_add() {
	// 0 + 0 = 0
	assert_eq!(Uint512::zero() + Uint512::zero(), Uint512::zero());

	// 1 + 0 = 1
	assert_eq!(Uint512::zero() + Uint512::from(1), Uint512::from(1));

	// a + b = b + a
	assert_eq!(
		Uint512::from_raw([0x81ed6a6eaaebf032, 0xf592d49cc1e04b7d, 0xff1b2c1df7319961, 0x9b2004c345900c7, 0x9f405bc537233e46, 0xd13e0859feec078, 0, 0])
		+ Uint512::from_raw([0xf81f7ba7c7508848, 0x9179c1ad669856ee, 0x4aafdb2585ec88d8, 0x18746c483fffa94c, 0, 0, 0, 0]),
		Uint512::from_raw([0xf81f7ba7c7508848, 0x9179c1ad669856ee, 0x4aafdb2585ec88d8, 0x18746c483fffa94c, 0, 0, 0, 0])
		+ Uint512::from_raw([0x81ed6a6eaaebf032, 0xf592d49cc1e04b7d, 0xff1b2c1df7319961, 0x9b2004c345900c7, 0x9f405bc537233e46, 0xd13e0859feec078, 0, 0])
	);

	// Overflow checks

	// max + 1 = 0
	assert_eq!(Uint512::max() + Uint512::from(1), Uint512::zero());

	// max + 2 = 1
	assert_eq!(Uint512::max() + Uint512::from(2), Uint512::from(1));

	// Partial overflow check
	assert_eq!(
		Uint512::from_raw([0, 0xFFFFFFFFFFFFFFFF, 1, 0, 0, 0, 0, 0])
			+ Uint512([0, 1, 0, 0, 0, 0, 0, 0]),
		Uint512::from_raw([0, 0, 2, 0, 0, 0, 0, 0])
	);

	// 0x 1aef7e8dafe7c33d 226288f94d0b5895 02e18e29c22c77d1 b76b4068b06ebbee 7d758c58922215ca dd94702a1f71acd0 dd9c23b96abd8ec0 f7b868a38456e817
	// +
	// 0x 6f3b44632ee66573 82389a711c069701 538bf0e987ebadea de36a05fec9c58ca 4a1dc4bc389590cf caf2f4054eebc5b5 ab86071183b63c8d 77ba5f632f8c1baa
	// =
	// 0x 8a2ac2f0dece28b0 a49b236a6911ef96 566d7f134a1825bc 95a1e0c89d0b14b8 c7935114cab7a69a a887642f6e5d7286 89222acaee73cb4e 6f72c806b3e303c1
	assert_eq!(
		Uint512::from_raw([0xf7b868a38456e817, 0xdd9c23b96abd8ec0, 0xdd94702a1f71acd0, 0x7d758c58922215ca, 0xb76b4068b06ebbee, 0x02e18e29c22c77d1, 0x226288f94d0b5895, 0x1aef7e8dafe7c33d])
			+ Uint512::from_raw([0x77ba5f632f8c1baa, 0xab86071183b63c8d, 0xcaf2f4054eebc5b5, 0x4a1dc4bc389590cf, 0xde36a05fec9c58ca, 0x538bf0e987ebadea, 0x82389a711c069701, 0x6f3b44632ee66573]),
		Uint512::from_raw([0x6f72c806b3e303c1, 0x89222acaee73cb4e, 0xa887642f6e5d7286, 0xc7935114cab7a69a, 0x95a1e0c89d0b14b8, 0x566d7f134a1825bc, 0xa49b236a6911ef96, 0x8a2ac2f0dece28b0])
	);
}

#[test]
fn test_sub() {
	// 0 - 0 = 0
	assert_eq!(Uint512::zero() - Uint512::zero(), Uint512::zero());

	// 1 - 0 = 0
	assert_eq!(Uint512::from(1) - Uint512::zero(), Uint512::from(1));

	// 1 - 1 = 0
	assert_eq!(Uint512::from(1) - Uint512::from(1), Uint512::zero());

	// max - max = 0
	assert_eq!(Uint512::max() - Uint512::max(), Uint512::zero());

	// Overflow checks

	// min - 1 = max
	assert_eq!(Uint512::zero() - Uint512::from(1), Uint512::max());

	//min - 2 = max - 1
	assert_eq!(Uint512::zero() - Uint512::from(2), Uint512::max() - Uint512::from(1));

	// Partial overflow check
	assert_eq!(Uint512([0, 1, 1, 0, 0, 0, 0, 0]) - Uint512([0, 2, 0, 0, 0, 0, 0, 0]), Uint512([0, u64::max_value(), 0, 0, 0, 0, 0, 0]));

	// 0x 8e67b7f577ece0d4 75f39940bef10172 d1b1d3e3dfe37902 a396a0c8bc8d1a23 62bc66361b6102d9 67945a3ea4f92682 682b42fc1e2bf2a4 873b033421617f9f
	// -
	// 0x 63b3e95b1faaab6c 79b0a231fc3492d6 9fad8855c3df0300 c971c8c46604cc65 ecbb47cf9365df27 12c78531c310437d 4613e4f56b512128 2e39b8ea52131f55
	// =
	// 0x 2ab3ce9a58423567 fc42f70ec2bc6e9c 32044b8e1c047601 da24d80456884dbd 76011e6687fb23b2 54ccd50ce1e8e305 22175e06b2dad17c 59014a49cf4e604a
	assert_eq!(
		Uint512::from_raw([0x873b033421617f9f, 0x682b42fc1e2bf2a4, 0x67945a3ea4f92682, 0x62bc66361b6102d9, 0xa396a0c8bc8d1a23, 0xd1b1d3e3dfe37902, 0x75f39940bef10172, 0x8e67b7f577ece0d4])
			- Uint512::from_raw([0x2e39b8ea52131f55, 0x4613e4f56b512128, 0x12c78531c310437d, 0xecbb47cf9365df27, 0xc971c8c46604cc65, 0x9fad8855c3df0300, 0x79b0a231fc3492d6, 0x63b3e95b1faaab6c]),
		Uint512::from_raw([0x59014a49cf4e604a, 0x22175e06b2dad17c, 0x54ccd50ce1e8e305, 0x76011e6687fb23b2, 0xda24d80456884dbd, 0x32044b8e1c047601, 0xfc42f70ec2bc6e9c, 0x2ab3ce9a58423567])
	);
}

#[test]
fn test_mul() {
	// 0 * 0 = 0
	assert_eq!(Uint512::zero() * Uint512::zero(), Uint512::zero());

	// 0 * 1 = 0
	assert_eq!(Uint512::zero() * Uint512::from(1), Uint512::zero());

	// 1 * 1 = 1
	assert_eq!(Uint512::from(1) * Uint512::from(1), Uint512::from(1));

	// 1 * max = max
	assert_eq!(Uint512::from(1) * Uint512::max(), Uint512::max());

	// max(u64) * max(u64) = 0x FFFFFFFFFFFFFFFE 0000000000000001
	assert_eq!(
		Uint512::from(u64::max_value()) * Uint512::from(u64::max_value()),
		Uint512([0x0000000000000001, 0xFFFFFFFFFFFFFFFE, 0, 0, 0, 0, 0, 0])
	);

	// a * b = b * a
	assert_eq!(
		Uint512::from(u64::max_value() / 2) * Uint512::from(u64::max_value()),
		Uint512::from(u64::max_value()) * Uint512::from(u64::max_value() / 2)
	);

	// 0x ec6f129fcc139b63 fddc1c21da170af4 fea0ecbb8c4782d6 54e0de400564b37c
	// *
	// 0x 872a2849f50c77f3 3dc0115d77028321 aba484c137192e63 e23ef70a6681daf1
	// =
	// 0x 7cd58240fbbcea6d 9110cd15cc2ed56a f47ab58da1b12e29 15c3c5df7cc26466 bbe5bfac0f6b442e 808badbe75c243a9 a19198b84159676a 34803cf6ae208fbc
	assert_eq!(
		Uint512::from_raw([0x54e0de400564b37c, 0xfea0ecbb8c4782d6, 0xfddc1c21da170af4, 0xec6f129fcc139b63, 0, 0, 0, 0])
			* Uint512::from_raw([0xe23ef70a6681daf1, 0xaba484c137192e63, 0x3dc0115d77028321, 0x872a2849f50c77f3, 0, 0, 0, 0]),
		Uint512::from_raw([0x34803cf6ae208fbc, 0xa19198b84159676a, 0x808badbe75c243a9, 0xbbe5bfac0f6b442e, 0x15c3c5df7cc26466, 0xf47ab58da1b12e29, 0x9110cd15cc2ed56a, 0x7cd58240fbbcea6d])
	);

	// 0x ec6f dd65611fb491f071 c21da170af4fea0e c1d480985cb1b3a5
	// *
	// 0x 1655712abe 8eeb95f3dc0115d7 70b8b92b41216a80 883ef70a6681daf1
	// =
	// 0x 14a086b0141511 7fa7e95b95df0241 438ff94d052c0beb 2429a9daecdbcc21 fe15231aaf745b0c 29703411953c46a1 2d7f88efda69a055
	assert_eq!(
		Uint512::from_raw([0xc1d480985cb1b3a5, 0xc21da170af4fea0e, 0xdd65611fb491f071, 0xec6f, 0, 0, 0, 0])
		* Uint512::from_raw([0x883ef70a6681daf1, 0x70b8b92b41216a80, 0x8eeb95f3dc0115d7, 0x1655712abe, 0, 0, 0, 0]),
		Uint512::from_raw([0x2d7f88efda69a055, 0x29703411953c46a1, 0xfe15231aaf745b0c, 0x2429a9daecdbcc21, 0x438ff94d052c0beb, 0x7fa7e95b95df0241, 0x14a086b0141511, 0])
	);

	assert_eq!(
		Uint512::from_raw([8886340722334833766, 14229879991289077733, 15469762598397178558, 2661540, 1, 1, 0, 0])
		* Uint512::from(u64::max_value()),
		Uint512::from_raw([0x84ad6031e424e39a, 0xb5d7ef589ea1bc80, 0xeecb0d568822ed26, 0xd6afa31ef4edd619, 0x0000000000289ca3, 0, 1, 0])
	);
}

#[test]
fn test_short_div() {
	// 0 / 1 = 0
	assert_eq!(Uint512::zero() / 1, Uint512::zero());

	// a / 1 = a
	assert_eq!(Uint512::from_raw([1, 2, 3, 4, 5, 6, 7, 8]) / 1,
			   Uint512::from_raw([1, 2, 3, 4, 5, 6, 7, 8]));

	// if a < b then a / b = 0
	// a: 9223372036854775807
	// b: 17233868258737669948
	assert_eq!(Uint512::from(9223372036854775807u64) / 17233868258737669948u64, Uint512::zero());

	// 0x ef35067b199c17c4 46bc658083ec0d0f ab6dfc62aac8381e 09fa40df3cbb4f7e da43292e8808f31b e534f7990b1dac7e bb3f19cdcc15f8a0 f90d95973f217ece
	// /
	// 0x 63af6a5f03bd18ea
	// =
	// 0x 2 664dd18ab2e5f912 ca1402635765f6a7 cbf683d4be8c62f0 fde4a531afe63b82 9f73c7016af9f654 b98846e94c08097a 34e5e61226d85257
	assert_eq!(
		Uint512::from_raw([0xf90d95973f217ece, 0xbb3f19cdcc15f8a0, 0xe534f7990b1dac7e, 0xda43292e8808f31b, 0x09fa40df3cbb4f7e, 0xab6dfc62aac8381e, 0x46bc658083ec0d0f, 0xef35067b199c17c4])
			/ 0x63af6a5f03bd18ea,
		Uint512::from_raw([0x34e5e61226d85257, 0xb98846e94c08097a, 0x9f73c7016af9f654, 0xfde4a531afe63b82, 0xcbf683d4be8c62f0, 0xca1402635765f6a7, 0x664dd18ab2e5f912, 0x2])
	);
}

#[test]
fn test_long_div() {

	// a / a = 1
	assert_eq!(Uint512::from_raw([1, 2, 3, 4, 5, 6, 7, 8]) / Uint512::from_raw([1, 2, 3, 4, 5, 6, 7, 8]), Uint512::from(1));

	// max / max = 1
	assert_eq!(Uint512::max() / Uint512::max(), Uint512::from(1));

	// a / max = 0, where a != max
	assert_eq!(Uint512::from_raw([1, 2, 3, 4, 5, 6, 7, 8]) / Uint512::max(), Uint512::zero());

	// 0x 7492982a773db4b 6b8494bef2b3b765 abd9c23adce5680e d118988878b2ff92 b0cc14275324778a 4a7cc807bd0fa895 26fed9febab5623c 3d5439b625c34712
	// /
	// 0x 46393599d78d1c 1c77235930b86434 fe087464f6be8254 44cc641e1b0f3008 cfa2e3d2226fab01 d563cc2cd2ca4693
	// =
	// 0x 1a 8f750e5ec8086ecd 74c3b64ba2c45cd8
	assert_eq!(
		Uint512::from_raw([0x3d5439b625c34712, 0x26fed9febab5623c, 0x4a7cc807bd0fa895, 0xb0cc14275324778a, 0xd118988878b2ff92, 0xabd9c23adce5680e, 0x6b8494bef2b3b765, 0x7492982a773db4b])
			/ Uint512::from_raw([0xd563cc2cd2ca4693, 0xcfa2e3d2226fab01, 0x44cc641e1b0f3008, 0xfe087464f6be8254, 0x1c77235930b86434, 0x46393599d78d1c, 0, 0]),
		Uint512::from_raw([0x74c3b64ba2c45cd8, 0x8f750e5ec8086ecd, 0x1a, 0, 0, 0, 0, 0])
	);

	// 0x 925d24a146438517 b4540ad02a4de8d4 0a486b9ae4fbf5b7 92e563dbdf074c52 d9897e2e16878e3e 2d0d431ab2f548ca 8bc7e55bb36c52d4 ae880b8fe4fc6444
	// /
	// 0x 88b4ed8943247ce5 b6ea417a81db45a8 bf5fc5ff8c6d8339 b84ba9aefcfa9b2e 2d9ec04ab5c85fb9 1be645806ec6a16c 3160fe8d059bc88a 0a395a9244b2998e
	// =
	// 0x 1
	assert_eq!(
		Uint512::from_raw([0xae880b8fe4fc6444, 0x8bc7e55bb36c52d4, 0x2d0d431ab2f548ca, 0xd9897e2e16878e3e, 0x92e563dbdf074c52, 0x0a486b9ae4fbf5b7, 0xb4540ad02a4de8d4, 0x925d24a146438517])
			/ Uint512::from_raw([0x0a395a9244b2998e, 0x3160fe8d059bc88a, 0x1be645806ec6a16c, 0x2d9ec04ab5c85fb9, 0xb84ba9aefcfa9b2e, 0xbf5fc5ff8c6d8339, 0xb6ea417a81db45a8, 0x88b4ed8943247ce5]),
		Uint512::from_raw([0x1, 0, 0, 0, 0, 0, 0, 0])
	);

	// 0x 58d372 7278248c0a0d9d41 46f88e99e8f2d2c0 a074ca39e682eb43 354e18bdfe241438 97c51663f944deab d23ed03206e08795 c8885bcb9291d1a2
	// /
	// 0x 289ca4 d6afa31ef51672be c57ab0757d395fe5 7b529fce1bdb1c66
	// =
	// 0x 2 2febc198fdd3c228 b9f7d63e4218bff8 1c655000c94511c1 57d818cf3387e6b6
	assert_eq!(
		Uint512::from_raw([0xc8885bcb9291d1a2, 0xd23ed03206e08795, 0x97c51663f944deab, 0x354e18bdfe241438, 0xa074ca39e682eb43, 0x46f88e99e8f2d2c0, 0x7278248c0a0d9d41, 0x58d372])
			/ Uint512::from_raw([0x7b529fce1bdb1c66, 0xc57ab0757d395fe5, 0xd6afa31ef51672be, 0x289ca4, 0, 0, 0, 0]),
		Uint512::from_raw([0x57d818cf3387e6b6, 0x1c655000c94511c1, 0xb9f7d63e4218bff8, 0x2febc198fdd3c228, 0x2, 0, 0, 0])
	);

	// 0x ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
	// /
	// 0x 1 ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
	// =
	// 0x 8000000000000000
	assert_eq!(
		Uint512::max()
			/ Uint512::from_raw([0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 1]),
		Uint512::from(0x8000000000000000u64)
	);
}