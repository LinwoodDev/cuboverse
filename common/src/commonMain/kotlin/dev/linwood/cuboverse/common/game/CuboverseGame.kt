package dev.linwood.cuboverse.common.game

import CuboversePlayer
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Canvas
import androidx.compose.ui.input.key.KeyEvent
import dev.linwood.cubos.api.Chunk
import dev.linwood.cubos.api.ChunkCoordinate
import dev.linwood.cubos.api.Vector3D
import dev.linwood.cubos.api.World
import java.math.BigInteger

class CuboverseGame {
    private val world = World(
        Pair(16, 16), Triple(
            48.0f,
            16.0f,
            48.0f
        ), -1f,
        ::chunkBuilder
    )
    private val player: CuboversePlayer = world.getChunk(ChunkCoordinate(BigInteger.ZERO, BigInteger.ZERO))
        .addEntityOfType(::CuboversePlayer, Vector3D(0f, 0f, 1f))

    private fun chunkBuilder(chunk: Chunk) {
        for (i in 0..15) {
            for (j in 0..15) {
                chunk.addEntity(::TestBlock, Vector3D(i.toFloat(), j.toFloat(), 0f))
            }
        }
    }

    fun onKeyEvent(event : KeyEvent) = player.onKeyEvent(event)
    fun paint(canvas: Canvas, size: Size) = player.paintWorld(canvas, size)
}