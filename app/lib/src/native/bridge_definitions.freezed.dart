// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'bridge_definitions.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError('It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$NativeMessage {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(ChunkLocation chunk, BlockInformation block) addBlock,
    required TResult Function(ChunkPosition position, ChunkLocation chunk) removeBlock,
    required TResult Function(ChunkLocation location, List<BlockInformation> blocks) addChunk,
    required TResult Function(ChunkLocation location) removeChunk,
    required TResult Function(int x, int y, int z) playerTeleported,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(ChunkLocation chunk, BlockInformation block)? addBlock,
    TResult? Function(ChunkPosition position, ChunkLocation chunk)? removeBlock,
    TResult? Function(ChunkLocation location, List<BlockInformation> blocks)? addChunk,
    TResult? Function(ChunkLocation location)? removeChunk,
    TResult? Function(int x, int y, int z)? playerTeleported,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(ChunkLocation chunk, BlockInformation block)? addBlock,
    TResult Function(ChunkPosition position, ChunkLocation chunk)? removeBlock,
    TResult Function(ChunkLocation location, List<BlockInformation> blocks)? addChunk,
    TResult Function(ChunkLocation location)? removeChunk,
    TResult Function(int x, int y, int z)? playerTeleported,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(NativeMessage_AddBlock value) addBlock,
    required TResult Function(NativeMessage_RemoveBlock value) removeBlock,
    required TResult Function(NativeMessage_AddChunk value) addChunk,
    required TResult Function(NativeMessage_RemoveChunk value) removeChunk,
    required TResult Function(NativeMessage_PlayerTeleported value) playerTeleported,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(NativeMessage_AddBlock value)? addBlock,
    TResult? Function(NativeMessage_RemoveBlock value)? removeBlock,
    TResult? Function(NativeMessage_AddChunk value)? addChunk,
    TResult? Function(NativeMessage_RemoveChunk value)? removeChunk,
    TResult? Function(NativeMessage_PlayerTeleported value)? playerTeleported,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(NativeMessage_AddBlock value)? addBlock,
    TResult Function(NativeMessage_RemoveBlock value)? removeBlock,
    TResult Function(NativeMessage_AddChunk value)? addChunk,
    TResult Function(NativeMessage_RemoveChunk value)? removeChunk,
    TResult Function(NativeMessage_PlayerTeleported value)? playerTeleported,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $NativeMessageCopyWith<$Res> {
  factory $NativeMessageCopyWith(NativeMessage value, $Res Function(NativeMessage) then) = _$NativeMessageCopyWithImpl<$Res, NativeMessage>;
}

/// @nodoc
class _$NativeMessageCopyWithImpl<$Res, $Val extends NativeMessage> implements $NativeMessageCopyWith<$Res> {
  _$NativeMessageCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$NativeMessage_AddBlockCopyWith<$Res> {
  factory _$$NativeMessage_AddBlockCopyWith(_$NativeMessage_AddBlock value, $Res Function(_$NativeMessage_AddBlock) then) = __$$NativeMessage_AddBlockCopyWithImpl<$Res>;
  @useResult
  $Res call({ChunkLocation chunk, BlockInformation block});
}

/// @nodoc
class __$$NativeMessage_AddBlockCopyWithImpl<$Res> extends _$NativeMessageCopyWithImpl<$Res, _$NativeMessage_AddBlock> implements _$$NativeMessage_AddBlockCopyWith<$Res> {
  __$$NativeMessage_AddBlockCopyWithImpl(_$NativeMessage_AddBlock _value, $Res Function(_$NativeMessage_AddBlock) _then) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? chunk = null,
    Object? block = null,
  }) {
    return _then(_$NativeMessage_AddBlock(
      chunk: null == chunk
          ? _value.chunk
          : chunk // ignore: cast_nullable_to_non_nullable
              as ChunkLocation,
      block: null == block
          ? _value.block
          : block // ignore: cast_nullable_to_non_nullable
              as BlockInformation,
    ));
  }
}

/// @nodoc

