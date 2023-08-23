import 'package:collection/collection.dart';
import 'package:cuboverse/game/renderer.dart';
import 'package:cuboverse/helpers/position.dart';
import 'package:cuboverse/src/native/bridge_definitions.dart';
import 'package:flame/components.dart';

import 'block.dart';

class CuboverseChunk extends PositionComponent {
  final Map<BlockPosition, CuboverseBlock> blocks = {};
  final ChunkLocation location;
  CuboverseChunk(this.location)
      : super(
          position: toRenderPosition(location.toVector3()),
        );

  void addBlock(BlockInformation block) => addBlocks([block]);
  void addBlocks(List<BlockInformation> blocks) {
    removeAll(this.blocks.values);
    rebuild(blocks);
  }

  void rebuild(List<BlockInformation> blocks) {
    this
        .blocks
        .addEntries(blocks.map((e) => MapEntry(e.position, CuboverseBlock(e))));
    addAll(this.blocks.values.sorted(
          (a, b) => compareChunkPriorities(
            a.blockInformation.position.toVector3(),
            b.blockInformation.position.toVector3(),
          ),
        ));
  }

  void removeBlock(BlockPosition position) => removeBlocks([position]);
  void removeBlocks(List<BlockPosition> positions) {
    removeAll(positions.map((e) => blocks[e]).whereNotNull());
    positions.forEach(blocks.remove);
  }
}
