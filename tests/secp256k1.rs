extern crate metamint;

use metamint::crypto::ecdsa::{SECP256K1, ECPoint};
use metamint::utils::bignum::{Uint256, Zero};

#[test]
fn test_generation_point() {
	//0x 79BE667EF9DCBBAC 55A06295CE870B07 029BFCDB2DCE28D9 59F2815B16F81798
	//0x 483ADA7726A3C465 5DA4FBFC0E1108A8 FD17B448A6855419 9C47D08FFB10D4B8
	assert_eq!(*SECP256K1.gen_point(), SECP256K1.create_point(
		Uint256::from_raw([0x59F2815B16F81798, 0x029BFCDB2DCE28D9, 0x55A06295CE870B07, 0x79BE667EF9DCBBAC]),
		Uint256::from_raw([0x9C47D08FFB10D4B8, 0xFD17B448A6855419, 0x5DA4FBFC0E1108A8, 0x483ADA7726A3C465])
	));
}

#[test]
fn test_double() {
	let gp = *SECP256K1.gen_point();

	// 2G:
	// X: 0x c6047f9441ed7d6d 3045406e95c07cd8 5c778e4b8cef3ca7 abac09b95c709ee5
	// Y: 0x 1ae168fea63dc339 a3c58419466ceaee f7f632653266d0e1 236431a950cfe52a

	assert_eq!(
		gp.double(), ECPoint::new(
			Uint256::from_raw([0xabac09b95c709ee5, 0x5c778e4b8cef3ca7, 0x3045406e95c07cd8, 0xc6047f9441ed7d6d]),
			Uint256::from_raw([0x236431a950cfe52a, 0xf7f632653266d0e1, 0xa3c58419466ceaee, 0x1ae168fea63dc339]),
			&SECP256K1
		)
	);
}

#[test]
fn test_add() {
	// X: 0x 162ebcd38c90b56f bdb4b0390695afb4 71c944a6003cb334 bbf030a89c42b584
	// Y: 0x f089012beb484248 3692bdff9fcab867 6fed42c47bffb081 001209079bbcb8db
	// +
	// G
	// =
	// X: 0x 2585e5ca09115735 c90559d35cf3cbbf 685cb9ecbfbe242b fb7238c5d735f38a
	// Y: 0x b1abc72f727dd755 500a2c543d500d80 6acb43da021eee4a 800cd35bf68c3e04

	let A = SECP256K1.create_point(
		Uint256::from_raw([0xbbf030a89c42b584, 0x71c944a6003cb334, 0xbdb4b0390695afb4, 0x162ebcd38c90b56f]),
		Uint256::from_raw([0x001209079bbcb8db, 0x6fed42c47bffb081, 0x3692bdff9fcab867, 0xf089012beb484248])
	);

	assert_eq!(
		A + *SECP256K1.gen_point(),
		SECP256K1.create_point(
			Uint256::from_raw([0xfb7238c5d735f38a, 0x685cb9ecbfbe242b, 0xc90559d35cf3cbbf, 0x2585e5ca09115735]),
			Uint256::from_raw([0x800cd35bf68c3e04, 0x6acb43da021eee4a, 0x500a2c543d500d80, 0xb1abc72f727dd755])
		)
	);
}

#[test]
fn test_mul() {
	let G = *SECP256K1.gen_point();

	let A = SECP256K1.create_point(
		Uint256::from_raw([0xbbf030a89c42b584, 0x71c944a6003cb334, 0xbdb4b0390695afb4, 0x162ebcd38c90b56f]),
		Uint256::from_raw([0x001209079bbcb8db, 0x6fed42c47bffb081, 0x3692bdff9fcab867, 0xf089012beb484248])
	);

	// 0x 45b0c38fa5476635 4cf3409d38b87325 5dfa9ed3407a542b a48eb9cab9dfca67
	let Pk = Uint256::from_raw([0xa48eb9cab9dfca67, 0x5dfa9ed3407a542b, 0x4cf3409d38b87325, 0x45b0c38fa5476635]);

	// Pk * G = A
	assert_eq!(G * Pk, A);
}