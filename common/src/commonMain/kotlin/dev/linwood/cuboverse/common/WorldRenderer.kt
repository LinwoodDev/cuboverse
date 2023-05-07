package dev.linwood.cuboverse.common

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.drawscope.clipRect
import androidx.compose.ui.graphics.drawscope.drawIntoCanvas
import androidx.compose.ui.res.loadImageBitmap
import androidx.compose.ui.res.useResource
import dev.linwood.cubos.api.*
import java.math.BigDecimal
import java.math.BigInteger

val world = World(
    Pair(16, 16), Triple(
        48.0f,
        16.0f,
        48.0f
    ), -1f
).apply {
    addChunk(ChunkCoordinate(BigInteger.ZERO, BigInteger.ZERO)).let {
        // H
        it.addEntity(::TestBlock, Vector3D(1f, 0f, 0f))
        it.addEntity(::TestBlock, Vector3D(1f, 1f, 0f))
        it.addEntity(::TestBlock, Vector3D(1f, 2f, 0f))
        it.addEntity(::TestBlock, Vector3D(2f, 1f, 0f))
        it.addEntity(::TestBlock, Vector3D(3f, 0f, 0f))
        it.addEntity(::TestBlock, Vector3D(3f, 1f, 0f))
        it.addEntity(::TestBlock, Vector3D(3f, 2f, 0f))

        // I
        it.addEntity(::TestBlock, Vector3D(5f, 0f, 0f))
        it.addEntity(::TestBlock, Vector3D(5f, 1f, 0f))
        it.addEntity(::TestBlock, Vector3D(5f, 2f, 0f))
    }
    addChunk(ChunkCoordinate(BigInteger.ONE, BigInteger.ZERO)).let {
        // O
        it.addEntity(::TestBlock, Vector3D(7f, 0f, 0f))
        it.addEntity(::TestBlock, Vector3D(7f, 1f, 0f))
        it.addEntity(::TestBlock, Vector3D(7f, 2f, 0f))
        it.addEntity(::TestBlock, Vector3D(8f, 0f, 0f))
        it.addEntity(::TestBlock, Vector3D(8f, 2f, 0f))
        it.addEntity(::TestBlock, Vector3D(9f, 0f, 0f))
        it.addEntity(::TestBlock, Vector3D(9f, 1f, 0f))
        it.addEntity(::TestBlock, Vector3D(9f, 2f, 0f))

        for (i in 0..15) {
            it.addEntity(::TestBlock, Vector3D(0f, i.toFloat(), 0f))
        }
    }
    addChunk(ChunkCoordinate(BigInteger.ONE, BigInteger.ONE)).let {
        for (i in 0..15) {
            it.addEntity(::TestBlock, Vector3D(0f, i.toFloat(), 0f))
        }
    }
    addChunk(ChunkCoordinate(BigInteger.TWO, BigInteger.valueOf(3))).let {
        for (i in 0..15) {
            for (j in 0..15) {
                it.addEntity(::TestBlock, Vector3D(i.toFloat(), j.toFloat(), 0f))
            }
        }
    }
    addChunk(ChunkCoordinate(BigInteger.valueOf(3), BigInteger.ZERO)).let {
        it.addEntity(::TestBlock, Vector3D(0.0f, 0.0f, 0.0f))
        it.addEntity(::TestBlock, Vector3D(1.0f, 0.0f, 0.0f))
        it.addEntity(::TestBlock, Vector3D(1.0f, 0.0f, 1.0f))
        it.addEntity(::TestBlock, Vector3D(2.0f, 0.0f, 0.0f))
        it.addEntity(::TestBlock, Vector3D(2.0f, 0.0f, 1.0f))
        it.addEntity(::TestBlock, Vector3D(2.0f, 0.0f, 2.0f))
        it.addEntity(::TestBlock, Vector3D(3.0f, 0.0f, 0.0f))
        it.addEntity(::TestBlock, Vector3D(3.0f, 0.0f, 1.0f))
        it.addEntity(::TestBlock, Vector3D(4.0f, 0.0f, 0.0f))

        it.addEntity(::TestBlock, Vector3D(1.0f, 1.0f, 0.0f))
        it.addEntity(::TestBlock, Vector3D(2.0f, 1.0f, 0.0f))
        it.addEntity(::TestBlock, Vector3D(2.0f, 1.0f, 1.0f))
        it.addEntity(::TestBlock, Vector3D(3.0f, 1.0f, 0.0f))
        it.addEntity(::TestBlock, Vector3D(2.0f, 2.0f, 0.0f))

        it.addEntity(::TestBlock, Vector3D(1.0f, 0.0f, -1.0f))
        it.addEntity(::TestBlock, Vector3D(2.0f, 0.0f, -1.0f))
        it.addEntity(::TestBlock, Vector3D(2.0f, 0.0f, -2.0f))
        it.addEntity(::TestBlock, Vector3D(3.0f, 0.0f, -1.0f))
        it.addEntity(::TestBlock, Vector3D(2.0f, 1.0f, -1.0f))
    }
}

class TestBlock(chunk: Chunk, vector3D: Vector3D) : CubicEntity(chunk, vector3D) {
    override fun getImage(context: RenderContext): ImageBitmap {
        // Load from resources
        return useResource("test.png", ::loadImageBitmap)
    }
}

class DefaultPlayer(currentChunk: Chunk, vector3D: Vector3D) : Player(currentChunk, vector3D) {
    override fun getImage(context: RenderContext): ImageBitmap {
        return useResource("player.png", ::loadImageBitmap)
    }
}

var player: DefaultPlayer? = null

@Composable
fun WorldRenderer(
    renderLocation: Pair<BigDecimal, BigDecimal> = Pair(BigDecimal.ZERO, BigDecimal.ZERO),
    renderScale: Pair<Int, Int>
) {
    Canvas(
        modifier = Modifier.fillMaxSize()
    ) {
        this.clipRect {
            drawIntoCanvas {
                if (player == null)
                    player = world.getChunk(ChunkCoordinate(BigInteger.ZERO, BigInteger.ZERO))
                        .addEntityOfType(
                            ::DefaultPlayer,
                            Vector3D(renderLocation.first.toFloat(), renderLocation.second.toFloat(), 1f)
                        )
                else
                    player?.vector3D = Vector3D(renderLocation.first.toFloat(), renderLocation.second.toFloat(), 1f)
                player?.paintWorld(it, size)
            }
        }
    }
}