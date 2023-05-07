package dev.linwood.cubos.api

import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Canvas
import java.math.BigDecimal

abstract class Player(currentChunk: Chunk, vector3D: Vector3D) : CubicEntity(currentChunk, vector3D) {
    private var size : Size = Size(0f, 0f)
    fun paintWorld(canvas: Canvas, size: Size) {
        val renderContext = getRenderContext(size)
        val globalLoc = renderContext.getLocationFromVector(vector)
        val center = Pair(
            (size.width / -2).toBigDecimal() + globalLoc.first,
            (size.height / -2).toBigDecimal() + globalLoc.second
        )
        chunk.world.paint(canvas, center, Pair(size.width, size.height))
    }

    private fun getRenderContext(size: Size? = null): RenderContext {
        val current = size ?: this.size
        return RenderContext(
            chunk,
            Position(BigDecimal.ZERO, BigDecimal.ZERO),
            Pair(current.width, current.height),
            Pair(1, 1)
        )
    }
}
