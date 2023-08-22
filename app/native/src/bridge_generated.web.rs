use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_create_world_manager(port_: MessagePort) {
    wire_create_world_manager_impl(port_)
}

#[wasm_bindgen]
pub fn wire_what_is_the_answer(port_: MessagePort) {
    wire_what_is_the_answer_impl(port_)
}

#[wasm_bindgen]
pub fn wire_add_block__method__WorldManager(
    port_: MessagePort,
    that: JsValue,
    position: JsValue,
    block: String,
) {
    wire_add_block__method__WorldManager_impl(port_, that, position, block)
}

#[wasm_bindgen]
pub fn wire_remove_block__method__WorldManager(
    port_: MessagePort,
    that: JsValue,
    position: JsValue,
) {
    wire_remove_block__method__WorldManager_impl(port_, that, position)
}

#[wasm_bindgen]
pub fn wire_add_entity__method__WorldManager(port_: MessagePort, that: JsValue, entity: String) {
    wire_add_entity__method__WorldManager_impl(port_, that, entity)
}

#[wasm_bindgen]
pub fn wire_entities__method__WorldManager(port_: MessagePort, that: JsValue) {
    wire_entities__method__WorldManager_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_create_message_stream__method__WorldManager(port_: MessagePort, that: JsValue) {
    wire_create_message_stream__method__WorldManager_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_player_position__method__WorldManager(port_: MessagePort, that: JsValue) {
    wire_player_position__method__WorldManager_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_send_message__method__WorldManager(
    port_: MessagePort,
    that: JsValue,
    message: JsValue,
) {
    wire_send_message__method__WorldManager_impl(port_, that, message)
}

#[wasm_bindgen]
pub fn wire_move_player__method__WorldManager(
    port_: MessagePort,
    that: JsValue,
    x: i64,
    y: i64,
    z: i64,
) {
    wire_move_player__method__WorldManager_impl(port_, that, x, y, z)
}

// Section: allocate functions

// Section: related functions

#[wasm_bindgen]
pub fn drop_opaque_MutexOptionStreamSinkNativeMessage(ptr: *const c_void) {
    unsafe {
        Arc::<Mutex<Option<StreamSink<NativeMessage>>>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_MutexOptionStreamSinkNativeMessage(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Mutex<Option<StreamSink<NativeMessage>>>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_MutexPlayer(ptr: *const c_void) {
    unsafe {
        Arc::<Mutex<Player>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_MutexPlayer(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Mutex<Player>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_MutexWorld(ptr: *const c_void) {
    unsafe {
        Arc::<Mutex<World>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_MutexWorld(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Mutex<World>>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<String> for String {
    fn wire2api(self) -> String {
        self
    }
}
impl Wire2Api<BlockInformation> for JsValue {
    fn wire2api(self) -> BlockInformation {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        BlockInformation {
            name: self_.get(0).wire2api(),
            position: self_.get(1).wire2api(),
        }
    }
}

impl Wire2Api<ChunkLocation> for JsValue {
    fn wire2api(self) -> ChunkLocation {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            3,
            "Expected 3 elements, got {}",
            self_.length()
        );
        ChunkLocation(
            self_.get(0).wire2api(),
            self_.get(1).wire2api(),
            self_.get(2).wire2api(),
        )
    }
}
impl Wire2Api<ChunkPosition> for JsValue {
    fn wire2api(self) -> ChunkPosition {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            3,
            "Expected 3 elements, got {}",
            self_.length()
        );
        ChunkPosition(
            self_.get(0).wire2api(),
            self_.get(1).wire2api(),
            self_.get(2).wire2api(),
        )
    }
}
impl Wire2Api<GlobalPosition> for JsValue {
    fn wire2api(self) -> GlobalPosition {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        GlobalPosition(self_.get(0).wire2api(), self_.get(1).wire2api())
    }
}

impl Wire2Api<Vec<BlockInformation>> for JsValue {
    fn wire2api(self) -> Vec<BlockInformation> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<NativeMessage> for JsValue {
    fn wire2api(self) -> NativeMessage {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => NativeMessage::AddBlock {
                chunk: self_.get(1).wire2api(),
                block: self_.get(2).wire2api(),
            },
            1 => NativeMessage::RemoveBlock {
                position: self_.get(1).wire2api(),
                chunk: self_.get(2).wire2api(),
            },
            2 => NativeMessage::AddChunk {
                location: self_.get(1).wire2api(),
                blocks: self_.get(2).wire2api(),
            },
            3 => NativeMessage::RemoveChunk {
                location: self_.get(1).wire2api(),
            },
            4 => NativeMessage::PlayerTeleported {
                x: self_.get(1).wire2api(),
                y: self_.get(2).wire2api(),
                z: self_.get(3).wire2api(),
            },
            _ => unreachable!(),
        }
    }
}

impl Wire2Api<Vec<u8>> for Box<[u8]> {
    fn wire2api(self) -> Vec<u8> {
        self.into_vec()
    }
}
impl Wire2Api<WorldManager> for JsValue {
    fn wire2api(self) -> WorldManager {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            3,
            "Expected 3 elements, got {}",
            self_.length()
        );
        WorldManager {
            world: self_.get(0).wire2api(),
            sink: self_.get(1).wire2api(),
            player: self_.get(2).wire2api(),
        }
    }
}
// Section: impl Wire2Api for JsValue

impl Wire2Api<RustOpaque<Mutex<Option<StreamSink<NativeMessage>>>>> for JsValue {
    fn wire2api(self) -> RustOpaque<Mutex<Option<StreamSink<NativeMessage>>>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<RustOpaque<Mutex<Player>>> for JsValue {
    fn wire2api(self) -> RustOpaque<Mutex<Player>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<RustOpaque<Mutex<World>>> for JsValue {
    fn wire2api(self) -> RustOpaque<Mutex<World>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<String> for JsValue {
    fn wire2api(self) -> String {
        self.as_string().expect("non-UTF-8 string, or not a string")
    }
}
impl Wire2Api<i32> for JsValue {
    fn wire2api(self) -> i32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<i64> for JsValue {
    fn wire2api(self) -> i64 {
        ::std::convert::TryInto::try_into(self.dyn_into::<js_sys::BigInt>().unwrap()).unwrap()
    }
}
impl Wire2Api<i8> for JsValue {
    fn wire2api(self) -> i8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<u8> for JsValue {
    fn wire2api(self) -> u8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Vec<u8>> for JsValue {
    fn wire2api(self) -> Vec<u8> {
        self.unchecked_into::<js_sys::Uint8Array>().to_vec().into()
    }
}
