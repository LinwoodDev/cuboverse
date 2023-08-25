import 'dart:async';

import 'package:cuboverse/game/renderer.dart';
import 'package:cuboverse/helpers/position.dart';
import 'package:cuboverse/src/native.dart';
import 'package:flame/components.dart';
import 'package:flame/events.dart';
import 'package:flame/flame.dart';

class CuboverseBlock extends PositionComponent with TapCallbacks {
  final BlockInformation blockInformation;
  CuboverseBlock(this.blockInformation) : super(size: Vector2(16, 16));

  @override
  Future<void> onLoad() async {
    position = toRenderPosition(blockInformation.position.toVector3());
    final image = await Flame.images.load("blocks/test.png");
    add(
      SpriteComponent(sprite: Sprite(image), autoResize: true),
    );
  }

  @override
  void onTapUp(TapUpEvent event) {
    print("Tapped on block with $blockInformation");
  }
}
