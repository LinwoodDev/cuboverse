package dev.linwood.cubos.api

import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Canvas
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.Paint
import androidx.compose.ui.unit.IntOffset
import androidx.compose.ui.unit.IntSize

abstract class ChunkObject(val x: Float, val y: Float, val z: Float) {
    abstract fun load(context: ChunkContext)
    abstract fun update(context: ChunkContext)
    abstract fun tick(context: GlobalChunkContext)
    abstract fun unload(context: ChunkContext)
    abstract fun canLoad(context: ChunkContext): Boolean

    abstract fun paint(context: RenderContext);

    fun getRenderPriority(): Int {
        return 0;
    }
}


interface CubicChunkRenderer {
    val x: Float
    val y: Float

    fun getImage(context: RenderContext): ImageBitmap
    fun getPaint(context: RenderContext): Paint = Paint()

    fun paint(context: RenderContext) {
        val image = getImage(context)
        val paint = getPaint(context)
        val location = context.getRenderPositionFromVector(Pair(x, y))
        context.canvas.drawImageRect(
            image,
            IntOffset.Zero,
            IntSize(image.width, image.height),
            IntOffset(location.first.toInt(), location.second.toInt()),
            IntSize(context.renderScale.first, context.renderScale.second),
            paint
        )
    }
}
