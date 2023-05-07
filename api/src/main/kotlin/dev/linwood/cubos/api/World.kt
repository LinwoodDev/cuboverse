package dev.linwood.cubos.api

import androidx.compose.ui.graphics.Canvas
import java.math.BigDecimal
import java.math.BigInteger

class World(val chunkSize: Pair<Int, Int>, val offset: Triple<Float, Float, Float>, val distanceOffsetX: Float) {
    private val chunks = mutableMapOf<Pair<BigInteger, BigInteger>, Chunk>()
    internal val entityBuilders = mutableMapOf<String, EntityBuilder>()

    fun addChunk(coordinate: ChunkCoordinate): Chunk {
        val chunk = Chunk(this)
        chunks[coordinate] = chunk
        return chunk
    }

    fun getChunk(coordinate: ChunkCoordinate): Chunk {
        return chunks.getOrPut(coordinate) { Chunk(this) }
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
            it.value.paint(
                canvas,
                this,
                renderLocation,
                renderSize,
                renderScale,
                it.key
            )
        }
    }

    fun getChunkByObject(entity: Entity): Chunk? {
        return chunks.values.firstOrNull { it.getObjects().contains(entity) }
    }

    fun getChunkPositionByObject(entity: Entity): Pair<BigInteger, BigInteger>? {
        return chunks.keys.firstOrNull { chunks[it]?.getObjects()?.contains(entity) ?: false }
    }
}
