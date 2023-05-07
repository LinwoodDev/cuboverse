package dev.linwood.cubos.api

import androidx.compose.ui.graphics.Canvas
import java.math.BigDecimal
import java.math.BigInteger

class World(val chunkSize: Pair<Int, Int>, val offset: Triple<Float, Float, Float>, val distanceOffsetX: Float, val chunkBuilder : ChunkBuilder = EmptyChunkBuilder) {
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
                position.first.toBigInteger() / chunkSize.first.toBigInteger(),
                position.second.toBigInteger() / chunkSize.second.toBigInteger()
            )
        )
    }

    fun paint(
        canvas: Canvas,
        renderLocation: Pair<BigDecimal, BigDecimal>,
        renderSize: Pair<Float, Float>,
        renderScale: Pair<Int, Int> = Pair(1, 1)
    ) {
        chunks.forEach {
            it.paint(
                canvas,
                this,
                renderLocation,
                renderSize,
                renderScale
            )
        }
    }

    fun getChunkByObject(entity: Entity): Chunk? {
        return chunks.firstOrNull { it.getObjects().contains(entity) }
    }
}
