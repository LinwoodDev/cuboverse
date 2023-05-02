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
            vector.first.toBigDecimal() + chunkPos.first.toBigDecimal(),
            vector.second.toBigDecimal() + chunkPos.second.toBigDecimal()
        );
    }

    fun getGlobalChunkPosition(): Pair<BigInteger, BigInteger> {
        val globChunkPos = chunkPosition
        return Pair(
            globChunkPos.first * chunkSize.first.toBigInteger(),
            globChunkPos.second * chunkSize.second.toBigInteger()
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
    val distanceOffsetX get() = chunkContext.world.distanceOffsetX
    val renderChunkScale get() = Pair(renderScale.first * chunkSize.first, renderScale.second * chunkSize.second)
    fun getPositionFromVector(vector: Pair<Float, Float>) =
        globalChunkContext.getPositionFromVector(getPointFromVector(vector))

    fun getGlobalChunkPosition() = globalChunkContext.getGlobalChunkPosition()
    fun getGlobalChunkLocation(): Pair<BigDecimal, BigDecimal> {
        val globChunkPos = getGlobalChunkPosition()
        val y = globChunkPos.second.toBigDecimal() * offset.second.toBigDecimal() * renderScale.second.toBigDecimal()
        val x =
            (globChunkPos.first.toBigDecimal() * offset.first.toBigDecimal() + globChunkPos.second.toBigDecimal() * offset.second.toBigDecimal() * distanceOffsetX.toBigDecimal()) * renderScale.first.toBigDecimal()
        return Pair(
            x,
            y
        );
    }

    fun getPointFromVector(vector: Pair<Float, Float>): Pair<Float, Float> {
        val y = vector.second * offset.second * renderScale.second
        val x = (vector.first * offset.first + vector.second * offset.second * distanceOffsetX) * renderScale.first
        return Pair(
            x,
            y
        );
    }

    fun getLocationFromVector(vector: Pair<Float, Float>) = getLocationFromPoint(getPointFromVector(vector))

    fun getLocationFromPoint(point: Pair<Float, Float>): Pair<BigDecimal, BigDecimal> {
        val globalChunkLocation = getGlobalChunkLocation()
        return Pair(
            globalChunkLocation.first + point.first.toBigDecimal(),
            globalChunkLocation.second + point.second.toBigDecimal()
        )
    }

    fun getChunkLocation(): Pair<BigDecimal, BigDecimal> = getLocationFromPoint(Pair(0f, 0f))

    fun getRenderLocationFromLocation(position: Pair<BigDecimal, BigDecimal>): Pair<BigDecimal, BigDecimal> {
        return Pair(
            position.first - renderPosition.first * renderScale.first.toBigDecimal(),
            position.second - renderPosition.second * renderScale.second.toBigDecimal()
        )
    }

    fun getRenderLocationFromVector(vector: Pair<Float, Float>) =
        getRenderLocationFromLocation(getLocationFromVector(vector))

    fun getRenderLocationFrom3DVector(vector: Triple<Float, Float, Float>): Pair<BigDecimal, BigDecimal> {
        val loc = getRenderLocationFromLocation(
            getLocationFromVector(
                Pair(
                    vector.first,
                    vector.second
                )
            )
        )
        return Pair(
            loc.first, loc.second -
                    vector.third.toBigDecimal() * offset.third.toBigDecimal() * renderScale.second.toBigDecimal()
        )
    }
}
