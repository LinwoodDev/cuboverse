package dev.linwood.cuboverse.common

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ImageBitmap
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
).let {
    val firstChunk = Chunk().apply {
        // H
        addObject(TestBlock(1f, 0f))
        addObject(TestBlock(1f, 1f))
        addObject(TestBlock(1f, 2f))
        addObject(TestBlock(2f, 1f))
        addObject(TestBlock(3f, 0f))
        addObject(TestBlock(3f, 1f))
        addObject(TestBlock(3f, 2f))

        // I
        addObject(TestBlock(5f, 0f))
        addObject(TestBlock(5f, 1f))
        addObject(TestBlock(5f, 2f))
    }
    it.addChunk(firstChunk, BigInteger.ZERO, BigInteger.ZERO)
    it.addChunk(Chunk().apply {
        // O
        addObject(TestBlock(7f, 0f))
        addObject(TestBlock(7f, 1f))
        addObject(TestBlock(7f, 2f))
        addObject(TestBlock(8f, 0f))
        addObject(TestBlock(8f, 2f))
        addObject(TestBlock(9f, 0f))
        addObject(TestBlock(9f, 1f))
        addObject(TestBlock(9f, 2f))

        for (i in 0..15) {
            addObject(TestBlock(0f, i.toFloat()))
        }
    }, BigInteger.ONE, BigInteger.ZERO)
    it.addChunk(Chunk().apply {
        for (i in 0..15) {
            addObject(TestBlock(0f, i.toFloat()))
        }
    }, BigInteger.ONE, BigInteger.ONE)
    it.addChunk(firstChunk, BigInteger.TWO, BigInteger.TWO)
    it.addChunk(Chunk().apply {
        for (i in 0..15) {
            for (j in 0..15) {
                addObject(TestBlock(i.toFloat(), j.toFloat()))
            }
        }
    }, BigInteger.TWO, BigInteger.valueOf(3))
    it.addChunk(Chunk().apply {
        addObject(TestBlock(0.0f, 0.0f, 0.0f))
        addObject(TestBlock(1.0f, 0.0f, 0.0f))
        addObject(TestBlock(1.0f, 0.0f, 1.0f))
        addObject(TestBlock(2.0f, 0.0f, 0.0f))
        addObject(TestBlock(2.0f, 0.0f, 1.0f))
        addObject(TestBlock(2.0f, 0.0f, 2.0f))
        addObject(TestBlock(3.0f, 0.0f, 0.0f))
        addObject(TestBlock(3.0f, 0.0f, 1.0f))
        addObject(TestBlock(4.0f, 0.0f, 0.0f))

        addObject(TestBlock(1.0f, 1.0f, 0.0f))
        addObject(TestBlock(2.0f, 1.0f, 0.0f))
        addObject(TestBlock(2.0f, 1.0f, 1.0f))
        addObject(TestBlock(3.0f, 1.0f, 0.0f))
        addObject(TestBlock(2.0f, 2.0f, 0.0f))

        addObject(TestBlock(1.0f, 0.0f, -1.0f))
        addObject(TestBlock(2.0f, 0.0f, -1.0f))
        addObject(TestBlock(2.0f, 0.0f, -2.0f))
        addObject(TestBlock(3.0f, 0.0f, -1.0f))
        addObject(TestBlock(2.0f, 1.0f, -1.0f))
    }, BigInteger.valueOf(3), BigInteger.ZERO)
    it
}

class TestBlock(x: Float, y: Float, z: Float = 0.0f) : CubicChunkObject(x, y, z) {
    override fun getImage(context: RenderContext): ImageBitmap {
        // Load from resources
        return useResource("test.png", ::loadImageBitmap)
    }
}

@Composable
fun WorldRenderer(
    renderLocation: Pair<BigDecimal, BigDecimal> = Pair(BigDecimal.ZERO, BigDecimal.ZERO),
    renderScale: Pair<Int, Int>
) {
    Canvas(
        modifier = Modifier.fillMaxSize()
    ) {
        // Set width and height
        drawIntoCanvas {
            world.paint(
                it,
                renderLocation,
                Pair(1920.0, 1080.0),
                renderScale,
            )
        }
    }
}