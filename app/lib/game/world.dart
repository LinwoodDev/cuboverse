import 'dart:async';

import 'package:cuboverse/game/chunk.dart';
import 'package:flame/components.dart';
import 'package:flame/game.dart';

final FORWARD_OFFSET = Vector2(8, -4);
final RIGHT_OFFSET = Vector2(8, 4);
final TOP_OFFSET = Vector2(0, -8);

class CuboverseWorld extends FlameGame {
  final world = World();
  late final CameraComponent cameraComponent;
  @override
  FutureOr<void> onLoad() {
    cameraComponent = CameraComponent(world: world);
    cameraComponent.viewfinder.zoom = 10;
    addAll([cameraComponent, world]);
    world.add(CuboverseChunk());
  }
}
