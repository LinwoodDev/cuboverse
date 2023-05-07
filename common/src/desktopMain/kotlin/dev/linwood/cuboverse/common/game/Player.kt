package dev.linwood.cuboverse.common.game

import androidx.compose.ui.graphics.Canvas
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.res.loadImageBitmap
import androidx.compose.ui.res.useResource
import androidx.compose.ui.window.WindowPosition.PlatformDefault.x
import dev.linwood.cubos.api.*

class Player(chunk : Chunk, vector3D : Vector3D) : CubicEntity(chunk, vector3D) {
    override fun getImage(context: RenderContext): ImageBitmap = useResource("player.png", ::loadImageBitmap)

    fun input() {

    }

    fun paintWorld(
        world: World, canvas: Canvas,
        renderSize: Pair<Float, Float>,
        renderScale: Pair<Int, Int> = Pair(1, 1)
    ) {
        val chunk = world.getChunkPositionByObject(this) ?: return
        val globalX = chunk.first.toBigDecimal() + vector3D.first.toBigDecimal()
        val globalY = chunk.second.toBigDecimal() + vector3D.second.toBigDecimal()
        val center = Pair(
            (renderSize.first / 2 * renderScale.first).toBigDecimal() + globalX,
            (renderSize.second / 2 * renderScale.second).toBigDecimal() + globalY
        )
        world.paint(
            canvas,
            center,
            renderSize,
            renderScale
        );
    }
}