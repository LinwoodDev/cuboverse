package dev.linwood.cubos.api

import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Canvas
import androidx.compose.ui.unit.IntSize
import java.math.BigDecimal
import java.math.BigInteger

abstract class Player(currentChunk: Chunk, vector3D: Vector3D) : CubicEntity(currentChunk, vector3D) {
    fun paintWorld(canvas: Canvas, size: Size) {
        val globalPos = chunk.getPositionFromVector(vector)
        val renderContext = RenderContext(
            canvas,
            chunk,
            Position(BigDecimal.ZERO, BigDecimal.ZERO),
            Pair(size.width, size.height),
            Pair(1, 1)
        )
        val globalLoc = renderContext.getLocationFromVector(vector)
        val center = Pair(
            (size.width / -2).toBigDecimal() + globalLoc.first,
            (size.height / -2).toBigDecimal() + globalLoc.second
        )
        chunk.world.paint(canvas, center, Pair(size.width, size.height))
    }
}
