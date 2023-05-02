package dev.linwood.cubos.api

import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.Paint
import androidx.compose.ui.unit.IntOffset
import androidx.compose.ui.unit.IntSize

abstract class ChunkObject(val x: Float, val y: Float, val z: Float = 0.0f) {
    open fun load(context: ChunkContext) {}
    open fun update(context: ChunkContext) {}
    open fun tick(context: GlobalChunkContext) {}
    open fun unload(context: ChunkContext) {}
    open fun canLoad(context: ChunkContext): Boolean = false

    open fun paint(context: RenderContext) {}

    fun getRenderPriority(): Int {
        return 0;
    }
}


abstract class CubicChunkObject(x: Float, y: Float, z: Float) : ChunkObject(x, y, z) {
    abstract fun getImage(context: RenderContext): ImageBitmap
    fun getPaint(context: RenderContext): Paint = Paint()

    override fun paint(context: RenderContext) {
        val image = getImage(context)
        val paint = getPaint(context)
        val location = context.getRenderLocationFrom3DVector(Triple(x, y, z))
        val size = IntSize(
            context.renderScale.first * image.width,
            context.renderScale.second * image.height
        )
        context.canvas.drawImageRect(
            image,
            IntOffset.Zero,
            IntSize(image.width, image.height),
            IntOffset(location.first.toInt(), location.second.toInt()),
            size,
            paint
        )
    }
}
