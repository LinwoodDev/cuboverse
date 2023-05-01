package dev.linwood.cubos.api

import androidx.compose.ui.graphics.Canvas
import java.math.BigDecimal
import java.math.BigInteger

data class ChunkContext(
    val world: World,
    val chunk: Chunk,
    val chunkSize: Pair<Int, Int>
)

data class RenderContext(
    val canvas: Canvas,
    val chunkPosition: Pair<BigInteger, BigInteger>,
    val chunkContext: ChunkContext,
    val renderLocation: Pair<BigDecimal, BigDecimal>,
    val renderSize: Pair<Double, Double>,
    val renderScale: Pair<Float, Float>,
) {
    val offset get() = chunkContext.world.offset
    val chunkSize get() = chunkContext.chunkSize

    fun getPointFromVector(vector: Pair<Float, Float>): Pair<Float, Float> {
        return Pair(
            vector.first * offset.first,
            vector.second * offset.second
        );
    }

    fun getPositionFromVector(vector: Pair<Float, Float>): Pair<BigDecimal, BigDecimal> {
        val chunkPos = getGlobalChunkPosition()
        return Pair(
            vector.first.toBigDecimal() + chunkPos.first,
            vector.second.toBigDecimal() + chunkPos.second
        );
    }

    fun getLocationFromVector(vector: Pair<Float, Float>) = getLocationFromPoint(getPointFromVector(vector))

    fun getLocationFromPoint(point: Pair<Float, Float>): Pair<BigDecimal, BigDecimal> {
        val chunkLoc = getGlobalChunkLocation()
        return Pair(
            point.first.toBigDecimal() + chunkLoc.first,
            point.second.toBigDecimal() + chunkLoc.second
        );
    }

    fun getGlobalChunkPosition(): Pair<BigDecimal, BigDecimal> {
        return Pair(
            chunkPosition.first.toBigDecimal() * chunkSize.first.toBigDecimal(),
            chunkPosition.second.toBigDecimal() * chunkSize.second.toBigDecimal()
        )
    }

    fun getGlobalChunkLocation(): Pair<BigDecimal, BigDecimal> {
        return Pair(
            chunkPosition.first.toBigDecimal() * chunkSize.first.toBigDecimal() * offset.first.toBigDecimal(),
            chunkPosition.second.toBigDecimal() * chunkSize.second.toBigDecimal() * offset.second.toBigDecimal()
        );
    }

}
