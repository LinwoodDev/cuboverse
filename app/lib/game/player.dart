import 'dart:async';

import 'package:flame/components.dart';
import 'package:flame/flame.dart';

import 'renderer.dart';

class CuboversePlayer extends PositionComponent {
  Vector3 _globalPosition = Vector3.zero();

  CuboversePlayer() : super(size: Vector2(16, 16), priority: 1);

  Vector3 get globalPosition => _globalPosition;
  set globalPosition(Vector3 value) {
    _globalPosition = value;
    updatePosition();
  }

  void updatePosition() {
    position = toRenderPosition(globalPosition);
  }

  @override
  Future<void> onLoad() async {
    position = toRenderPosition(globalPosition);
    final image = await Flame.images.load("player.png");
    add(
      SpriteComponent(sprite: Sprite(image), size: Vector2(16, 16)),
    );
  }

  void move(int x, int y, int z) {
    globalPosition += Vector3(x.toDouble(), y.toDouble(), z.toDouble());
  }
}
