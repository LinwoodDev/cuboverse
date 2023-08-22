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
    position: *mut wire_GlobalPosition,
    block: *mut wire_uint_8_list,
) {
    wire_add_block__method__WorldManager_impl(port_, that, position, block)
}

#[no_mangle]
pub extern "C" fn wire_remove_block__method__WorldManager(
    port_: i64,
    that: *mut wire_WorldManager,
    position: *mut wire_GlobalPosition,
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
pub extern "C" fn wire_player_position__method__WorldManager(
    port_: i64,
    that: *mut wire_WorldManager,
) {
    wire_player_position__method__WorldManager_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_send_message__method__WorldManager(
    port_: i64,
    that: *mut wire_WorldManager,
    message: *mut wire_NativeMessage,
) {
    wire_send_message__method__WorldManager_impl(port_, that, message)
}

#[no_mangle]
pub extern "C" fn wire_move_player__method__WorldManager(
    port_: i64,
    that: *mut wire_WorldManager,
    x: i64,
    y: i64,
    z: i64,
) {
    wire_move_player__method__WorldManager_impl(port_, that, x, y, z)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_MutexOptionStreamSinkNativeMessage() -> wire_MutexOptionStreamSinkNativeMessage
{
    wire_MutexOptionStreamSinkNativeMessage::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_MutexPlayer() -> wire_MutexPlayer {
    wire_MutexPlayer::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_MutexWorld() -> wire_MutexWorld {
    wire_MutexWorld::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_block_information_0() -> *mut wire_BlockInformation {
    support::new_leak_box_ptr(wire_BlockInformation::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_chunk_location_0() -> *mut wire_ChunkLocation {
    support::new_leak_box_ptr(wire_ChunkLocation::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_chunk_position_0() -> *mut wire_ChunkPosition {
    support::new_leak_box_ptr(wire_ChunkPosition::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_global_position_0() -> *mut wire_GlobalPosition {
    support::new_leak_box_ptr(wire_GlobalPosition::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_native_message_0() -> *mut wire_NativeMessage {
    support::new_leak_box_ptr(wire_NativeMessage::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_world_manager_0() -> *mut wire_WorldManager {
    support::new_leak_box_ptr(wire_WorldManager::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_list_block_information_0(len: i32) -> *mut wire_list_block_information {
    let wrap = wire_list_block_information {
        ptr: support::new_leak_vec_ptr(<wire_BlockInformation>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
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
pub extern "C" fn drop_opaque_MutexOptionStreamSinkNativeMessage(ptr: *const c_void) {
    unsafe {
        Arc::<Mutex<Option<StreamSink<NativeMessage>>>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_MutexOptionStreamSinkNativeMessage(
    ptr: *const c_void,
) -> *const c_void {
    unsafe {
        Arc::<Mutex<Option<StreamSink<NativeMessage>>>>::increment_strong_count(ptr as _);
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

impl Wire2Api<RustOpaque<Mutex<Option<StreamSink<NativeMessage>>>>>
    for wire_MutexOptionStreamSinkNativeMessage
{
    fn wire2api(self) -> RustOpaque<Mutex<Option<StreamSink<NativeMessage>>>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<Mutex<Player>>> for wire_MutexPlayer {
    fn wire2api(self) -> RustOpaque<Mutex<Player>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
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
impl Wire2Api<BlockInformation> for wire_BlockInformation {
    fn wire2api(self) -> BlockInformation {
        BlockInformation {
            name: self.name.wire2api(),
            position: self.position.wire2api(),
        }
    }
}
impl Wire2Api<BlockInformation> for *mut wire_BlockInformation {
    fn wire2api(self) -> BlockInformation {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<BlockInformation>::wire2api(*wrap).into()
    }
}
impl Wire2Api<ChunkLocation> for *mut wire_ChunkLocation {
    fn wire2api(self) -> ChunkLocation {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ChunkLocation>::wire2api(*wrap).into()
    }
}
impl Wire2Api<ChunkPosition> for *mut wire_ChunkPosition {
    fn wire2api(self) -> ChunkPosition {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ChunkPosition>::wire2api(*wrap).into()
    }
}
impl Wire2Api<GlobalPosition> for *mut wire_GlobalPosition {
    fn wire2api(self) -> GlobalPosition {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<GlobalPosition>::wire2api(*wrap).into()
    }
}
impl Wire2Api<NativeMessage> for *mut wire_NativeMessage {
    fn wire2api(self) -> NativeMessage {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<NativeMessage>::wire2api(*wrap).into()
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
impl Wire2Api<ChunkPosition> for wire_ChunkPosition {
    fn wire2api(self) -> ChunkPosition {
        ChunkPosition(
            self.field0.wire2api(),
            self.field1.wire2api(),
            self.field2.wire2api(),
        )
    }
}
impl Wire2Api<GlobalPosition> for wire_GlobalPosition {
    fn wire2api(self) -> GlobalPosition {
        GlobalPosition(self.field0.wire2api(), self.field1.wire2api())
    }
}

impl Wire2Api<Vec<BlockInformation>> for *mut wire_list_block_information {
    fn wire2api(self) -> Vec<BlockInformation> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<NativeMessage> for wire_NativeMessage {
    fn wire2api(self) -> NativeMessage {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.AddBlock);
                NativeMessage::AddBlock {
                    chunk: ans.chunk.wire2api(),
                    block: ans.block.wire2api(),
                }
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.RemoveBlock);
                NativeMessage::RemoveBlock {
                    position: ans.position.wire2api(),
                    chunk: ans.chunk.wire2api(),
                }
            },
            2 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.AddChunk);
                NativeMessage::AddChunk {
                    location: ans.location.wire2api(),
                    blocks: ans.blocks.wire2api(),
                }
            },
            3 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.RemoveChunk);
                NativeMessage::RemoveChunk {
                    location: ans.location.wire2api(),
                }
            },
            4 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.PlayerTeleported);
                NativeMessage::PlayerTeleported {
                    x: ans.x.wire2api(),
                    y: ans.y.wire2api(),
                    z: ans.z.wire2api(),
                }
            },
            _ => unreachable!(),
        }
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
            sink: self.sink.wire2api(),
            player: self.player.wire2api(),
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_MutexOptionStreamSinkNativeMessage {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MutexPlayer {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MutexWorld {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_BlockInformation {
    name: *mut wire_uint_8_list,
    position: wire_ChunkPosition,
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
pub struct wire_ChunkPosition {
    field0: i8,
    field1: i8,
    field2: i8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_GlobalPosition {
    field0: wire_ChunkLocation,
    field1: wire_ChunkPosition,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_block_information {
    ptr: *mut wire_BlockInformation,
    len: i32,
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
    sink: wire_MutexOptionStreamSinkNativeMessage,
    player: wire_MutexPlayer,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_NativeMessage {
    tag: i32,
    kind: *mut NativeMessageKind,
}

#[repr(C)]
pub union NativeMessageKind {
    AddBlock: *mut wire_NativeMessage_AddBlock,
    RemoveBlock: *mut wire_NativeMessage_RemoveBlock,
    AddChunk: *mut wire_NativeMessage_AddChunk,
    RemoveChunk: *mut wire_NativeMessage_RemoveChunk,
    PlayerTeleported: *mut wire_NativeMessage_PlayerTeleported,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_NativeMessage_AddBlock {
    chunk: *mut wire_ChunkLocation,
    block: *mut wire_BlockInformation,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_NativeMessage_RemoveBlock {
    position: *mut wire_ChunkPosition,
    chunk: *mut wire_ChunkLocation,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_NativeMessage_AddChunk {
    location: *mut wire_ChunkLocation,
    blocks: *mut wire_list_block_information,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_NativeMessage_RemoveChunk {
    location: *mut wire_ChunkLocation,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_NativeMessage_PlayerTeleported {
    x: i64,
    y: i64,
    z: i64,
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

impl NewWithNullPtr for wire_MutexOptionStreamSinkNativeMessage {
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
impl NewWithNullPtr for wire_MutexWorld {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}

impl NewWithNullPtr for wire_BlockInformation {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
            position: Default::default(),
        }
    }
}

impl Default for wire_BlockInformation {
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

impl NewWithNullPtr for wire_ChunkPosition {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
            field1: Default::default(),
            field2: Default::default(),
        }
    }
}

impl Default for wire_ChunkPosition {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_GlobalPosition {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
            field1: Default::default(),
        }
    }
}

impl Default for wire_GlobalPosition {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl Default for wire_NativeMessage {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_NativeMessage {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_NativeMessage_AddBlock() -> *mut NativeMessageKind {
    support::new_leak_box_ptr(NativeMessageKind {
        AddBlock: support::new_leak_box_ptr(wire_NativeMessage_AddBlock {
            chunk: core::ptr::null_mut(),
            block: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_NativeMessage_RemoveBlock() -> *mut NativeMessageKind {
    support::new_leak_box_ptr(NativeMessageKind {
        RemoveBlock: support::new_leak_box_ptr(wire_NativeMessage_RemoveBlock {
            position: core::ptr::null_mut(),
            chunk: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_NativeMessage_AddChunk() -> *mut NativeMessageKind {
    support::new_leak_box_ptr(NativeMessageKind {
        AddChunk: support::new_leak_box_ptr(wire_NativeMessage_AddChunk {
            location: core::ptr::null_mut(),
            blocks: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_NativeMessage_RemoveChunk() -> *mut NativeMessageKind {
    support::new_leak_box_ptr(NativeMessageKind {
        RemoveChunk: support::new_leak_box_ptr(wire_NativeMessage_RemoveChunk {
            location: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_NativeMessage_PlayerTeleported() -> *mut NativeMessageKind {
    support::new_leak_box_ptr(NativeMessageKind {
        PlayerTeleported: support::new_leak_box_ptr(wire_NativeMessage_PlayerTeleported {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }),
    })
}

impl NewWithNullPtr for wire_WorldManager {
    fn new_with_null_ptr() -> Self {
        Self {
            world: wire_MutexWorld::new_with_null_ptr(),
            sink: wire_MutexOptionStreamSinkNativeMessage::new_with_null_ptr(),
            player: wire_MutexPlayer::new_with_null_ptr(),
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
