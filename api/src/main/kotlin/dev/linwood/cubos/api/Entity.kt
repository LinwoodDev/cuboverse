package dev.linwood.cubos.api

import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.Paint
import androidx.compose.ui.unit.IntOffset
import androidx.compose.ui.unit.IntSize
import kotlinx.serialization.*
import kotlinx.serialization.json.*

typealias EntityBuilder<T> = (chunk: Chunk, vector3D: Vector3D) -> T

abstract class Entity(private var currentChunk: Chunk, var vector3D : Vector3D) {
    open var chunk get() = currentChunk
        set(value) {
            currentChunk = value
            currentChunk.removeEntity(this)
            value.addEntity(this)
        }
    val vector get() = Vector(vector3D.first, vector3D.second)
    open fun load() {}
    open fun update() {}
    open fun tick(context: GlobalChunkContext) {}
    open fun unload() {}
    open fun canLoad(): Boolean = false

    open fun paint(context: RenderContext) {}

    fun getRenderPriority(): Int {
        return 0;
    }

    fun save(): String {
        return Json.encodeToString(this)
    }

    companion object {
        inline fun <reified T : Entity> fromJson(json: String): T {
            return Json.decodeFromString(json)
        }

        inline fun <reified T : Entity> create(chunk: Chunk, vector3D: Vector3D): T {
            // Get constructor with Chunk and Vector3D
            val constructor = T::class.constructors.first { it.parameters.size == 2 }
            val entity = constructor.call(chunk, vector3D)
            chunk.addEntity(entity)
            return entity
        }
    }
}


abstract class CubicEntity(chunk: Chunk, vector3D: Vector3D) : Entity(chunk, vector3D) {
    abstract fun getImage(context: RenderContext): ImageBitmap
    fun getPaint(context: RenderContext): Paint = Paint()

    override fun paint(context: RenderContext) {
        val image = getImage(context)
        val paint = getPaint(context)
        val location = context.getRenderLocationFrom3DVector(vector3D)
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
