extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate schnorrkel;

mod wrapper;
use wrapper::*;

/// Perform a derivation on a secret
///
/// * secret: UIntArray with 64 bytes
/// * cc: UIntArray with 32 bytes
///
/// returned vector the derived keypair as a array of 96 bytes
#[wasm_bindgen]
pub fn derive_keypair_hard(pair: &[u8], cc: &[u8]) -> Vec<u8> {
	__derive_keypair_hard(pair, cc).to_vec()
}

/// Perform a derivation on a secret
///
/// * secret: UIntArray with 64 bytes
/// * cc: UIntArray with 32 bytes
///
/// returned vector the derived keypair as a array of 96 bytes
#[wasm_bindgen]
pub fn derive_keypair_soft(pair: &[u8], cc: &[u8]) -> Vec<u8> {
	__derive_keypair_soft(pair, cc).to_vec()
}

/// Perform a derivation on a publicKey
///
/// * pubkey: UIntArray with 32 bytes
/// * cc: UIntArray with 32 bytes
///
/// returned vector is the derived publicKey as a array of 32 bytes
#[wasm_bindgen]
pub fn derive_public_soft(public: &[u8], cc: &[u8]) -> Vec<u8> {
	__derive_public_soft(public, cc).to_vec()
}

/// Sign a message
///
/// The combination of both public and private key must be provided.
/// This is effectively equivalent to a keypair.
///
/// * public: UIntArray with 32 element
/// * private: UIntArray with 64 element
/// * message: Arbitrary length UIntArray
///
/// * returned vector is the signature consisting of 64 bytes.
#[wasm_bindgen]
pub fn sign(public: &[u8], private: &[u8], message: &[u8]) -> Vec<u8> {
	__sign(public, private, message).to_vec()
}

/// Sign a message with a given signing context
///
/// The combination of both public and private key must be provided.
/// This is effectively equivalent to a keypair.
///
/// * public: UIntArray with 32 element
/// * private: UIntArray with 64 element
/// * message: Arbitrary length UIntArray
/// * signing_ctx: Arbitrary length UIntArray
///
/// * returned vector is the signature consisting of 64 bytes.
#[wasm_bindgen]
pub fn sign_with_ctx(
	public: &[u8],
	private: &[u8],
	message: &[u8],
	signing_ctx: &[u8],
	) -> Vec<u8> {
	__sign_with_ctx(public, private, message, signing_ctx).to_vec()
}

/// Verify a message and its corresponding signature against a public key;
///
/// * signature: UIntArray with 64 element
/// * message: Arbitrary length UIntArray
/// * pubkey: UIntArray with 32 element
#[wasm_bindgen]
pub fn verify(signature: &[u8], message: &[u8], pubkey: &[u8]) -> bool {
	__verify(signature, message, pubkey)
}

/// Verify a message with a given signing context;
///
/// * signature: UIntArray with 64 element
/// * message: Arbitrary length UIntArray
/// * pubkey: UIntArray with 32 element
/// * signing_ctx: UIntArray with 32 element
#[wasm_bindgen]
pub fn verify_with_ctx(
	signature: &[u8],
	message: &[u8],
	pubkey: &[u8],
	signing_ctx: &[u8]
	) -> bool {
	__verify_with_ctx(signature, message, pubkey, signing_ctx)
}

/// Generate a secret key (aka. private key) from a seed phrase.
///
/// * seed: UIntArray with 32 element
///
/// returned vector is the private key consisting of 64 bytes.
#[wasm_bindgen]
pub fn secret_from_seed(seed: &[u8]) -> Vec<u8> {
	__secret_from_seed(seed).to_vec()
}

/// Generate a key pair. .
///
/// * seed: UIntArray with 32 element
///
/// returned vector is the concatenation of first the private key (64 bytes)
/// followed by the public key (32) bytes.
#[wasm_bindgen]
pub fn keypair_from_seed(seed: &[u8]) -> Vec<u8> {
	__keypair_from_seed(seed).to_vec()
}

#[cfg(test)]
pub mod tests {
	extern crate wasm_bindgen_test;
	extern crate rand;
	extern crate schnorrkel;

	use hex_literal::{hex, hex_impl};
	use wasm_bindgen_test::*;
	use super::*;
	use schnorrkel::{SIGNATURE_LENGTH, KEYPAIR_LENGTH, SECRET_KEY_LENGTH};


	// to enable browser tests
	// wasm_bindgen_test_configure!(run_in_browser);

