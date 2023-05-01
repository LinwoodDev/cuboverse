package dev.linwood.cuboverse.common

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.drawscope.drawIntoCanvas
import dev.linwood.cubos.api.Chunk
import dev.linwood.cubos.api.World
import java.math.BigDecimal
import java.math.BigInteger

val world = World(Pair(16, 16), 1f).let {
    it.addChunk(Chunk(), BigInteger.ZERO, BigInteger.ZERO)
    it
}

@Composable
fun WorldRenderer() {
    Canvas(
        modifier = Modifier.fillMaxSize()
    ) {
        drawIntoCanvas {
            world.getChunk(0, 0)
                .paint(
                    it,
                    world,
                    Pair(BigDecimal.ZERO, BigDecimal.ZERO),
                    Pair(1920.0, 1080.0),
                    Pair(16, 16),
                    Pair(BigInteger.ZERO, BigInteger.ZERO),
                )
        }
    }
}