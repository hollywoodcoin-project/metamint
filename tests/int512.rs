extern crate metamint;

use metamint::utils::bignum::{Uint512, Int512, Zero};

#[test]
fn test_zero() {
	assert_eq!(Int512::zero(), Int512::from_raw(Uint512::zero(), true));
}

#[test]
fn test_eq() {
	let a = Int512::from_raw(Uint512::from_raw([0x5340d92f6c5155f3, 0x2a2db477db2a939a, 0x2a5f6398a0e1fe86, 0xff954daf3f2c2cf3, 0xef8fc610c5603195, 0x2c908de78a593ba0, 0x59f24e98fb556bd1, 0x752fc6c82740ec88]), true);
	let b = Int512::from_raw(Uint512::from_raw([0x5340d92f6c5155f3, 0x2a2db477db2a939a, 0x2a5f6398a0e1fe86, 0xff954daf3f2c2cf3, 0xef8fc610c5603195, 0x2c908de78a593ba0, 0x59f24e98fb556bd1, 0x752fc6c82740ec88]), true);
	let c = Int512::from_raw(Uint512::from_raw([0xc22a2a6a1efcba75, 0x57da23739104ffb3, 0xba1d9d4a6d17b601, 0xc3f03afefc2dd30f, 0x3b0f1fdad6ae4350, 0x987d13b45744bd15, 0x747a5ca8f448094f, 0xe6a8dbbea22406ef]), false);
	let d = Int512::from_raw(Uint512::from_raw([0x5340d92f6c5155f3, 0x2a2db477db2a939a, 0x2a5f6398a0e1fe86, 0xff954daf3f2c2cf3, 0xef8fc610c5603195, 0x2c908de78a593ba0, 0x59f24e98fb556bd1, 0x752fc6c82740ec88]), false);

	assert!(a == b);
	assert!(a != c);
	assert!(a != d);
}

#[test]
fn test_ord() {
	// [+] 0x 752fc6c82740ec88 59f24e98fb556bd1 2c908de78a593ba0 ef8fc610c5603195 ff954daf3f2c2cf3 2a5f6398a0e1fe86 2a2db477db2a939a 5340d92f6c5155f3
	let a = Int512::from_raw(Uint512::from_raw([0x5340d92f6c5155f3, 0x2a2db477db2a939a, 0x2a5f6398a0e1fe86, 0xff954daf3f2c2cf3, 0xef8fc610c5603195, 0x2c908de78a593ba0, 0x59f24e98fb556bd1, 0x752fc6c82740ec88]), true);

	// [+] 0x 1261b1d3c41a3af2 3e8a8c79f9c54def 69f61ea6dd5b4aac 49e57fac7b86d506 a051371e77789dac 5bbcff37c1b20d02 95af687a33fe12b3 9b9b9db77c29a258
	let b = Int512::from_raw(Uint512::from_raw([0x9b9b9db77c29a258, 0x95af687a33fe12b3, 0x5bbcff37c1b20d02, 0xa051371e77789dac, 0x49e57fac7b86d506, 0x69f61ea6dd5b4aac, 0x3e8a8c79f9c54def, 0x1261b1d3c41a3af2]), true);

	// [-] 0x e6a8dbbea22406ef 747a5ca8f448094f 987d13b45744bd15 3b0f1fdad6ae4350 c3f03afefc2dd30f ba1d9d4a6d17b601 57da23739104ffb3 c22a2a6a1efcba75
	let c = Int512::from_raw(Uint512::from_raw([0xc22a2a6a1efcba75, 0x57da23739104ffb3, 0xba1d9d4a6d17b601, 0xc3f03afefc2dd30f, 0x3b0f1fdad6ae4350, 0x987d13b45744bd15, 0x747a5ca8f448094f, 0xe6a8dbbea22406ef]), false);

	// [-] 0x a1832fce900bdfc1 c98a411d160754fc 3eb37f58b04867db f266c9ca1779abd0 a1f71a781861945e 4189c2ca5c9d747d 9c6419c9f1fd36cc eb9ace46131a9d96
	let d = Int512::from_raw(Uint512::from_raw([0xeb9ace46131a9d96, 0x9c6419c9f1fd36cc, 0x4189c2ca5c9d747d, 0xa1f71a781861945e, 0xf266c9ca1779abd0, 0x3eb37f58b04867db, 0xc98a411d160754fc, 0xa1832fce900bdfc1]), false);

	let zero = Int512::zero();

	assert!(a > b);
	assert!(b < a);

	assert!(c < a && c < b);
	assert!(a > c && a > b);

	assert!(c < d);
	assert!(d > c);

	assert!(a > zero);
	assert!(c < zero);

	assert!(!(zero > Int512::zero()));
	assert!(!(zero < Int512::zero()));

	assert!(!(Int512::from(1) < Int512::zero()));
	assert!(Int512::from(1) > Int512::zero());
	assert!(Int512::zero() < Int512::from(1));
}