class _$NativeMessage_AddBlock implements NativeMessage_AddBlock {
  const _$NativeMessage_AddBlock({required this.chunk, required this.block});

  @override
  final ChunkLocation chunk;
  @override
  final BlockInformation block;

  @override
  String toString() {
    return 'NativeMessage.addBlock(chunk: $chunk, block: $block)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$NativeMessage_AddBlock && (identical(other.chunk, chunk) || other.chunk == chunk) && (identical(other.block, block) || other.block == block));
  }

  @override
  int get hashCode => Object.hash(runtimeType, chunk, block);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$NativeMessage_AddBlockCopyWith<_$NativeMessage_AddBlock> get copyWith => __$$NativeMessage_AddBlockCopyWithImpl<_$NativeMessage_AddBlock>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(ChunkLocation chunk, BlockInformation block) addBlock,
    required TResult Function(ChunkPosition position, ChunkLocation chunk) removeBlock,
    required TResult Function(ChunkLocation location, List<BlockInformation> blocks) addChunk,
    required TResult Function(ChunkLocation location) removeChunk,
    required TResult Function(int x, int y, int z) playerTeleported,
  }) {
    return addBlock(chunk, block);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(ChunkLocation chunk, BlockInformation block)? addBlock,
    TResult? Function(ChunkPosition position, ChunkLocation chunk)? removeBlock,
    TResult? Function(ChunkLocation location, List<BlockInformation> blocks)? addChunk,
    TResult? Function(ChunkLocation location)? removeChunk,
    TResult? Function(int x, int y, int z)? playerTeleported,
  }) {
    return addBlock?.call(chunk, block);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(ChunkLocation chunk, BlockInformation block)? addBlock,
    TResult Function(ChunkPosition position, ChunkLocation chunk)? removeBlock,
    TResult Function(ChunkLocation location, List<BlockInformation> blocks)? addChunk,
    TResult Function(ChunkLocation location)? removeChunk,
    TResult Function(int x, int y, int z)? playerTeleported,
    required TResult orElse(),
  }) {
    if (addBlock != null) {
      return addBlock(chunk, block);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(NativeMessage_AddBlock value) addBlock,
    required TResult Function(NativeMessage_RemoveBlock value) removeBlock,
    required TResult Function(NativeMessage_AddChunk value) addChunk,
    required TResult Function(NativeMessage_RemoveChunk value) removeChunk,
    required TResult Function(NativeMessage_PlayerTeleported value) playerTeleported,
  }) {
    return addBlock(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(NativeMessage_AddBlock value)? addBlock,
    TResult? Function(NativeMessage_RemoveBlock value)? removeBlock,
    TResult? Function(NativeMessage_AddChunk value)? addChunk,
    TResult? Function(NativeMessage_RemoveChunk value)? removeChunk,
    TResult? Function(NativeMessage_PlayerTeleported value)? playerTeleported,
  }) {
    return addBlock?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(NativeMessage_AddBlock value)? addBlock,
    TResult Function(NativeMessage_RemoveBlock value)? removeBlock,
    TResult Function(NativeMessage_AddChunk value)? addChunk,
    TResult Function(NativeMessage_RemoveChunk value)? removeChunk,
    TResult Function(NativeMessage_PlayerTeleported value)? playerTeleported,
    required TResult orElse(),
  }) {
    if (addBlock != null) {
      return addBlock(this);
    }
    return orElse();
  }
}

abstract class NativeMessage_AddBlock implements NativeMessage {
  const factory NativeMessage_AddBlock({required final ChunkLocation chunk, required final BlockInformation block}) = _$NativeMessage_AddBlock;

