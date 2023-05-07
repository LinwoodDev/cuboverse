package dev.linwood.cubos.api

import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Canvas
import androidx.compose.ui.unit.IntSize
import java.math.BigInteger

abstract class Player(currentChunk: Chunk, vector3D: Vector3D) : CubicEntity(currentChunk, vector3D) {
    private var chunkPos: Pair<BigInteger, BigInteger>? = null
    override var chunk
        get() = super.chunk
        set(value) {
            super.chunk = value
            refreshChunkPos()
        }

    private fun refreshChunkPos() {
        chunkPos = chunk.world.getChunkPositionByObject(this)
    }

    override fun load() {
        refreshChunkPos()
    }

    fun paintWorld(canvas: Canvas, size: Size) {
        val cChunkPos = chunkPos ?: return
        val globalX = cChunkPos.first.toBigDecimal() + vector3D.first.toBigDecimal()
        val globalY = cChunkPos.second.toBigDecimal() + vector3D.second.toBigDecimal()
        val center = Pair(
            (size.width / -2).toBigDecimal() + globalX,
            (size.height / -2).toBigDecimal() + globalY
        )
        chunk.world.paint(canvas, center, Pair(size.width, size.height))
    }
}
