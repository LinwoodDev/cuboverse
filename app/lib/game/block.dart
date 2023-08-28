import 'dart:async';
import 'dart:ui';

import 'package:cuboverse/game/renderer.dart';
import 'package:cuboverse/helpers/position.dart';
import 'package:cuboverse/src/native.dart';
import 'package:flame/components.dart';
import 'package:flame/events.dart';
import 'package:flame/flame.dart';

class CuboverseBlock extends PositionComponent with TapCallbacks {
  final BlockInformation blockInformation;
  late final SpriteComponent sprite;
  CuboverseBlock(this.blockInformation) : super(size: Vector2(16, 16));
  final _paint = Paint()..isAntiAlias = false;

  @override
  Future<void> onLoad() async {
    position = toRenderPosition(blockInformation.position.toVector3());
    final image = await Flame.images.load("blocks/test.png");
    sprite = SpriteComponent(
        sprite: Sprite(image),
        size: Vector2(16.01, 16.01),
        paint: _paint,
        autoResize: false);
    add(sprite);
  }

  @override
  void onTapUp(TapUpEvent event) {
    print("Tapped on block at ${blockInformation.position.toVector3()}");
  }
}
