import 'dart:async';

import 'package:collection/collection.dart';
import 'package:cuboverse/game/chunk.dart';
import 'package:cuboverse/game/player.dart';
import 'package:cuboverse/game/renderer.dart';
import 'package:cuboverse/helpers/position.dart';
import 'package:cuboverse/src/native.dart';
import 'package:flame/components.dart';
import 'package:flame/game.dart';
import 'package:flame/input.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:flutter/widgets.dart';

final forwardOffset = Vector2(-8, 4);
final rightOffset = Vector2(8, 4);
final topOffset = Vector2(0, -8);

class CuboverseWorld extends FlameGame with KeyboardEvents {
  final world = World();
  late final CameraComponent cameraComponent;
  late final CuboversePlayer player;
  final WorldManager worldManager;
  final Map<ChunkLocation, CuboverseChunk> chunks = {};
  Vector3 _movement = Vector3.zero();

  CuboverseWorld(this.worldManager);

  @override
  FutureOr<void> onLoad() {
    player = CuboversePlayer();
    cameraComponent = CameraComponent(world: world);
    addAll([world, cameraComponent]);
    world.add(player);
    cameraComponent.follow(player, snap: true);
    cameraComponent.viewfinder
      ..zoom = 5
      ..anchor = Anchor.center;
    worldManager.createMessageStream().listen(_onMessage);
  }

  @override
  void update(double dt) {
    super.update(dt);
    _updateMovement(dt);
  }

  bool _currentlyMoving = false;
  Future<void> _updateMovement(double dt) async {
    if (_currentlyMoving || !(await worldManager.playerOnGround())) return;
    _currentlyMoving = true;
    await worldManager.movePlayer(
      x: _movement.x,
      y: _movement.y,
      z: _movement.z,
      relative: true,
      teleport: false,
    );
    _currentlyMoving = false;
  }

  void addChunk(ChunkLocation location, List<BlockInformation> blocks) {
    final chunk = CuboverseChunk(location)..addBlocks(blocks);
    chunks[location] = chunk;
    world.add(chunk);
    rebalance();
  }

  void removeChunk(ChunkLocation location) {
    final chunk = chunks.remove(location);
    if (chunk == null) return;
    world.remove(chunk);
  }

  void rebalance() {
    final balanced = chunks.keys.sorted(
      (a, b) => compareChunkPriorities(
        a.toVector3(),
        b.toVector3(),
      ),
    );
    balanced.forEachIndexed((index, element) {
      chunks[element]?.priority = index;
    });
    player.priority = balanced.length;
  }

  void _onMessage(NativeMessage event) {
    event.maybeMap(
      playerTeleported: (value) {
        player.globalPosition =
            Vector3(value.x.toDouble(), value.y.toDouble(), value.z.toDouble());
      },
      addBlock: (value) => chunks[value.chunk]?.addBlock(value.block),
      removeBlock: (value) => chunks[value.chunk]?.removeBlock(value.position),
      addChunk: (value) => addChunk(value.location, value.blocks),
      removeChunk: (value) => removeChunk(value.location),
      orElse: () {
        if (kDebugMode) {
          print("Unknown message: $event");
        }
      },
    );
  }

  @override
  KeyEventResult onKeyEvent(
      RawKeyEvent event, Set<LogicalKeyboardKey> keysPressed) {
    _movement = Vector3.zero();
    bool handled = false;
    if (keysPressed.contains(LogicalKeyboardKey.keyW)) {
      _movement += Vector3(0, -1, 0);
      handled = true;
    }
    if (keysPressed.contains(LogicalKeyboardKey.keyS)) {
      _movement += Vector3(0, 1, 0);
      handled = true;
    }
    if (keysPressed.contains(LogicalKeyboardKey.keyA)) {
      _movement += Vector3(-1, 0, 0);
      handled = true;
    }
    if (keysPressed.contains(LogicalKeyboardKey.keyD)) {
      _movement += Vector3(1, 0, 0);
      handled = true;
    }
    if (keysPressed.contains(LogicalKeyboardKey.space)) {
      _movement += Vector3(0, 0, 1);
      handled = true;
    }
    if (keysPressed.contains(LogicalKeyboardKey.shiftLeft)) {
      _movement += Vector3(0, 0, 5);
      handled = true;
    }
    _movement /= 15;
    if (keysPressed.contains(LogicalKeyboardKey.escape)) {
      handled = true;
      if (overlays.isActive("pause")) {
        overlays.remove("pause");
      } else {
        overlays.add("pause");
      }
    }
    if (handled) {
      return KeyEventResult.handled;
    }
    return super.onKeyEvent(event, keysPressed);
  }
}