  ChunkLocation get chunk;
  BlockInformation get block;
  @JsonKey(ignore: true)
  _$$NativeMessage_AddBlockCopyWith<_$NativeMessage_AddBlock> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$NativeMessage_RemoveBlockCopyWith<$Res> {
  factory _$$NativeMessage_RemoveBlockCopyWith(_$NativeMessage_RemoveBlock value, $Res Function(_$NativeMessage_RemoveBlock) then) = __$$NativeMessage_RemoveBlockCopyWithImpl<$Res>;
  @useResult
  $Res call({ChunkPosition position, ChunkLocation chunk});
}

/// @nodoc
class __$$NativeMessage_RemoveBlockCopyWithImpl<$Res> extends _$NativeMessageCopyWithImpl<$Res, _$NativeMessage_RemoveBlock> implements _$$NativeMessage_RemoveBlockCopyWith<$Res> {
  __$$NativeMessage_RemoveBlockCopyWithImpl(_$NativeMessage_RemoveBlock _value, $Res Function(_$NativeMessage_RemoveBlock) _then) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? position = null,
    Object? chunk = null,
  }) {
    return _then(_$NativeMessage_RemoveBlock(
      position: null == position
          ? _value.position
          : position // ignore: cast_nullable_to_non_nullable
              as ChunkPosition,
      chunk: null == chunk
          ? _value.chunk
          : chunk // ignore: cast_nullable_to_non_nullable
              as ChunkLocation,
    ));
  }
}

/// @nodoc

class _$NativeMessage_RemoveBlock implements NativeMessage_RemoveBlock {
  const _$NativeMessage_RemoveBlock({required this.position, required this.chunk});

  @override
  final ChunkPosition position;
  @override
  final ChunkLocation chunk;

  @override
  String toString() {
    return 'NativeMessage.removeBlock(position: $position, chunk: $chunk)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$NativeMessage_RemoveBlock && (identical(other.position, position) || other.position == position) && (identical(other.chunk, chunk) || other.chunk == chunk));
  }

  @override
  int get hashCode => Object.hash(runtimeType, position, chunk);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$NativeMessage_RemoveBlockCopyWith<_$NativeMessage_RemoveBlock> get copyWith => __$$NativeMessage_RemoveBlockCopyWithImpl<_$NativeMessage_RemoveBlock>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(ChunkLocation chunk, BlockInformation block) addBlock,
    required TResult Function(ChunkPosition position, ChunkLocation chunk) removeBlock,
    required TResult Function(ChunkLocation location, List<BlockInformation> blocks) addChunk,
    required TResult Function(ChunkLocation location) removeChunk,
    required TResult Function(int x, int y, int z) playerTeleported,
  }) {
    return removeBlock(position, chunk);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(ChunkLocation chunk, BlockInformation block)? addBlock,
    TResult? Function(ChunkPosition position, ChunkLocation chunk)? removeBlock,
    TResult? Function(ChunkLocation location, List<BlockInformation> blocks)? addChunk,
    TResult? Function(ChunkLocation location)? removeChunk,
    TResult? Function(int x, int y, int z)? playerTeleported,
  }) {
    return removeBlock?.call(position, chunk);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(ChunkLocation chunk, BlockInformation block)? addBlock,
    TResult Function(ChunkPosition position, ChunkLocation chunk)? removeBlock,
    TResult Function(ChunkLocation location, List<BlockInformation> blocks)? addChunk,
    TResult Function(ChunkLocation location)? removeChunk,
    TResult Function(int x, int y, int z)? playerTeleported,
    required TResult orElse(),
  }) {
    if (removeBlock != null) {
      return removeBlock(position, chunk);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(NativeMessage_AddBlock value) addBlock,
    required TResult Function(NativeMessage_RemoveBlock value) removeBlock,
    required TResult Function(NativeMessage_AddChunk value) addChunk,
    required TResult Function(NativeMessage_RemoveChunk value) removeChunk,
    required TResult Function(NativeMessage_PlayerTeleported value) playerTeleported,
  }) {
    return removeBlock(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(NativeMessage_AddBlock value)? addBlock,
    TResult? Function(NativeMessage_RemoveBlock value)? removeBlock,
    TResult? Function(NativeMessage_AddChunk value)? addChunk,
    TResult? Function(NativeMessage_RemoveChunk value)? removeChunk,
    TResult? Function(NativeMessage_PlayerTeleported value)? playerTeleported,
  }) {
    return removeBlock?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(NativeMessage_AddBlock value)? addBlock,
    TResult Function(NativeMessage_RemoveBlock value)? removeBlock,
    TResult Function(NativeMessage_AddChunk value)? addChunk,
    TResult Function(NativeMessage_RemoveChunk value)? removeChunk,
    TResult Function(NativeMessage_PlayerTeleported value)? playerTeleported,
    required TResult orElse(),
  }) {
    if (removeBlock != null) {
      return removeBlock(this);
    }
    return orElse();
  }
}

