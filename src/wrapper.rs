
use schnorrkel::keys::*;
use schnorrkel::context::{signing_context};
use schnorrkel::derive::{Derivation, ChainCode, CHAIN_CODE_LENGTH};
use schnorrkel::sign::{Signature,SIGNATURE_LENGTH};

// We must make sure that this is the same as declared in the substrate source code.
const SIGNING_CTX: &'static [u8] = b"substrate";

/// Private helper function.
fn keypair_from_seed(seed: &[u8]) -> Keypair {
	let mini_key: MiniSecretKey = MiniSecretKey::from_bytes(seed)
		.expect("32 bytes can always build a key; qed");
	mini_key.expand_to_keypair()
}

/// ChainCode construction helper
fn create_cc(data: &[u8]) -> ChainCode {
	let mut cc = [0u8; CHAIN_CODE_LENGTH];

	cc.copy_from_slice(&data);

	ChainCode(cc)
}

pub fn __derive_keypair_hard(pair: &[u8], cc: &[u8]) -> [u8; KEYPAIR_LENGTH] {
	let derived = match Keypair::from_bytes(pair) {
		Ok(kp) => kp.hard_derive_mini_secret_key(Some(create_cc(cc)), &[]).0.expand_to_keypair(),
		Err(_) => panic!("Provided pair is invalid.")
	};
	let mut kp = [0u8; KEYPAIR_LENGTH];
	kp.copy_from_slice(&derived.to_bytes());
	kp
}

pub fn __derive_keypair_soft(pair: &[u8], cc: &[u8]) -> [u8; KEYPAIR_LENGTH] {
	let derived = match Keypair::from_bytes(pair) {
		Ok(kp) => kp.derived_key_simple(create_cc(cc), &[]).0,
		Err(_) => panic!("Provided pair is invalid.")
	};
	let mut kp = [0u8; KEYPAIR_LENGTH];
	kp.copy_from_slice(&derived.to_bytes());
	kp
}

pub fn __derive_public_soft(public: &[u8], cc: &[u8]) -> [u8; PUBLIC_KEY_LENGTH] {
	let derived = match PublicKey::from_bytes(public) {
		Ok(pk) => pk.derived_key_simple(create_cc(cc), &[]).0,
		Err(_) => panic!("Provided publickey is invalid.")
	};
	let mut pk = [0u8; PUBLIC_KEY_LENGTH];
	pk.copy_from_slice(&derived.to_bytes());
	pk
}

pub fn __keypair_from_seed(seed: &[u8]) -> [u8; KEYPAIR_LENGTH] {
	let keypair = keypair_from_seed(seed).to_bytes();
	let mut kp = [0u8; KEYPAIR_LENGTH];
	kp.copy_from_slice(&keypair);
	kp
}

pub fn __secret_from_seed(seed: &[u8]) -> [u8; SECRET_KEY_LENGTH] {
	let secret = keypair_from_seed(seed).secret.to_bytes();
	let mut s = [0u8; SECRET_KEY_LENGTH];
	s.copy_from_slice(&secret);
	s
}

pub fn __verify(signature: &[u8], message: &[u8], pubkey: &[u8]) -> bool {
	let sig = match Signature::from_bytes(signature) {
		Ok(some_sig) => some_sig,
		Err(_) => return false
	};
	let pk = match PublicKey::from_bytes(pubkey) {
		Ok(some_pk) => some_pk,
		Err(_) => return false
	};
	pk.verify_simple(SIGNING_CTX, message, &sig)
}

pub fn __sign(public: &[u8], private: &[u8], message: &[u8]) -> [u8; SIGNATURE_LENGTH] {
	// despite being a method of KeyPair, only the secret is used for signing.
	let secret = match SecretKey::from_bytes(private) {
		Ok(some_secret) => some_secret,
		Err(_) => panic!("Provided private key is invalid.")
	};

	let public = match PublicKey::from_bytes(public) {
		Ok(some_public) => some_public,
		Err(_) => panic!("Provided public key is invalid.")
	};

	let context = signing_context(SIGNING_CTX);
	secret.sign(context.bytes(message), &public).to_bytes()
}
