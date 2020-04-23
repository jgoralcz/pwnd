use aes::{
	Aes256,
	block_cipher_trait::{
		BlockCipher,
		generic_array::{GenericArray, typenum::U16},
	},
};
use rand::rngs::OsRng;
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;
use x25519_dalek::{
	PublicKey as DalekPublicKey,
	SharedSecret as DalekSharedSecret,
	StaticSecret,
};

fn crypt(pk: [u8; 32], data: &mut Vec<u8>, f: impl Fn(&Aes256, &mut GenericArray<u8, U16>)) {
	let key = GenericArray::from_slice(&pk);
	let cipher = Aes256::new(&key);

	let new_len = (data.len() + 15) / 16 * 16;
	data.resize(new_len, 0);

	let mut chunks = data.chunks_exact_mut(16);
	for chunk in &mut chunks {
		f(&cipher, GenericArray::from_mut_slice(chunk));
	}
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct KeyPair(StaticSecret);

#[wasm_bindgen]
impl KeyPair {
	pub fn generate() -> Self {
		Self(StaticSecret::new(&mut OsRng{}))
	}

	#[wasm_bindgen(getter)]
	pub fn private_key(&self) -> Vec<u8> {
		self.0.to_bytes().to_vec()
	}

	#[wasm_bindgen(getter)]
	pub fn public_key(&self) -> PublicKey {
		PublicKey(DalekPublicKey::from(&self.0))
	}

	pub fn shared_secret(&self, other: &PublicKey) -> SharedSecret {
		SharedSecret(self.0.diffie_hellman(other.raw()))
	}
}

#[wasm_bindgen]
pub struct PublicKey(DalekPublicKey);

#[wasm_bindgen]
impl PublicKey {
	fn raw(&self) -> &DalekPublicKey {
		&self.0
	}

	pub fn to_vec(&self) -> Vec<u8> {
		self.0.as_bytes().to_vec()
	}
}

#[wasm_bindgen]
pub struct SharedSecret(DalekSharedSecret);

#[wasm_bindgen]
impl SharedSecret {
	pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
		let mut original = data.to_vec();
		let mut data = u32::try_from(data.len()).unwrap().to_be_bytes().to_vec();
		data.append(&mut original);

		crypt(*self.0.as_bytes(), &mut data, Aes256::encrypt_block);
		data
	}

	pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
		let mut data = data.to_vec();
		crypt(*self.0.as_bytes(), &mut data, Aes256::decrypt_block);

		let (len, data) = data.split_at(std::mem::size_of::<u32>());
		let len = u32::from_be_bytes(*array_ref![len, 0, 4]);

		let mut data = data.to_vec();
		data.truncate(usize::try_from(len).unwrap());
		data
	}
}

#[cfg(test)]
mod test {
	use quickcheck_macros::quickcheck;
	use super::KeyPair;

	#[quickcheck]
	fn encrypt_decrypt_identity(xs: Vec<u8>) -> bool {
		let client_a = KeyPair::generate();
		let client_b = KeyPair::generate();

		let a_shared_secret = client_a.shared_secret(&client_b.public_key());
		let b_shared_secret = client_b.shared_secret(&client_a.public_key());

		let data = xs.to_vec();
		let a_encrypted = a_shared_secret.encrypt(&data);
		let b_encrypted = b_shared_secret.encrypt(&data);

		let a_decrypted = a_shared_secret.decrypt(&b_encrypted);
		let b_decrypted = b_shared_secret.decrypt(&a_encrypted);

		a_encrypted == b_encrypted &&
			a_decrypted == b_decrypted &&
			a_decrypted == data &&
			a_encrypted != a_decrypted
	}
}
