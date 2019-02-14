extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate schnorrkel;

mod wrapper;
use wrapper::*;

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
pub fn sign(secret: &[u8], message: &[u8]) -> Vec<u8> {
	__sign(secret, message).to_vec()
}

/// Verify a message and its corresponding against a public key;
/// 
/// * signature: UIntArray with 64 element
/// * message: Arbitrary length UIntArray
/// * pubkey: UIntArray with 32 element
#[wasm_bindgen]
pub fn verify(signature: &[u8], message: &[u8], pubkey: &[u8]) -> bool {
	__verify(signature, message, pubkey)
}

#[wasm_bindgen]
pub fn expand_to_public(secret: &[u8]) -> Vec<u8> {
	
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
	fn can_verify_message() {
		let seed = generate_random_seed();
		let keypair = keypair_from_seed(seed.as_slice());
		let private = &keypair[0..SECRET_KEY_LENGTH];
		let public = &keypair[SECRET_KEY_LENGTH..KEYPAIR_LENGTH];
		let message = b"this is a message";
		let signature = sign(public, private, message);
		assert!(verify(&signature[..], message, public));
	}
}