abstract class NativeMessage_RemoveBlock implements NativeMessage {
  const factory NativeMessage_RemoveBlock({required final ChunkPosition position, required final ChunkLocation chunk}) = _$NativeMessage_RemoveBlock;

  ChunkPosition get position;
  ChunkLocation get chunk;
  @JsonKey(ignore: true)
  _$$NativeMessage_RemoveBlockCopyWith<_$NativeMessage_RemoveBlock> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$NativeMessage_AddChunkCopyWith<$Res> {
  factory _$$NativeMessage_AddChunkCopyWith(_$NativeMessage_AddChunk value, $Res Function(_$NativeMessage_AddChunk) then) = __$$NativeMessage_AddChunkCopyWithImpl<$Res>;
  @useResult
  $Res call({ChunkLocation location, List<BlockInformation> blocks});
}

/// @nodoc
class __$$NativeMessage_AddChunkCopyWithImpl<$Res> extends _$NativeMessageCopyWithImpl<$Res, _$NativeMessage_AddChunk> implements _$$NativeMessage_AddChunkCopyWith<$Res> {
  __$$NativeMessage_AddChunkCopyWithImpl(_$NativeMessage_AddChunk _value, $Res Function(_$NativeMessage_AddChunk) _then) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? location = null,
    Object? blocks = null,
  }) {
    return _then(_$NativeMessage_AddChunk(
      location: null == location
          ? _value.location
          : location // ignore: cast_nullable_to_non_nullable
              as ChunkLocation,
      blocks: null == blocks
          ? _value._blocks
          : blocks // ignore: cast_nullable_to_non_nullable
              as List<BlockInformation>,
    ));
  }
}

/// @nodoc

class _$NativeMessage_AddChunk implements NativeMessage_AddChunk {
  const _$NativeMessage_AddChunk({required this.location, required final List<BlockInformation> blocks}) : _blocks = blocks;

  @override
  final ChunkLocation location;
  final List<BlockInformation> _blocks;
  @override
  List<BlockInformation> get blocks {
    if (_blocks is EqualUnmodifiableListView) return _blocks;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_blocks);
  }

  @override
  String toString() {
    return 'NativeMessage.addChunk(location: $location, blocks: $blocks)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$NativeMessage_AddChunk && (identical(other.location, location) || other.location == location) && const DeepCollectionEquality().equals(other._blocks, _blocks));
  }

  @override
  int get hashCode => Object.hash(runtimeType, location, const DeepCollectionEquality().hash(_blocks));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$NativeMessage_AddChunkCopyWith<_$NativeMessage_AddChunk> get copyWith => __$$NativeMessage_AddChunkCopyWithImpl<_$NativeMessage_AddChunk>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(ChunkLocation chunk, BlockInformation block) addBlock,
    required TResult Function(ChunkPosition position, ChunkLocation chunk) removeBlock,
    required TResult Function(ChunkLocation location, List<BlockInformation> blocks) addChunk,
    required TResult Function(ChunkLocation location) removeChunk,
    required TResult Function(int x, int y, int z) playerTeleported,
  }) {
    return addChunk(location, blocks);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(ChunkLocation chunk, BlockInformation block)? addBlock,
    TResult? Function(ChunkPosition position, ChunkLocation chunk)? removeBlock,
    TResult? Function(ChunkLocation location, List<BlockInformation> blocks)? addChunk,
    TResult? Function(ChunkLocation location)? removeChunk,
    TResult? Function(int x, int y, int z)? playerTeleported,
  }) {
    return addChunk?.call(location, blocks);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(ChunkLocation chunk, BlockInformation block)? addBlock,
    TResult Function(ChunkPosition position, ChunkLocation chunk)? removeBlock,
    TResult Function(ChunkLocation location, List<BlockInformation> blocks)? addChunk,
    TResult Function(ChunkLocation location)? removeChunk,
    TResult Function(int x, int y, int z)? playerTeleported,
    required TResult orElse(),
  }) {
    if (addChunk != null) {
      return addChunk(location, blocks);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(NativeMessage_AddBlock value) addBlock,
    required TResult Function(NativeMessage_RemoveBlock value) removeBlock,
    required TResult Function(NativeMessage_AddChunk value) addChunk,
    required TResult Function(NativeMessage_RemoveChunk value) removeChunk,
    required TResult Function(NativeMessage_PlayerTeleported value) playerTeleported,
  }) {
    return addChunk(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(NativeMessage_AddBlock value)? addBlock,
    TResult? Function(NativeMessage_RemoveBlock value)? removeBlock,
    TResult? Function(NativeMessage_AddChunk value)? addChunk,
    TResult? Function(NativeMessage_RemoveChunk value)? removeChunk,
    TResult? Function(NativeMessage_PlayerTeleported value)? playerTeleported,
  }) {
    return addChunk?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(NativeMessage_AddBlock value)? addBlock,
    TResult Function(NativeMessage_RemoveBlock value)? removeBlock,
    TResult Function(NativeMessage_AddChunk value)? addChunk,
    TResult Function(NativeMessage_RemoveChunk value)? removeChunk,
    TResult Function(NativeMessage_PlayerTeleported value)? playerTeleported,
    required TResult orElse(),
  }) {
    if (addChunk != null) {
      return addChunk(this);
    }
    return orElse();
  }
}

