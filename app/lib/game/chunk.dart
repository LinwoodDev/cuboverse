import 'dart:async';

import 'package:flame/components.dart';

import 'block.dart';

class CuboverseChunk extends Component {
  CuboverseChunk() : super();

  @override
  FutureOr<void> onLoad() {
    final locations = [
      Vector3(1, 2, 0),
      Vector3(0, 2, 0),
      Vector3(0, 1, 0),
      Vector3(-1, 2, 0),
      Vector3(0, 2, 1),
      Vector3(0, 0, 0),
      Vector3(0, 3, 0),
    ];
    locations.sort(compareChunkPriorities);
    addAll(locations.map((e) => CuboverseBlock(e)));
  }
}

int compareChunkPriorities(Vector3 a, Vector3 b) {
  final zCompare = a.z.compareTo(b.z);
  if (zCompare != 0) return zCompare;
  final yCompare = a.y.compareTo(b.y);
  if (yCompare != 0) return yCompare;
  return a.x.compareTo(b.x);
}
