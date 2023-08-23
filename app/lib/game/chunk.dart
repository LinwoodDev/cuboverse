import 'package:collection/collection.dart';
import 'package:cuboverse/src/native/bridge_definitions.dart';
import 'package:flame/components.dart';

import 'block.dart';

class CuboverseChunk extends PositionComponent {
  final Map<BlockPosition, CuboverseBlock> blocks = {};
  final ChunkLocation location;
  CuboverseChunk(this.location) : super();

  void addBlock(BlockInformation block) => addBlocks([block]);
  void addBlocks(List<BlockInformation> blocks) {
    removeAll(this.blocks.values);
    rebuild(blocks);
  }

  void rebuild(List<BlockInformation> blocks) {
    this
        .blocks
        .addEntries(blocks.map((e) => MapEntry(e.position, CuboverseBlock(e))));
    addAll(this.blocks.values.sorted(compareChunkPriorities));
  }

  void removeBlock(BlockPosition position) => removeBlocks([position]);
  void removeBlocks(List<BlockPosition> positions) {
    removeAll(positions.map((e) => blocks[e]).whereNotNull());
    positions.forEach(blocks.remove);
  }
}

int compareChunkPriorities(CuboverseBlock blockA, CuboverseBlock blockB) {
  final a = blockA.blockInformation.position;
  final b = blockB.blockInformation.position;
  final zCompare = a.field2.compareTo(b.field2);
  if (zCompare != 0) return zCompare;
  final yCompare = a.field1.compareTo(b.field1);
  if (yCompare != 0) return yCompare;
  return a.field0.compareTo(b.field0);
}
