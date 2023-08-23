#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.81.0.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_create_world_manager_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, WorldManager>(
        WrapInfo {
            debug_name: "create_world_manager",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(create_world_manager()),
    )
}
fn wire_what_is_the_answer_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, i32>(
        WrapInfo {
            debug_name: "what_is_the_answer",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(what_is_the_answer()),
    )
}
fn wire_add_block__method__WorldManager_impl(
    port_: MessagePort,
    that: impl Wire2Api<WorldManager> + UnwindSafe,
    position: impl Wire2Api<GlobalBlockPosition> + UnwindSafe,
    block: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "add_block__method__WorldManager",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_position = position.wire2api();
            let api_block = block.wire2api();
            move |task_callback| Ok(WorldManager::add_block(&api_that, api_position, api_block))
        },
    )
}
fn wire_remove_block__method__WorldManager_impl(
    port_: MessagePort,
    that: impl Wire2Api<WorldManager> + UnwindSafe,
    position: impl Wire2Api<GlobalBlockPosition> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "remove_block__method__WorldManager",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_position = position.wire2api();
            move |task_callback| Ok(WorldManager::remove_block(&api_that, api_position))
        },
    )
}
fn wire_add_entity__method__WorldManager_impl(
    port_: MessagePort,
    that: impl Wire2Api<WorldManager> + UnwindSafe,
    entity: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "add_entity__method__WorldManager",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_entity = entity.wire2api();
            move |task_callback| Ok(WorldManager::add_entity(&api_that, api_entity))
        },
    )
}
fn wire_entities__method__WorldManager_impl(
    port_: MessagePort,
    that: impl Wire2Api<WorldManager> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, usize>(
        WrapInfo {
            debug_name: "entities__method__WorldManager",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(WorldManager::entities(&api_that))
        },
    )
}
fn wire_create_message_stream__method__WorldManager_impl(
    port_: MessagePort,
    that: impl Wire2Api<WorldManager> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "create_message_stream__method__WorldManager",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| {
                Ok(WorldManager::create_message_stream(
                    &api_that,
                    task_callback.stream_sink::<_, NativeMessage>(),
                ))
            }
        },
    )
}
fn wire_close__method__WorldManager_impl(
    port_: MessagePort,
    that: impl Wire2Api<WorldManager> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "close__method__WorldManager",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(WorldManager::close(&api_that))
        },
    )
}
fn wire_player_position__method__WorldManager_impl(
    port_: MessagePort,
    that: impl Wire2Api<WorldManager> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, mirror_GlobalEntityPosition>(
        WrapInfo {
            debug_name: "player_position__method__WorldManager",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(WorldManager::player_position(&api_that))
        },
    )
}
fn wire_move_player__method__WorldManager_impl(
    port_: MessagePort,
    that: impl Wire2Api<WorldManager> + UnwindSafe,
    x: impl Wire2Api<f64> + UnwindSafe,
    y: impl Wire2Api<f64> + UnwindSafe,
    z: impl Wire2Api<f64> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "move_player__method__WorldManager",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_x = x.wire2api();
            let api_y = y.wire2api();
            let api_z = z.wire2api();
            move |task_callback| Ok(WorldManager::move_player(&api_that, api_x, api_y, api_z))
        },
    )
}
// Section: wrapper structs

#[derive(Clone)]
pub struct mirror_BlockPosition(BlockPosition);

#[derive(Clone)]
pub struct mirror_ChunkLocation(ChunkLocation);

#[derive(Clone)]
pub struct mirror_EntityPosition(EntityPosition);

#[derive(Clone)]
pub struct mirror_GlobalEntityPosition(GlobalEntityPosition);

// Section: static checks

