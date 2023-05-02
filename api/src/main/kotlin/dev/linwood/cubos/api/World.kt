package dev.linwood.cubos.api

import androidx.compose.ui.graphics.Canvas
import java.math.BigDecimal
import java.math.BigInteger

class World(val chunkSize: Pair<Int, Int>, val offset : Triple<Float, Float, Float>, val distanceOffsetX : Float) {
    private val chunks = mutableMapOf<Pair<BigInteger, BigInteger>, Chunk>()

    fun addChunk(chunk: Chunk, x: BigInteger, y: BigInteger) {
        chunks[Pair(x, y)] = chunk
    }

    fun getChunk(x: BigInteger, y: BigInteger): Chunk {
        return chunks.getOrPut(Pair(x, y)) { Chunk() }
    }

    fun getChunk(x: Int, y: Int): Chunk {
        return getChunk(x.toBigInteger(), y.toBigInteger())
    }

    fun getChunkByPosition(position : Pair<BigDecimal, BigDecimal>) : Chunk {
        return getChunk(
            position.first.toBigInteger() / chunkSize.first.toBigInteger(),
            position.second.toBigInteger() / chunkSize.second.toBigInteger()
        )
    }

    fun paint(
        canvas: Canvas,
        renderLocation: Pair<BigDecimal, BigDecimal>,
        renderSize: Pair<Double, Double>,
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
}