#[test]
fn test_add() {
	// 0 + 0 = 0
	assert_eq!(Int512::zero() + Int512::zero(), Int512::zero());

	// [-] 0x 1b420c36c04b9b1d 6981659bfd3b2b8a 834016557629ae01 21223f27e1e88566 817b7694d8be5b8f 2ee2815c0eb490f5 c77b96141abf035a 5899130e4d1c3a8e
	// +
	// [+] 0x e7f6d1029dd43cad 68a1b7914077f42c eba7501cc50c86f5 f1b9e5ae8fe1fcb7 71ddbd111e32f0de 69ef143149c2b901 44cc9d31bb24a53a 3c277670fc6a82fe
	// =
	// [+] 0x ccb4c4cbdd88a18f ff2051f5433cc8a2 686739c74ee2d8f4 d097a686adf97750 f062467c4574954f 3b0c92d53b0e280b 7d51071da065a1df e38e6362af4e4870
	assert_eq!(
		Int512::from_raw(Uint512::from_raw([0x5899130e4d1c3a8e, 0xc77b96141abf035a, 0x2ee2815c0eb490f5, 0x817b7694d8be5b8f, 0x21223f27e1e88566, 0x834016557629ae01, 0x6981659bfd3b2b8a, 0x1b420c36c04b9b1d]), false)
		+ Int512::from_raw(Uint512::from_raw([0x3c277670fc6a82fe, 0x44cc9d31bb24a53a, 0x69ef143149c2b901, 0x71ddbd111e32f0de, 0xf1b9e5ae8fe1fcb7, 0xeba7501cc50c86f5, 0x68a1b7914077f42c, 0xe7f6d1029dd43cad]), true),
		Int512::from_raw(Uint512::from_raw([0xe38e6362af4e4870, 0x7d51071da065a1df, 0x3b0c92d53b0e280b, 0xf062467c4574954f, 0xd097a686adf97750, 0x686739c74ee2d8f4, 0xff2051f5433cc8a2, 0xccb4c4cbdd88a18f]), true)
	);

	// [-] 0x a96314ee8d9b147f f7c07c0f57928036 56da57cd66c6ec5b 22ed57a8c736a25e d15571fa2b95f1fc 9a2f8d0287b35ba8 b69805ebe7e876b9 b452aa7a80f7490e
	// +
	// [+] 0x 5aecd6da973ffda6 36437daa99f1e1b8 5ae97df33ab9ae87 f5e741f5de5b03e3 a9b03c6a1d434f02 bff38cb2efd9ac95 23646d7ac98f6f67 a5fa15bd645b2aff
	// =
	// [-] 0x 4e763e13f65b16d9 c17cfe64bda09e7d fbf0d9da2c0d3dd3 2d0615b2e8db9e7b 27a535900e52a2f9 da3c004f97d9af13 933398711e590752 0e5894bd1c9c1e0f
	assert_eq!(
		Int512::from_raw(Uint512::from_raw([0xb452aa7a80f7490e, 0xb69805ebe7e876b9, 0x9a2f8d0287b35ba8, 0xd15571fa2b95f1fc, 0x22ed57a8c736a25e, 0x56da57cd66c6ec5b, 0xf7c07c0f57928036, 0xa96314ee8d9b147f]), false)
			+ Int512::from_raw(Uint512::from_raw([0xa5fa15bd645b2aff, 0x23646d7ac98f6f67, 0xbff38cb2efd9ac95, 0xa9b03c6a1d434f02, 0xf5e741f5de5b03e3, 0x5ae97df33ab9ae87, 0x36437daa99f1e1b8, 0x5aecd6da973ffda6]), true),
		Int512::from_raw(Uint512::from_raw([0x0e5894bd1c9c1e0f, 0x933398711e590752, 0xda3c004f97d9af13, 0x27a535900e52a2f9, 0x2d0615b2e8db9e7b, 0xfbf0d9da2c0d3dd3, 0xc17cfe64bda09e7d, 0x4e763e13f65b16d9]), false)
	);
}

