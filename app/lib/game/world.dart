import 'dart:async';

import 'package:cuboverse/game/chunk.dart';
import 'package:cuboverse/game/player.dart';
import 'package:cuboverse/src/native.dart';
import 'package:flame/components.dart';
import 'package:flame/game.dart';
import 'package:flame/input.dart';
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

  CuboverseWorld(this.worldManager);

  @override
  FutureOr<void> onLoad() {
    player = CuboversePlayer();
    cameraComponent = CameraComponent(world: world);
    cameraComponent.viewfinder.zoom = 10;
    addAll([cameraComponent, world]);
    world.addAll([CuboverseChunk(), player]);
    cameraComponent.follow(player);
    worldManager.createMessageStream().listen(_onMessage);
    worldManager.addBlock(
        position: const GlobalBlockPosition(
            field0: ChunkLocation(field0: 0, field1: 0, field2: 0),
            field1: BlockPosition(field0: 0, field1: 0, field2: -1)),
        block: "test");
  }

  @override
  void onDetach() {
    worldManager.close();
  }

  void _onMessage(NativeMessage event) {
    event.maybeMap(
      playerTeleported: (value) {
        player.globalPosition =
            Vector3(value.x.toDouble(), value.y.toDouble(), value.z.toDouble());
      },
      orElse: () {},
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
