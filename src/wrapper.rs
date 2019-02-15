use schnorrkel::keys::*;
use schnorrkel::context::{signing_context}; 
use schnorrkel::sign::{Signature,SIGNATURE_LENGTH};
use schnorrkel::SignatureError;

use sha2::Sha512;

// We must make sure that this is the same as declared in the substrate source code.
const SIGNING_CTX: &'static [u8] = b"substrate transaction";

pub fn __sign(secret: &[u8], message: &[u8]) -> Result<[u8; SIGNATURE_LENGTH], SignatureError> {
	let secret_key = SecretKey::from_bytes(secret)?;
	// TODO: This can be used once the new version of schnorrkel is out on crates.io
	// let kp = secret_key.to_keypair();
	// let context = signing_context(SIGNING_CTX);
	// Ok(kp.sign(context.bytes(message)).to_bytes())

	// Temporary replacement method:
	let public = secret_key.to_public();
	let kp = Keypair {
		public: public,
		secret: secret_key
	};

	let context = signing_context(SIGNING_CTX);
	Ok(kp.sign(context.bytes(message)).to_bytes())
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

/// Private helper function.
fn keypair_from_seed(seed: &[u8]) -> Result<Keypair, SignatureError>  {
	let mini_key: MiniSecretKey = MiniSecretKey::from_bytes(seed)?;
	Ok(mini_key.expand_to_keypair::<Sha512>())
}

pub fn __keypair_from_seed(seed: &[u8]) -> Result<[u8; KEYPAIR_LENGTH], SignatureError> {
	let keypair = keypair_from_seed(seed)?.to_bytes();
	let mut kp = [0u8; KEYPAIR_LENGTH];
	kp.copy_from_slice(&keypair);
	Ok(kp)
}

pub fn __secret_from_seed(seed: &[u8]) -> Result<[u8; SECRET_KEY_LENGTH], SignatureError> {
	let secret = keypair_from_seed(seed)?.secret.to_bytes();
	let mut s = [0u8; SECRET_KEY_LENGTH];
	s.copy_from_slice(&secret);
	Ok(s)
}

pub fn __expand_to_public(secret: &[u8]) -> Result<[u8; PUBLIC_KEY_LENGTH], SignatureError> {
	Ok(SecretKey::from_bytes(secret)?.to_public().to_bytes())
}