abstract class NativeMessage_AddChunk implements NativeMessage {
  const factory NativeMessage_AddChunk({required final ChunkLocation location, required final List<BlockInformation> blocks}) = _$NativeMessage_AddChunk;

  ChunkLocation get location;
  List<BlockInformation> get blocks;
  @JsonKey(ignore: true)
  _$$NativeMessage_AddChunkCopyWith<_$NativeMessage_AddChunk> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$NativeMessage_RemoveChunkCopyWith<$Res> {
  factory _$$NativeMessage_RemoveChunkCopyWith(_$NativeMessage_RemoveChunk value, $Res Function(_$NativeMessage_RemoveChunk) then) = __$$NativeMessage_RemoveChunkCopyWithImpl<$Res>;
  @useResult
  $Res call({ChunkLocation location});
}

/// @nodoc
class __$$NativeMessage_RemoveChunkCopyWithImpl<$Res> extends _$NativeMessageCopyWithImpl<$Res, _$NativeMessage_RemoveChunk> implements _$$NativeMessage_RemoveChunkCopyWith<$Res> {
  __$$NativeMessage_RemoveChunkCopyWithImpl(_$NativeMessage_RemoveChunk _value, $Res Function(_$NativeMessage_RemoveChunk) _then) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? location = null,
  }) {
    return _then(_$NativeMessage_RemoveChunk(
      location: null == location
          ? _value.location
          : location // ignore: cast_nullable_to_non_nullable
              as ChunkLocation,
    ));
  }
}

/// @nodoc

class _$NativeMessage_RemoveChunk implements NativeMessage_RemoveChunk {
  const _$NativeMessage_RemoveChunk({required this.location});

  @override
  final ChunkLocation location;

  @override
  String toString() {
    return 'NativeMessage.removeChunk(location: $location)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$NativeMessage_RemoveChunk && (identical(other.location, location) || other.location == location));
  }

