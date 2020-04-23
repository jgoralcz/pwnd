use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum WebSocketOpCodes {
	MasterSignal = 0,
	SlaveSignal = 1,
	JoinRequest = 2,
	Join = 3,
	JoinInitiatorSignal = 4,
	JoinSignal = 5,
	JoinSuccess = 6,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum RTCOpCodes {
	SyncRequest = 0,
	SyncResponse = 1,
	SyncTruth = 2,
	Update = 3,
	Ping = 4,
	Pong = 5,
	JoinResponse = 6,
}
