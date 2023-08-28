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
pub fn wire_close__method__WorldManager(that: JsValue) -> support::WireSyncReturn {
    wire_close__method__WorldManager_impl(that)
}

#[wasm_bindgen]
pub fn wire_player_position__method__WorldManager(port_: MessagePort, that: JsValue) {
    wire_player_position__method__WorldManager_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_move_player__method__WorldManager(
    port_: MessagePort,
    that: JsValue,
    x: JsValue,
    y: JsValue,
    z: JsValue,
    relative: JsValue,
    teleport: JsValue,
) {
    wire_move_player__method__WorldManager_impl(port_, that, x, y, z, relative, teleport)
}

#[wasm_bindgen]
pub fn wire_player_on_ground__method__WorldManager(port_: MessagePort, that: JsValue) {
    wire_player_on_ground__method__WorldManager_impl(port_, that)
}

// Section: allocate functions

// Section: related functions

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
pub fn drop_opaque_MutexVecChunkLocation(ptr: *const c_void) {
    unsafe {
        Arc::<Mutex<Vec<ChunkLocation>>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_MutexVecChunkLocation(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Mutex<Vec<ChunkLocation>>>::increment_strong_count(ptr as _);
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

#[wasm_bindgen]
pub fn drop_opaque_MutexWorldMessenger(ptr: *const c_void) {
    unsafe {
        Arc::<Mutex<WorldMessenger>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_MutexWorldMessenger(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Mutex<WorldMessenger>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_MutexWorldTicker(ptr: *const c_void) {
    unsafe {
        Arc::<Mutex<WorldTicker>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_MutexWorldTicker(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Mutex<WorldTicker>>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<String> for String {
    fn wire2api(self) -> String {
        self
    }
}
impl Wire2Api<BlockPosition> for JsValue {
    fn wire2api(self) -> BlockPosition {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            3,
            "Expected 3 elements, got {}",
            self_.length()
        );
        BlockPosition(
            self_.get(0).wire2api(),
            self_.get(1).wire2api(),
            self_.get(2).wire2api(),
        )
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

impl Wire2Api<GlobalBlockPosition> for JsValue {
    fn wire2api(self) -> GlobalBlockPosition {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        GlobalBlockPosition(self_.get(0).wire2api(), self_.get(1).wire2api())
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
            5,
            "Expected 5 elements, got {}",
            self_.length()
        );
        WorldManager {
            world: self_.get(0).wire2api(),
            messenger: self_.get(1).wire2api(),
            loaded_chunks: self_.get(2).wire2api(),
            player: self_.get(3).wire2api(),
            update_thread: self_.get(4).wire2api(),
        }
    }
}
// Section: impl Wire2Api for JsValue

impl Wire2Api<RustOpaque<Mutex<Player>>> for JsValue {
    fn wire2api(self) -> RustOpaque<Mutex<Player>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<RustOpaque<Mutex<Vec<ChunkLocation>>>> for JsValue {
    fn wire2api(self) -> RustOpaque<Mutex<Vec<ChunkLocation>>> {
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
impl Wire2Api<RustOpaque<Mutex<WorldMessenger>>> for JsValue {
    fn wire2api(self) -> RustOpaque<Mutex<WorldMessenger>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<RustOpaque<Mutex<WorldTicker>>> for JsValue {
    fn wire2api(self) -> RustOpaque<Mutex<WorldTicker>> {
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
impl Wire2Api<bool> for JsValue {
    fn wire2api(self) -> bool {
        self.is_truthy()
    }
}
impl Wire2Api<f64> for JsValue {
    fn wire2api(self) -> f64 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<i32> for JsValue {
    fn wire2api(self) -> i32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<i8> for JsValue {
    fn wire2api(self) -> i8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Option<bool>> for JsValue {
    fn wire2api(self) -> Option<bool> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<f64>> for JsValue {
    fn wire2api(self) -> Option<f64> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
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