  @override
  int get hashCode => Object.hash(runtimeType, location);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$NativeMessage_RemoveChunkCopyWith<_$NativeMessage_RemoveChunk> get copyWith => __$$NativeMessage_RemoveChunkCopyWithImpl<_$NativeMessage_RemoveChunk>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(ChunkLocation chunk, BlockInformation block) addBlock,
    required TResult Function(ChunkPosition position, ChunkLocation chunk) removeBlock,
    required TResult Function(ChunkLocation location, List<BlockInformation> blocks) addChunk,
    required TResult Function(ChunkLocation location) removeChunk,
    required TResult Function(int x, int y, int z) playerTeleported,
  }) {
    return removeChunk(location);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(ChunkLocation chunk, BlockInformation block)? addBlock,
    TResult? Function(ChunkPosition position, ChunkLocation chunk)? removeBlock,
    TResult? Function(ChunkLocation location, List<BlockInformation> blocks)? addChunk,
    TResult? Function(ChunkLocation location)? removeChunk,
    TResult? Function(int x, int y, int z)? playerTeleported,
  }) {
    return removeChunk?.call(location);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(ChunkLocation chunk, BlockInformation block)? addBlock,
    TResult Function(ChunkPosition position, ChunkLocation chunk)? removeBlock,
    TResult Function(ChunkLocation location, List<BlockInformation> blocks)? addChunk,
    TResult Function(ChunkLocation location)? removeChunk,
    TResult Function(int x, int y, int z)? playerTeleported,
    required TResult orElse(),
  }) {
    if (removeChunk != null) {
      return removeChunk(location);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(NativeMessage_AddBlock value) addBlock,
    required TResult Function(NativeMessage_RemoveBlock value) removeBlock,
    required TResult Function(NativeMessage_AddChunk value) addChunk,
    required TResult Function(NativeMessage_RemoveChunk value) removeChunk,
    required TResult Function(NativeMessage_PlayerTeleported value) playerTeleported,
  }) {
    return removeChunk(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(NativeMessage_AddBlock value)? addBlock,
    TResult? Function(NativeMessage_RemoveBlock value)? removeBlock,
    TResult? Function(NativeMessage_AddChunk value)? addChunk,
    TResult? Function(NativeMessage_RemoveChunk value)? removeChunk,
    TResult? Function(NativeMessage_PlayerTeleported value)? playerTeleported,
  }) {
    return removeChunk?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(NativeMessage_AddBlock value)? addBlock,
    TResult Function(NativeMessage_RemoveBlock value)? removeBlock,
    TResult Function(NativeMessage_AddChunk value)? addChunk,
    TResult Function(NativeMessage_RemoveChunk value)? removeChunk,
    TResult Function(NativeMessage_PlayerTeleported value)? playerTeleported,
    required TResult orElse(),
  }) {
    if (removeChunk != null) {
      return removeChunk(this);
    }
    return orElse();
  }
}

abstract class NativeMessage_RemoveChunk implements NativeMessage {
  const factory NativeMessage_RemoveChunk({required final ChunkLocation location}) = _$NativeMessage_RemoveChunk;