const _: fn() = || {
    {
        let BlockPosition_ = None::<BlockPosition>.unwrap();
        let _: i8 = BlockPosition_.0;
        let _: i8 = BlockPosition_.1;
        let _: i8 = BlockPosition_.2;
    }
    {
        let ChunkLocation_ = None::<ChunkLocation>.unwrap();
        let _: i32 = ChunkLocation_.0;
        let _: i32 = ChunkLocation_.1;
        let _: i32 = ChunkLocation_.2;
    }
    {
        let EntityPosition_ = None::<EntityPosition>.unwrap();
        let _: f32 = EntityPosition_.0;
        let _: f32 = EntityPosition_.1;
        let _: f32 = EntityPosition_.2;
    }
    {
        let GlobalEntityPosition_ = None::<GlobalEntityPosition>.unwrap();
        let _: ChunkLocation = GlobalEntityPosition_.0;
        let _: EntityPosition = GlobalEntityPosition_.1;
    }
};
// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<f64> for f64 {
    fn wire2api(self) -> f64 {
        self
    }
}

impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}
impl Wire2Api<i8> for i8 {
    fn wire2api(self) -> i8 {
        self
    }
}
impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

impl support::IntoDart for BlockInformation {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.name.into_into_dart().into_dart(),
            self.position.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for BlockInformation {}
impl rust2dart::IntoIntoDart<BlockInformation> for BlockInformation {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for mirror_BlockPosition {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.0 .0.into_into_dart().into_dart(),
            self.0 .1.into_into_dart().into_dart(),
            self.0 .2.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for mirror_BlockPosition {}
impl rust2dart::IntoIntoDart<mirror_BlockPosition> for BlockPosition {
    fn into_into_dart(self) -> mirror_BlockPosition {
        mirror_BlockPosition(self)
    }
}

impl support::IntoDart for mirror_ChunkLocation {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.0 .0.into_into_dart().into_dart(),
            self.0 .1.into_into_dart().into_dart(),
            self.0 .2.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for mirror_ChunkLocation {}
impl rust2dart::IntoIntoDart<mirror_ChunkLocation> for ChunkLocation {
    fn into_into_dart(self) -> mirror_ChunkLocation {
        mirror_ChunkLocation(self)
    }
}

impl support::IntoDart for mirror_EntityPosition {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.0 .0.into_into_dart().into_dart(),
            self.0 .1.into_into_dart().into_dart(),
            self.0 .2.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for mirror_EntityPosition {}
impl rust2dart::IntoIntoDart<mirror_EntityPosition> for EntityPosition {
    fn into_into_dart(self) -> mirror_EntityPosition {
        mirror_EntityPosition(self)
    }
}

impl support::IntoDart for mirror_GlobalEntityPosition {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.0 .0.into_into_dart().into_dart(),
            self.0 .1.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for mirror_GlobalEntityPosition {}
impl rust2dart::IntoIntoDart<mirror_GlobalEntityPosition> for GlobalEntityPosition {
    fn into_into_dart(self) -> mirror_GlobalEntityPosition {
        mirror_GlobalEntityPosition(self)
    }
}

impl support::IntoDart for NativeMessage {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::AddBlock { chunk, block } => vec![
                0.into_dart(),
                chunk.into_into_dart().into_dart(),
                block.into_into_dart().into_dart(),
            ],
            Self::RemoveBlock { position, chunk } => vec![
                1.into_dart(),
                position.into_into_dart().into_dart(),
                chunk.into_into_dart().into_dart(),
            ],
            Self::AddChunk { location, blocks } => vec![
                2.into_dart(),
                location.into_into_dart().into_dart(),
                blocks.into_into_dart().into_dart(),
            ],
            Self::RemoveChunk { location } => {
                vec![3.into_dart(), location.into_into_dart().into_dart()]
            }
            Self::PlayerTeleported { x, y, z } => vec![
                4.into_dart(),
                x.into_into_dart().into_dart(),
                y.into_into_dart().into_dart(),
                z.into_into_dart().into_dart(),
            ],
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for NativeMessage {}
impl rust2dart::IntoIntoDart<NativeMessage> for NativeMessage {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for WorldManager {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.world.into_dart(),
            self.messenger.into_dart(),
            self.player.into_dart(),
            self.update_thread.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for WorldManager {}
impl rust2dart::IntoIntoDart<WorldManager> for WorldManager {
    fn into_into_dart(self) -> Self {
        self
    }
}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
#[path = "bridge_generated.web.rs"]
mod web;
#[cfg(target_family = "wasm")]
pub use web::*;

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
