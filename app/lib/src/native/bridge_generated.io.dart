// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.81.0.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const

import "bridge_definitions.dart";
import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';
import 'ffi.io.dart' if (dart.library.html) 'ffi.web.dart';
import 'bridge_generated.dart';
export 'bridge_generated.dart';
import 'dart:ffi' as ffi;

class NativePlatform extends FlutterRustBridgeBase<NativeWire> {
  NativePlatform(ffi.DynamicLibrary dylib) : super(NativeWire(dylib));

// Section: api2wire

  @protected
  wire_MutexOptionStreamSinkNativeMessage api2wire_MutexOptionStreamSinkNativeMessage(MutexOptionStreamSinkNativeMessage raw) {
    final ptr = inner.new_MutexOptionStreamSinkNativeMessage();
    _api_fill_to_wire_MutexOptionStreamSinkNativeMessage(raw, ptr);
    return ptr;
  }

  @protected
  wire_MutexWorld api2wire_MutexWorld(MutexWorld raw) {
    final ptr = inner.new_MutexWorld();
    _api_fill_to_wire_MutexWorld(raw, ptr);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_String(String raw) {
    return api2wire_uint_8_list(utf8.encoder.convert(raw));
  }

  @protected
  ffi.Pointer<wire_WorldManager> api2wire_box_autoadd_world_manager(WorldManager raw) {
    final ptr = inner.new_box_autoadd_world_manager_0();
    _api_fill_to_wire_world_manager(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_uint_8_list(Uint8List raw) {
    final ans = inner.new_uint_8_list_0(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

// Section: finalizer

  late final OpaqueTypeFinalizer _MutexOptionStreamSinkNativeMessageFinalizer = OpaqueTypeFinalizer(inner._drop_opaque_MutexOptionStreamSinkNativeMessagePtr);
  OpaqueTypeFinalizer get MutexOptionStreamSinkNativeMessageFinalizer => _MutexOptionStreamSinkNativeMessageFinalizer;
  late final OpaqueTypeFinalizer _MutexWorldFinalizer = OpaqueTypeFinalizer(inner._drop_opaque_MutexWorldPtr);
  OpaqueTypeFinalizer get MutexWorldFinalizer => _MutexWorldFinalizer;
// Section: api_fill_to_wire

  void _api_fill_to_wire_MutexOptionStreamSinkNativeMessage(MutexOptionStreamSinkNativeMessage apiObj, wire_MutexOptionStreamSinkNativeMessage wireObj) {
    wireObj.ptr = apiObj.shareOrMove();
  }

  void _api_fill_to_wire_MutexWorld(MutexWorld apiObj, wire_MutexWorld wireObj) {
    wireObj.ptr = apiObj.shareOrMove();
  }

  void _api_fill_to_wire_box_autoadd_world_manager(WorldManager apiObj, ffi.Pointer<wire_WorldManager> wireObj) {
    _api_fill_to_wire_world_manager(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_world_manager(WorldManager apiObj, wire_WorldManager wireObj) {
    wireObj.world = api2wire_MutexWorld(apiObj.world);
    wireObj.sink = api2wire_MutexOptionStreamSinkNativeMessage(apiObj.sink);
  }
}

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names

// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.
// ignore_for_file: type=lint

/// generated by flutter_rust_bridge
class NativeWire implements FlutterRustBridgeWireBase {
  @internal
  late final dartApi = DartApiDl(init_frb_dart_api_dl);

  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName) _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  NativeWire(ffi.DynamicLibrary dynamicLibrary) : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  NativeWire.fromLookup(ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName) lookup) : _lookup = lookup;

  void store_dart_post_cobject(
    DartPostCObjectFnType ptr,
  ) {
    return _store_dart_post_cobject(
      ptr,
    );
  }

  late final _store_dart_post_cobjectPtr = _lookup<ffi.NativeFunction<ffi.Void Function(DartPostCObjectFnType)>>('store_dart_post_cobject');
  late final _store_dart_post_cobject = _store_dart_post_cobjectPtr.asFunction<void Function(DartPostCObjectFnType)>();

  Object get_dart_object(
    int ptr,
  ) {
    return _get_dart_object(
      ptr,
    );
  }

  late final _get_dart_objectPtr = _lookup<ffi.NativeFunction<ffi.Handle Function(ffi.UintPtr)>>('get_dart_object');
  late final _get_dart_object = _get_dart_objectPtr.asFunction<Object Function(int)>();

  void drop_dart_object(
    int ptr,
  ) {
    return _drop_dart_object(
      ptr,
    );
  }

  late final _drop_dart_objectPtr = _lookup<ffi.NativeFunction<ffi.Void Function(ffi.UintPtr)>>('drop_dart_object');
  late final _drop_dart_object = _drop_dart_objectPtr.asFunction<void Function(int)>();

  int new_dart_opaque(
    Object handle,
  ) {
    return _new_dart_opaque(
      handle,
    );
  }

  late final _new_dart_opaquePtr = _lookup<ffi.NativeFunction<ffi.UintPtr Function(ffi.Handle)>>('new_dart_opaque');
  late final _new_dart_opaque = _new_dart_opaquePtr.asFunction<int Function(Object)>();

  int init_frb_dart_api_dl(
    ffi.Pointer<ffi.Void> obj,
  ) {
    return _init_frb_dart_api_dl(
      obj,
    );
  }

  late final _init_frb_dart_api_dlPtr = _lookup<ffi.NativeFunction<ffi.IntPtr Function(ffi.Pointer<ffi.Void>)>>('init_frb_dart_api_dl');
  late final _init_frb_dart_api_dl = _init_frb_dart_api_dlPtr.asFunction<int Function(ffi.Pointer<ffi.Void>)>();

  void wire_create_world_manager(
    int port_,
  ) {
    return _wire_create_world_manager(
      port_,
    );
  }

  late final _wire_create_world_managerPtr = _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>('wire_create_world_manager');
  late final _wire_create_world_manager = _wire_create_world_managerPtr.asFunction<void Function(int)>();

  void wire_what_is_the_answer(
    int port_,
  ) {
    return _wire_what_is_the_answer(
      port_,
    );
  }

  late final _wire_what_is_the_answerPtr = _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>('wire_what_is_the_answer');
  late final _wire_what_is_the_answer = _wire_what_is_the_answerPtr.asFunction<void Function(int)>();

  void wire_add_entity__method__WorldManager(
    int port_,
    ffi.Pointer<wire_WorldManager> that,
    ffi.Pointer<wire_uint_8_list> entity,
  ) {
    return _wire_add_entity__method__WorldManager(
      port_,
      that,
      entity,
    );
  }

  late final _wire_add_entity__method__WorldManagerPtr = _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Pointer<wire_WorldManager>, ffi.Pointer<wire_uint_8_list>)>>('wire_add_entity__method__WorldManager');
  late final _wire_add_entity__method__WorldManager = _wire_add_entity__method__WorldManagerPtr.asFunction<void Function(int, ffi.Pointer<wire_WorldManager>, ffi.Pointer<wire_uint_8_list>)>();

  void wire_entities__method__WorldManager(
    int port_,
    ffi.Pointer<wire_WorldManager> that,
  ) {
    return _wire_entities__method__WorldManager(
      port_,
      that,
    );
  }

  late final _wire_entities__method__WorldManagerPtr = _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Pointer<wire_WorldManager>)>>('wire_entities__method__WorldManager');
  late final _wire_entities__method__WorldManager = _wire_entities__method__WorldManagerPtr.asFunction<void Function(int, ffi.Pointer<wire_WorldManager>)>();

  void wire_create_message_stream__method__WorldManager(
    int port_,
    ffi.Pointer<wire_WorldManager> that,
  ) {
    return _wire_create_message_stream__method__WorldManager(
      port_,
      that,
    );
  }

  late final _wire_create_message_stream__method__WorldManagerPtr = _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Pointer<wire_WorldManager>)>>('wire_create_message_stream__method__WorldManager');
  late final _wire_create_message_stream__method__WorldManager = _wire_create_message_stream__method__WorldManagerPtr.asFunction<void Function(int, ffi.Pointer<wire_WorldManager>)>();

  wire_MutexOptionStreamSinkNativeMessage new_MutexOptionStreamSinkNativeMessage() {
    return _new_MutexOptionStreamSinkNativeMessage();
  }

  late final _new_MutexOptionStreamSinkNativeMessagePtr = _lookup<ffi.NativeFunction<wire_MutexOptionStreamSinkNativeMessage Function()>>('new_MutexOptionStreamSinkNativeMessage');
  late final _new_MutexOptionStreamSinkNativeMessage = _new_MutexOptionStreamSinkNativeMessagePtr.asFunction<wire_MutexOptionStreamSinkNativeMessage Function()>();

  wire_MutexWorld new_MutexWorld() {
    return _new_MutexWorld();
  }

  late final _new_MutexWorldPtr = _lookup<ffi.NativeFunction<wire_MutexWorld Function()>>('new_MutexWorld');
  late final _new_MutexWorld = _new_MutexWorldPtr.asFunction<wire_MutexWorld Function()>();

  ffi.Pointer<wire_WorldManager> new_box_autoadd_world_manager_0() {
    return _new_box_autoadd_world_manager_0();
  }

  late final _new_box_autoadd_world_manager_0Ptr = _lookup<ffi.NativeFunction<ffi.Pointer<wire_WorldManager> Function()>>('new_box_autoadd_world_manager_0');
  late final _new_box_autoadd_world_manager_0 = _new_box_autoadd_world_manager_0Ptr.asFunction<ffi.Pointer<wire_WorldManager> Function()>();

  ffi.Pointer<wire_uint_8_list> new_uint_8_list_0(
    int len,
  ) {
    return _new_uint_8_list_0(
      len,
    );
  }

  late final _new_uint_8_list_0Ptr = _lookup<ffi.NativeFunction<ffi.Pointer<wire_uint_8_list> Function(ffi.Int32)>>('new_uint_8_list_0');
  late final _new_uint_8_list_0 = _new_uint_8_list_0Ptr.asFunction<ffi.Pointer<wire_uint_8_list> Function(int)>();

  void drop_opaque_MutexOptionStreamSinkNativeMessage(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _drop_opaque_MutexOptionStreamSinkNativeMessage(
      ptr,
    );
  }

  late final _drop_opaque_MutexOptionStreamSinkNativeMessagePtr = _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>('drop_opaque_MutexOptionStreamSinkNativeMessage');
  late final _drop_opaque_MutexOptionStreamSinkNativeMessage = _drop_opaque_MutexOptionStreamSinkNativeMessagePtr.asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  ffi.Pointer<ffi.Void> share_opaque_MutexOptionStreamSinkNativeMessage(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _share_opaque_MutexOptionStreamSinkNativeMessage(
      ptr,
    );
  }

  late final _share_opaque_MutexOptionStreamSinkNativeMessagePtr = _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Void>)>>('share_opaque_MutexOptionStreamSinkNativeMessage');
  late final _share_opaque_MutexOptionStreamSinkNativeMessage = _share_opaque_MutexOptionStreamSinkNativeMessagePtr.asFunction<ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Void>)>();