#[test]
fn test_sub() {
	// 0 - a = a
	// 0x 5effa688958c818c 92baba6678998628 6799162fc7128034 090045faf3eec7a4 770372aa9d377b60 431798567b38632d afd192d0d8deee6d f62ac92c1cc532a6
	assert_eq!(
		Int512::zero() - Int512::from_raw(Uint512::from_raw([0xf62ac92c1cc532a6, 0xafd192d0d8deee6d, 0x431798567b38632d, 0x770372aa9d377b60, 0x090045faf3eec7a4, 0x6799162fc7128034, 0x92baba6678998628, 0x5effa688958c818c]), true),
		Int512::from_raw(Uint512::from_raw([0xf62ac92c1cc532a6, 0xafd192d0d8deee6d, 0x431798567b38632d, 0x770372aa9d377b60, 0x090045faf3eec7a4, 0x6799162fc7128034, 0x92baba6678998628, 0x5effa688958c818c]), false)
	);

	// [+] 0x bf8fb9f9d386d7c5 dad83c97f01beed4 ae850352d0fbba65 11f97c2ac8fa6280 b7bda25b6302445c 0b0520c9a13baa3e 7b950ba3ab2b98e3 acdb7820bfef2817
	// -
	// [+] 0x 88de342bfea1c710 57abdaeeb96631f8 d7e414d32b0502fc 1a2977c478a51578 3cafdd7416f2837b 1cbea931fd671f01 3dc6f7b0270e0873 2cceb14985ac2188
	// =
	// [+] 0x 36b185cdd4e510b5 832c61a936b5bcdb d6a0ee7fa5f6b768 f7d0046650554d08 7b0dc4e74c0fc0e0 ee467797a3d48b3d 3dce13f3841d9070 800cc6d73a43068f
	assert_eq!(
		Int512::from_raw(Uint512::from_raw([0xacdb7820bfef2817, 0x7b950ba3ab2b98e3, 0x0b0520c9a13baa3e, 0xb7bda25b6302445c, 0x11f97c2ac8fa6280, 0xae850352d0fbba65, 0xdad83c97f01beed4, 0xbf8fb9f9d386d7c5]), true)
			- Int512::from_raw(Uint512::from_raw([0x2cceb14985ac2188, 0x3dc6f7b0270e0873, 0x1cbea931fd671f01, 0x3cafdd7416f2837b, 0x1a2977c478a51578, 0xd7e414d32b0502fc, 0x57abdaeeb96631f8, 0x88de342bfea1c710]), true),
		Int512::from_raw(Uint512::from_raw([0x800cc6d73a43068f, 0x3dce13f3841d9070, 0xee467797a3d48b3d, 0x7b0dc4e74c0fc0e0, 0xf7d0046650554d08, 0xd6a0ee7fa5f6b768, 0x832c61a936b5bcdb, 0x36b185cdd4e510b5]), true)
	);

	// [+] 0x 1db6762e1525c4b3 88169895bb17a627 e72e3df91c70dcad 7a34f048d539f663 db8d768f7363b484 005efffb5d524c9a 5f05afff1d9d22be 34409a216620254b
	// -
	// [-] 0x dafe7b9b928a9cba ceef5c4085d03e83 cd6f77d8283054a9 51d05f53373e24b5 146787be6ea9634a 10af18388a29d872 68ca3ccdfc1de186 903db5599499649c
	// =
	// [+] 0x f8b4f1c9a7b0616e 5705f4d640e7e4ab b49db5d144a13156 cc054f9c0c781b18 eff4fe4de20d17ce 110e1833e77c250c c7cfeccd19bb0444 c47e4f7afab989e7
	assert_eq!(
		Int512::from_raw(Uint512::from_raw([0x34409a216620254b, 0x5f05afff1d9d22be, 0x005efffb5d524c9a, 0xdb8d768f7363b484, 0x7a34f048d539f663, 0xe72e3df91c70dcad, 0x88169895bb17a627, 0x1db6762e1525c4b3]), true)
			- Int512::from_raw(Uint512::from_raw([0x903db5599499649c, 0x68ca3ccdfc1de186, 0x10af18388a29d872, 0x146787be6ea9634a, 0x51d05f53373e24b5, 0xcd6f77d8283054a9, 0xceef5c4085d03e83, 0xdafe7b9b928a9cba]), false),
		Int512::from_raw(Uint512::from_raw([0xc47e4f7afab989e7, 0xc7cfeccd19bb0444, 0x110e1833e77c250c, 0xeff4fe4de20d17ce, 0xcc054f9c0c781b18, 0xb49db5d144a13156, 0x5705f4d640e7e4ab, 0xf8b4f1c9a7b0616e]), true)
	);
}

#[test]
fn test_mul() {
	// [+] * [+] = [+]
	assert_eq!(Int512::from(1) * Int512::from(1), Int512::from(1));

	// [-] * [+] = [-]
	assert_eq!(Int512::from(-1) * Int512::from(1), Int512::from(-1));

	// [+] * [-] = [-]
	assert_eq!(Int512::from(1) * Int512::from(-1), Int512::from(-1));

	// [-] * [-] = [+]
	assert_eq!(Int512::from(-1) * Int512::from(-1), Int512::from(1));
}