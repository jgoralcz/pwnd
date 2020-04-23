use std::time::Instant;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(PartialEq, Eq, Debug)]
pub enum SecretType {
	Empty = 0,
	Login = 1,
}

#[wasm_bindgen]
#[derive(PartialEq, Eq, Debug)]
pub enum SecretFieldType {
	Text = 0,
	Hidden = 1,
}

#[wasm_bindgen]
#[derive(PartialEq, Eq, Debug)]
pub struct Secret {
	id: String,
	name: String,
	type_: SecretType,
	icon: Option<String>,
	data: Vec<SecretSection>,
	custom: Vec<SecretSection>,
	notes: Option<String>,
	updated_at: Option<Instant>,
	created_at: Instant,
}

#[wasm_bindgen]
#[derive(PartialEq, Eq, Debug)]
pub struct SecretSection {
	name: Option<String>,
	fields: Vec<SecretField>,
}

#[wasm_bindgen]
#[derive(PartialEq, Eq, Debug)]
pub struct SecretField {
	name: String,
	value: String,
	r#type: SecretFieldType,
}

impl Secret {
	pub fn new(name: String) -> Self {
		Self{
			id: Uuid::new_v4().to_string(),
			type_: SecretType::Empty,
			name,
			icon: None,
			data: vec![],
			custom: vec![],
			notes: None,
			updated_at: None,
			created_at: Instant::now(),
		}
	}

	pub fn new_login(name: String, username: String, password: String) -> Self {
		Self{
			id: Uuid::new_v4().to_string(),
			type_: SecretType::Login,
			name,
			icon: None,
			data: vec![SecretSection{
				name: None,
				fields: vec![
					SecretField{
						name: "username".to_string(),
						value: username,
						r#type: SecretFieldType::Text,
					},
					SecretField{
						name: "password".to_string(),
						value: password,
						r#type: SecretFieldType::Hidden,
					},
				],
			}],
			custom: vec![],
			notes: None,
			updated_at: None,
			created_at: Instant::now(),
		}
	}

	pub fn name(&self) -> &str {
		return &self.name
	}
}

pub trait SecretStore {
	fn list(&self) -> Result<Vec<Secret>, String>;
	fn add(&self, secret: &Secret) -> Result<(), String>;
	fn get(&self, name: &str) -> Result<Option<Secret>, String>;
}
