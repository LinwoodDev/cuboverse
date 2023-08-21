use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_create_world_manager(port_: i64) {
    wire_create_world_manager_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_what_is_the_answer(port_: i64) {
    wire_what_is_the_answer_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_add_entity__method__WorldManager(
    port_: i64,
    that: *mut wire_WorldManager,
    entity: *mut wire_uint_8_list,
) {
    wire_add_entity__method__WorldManager_impl(port_, that, entity)
}

#[no_mangle]
pub extern "C" fn wire_entities__method__WorldManager(port_: i64, that: *mut wire_WorldManager) {
    wire_entities__method__WorldManager_impl(port_, that)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_MutexWorld() -> wire_MutexWorld {
    wire_MutexWorld::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_world_manager_0() -> *mut wire_WorldManager {
    support::new_leak_box_ptr(wire_WorldManager::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

#[no_mangle]
pub extern "C" fn drop_opaque_MutexWorld(ptr: *const c_void) {
    unsafe {
        Arc::<Mutex<World>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_MutexWorld(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Mutex<World>>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<RustOpaque<Mutex<World>>> for wire_MutexWorld {
    fn wire2api(self) -> RustOpaque<Mutex<World>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<WorldManager> for *mut wire_WorldManager {
    fn wire2api(self) -> WorldManager {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<WorldManager>::wire2api(*wrap).into()
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<WorldManager> for wire_WorldManager {
    fn wire2api(self) -> WorldManager {
        WorldManager {
            world: self.world.wire2api(),
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_MutexWorld {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_WorldManager {
    world: wire_MutexWorld,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_MutexWorld {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}

impl NewWithNullPtr for wire_WorldManager {
    fn new_with_null_ptr() -> Self {
        Self {
            world: wire_MutexWorld::new_with_null_ptr(),
        }
    }
}

impl Default for wire_WorldManager {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
