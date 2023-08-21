import 'package:flame/components.dart';

import 'block.dart';

class CuboverseChunk extends Component {
  CuboverseChunk()
      : super(
            children: [
          Vector3(0, 2, 1),
          Vector3(0, 0, 0),
          Vector3(0, 1, 0),
          Vector3(1, 2, 0),
          Vector3(0, 2, 0),
          Vector3(-1, 2, 0),
          Vector3(0, 3, 0),
        ].reversed.map((e) => CuboverseBlock(e)));
}
