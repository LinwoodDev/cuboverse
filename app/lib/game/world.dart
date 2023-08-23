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

  CuboverseWorld(this.worldManager);

  @override
  FutureOr<void> onLoad() {
    player = CuboversePlayer();
    cameraComponent = CameraComponent(world: world);
    addAll([world, cameraComponent]);
    world.add(player);
    cameraComponent.follow(player);
    cameraComponent.viewfinder
      ..zoom = 5
      ..anchor = Anchor.center;
    worldManager.createMessageStream().listen(_onMessage);
  }

  @override
  void onDetach() {
    worldManager.close();
  }

  void addChunk(ChunkLocation location, List<BlockInformation> blocks) {
    world.removeAll(chunks.values);
    rebuild(location, blocks);
  }

  void removeChunk(ChunkLocation location) {
    final chunk = chunks.remove(location);
    if (chunk == null) return;
    world.remove(chunk);
  }

  void rebuild(ChunkLocation location, List<BlockInformation> blocks) {
    chunks[location] = CuboverseChunk(location)..addBlocks(blocks);
    world.addAll(chunks.values.sorted(
      (a, b) => compareChunkPriorities(
        a.location.toVector3(),
        b.location.toVector3(),
      ),
    ));
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
    Vector3 movement = Vector3.zero();
    bool handled = false;
    if (keysPressed.contains(LogicalKeyboardKey.keyW)) {
      movement += Vector3(0, -1, 0);
      handled = true;
    }
    if (keysPressed.contains(LogicalKeyboardKey.keyS)) {
      movement += Vector3(0, 1, 0);
      handled = true;
    }
    if (keysPressed.contains(LogicalKeyboardKey.keyA)) {
      movement += Vector3(-1, 0, 0);
      handled = true;
    }
    if (keysPressed.contains(LogicalKeyboardKey.keyD)) {
      movement += Vector3(1, 0, 0);
      handled = true;
    }
    if (keysPressed.contains(LogicalKeyboardKey.space)) {
      movement += Vector3(0, 0, 1);
      handled = true;
    }
    if (keysPressed.contains(LogicalKeyboardKey.shiftLeft)) {
      movement += Vector3(0, 0, -1);
      handled = true;
    }
    if (movement != Vector3.zero()) {
      worldManager.movePlayer(x: movement.x, y: movement.y, z: movement.z);
    }
    if (handled) {
      return KeyEventResult.handled;
    }
    return super.onKeyEvent(event, keysPressed);
  }
}
