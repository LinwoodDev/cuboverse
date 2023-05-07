package dev.linwood.cubos.api

import androidx.compose.ui.graphics.Canvas
import java.math.BigDecimal
import java.math.BigInteger

/// Relative to the chunk
typealias Vector = Pair<Float, Float>
/// Relative to the chunk including z axis
typealias Vector3D = Triple<Float, Float, Float>
/// Absolute location with offset to the rendering location
typealias Location = Pair<BigDecimal, BigDecimal>
//// Absolute location without offset
typealias Position = Pair<BigDecimal, BigDecimal>
/// Chunk coordinates
typealias ChunkCoordinate = Pair<BigInteger, BigInteger>
/// Chunk coordinates multiplied with its size
typealias ChunkPlace = Pair<BigInteger, BigInteger>

data class RenderContext(
    val canvas: Canvas,
    val chunk: Chunk,
    val renderPosition: Position,
    val renderSize: Pair<Float, Float>,
    val renderScale: Pair<Int, Int>,
) {
    val offset get() = chunk.world.offset
    val distanceOffsetX get() = chunk.world.distanceOffsetX
    val chunkSize get() = chunk.chunkSize

    val renderChunkScale get() = Pair(renderScale.first * chunkSize.first, renderScale.second * chunkSize.second)

    fun getChunkPlace() = chunk.getChunkPlace()

    fun getPointFromVector(vector: Vector): Pair<Float, Float> {
        val y = vector.second * offset.second * renderScale.second
        val x = (vector.first * offset.first + vector.second * offset.second * distanceOffsetX) * renderScale.first
        return Pair(
            x,
            y
        );
    }

    fun getLocationFromVector(vector: Pair<Float, Float>) = getLocationFromPoint(getPointFromVector(vector))

    fun getLocationFromPoint(point: Pair<Float, Float>): Pair<BigDecimal, BigDecimal> {
        val globalChunkLocation = getChunkLocation()
        return Pair(
            globalChunkLocation.first + point.first.toBigDecimal(),
            globalChunkLocation.second + point.second.toBigDecimal()
        )
    }

    fun getChunkLocation(): Location {
        val globChunkPos = getChunkPlace()
        val y = globChunkPos.second.toBigDecimal() * offset.second.toBigDecimal()
        val x =
            (globChunkPos.first.toBigDecimal() * offset.first.toBigDecimal() + globChunkPos.second.toBigDecimal() * offset.second.toBigDecimal() * distanceOffsetX.toBigDecimal()) * renderScale.first.toBigDecimal()
        return Pair(
            x,
            y
        );
    }

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
