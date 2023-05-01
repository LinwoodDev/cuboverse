package dev.linwood.cubos.api

import androidx.compose.ui.graphics.Canvas
import java.math.BigDecimal
import java.math.BigInteger

data class ChunkContext(
    val world: World,
    val chunk: Chunk,
) {
    val chunkSize get() = world.chunkSize
}

data class GlobalChunkContext(
    val chunkPosition: Pair<BigInteger, BigInteger>,
    val chunkContext: ChunkContext
) {
    val world get() = chunkContext.world
    val chunk get() = chunkContext.chunk
    val chunkSize get() = chunkContext.chunkSize

    fun getPositionFromVector(vector: Pair<Float, Float>): Pair<BigDecimal, BigDecimal> {
        val chunkPos = getGlobalChunkPosition()
        return Pair(
            vector.first.toBigDecimal() + chunkPos.first,
            vector.second.toBigDecimal() + chunkPos.second
        );
    }

    fun getGlobalChunkPosition(): Pair<BigDecimal, BigDecimal> {
        val globChunkPos = getGlobalChunkPosition()
        return Pair(
            globChunkPos.first * chunkSize.first.toBigDecimal(),
            globChunkPos.second * chunkSize.second.toBigDecimal()
        )
    }

}

data class RenderContext(
    val canvas: Canvas,
    val globalChunkContext: GlobalChunkContext,
    val renderPosition: Pair<BigDecimal, BigDecimal>,
    val renderSize: Pair<Double, Double>,
    val renderScale: Pair<Int, Int>,
) {
    val chunkPosition get() = globalChunkContext.chunkPosition
    val chunkContext get() = globalChunkContext.chunkContext
    val world get() = chunkContext.world
    val chunk get() = chunkContext.chunk
    val chunkSize get() = chunkContext.chunkSize
    val offset get() = chunkContext.world.offset
    val renderChunkScale get() = Pair(renderScale.first * chunkSize.first, renderScale.second * chunkSize.second)
    fun getPositionFromVector(vector : Pair<Float, Float>) = globalChunkContext.getPositionFromVector(getPointFromVector(vector))
    fun getGlobalChunkPosition() = globalChunkContext.getGlobalChunkPosition()

    fun getPointFromVector(vector: Pair<Float, Float>): Pair<Float, Float> {
        return Pair(
            vector.first + offset,
            vector.second
        );
    }

    fun getLocationFromVector(vector: Pair<Float, Float>) = getLocationFromPoint(getPointFromVector(vector))

    fun getLocationFromPoint(point: Pair<Float, Float>): Pair<BigDecimal, BigDecimal> {
        val chunkLoc = getGlobalChunkPosition()
        val y = (point.second.toBigDecimal() + chunkLoc.second) * renderSize.first.toBigDecimal()
        val x = (point.first.toBigDecimal() + chunkLoc.first + y * offset.toBigDecimal()) * renderSize.second.toBigDecimal()
        return Pair(
            x,
            y
        );
    }

    fun getChunkLocation(): Pair<BigDecimal, BigDecimal> = getLocationFromPoint(Pair(0f, 0f))

    fun getRenderPositionFromPosition(position: Pair<BigDecimal, BigDecimal>): Pair<BigDecimal, BigDecimal> {
        return Pair(
            position.first - renderPosition.first,
            position.second - renderPosition.second
        )
    }

    fun getRenderPositionFromVector(vector: Pair<Float, Float>) =
        getRenderPositionFromPosition(getLocationFromVector(vector))

}
