import 'package:cuboverse/src/native.dart';
import 'package:flame/extensions.dart';

extension BlockPositionHelper on BlockPosition {
  Vector3 toVector3() =>
      Vector3(field0.toDouble(), field1.toDouble(), field2.toDouble());
}

const chunkSize = 16;

extension ChunkLocationHelper on ChunkLocation {
  Vector3 toVector3() => Vector3(
      field0.toDouble() * 16, field1.toDouble() * 16, field2.toDouble() * 16);
}