  ChunkLocation get location;
  @JsonKey(ignore: true)
  _$$NativeMessage_RemoveChunkCopyWith<_$NativeMessage_RemoveChunk> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$NativeMessage_PlayerTeleportedCopyWith<$Res> {
  factory _$$NativeMessage_PlayerTeleportedCopyWith(_$NativeMessage_PlayerTeleported value, $Res Function(_$NativeMessage_PlayerTeleported) then) = __$$NativeMessage_PlayerTeleportedCopyWithImpl<$Res>;
  @useResult
  $Res call({int x, int y, int z});
}

/// @nodoc
class __$$NativeMessage_PlayerTeleportedCopyWithImpl<$Res> extends _$NativeMessageCopyWithImpl<$Res, _$NativeMessage_PlayerTeleported> implements _$$NativeMessage_PlayerTeleportedCopyWith<$Res> {
  __$$NativeMessage_PlayerTeleportedCopyWithImpl(_$NativeMessage_PlayerTeleported _value, $Res Function(_$NativeMessage_PlayerTeleported) _then) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? x = null,
    Object? y = null,
    Object? z = null,
  }) {
    return _then(_$NativeMessage_PlayerTeleported(
      x: null == x
          ? _value.x
          : x // ignore: cast_nullable_to_non_nullable
              as int,
      y: null == y
          ? _value.y
          : y // ignore: cast_nullable_to_non_nullable
              as int,
      z: null == z
          ? _value.z
          : z // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$NativeMessage_PlayerTeleported implements NativeMessage_PlayerTeleported {
  const _$NativeMessage_PlayerTeleported({required this.x, required this.y, required this.z});

  @override
  final int x;
  @override
  final int y;
  @override
  final int z;

  @override
  String toString() {
    return 'NativeMessage.playerTeleported(x: $x, y: $y, z: $z)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$NativeMessage_PlayerTeleported && (identical(other.x, x) || other.x == x) && (identical(other.y, y) || other.y == y) && (identical(other.z, z) || other.z == z));
  }

  @override
  int get hashCode => Object.hash(runtimeType, x, y, z);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$NativeMessage_PlayerTeleportedCopyWith<_$NativeMessage_PlayerTeleported> get copyWith => __$$NativeMessage_PlayerTeleportedCopyWithImpl<_$NativeMessage_PlayerTeleported>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(ChunkLocation chunk, BlockInformation block) addBlock,
    required TResult Function(ChunkPosition position, ChunkLocation chunk) removeBlock,
    required TResult Function(ChunkLocation location, List<BlockInformation> blocks) addChunk,
    required TResult Function(ChunkLocation location) removeChunk,
    required TResult Function(int x, int y, int z) playerTeleported,
  }) {
    return playerTeleported(x, y, z);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(ChunkLocation chunk, BlockInformation block)? addBlock,
    TResult? Function(ChunkPosition position, ChunkLocation chunk)? removeBlock,
    TResult? Function(ChunkLocation location, List<BlockInformation> blocks)? addChunk,
    TResult? Function(ChunkLocation location)? removeChunk,
    TResult? Function(int x, int y, int z)? playerTeleported,
  }) {
    return playerTeleported?.call(x, y, z);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(ChunkLocation chunk, BlockInformation block)? addBlock,
    TResult Function(ChunkPosition position, ChunkLocation chunk)? removeBlock,
    TResult Function(ChunkLocation location, List<BlockInformation> blocks)? addChunk,
    TResult Function(ChunkLocation location)? removeChunk,
    TResult Function(int x, int y, int z)? playerTeleported,
    required TResult orElse(),
  }) {
    if (playerTeleported != null) {
      return playerTeleported(x, y, z);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(NativeMessage_AddBlock value) addBlock,
    required TResult Function(NativeMessage_RemoveBlock value) removeBlock,
    required TResult Function(NativeMessage_AddChunk value) addChunk,
    required TResult Function(NativeMessage_RemoveChunk value) removeChunk,
    required TResult Function(NativeMessage_PlayerTeleported value) playerTeleported,
  }) {
    return playerTeleported(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(NativeMessage_AddBlock value)? addBlock,
    TResult? Function(NativeMessage_RemoveBlock value)? removeBlock,
    TResult? Function(NativeMessage_AddChunk value)? addChunk,
    TResult? Function(NativeMessage_RemoveChunk value)? removeChunk,
    TResult? Function(NativeMessage_PlayerTeleported value)? playerTeleported,
  }) {
    return playerTeleported?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(NativeMessage_AddBlock value)? addBlock,
    TResult Function(NativeMessage_RemoveBlock value)? removeBlock,
    TResult Function(NativeMessage_AddChunk value)? addChunk,
    TResult Function(NativeMessage_RemoveChunk value)? removeChunk,
    TResult Function(NativeMessage_PlayerTeleported value)? playerTeleported,
    required TResult orElse(),
  }) {
    if (playerTeleported != null) {
      return playerTeleported(this);
    }
    return orElse();
  }
}

abstract class NativeMessage_PlayerTeleported implements NativeMessage {
  const factory NativeMessage_PlayerTeleported({required final int x, required final int y, required final int z}) = _$NativeMessage_PlayerTeleported;

  int get x;
  int get y;
  int get z;
  @JsonKey(ignore: true)
  _$$NativeMessage_PlayerTeleportedCopyWith<_$NativeMessage_PlayerTeleported> get copyWith => throw _privateConstructorUsedError;
}
