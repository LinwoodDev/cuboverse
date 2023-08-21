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
pub fn wire_add_entity__method__WorldManager(port_: MessagePort, that: JsValue, entity: String) {
    wire_add_entity__method__WorldManager_impl(port_, that, entity)
}

#[wasm_bindgen]
pub fn wire_entities__method__WorldManager(port_: MessagePort, that: JsValue) {
    wire_entities__method__WorldManager_impl(port_, that)
}

// Section: allocate functions

// Section: related functions

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
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        WorldManager {
            world: self_.get(0).wire2api(),
        }
    }
}
// Section: impl Wire2Api for JsValue

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
