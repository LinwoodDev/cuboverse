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
pub extern "C" fn wire_add_block__method__WorldManager(
    port_: i64,
    that: *mut wire_WorldManager,
    position: *mut wire_GlobalBlockPosition,
    block: *mut wire_uint_8_list,
) {
    wire_add_block__method__WorldManager_impl(port_, that, position, block)
}

#[no_mangle]
pub extern "C" fn wire_remove_block__method__WorldManager(
    port_: i64,
    that: *mut wire_WorldManager,
    position: *mut wire_GlobalBlockPosition,
) {
    wire_remove_block__method__WorldManager_impl(port_, that, position)
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

#[no_mangle]
pub extern "C" fn wire_create_message_stream__method__WorldManager(
    port_: i64,
    that: *mut wire_WorldManager,
) {
    wire_create_message_stream__method__WorldManager_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_close__method__WorldManager(
    that: *mut wire_WorldManager,
) -> support::WireSyncReturn {
    wire_close__method__WorldManager_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_player_position__method__WorldManager(
    port_: i64,
    that: *mut wire_WorldManager,
) {
    wire_player_position__method__WorldManager_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_move_player__method__WorldManager(
    port_: i64,
    that: *mut wire_WorldManager,
    x: *mut f64,
    y: *mut f64,
    z: *mut f64,
    relative: *mut bool,
    teleport: *mut bool,
) {
    wire_move_player__method__WorldManager_impl(port_, that, x, y, z, relative, teleport)
}

#[no_mangle]
pub extern "C" fn wire_player_on_ground__method__WorldManager(
    port_: i64,
    that: *mut wire_WorldManager,
) {
    wire_player_on_ground__method__WorldManager_impl(port_, that)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_BoxChunkGenerator() -> wire_BoxChunkGenerator {
    wire_BoxChunkGenerator::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_MutexPlayer() -> wire_MutexPlayer {
    wire_MutexPlayer::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_MutexVecChunkLocation() -> wire_MutexVecChunkLocation {
    wire_MutexVecChunkLocation::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_MutexWorld() -> wire_MutexWorld {
    wire_MutexWorld::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_MutexWorldMessenger() -> wire_MutexWorldMessenger {
    wire_MutexWorldMessenger::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_MutexWorldTicker() -> wire_MutexWorldTicker {
    wire_MutexWorldTicker::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_bool_0(value: bool) -> *mut bool {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_f64_0(value: f64) -> *mut f64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_global_block_position_0() -> *mut wire_GlobalBlockPosition {
    support::new_leak_box_ptr(wire_GlobalBlockPosition::new_with_null_ptr())
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
pub extern "C" fn drop_opaque_BoxChunkGenerator(ptr: *const c_void) {
    unsafe {
        Arc::<Box<dyn ChunkGenerator>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_BoxChunkGenerator(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Box<dyn ChunkGenerator>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_MutexPlayer(ptr: *const c_void) {
    unsafe {
        Arc::<Mutex<Player>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_MutexPlayer(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Mutex<Player>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_MutexVecChunkLocation(ptr: *const c_void) {
    unsafe {
        Arc::<Mutex<Vec<ChunkLocation>>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_MutexVecChunkLocation(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Mutex<Vec<ChunkLocation>>>::increment_strong_count(ptr as _);
        ptr
    }
}

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

#[no_mangle]
pub extern "C" fn drop_opaque_MutexWorldMessenger(ptr: *const c_void) {
    unsafe {
        Arc::<Mutex<WorldMessenger>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_MutexWorldMessenger(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Mutex<WorldMessenger>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_MutexWorldTicker(ptr: *const c_void) {
    unsafe {
        Arc::<Mutex<WorldTicker>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_MutexWorldTicker(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Mutex<WorldTicker>>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<RustOpaque<Box<dyn ChunkGenerator>>> for wire_BoxChunkGenerator {
    fn wire2api(self) -> RustOpaque<Box<dyn ChunkGenerator>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<Mutex<Player>>> for wire_MutexPlayer {
    fn wire2api(self) -> RustOpaque<Mutex<Player>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<Mutex<Vec<ChunkLocation>>>> for wire_MutexVecChunkLocation {
    fn wire2api(self) -> RustOpaque<Mutex<Vec<ChunkLocation>>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<Mutex<World>>> for wire_MutexWorld {
    fn wire2api(self) -> RustOpaque<Mutex<World>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<Mutex<WorldMessenger>>> for wire_MutexWorldMessenger {
    fn wire2api(self) -> RustOpaque<Mutex<WorldMessenger>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<Mutex<WorldTicker>>> for wire_MutexWorldTicker {
    fn wire2api(self) -> RustOpaque<Mutex<WorldTicker>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<BlockPosition> for wire_BlockPosition {
    fn wire2api(self) -> BlockPosition {
        BlockPosition(
            self.field0.wire2api(),
            self.field1.wire2api(),
            self.field2.wire2api(),
        )
    }
}

impl Wire2Api<bool> for *mut bool {
    fn wire2api(self) -> bool {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<f64> for *mut f64 {
    fn wire2api(self) -> f64 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<GlobalBlockPosition> for *mut wire_GlobalBlockPosition {
    fn wire2api(self) -> GlobalBlockPosition {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<GlobalBlockPosition>::wire2api(*wrap).into()
    }
}
impl Wire2Api<WorldManager> for *mut wire_WorldManager {
    fn wire2api(self) -> WorldManager {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<WorldManager>::wire2api(*wrap).into()
    }
}
impl Wire2Api<ChunkLocation> for wire_ChunkLocation {
    fn wire2api(self) -> ChunkLocation {
        ChunkLocation(
            self.field0.wire2api(),
            self.field1.wire2api(),
            self.field2.wire2api(),
        )
    }
}

impl Wire2Api<GlobalBlockPosition> for wire_GlobalBlockPosition {
    fn wire2api(self) -> GlobalBlockPosition {
        GlobalBlockPosition(self.field0.wire2api(), self.field1.wire2api())
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
            messenger: self.messenger.wire2api(),
            loaded_chunks: self.loaded_chunks.wire2api(),
            chunk_generator: self.chunk_generator.wire2api(),
            player: self.player.wire2api(),
            update_thread: self.update_thread.wire2api(),
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_BoxChunkGenerator {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MutexPlayer {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MutexVecChunkLocation {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MutexWorld {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MutexWorldMessenger {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MutexWorldTicker {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_BlockPosition {
    field0: i8,
    field1: i8,
    field2: i8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ChunkLocation {
    field0: i32,
    field1: i32,
    field2: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_GlobalBlockPosition {
    field0: wire_ChunkLocation,
    field1: wire_BlockPosition,
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
    messenger: wire_MutexWorldMessenger,
    loaded_chunks: wire_MutexVecChunkLocation,
    chunk_generator: wire_BoxChunkGenerator,
    player: wire_MutexPlayer,
    update_thread: wire_MutexWorldTicker,
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

impl NewWithNullPtr for wire_BoxChunkGenerator {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_MutexPlayer {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_MutexVecChunkLocation {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_MutexWorld {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_MutexWorldMessenger {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_MutexWorldTicker {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}

impl NewWithNullPtr for wire_BlockPosition {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
            field1: Default::default(),
            field2: Default::default(),
        }
    }
}

impl Default for wire_BlockPosition {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_ChunkLocation {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
            field1: Default::default(),
            field2: Default::default(),
        }
    }
}

impl Default for wire_ChunkLocation {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_GlobalBlockPosition {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
            field1: Default::default(),
        }
    }
}

impl Default for wire_GlobalBlockPosition {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_WorldManager {
    fn new_with_null_ptr() -> Self {
        Self {
            world: wire_MutexWorld::new_with_null_ptr(),
            messenger: wire_MutexWorldMessenger::new_with_null_ptr(),
            loaded_chunks: wire_MutexVecChunkLocation::new_with_null_ptr(),
            chunk_generator: wire_BoxChunkGenerator::new_with_null_ptr(),
            player: wire_MutexPlayer::new_with_null_ptr(),
            update_thread: wire_MutexWorldTicker::new_with_null_ptr(),
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
