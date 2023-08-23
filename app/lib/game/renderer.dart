import 'package:flame/extensions.dart';

import 'world.dart';

Vector2 toRenderPosition(Vector3 position) {
  return rightOffset * position.x +
      forwardOffset * position.y +
      topOffset * position.z;
}

int compareChunkPriorities(Vector3 a, Vector3 b) {
  final zCompare = a.z.compareTo(b.z);
  if (zCompare != 0) return zCompare;
  final yCompare = a.y.compareTo(b.y);
  if (yCompare != 0) return yCompare;
  return a.x.compareTo(b.x);
}
