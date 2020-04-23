use crate::{
	encryption::KeyPair,
	// websocket::WebSocket,
};
use rmp_serde;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
	Blob,
	ErrorEvent,
	MessageEvent,
	WebSocket,
	Request,
	RequestInit,
	RequestMode,
	Response,
};

#[wasm_bindgen]
#[derive(Clone)]
pub struct Client {
	id: Uuid,
	name: String,
	group: String,
	key_pair: Option<KeyPair>,
	ws: WebSocket,
	signaling_server: String,
	ready: bool,

	connection_active: bool,
	connection_ready: bool,
	sync_responses: Vec<()>,
	slave_peers: HashMap<String, ()>,
	master_peers: HashMap<String, ()>,
	pins: HashMap<String, ()>,
	join_peer: Option<()>,
	join_peers: HashMap<String, ()>,
}


#[wasm_bindgen]
impl Client {
	#[wasm_bindgen(constructor)]
	pub fn new(name: String, group: String, key_pair: Option<KeyPair>, ws: WebSocket, signaling_server: String) -> Client {
		Self{
			id: Uuid::new_v4(),
			name,
			group,
			key_pair,
			ws,
			signaling_server,
			ready: false,
			connection_active: false,
			connection_ready: false,
			sync_responses: vec![],
			slave_peers: HashMap::new(),
			master_peers: HashMap::new(),
			pins: HashMap::new(),
			join_peer: None,
			join_peers: HashMap::new(),
		}
	}

	pub fn is_ready(&self) -> bool {
		self.ready
	}

	pub fn setup_listeners(self) {
		let client = Rc::new(self);

		let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
			//
		}) as Box<dyn FnMut(MessageEvent)>);

		let onopen_client = Rc::clone(&client);
		let onopen_callback = Closure::wrap(Box::new(move || {
			onopen_client.onopen_callback();
		}) as Box<dyn FnMut()>);

		client.clone().ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
		onmessage_callback.forget();

		client.clone().ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
		onopen_callback.forget();
	}

	fn connect(&self) {}

	fn join(&self) {}

	fn close(&self) {}

	async fn onopen_callback(&self) -> Result<JsValue, JsValue> {
		if self.key_pair.is_some() {
			self.connect();
			Ok(JsValue::null())
		} else {
			let mut opts = RequestInit::new();
			opts.method("GET");
			opts.mode(RequestMode::Cors);

			let url = format!("http://{}/{}", self.signaling_server, self.group);

			let request = Request::new_with_str_and_init(&url, &opts)?;
			let window = web_sys::window().unwrap();
			let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

			let resp: Response = resp_value.dyn_into().unwrap();

			// Convert this other `Promise` into a rust `Future`.
			let body = JsFuture::from(resp.blob()?).await?;
			let body: Blob = body.dyn_into()?;
			let body = JsFuture::from(body.text()).await?;

			let body = body.as_string().ok_or(JsValue::from_str("empty body while fetching clients"))?;
			let clients: Vec<String> = rmp_serde::from_slice(body.as_bytes()).map_err(|err| JsValue::from(format!("{}", err)))?;

			if clients.len() == 1 {
				self.close();
				// TODO: emit error
			} else {
				self.join();
			}

			Ok(JsValue::null())
		}
	}
}