  void drop_opaque_MutexWorld(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _drop_opaque_MutexWorld(
      ptr,
    );
  }

  late final _drop_opaque_MutexWorldPtr = _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>('drop_opaque_MutexWorld');
  late final _drop_opaque_MutexWorld = _drop_opaque_MutexWorldPtr.asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  ffi.Pointer<ffi.Void> share_opaque_MutexWorld(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _share_opaque_MutexWorld(
      ptr,
    );
  }

  late final _share_opaque_MutexWorldPtr = _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Void>)>>('share_opaque_MutexWorld');
  late final _share_opaque_MutexWorld = _share_opaque_MutexWorldPtr.asFunction<ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Void>)>();

  void free_WireSyncReturn(
    WireSyncReturn ptr,
  ) {
    return _free_WireSyncReturn(
      ptr,
    );
  }

  late final _free_WireSyncReturnPtr = _lookup<ffi.NativeFunction<ffi.Void Function(WireSyncReturn)>>('free_WireSyncReturn');
  late final _free_WireSyncReturn = _free_WireSyncReturnPtr.asFunction<void Function(WireSyncReturn)>();
}

final class _Dart_Handle extends ffi.Opaque {}

final class wire_MutexWorld extends ffi.Struct {
  external ffi.Pointer<ffi.Void> ptr;
}

final class wire_MutexOptionStreamSinkNativeMessage extends ffi.Struct {
  external ffi.Pointer<ffi.Void> ptr;
}

final class wire_WorldManager extends ffi.Struct {
  external wire_MutexWorld world;

  external wire_MutexOptionStreamSinkNativeMessage sink;
}

final class wire_uint_8_list extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

typedef DartPostCObjectFnType = ffi.Pointer<ffi.NativeFunction<ffi.Bool Function(DartPort port_id, ffi.Pointer<ffi.Void> message)>>;
typedef DartPort = ffi.Int64;
