package dev.linwood.cubos.api

import androidx.compose.runtime.mutableStateOf
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.Paint
import androidx.compose.ui.unit.IntOffset
import androidx.compose.ui.unit.IntSize
import kotlinx.serialization.*
import kotlinx.serialization.json.*

typealias EntityBuilder<T> = (chunk: Chunk, vector3D: Vector3D) -> T

abstract class Entity(currentChunk: Chunk, vector3D: Vector3D) {
    private var currentChunk = mutableStateOf(currentChunk)
    private var currentVector3D = mutableStateOf(vector3D)
    open var chunk
        get() = currentChunk.value
        set(value) {
            chunk.removeEntity(this)
            currentChunk.value = value
            value.addEntity(this)
        }
    var vector3D
        get() = currentVector3D.value
        set(value) {
            currentVector3D.value = value
        }
    var vector
        get() = Vector(vector3D.first, vector3D.second)
        set(value) {
            vector3D = Vector3D(value.first, value.second, vector3D.third)
        }

    open fun load() {}
    open fun update() {}
    open fun tick() {}
    open fun unload() {}
    open fun canLoad(): Boolean = false

    open fun paint(context: RenderContext) {}

    fun getRenderPriority(): Int = 0

    fun save(): String {
        return Json.encodeToString(this)
    }

    fun move(delta: Vector3D) {
        val newVector = Vector(
            vector3D.first + delta.first,
            vector3D.second + delta.second
        )
        val globalPos = chunk.getPositionFromVector(newVector)
        val newChunk = chunk.world.getChunkByPosition(globalPos)
        var chunkPlace = newChunk.getChunkPlace()
        if (newChunk != chunk) {
            chunk = newChunk
        }
        vector3D = Vector3D(
            (globalPos.first - chunkPlace.first.toBigDecimal()).toFloat(),
            (globalPos.second - chunkPlace.second.toBigDecimal()).toFloat(),
            vector3D.third + delta.third
        )

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
