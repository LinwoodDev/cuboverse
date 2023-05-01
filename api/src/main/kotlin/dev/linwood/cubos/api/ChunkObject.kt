package dev.linwood.cubos.api

import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Canvas
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.Paint

abstract class ChunkObject(val x : Float, val y:Float, val z :Float) {
    abstract fun load(context : ChunkContext)
    abstract fun update(context : ChunkContext)
    abstract fun unload(context : ChunkContext)
    abstract fun canLoad(context : ChunkContext) : Boolean

    abstract fun paint(context : RenderContext);

    fun getRenderPriority() : Int {
        return 0;
    }
}

interface SimpleChunkRenderer {
    fun getRenderLocation(context: RenderContext): Pair<Float, Float>

    fun paint(context: RenderContext) {
        val (x, y) = getRenderLocation(context)
        paint(context, x, y)
    }

    fun paint(context: RenderContext, x: Float, y: Float)
}

interface CubicChunkRenderer : SimpleChunkRenderer {
    fun getImage(context: RenderContext): ImageBitmap
    fun getPaint(context: RenderContext): Paint = Paint()
    override fun paint(context: RenderContext, x: Float, y: Float) {
        val image = getImage(context)
        context.canvas.drawImage(image, Offset(x, y), getPaint(context))
    }
}
