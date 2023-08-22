import 'package:flame/extensions.dart';

import 'world.dart';

Vector2 toRenderPosition(Vector3 position) {
  return rightOffset * position.x +
      forwardOffset * position.y +
      topOffset * position.z;
}