	fn generate_random_seed() -> Vec<u8> {
		(0..32).map(|_| rand::random::<u8>() ).collect()
	}

	#[wasm_bindgen_test]
	fn can_create_keypair() {
		let seed = generate_random_seed();
		let keypair = keypair_from_seed(seed.as_slice());
		assert!(keypair.len() == KEYPAIR_LENGTH);
	}

	#[wasm_bindgen_test]
	fn can_create_secret() {
		let seed = generate_random_seed();
		let secret = secret_from_seed(seed.as_slice());
		assert!(secret.len() == SECRET_KEY_LENGTH);
	}

	#[wasm_bindgen_test]
	fn can_sign_message() {
		let seed = generate_random_seed();
		let keypair = keypair_from_seed(seed.as_slice());
		let private = &keypair[0..SECRET_KEY_LENGTH];
		let public = &keypair[SECRET_KEY_LENGTH..KEYPAIR_LENGTH];
		let message = b"this is a message";
		let signature = sign(public, private, message);
		assert!(signature.len() == SIGNATURE_LENGTH);
	}

	#[wasm_bindgen_test]
	fn can_sign_message_with_ctx() {
		let seed = generate_random_seed();
		let keypair = keypair_from_seed(seed.as_slice());
		let private = &keypair[0..SECRET_KEY_LENGTH];
		let public = &keypair[SECRET_KEY_LENGTH..KEYPAIR_LENGTH];
		let message = b"this is a message";
		let ctx = b"my context";
		let signature = sign_with_ctx(public, private, message, ctx);
		assert!(signature.len() == SIGNATURE_LENGTH);
	}

	#[wasm_bindgen_test]
	fn can_verify_message() {
		let seed = generate_random_seed();
		let keypair = keypair_from_seed(seed.as_slice());
		let private = &keypair[0..SECRET_KEY_LENGTH];
		let public = &keypair[SECRET_KEY_LENGTH..KEYPAIR_LENGTH];
		let message = b"this is a message";
		let signature = sign(public, private, message);
		assert!(verify(&signature[..], message, public));
	}

	#[wasm_bindgen_test]
	fn can_verify_message_with_ctx() {
		let seed = generate_random_seed();
		let keypair = keypair_from_seed(seed.as_slice());
		let private = &keypair[0..SECRET_KEY_LENGTH];
		let public = &keypair[SECRET_KEY_LENGTH..KEYPAIR_LENGTH];
		let message = b"this is a message";
		let signing_ctx = b"a signing context";
		let signature = sign_with_ctx(public, private, message, signing_ctx);
		assert!(verify_with_ctx(&signature[..], message, public, signing_ctx));
	}

	#[wasm_bindgen_test]
	fn hard_derives_pair() {
		let cc = hex!("14416c6963650000000000000000000000000000000000000000000000000000"); // Alice
		let seed = hex!("fac7959dbfe72f052e5a0c3c8d6530f202b02fd8f9f5ca3580ec8deb7797479e");
		let expected = hex!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d");
		let keypair = __keypair_from_seed(&seed);
		let derived = derive_keypair_hard(&keypair, &cc);
		let public = &derived[SECRET_KEY_LENGTH..KEYPAIR_LENGTH];

		assert_eq!(public, expected);
	}

	#[wasm_bindgen_test]
	fn soft_derives_pair() {
		let cc = hex!("0c666f6f00000000000000000000000000000000000000000000000000000000"); // foo
		let seed = hex!("fac7959dbfe72f052e5a0c3c8d6530f202b02fd8f9f5ca3580ec8deb7797479e");
		let expected = hex!("40b9675df90efa6069ff623b0fdfcf706cd47ca7452a5056c7ad58194d23440a");
		let keypair = __keypair_from_seed(&seed);
		let derived = derive_keypair_soft(&keypair, &cc);
		let public = &derived[SECRET_KEY_LENGTH..KEYPAIR_LENGTH];

		assert_eq!(public, expected);
	}

	#[wasm_bindgen_test]
	fn soft_derives_public() {
		let cc = hex!("0c666f6f00000000000000000000000000000000000000000000000000000000"); // foo
		let public = hex!("46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a");
		let expected = hex!("40b9675df90efa6069ff623b0fdfcf706cd47ca7452a5056c7ad58194d23440a");
		let derived = derive_public_soft(&public, &cc);

		assert_eq!(derived, expected);
	}
}
