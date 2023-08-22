import 'dart:async';

import 'package:cuboverse/game/renderer.dart';
import 'package:flame/components.dart';
import 'package:flame/flame.dart';

class CuboverseBlock extends PositionComponent {
  final Vector3 chunkPosition;
  CuboverseBlock(this.chunkPosition) : super(size: Vector2(16, 16));

  @override
  Future<void> onLoad() async {
    position = toRenderPosition(chunkPosition);
    final image = await Flame.images.load("blocks/test.png");
    add(
      SpriteComponent(sprite: Sprite(image), autoResize: true),
    );
  }
}
