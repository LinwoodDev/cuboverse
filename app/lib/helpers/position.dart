import 'package:cuboverse/src/native.dart';
import 'package:flame/extensions.dart';

extension BlockPositionHelper on BlockPosition {
  Vector3 toVector3() =>
      Vector3(field0.toDouble(), field1.toDouble(), field2.toDouble());
}
