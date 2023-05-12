package dev.linwood.cubos.api

import androidx.compose.ui.graphics.Canvas
import java.math.BigDecimal
import java.math.RoundingMode

class World(
    val chunkSize: Pair<Int, Int>,
    val offset: Triple<Float, Float, Float>,
    val distanceOffsetX: Float,
    val chunkBuilder: ChunkBuilder = EmptyChunkBuilder
) {
    private val chunks = mutableListOf<Chunk>()

    fun addChunk(coordinate: ChunkCoordinate): Chunk {
        val chunk = Chunk(this, coordinate)
        chunks.add(chunk)
        chunkBuilder(chunk)
        return chunk
    }

    fun getChunk(coordinate: ChunkCoordinate): Chunk {
        return chunks.firstOrNull { it.coordinate == coordinate } ?: addChunk(coordinate)
    }

    fun getChunkByPosition(position: Position): Chunk {
        return getChunk(
            ChunkCoordinate(
                position.first.divide(chunkSize.first.toBigDecimal(), RoundingMode.CEILING).toBigInteger(),
                position.second.divide(chunkSize.second.toBigDecimal(), RoundingMode.CEILING).toBigInteger()
            )
        )
    }

    fun getChunksByPriority(): Iterable<Chunk> {
        return chunks.sortedWith { a, b ->
            val aCoord = a.coordinate
            val bCoord = b.coordinate
            val y = aCoord.second.compareTo(bCoord.second)
            if (y != 0) return@sortedWith y
            return@sortedWith aCoord.first.compareTo(bCoord.first)
        }
    }

    fun paint(
        canvas: Canvas,
        renderLocation: Pair<BigDecimal, BigDecimal>,
        renderSize: Pair<Float, Float>,
        renderScale: Pair<Int, Int> = Pair(1, 1)
    ) {
        getChunksByPriority().forEach {
            it.paint(
                canvas,
                renderLocation,
                renderSize,
                renderScale
            )
        }
    }

    fun getChunkByObject(entity: Entity): Chunk? {
        return chunks.firstOrNull { it.getObjects().contains(entity) }
    }

    fun removeChunk(coordinate: ChunkCoordinate) {
        chunks.removeIf { it.coordinate == coordinate }
    }
}
