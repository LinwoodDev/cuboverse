// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.81.0.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const

import 'bridge_generated.io.dart' if (dart.library.html) 'bridge_generated.web.dart';
import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';
import 'ffi.io.dart' if (dart.library.html) 'ffi.web.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;

part 'bridge_definitions.freezed.dart';

abstract class Native {
  Future<WorldManager> createWorldManager({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kCreateWorldManagerConstMeta;

  Future<int> whatIsTheAnswer({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kWhatIsTheAnswerConstMeta;

  Future<void> addEntityMethodWorldManager({required WorldManager that, required String entity, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kAddEntityMethodWorldManagerConstMeta;

  Future<int> entitiesMethodWorldManager({required WorldManager that, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kEntitiesMethodWorldManagerConstMeta;

  Stream<NativeMessage> createMessageStreamMethodWorldManager({required WorldManager that, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kCreateMessageStreamMethodWorldManagerConstMeta;

  DropFnType get dropOpaqueMutexOptionStreamSinkNativeMessage;
  ShareFnType get shareOpaqueMutexOptionStreamSinkNativeMessage;
  OpaqueTypeFinalizer get MutexOptionStreamSinkNativeMessageFinalizer;

  DropFnType get dropOpaqueMutexWorld;
  ShareFnType get shareOpaqueMutexWorld;
  OpaqueTypeFinalizer get MutexWorldFinalizer;
}

@sealed
class MutexOptionStreamSinkNativeMessage extends FrbOpaque {
  MutexOptionStreamSinkNativeMessage.fromRaw(
    int ptr,
    int size,
  ) : super.unsafe(ptr, size);
  @override
  DropFnType get dropFn => api.dropOpaqueMutexOptionStreamSinkNativeMessage;

  @override
  ShareFnType get shareFn => api.shareOpaqueMutexOptionStreamSinkNativeMessage;

  @override
  OpaqueTypeFinalizer get staticFinalizer => api.MutexOptionStreamSinkNativeMessageFinalizer;
}

@sealed
class MutexWorld extends FrbOpaque {
  MutexWorld.fromRaw(
    int ptr,
    int size,
  ) : super.unsafe(ptr, size);
  @override
  DropFnType get dropFn => api.dropOpaqueMutexWorld;

  @override
  ShareFnType get shareFn => api.shareOpaqueMutexWorld;

  @override
  OpaqueTypeFinalizer get staticFinalizer => api.MutexWorldFinalizer;
}

class BlockInformation {
  final String name;
  final ChunkPosition position;

  const BlockInformation({
    required this.name,
    required this.position,
  });
}

class ChunkLocation {
  final int field0;
  final int field1;
  final int field2;

  const ChunkLocation({
    required this.field0,
    required this.field1,
    required this.field2,
  });
}

class ChunkPosition {
  final int field0;
  final int field1;
  final int field2;

  const ChunkPosition({
    required this.field0,
    required this.field1,
    required this.field2,
  });
}

class GlobalPosition {
  final ChunkLocation field0;
  final ChunkPosition field1;

  const GlobalPosition({
    required this.field0,
    required this.field1,
  });
}

@freezed
class NativeMessage with _$NativeMessage {
  const factory NativeMessage.addBlock({
    required ChunkLocation chunk,
    required BlockInformation block,
  }) = NativeMessage_AddBlock;
  const factory NativeMessage.removeBlock({
    required ChunkPosition position,
    required ChunkLocation chunk,
  }) = NativeMessage_RemoveBlock;
  const factory NativeMessage.addChunk({
    required ChunkLocation location,
    required List<BlockInformation> blocks,
  }) = NativeMessage_AddChunk;
  const factory NativeMessage.removeChunk({
    required ChunkLocation location,
  }) = NativeMessage_RemoveChunk;
  const factory NativeMessage.playerTeleported(
    GlobalPosition field0,
  ) = NativeMessage_PlayerTeleported;
}

class WorldManager {
  final MutexWorld world;
  final MutexOptionStreamSinkNativeMessage sink;

  const WorldManager({
    required this.world,
    required this.sink,
  });

  Future<void> addEntity({required String entity, dynamic hint}) => api.addEntityMethodWorldManager(
        that: this,
        entity: entity,
      );

  Future<int> entities({dynamic hint}) => api.entitiesMethodWorldManager(
        that: this,
      );

  Stream<NativeMessage> createMessageStream({dynamic hint}) => api.createMessageStreamMethodWorldManager(
        that: this,
      );
}
