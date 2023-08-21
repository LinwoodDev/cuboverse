import 'dart:async';

import 'package:cuboverse/game/world.dart';
import 'package:flame/components.dart';
import 'package:flame/flame.dart';

class CuboverseBlock extends Component {
  final Vector3 position;

  CuboverseBlock(this.position);

  @override
  Future<void> onLoad() async {
    final image = await Flame.images.load("blocks/test.png");
    add(SpriteComponent.fromImage(image,
        position: RIGHT_OFFSET * position.x +
            FORWARD_OFFSET * position.y +
            TOP_OFFSET * position.z));
  }
}
