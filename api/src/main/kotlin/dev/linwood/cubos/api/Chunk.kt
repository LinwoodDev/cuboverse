package dev.linwood.cubos.api

import androidx.compose.ui.graphics.Canvas
import java.math.BigDecimal
import java.math.BigInteger


class Chunk {
    private val objects = mutableListOf<ChunkObject>()



    fun getObjects(x: Double, y: Double, distance: Double): Iterable<ChunkObject> {
        return objects.filter {
            it.x >= x - distance && it.x <= x + distance &&
                    it.y >= y - distance && it.y <= y + distance
        }
    }

    inline fun <reified T : ChunkObject> getObjectsByType(x: Double, y: Double, distance: Double): Iterable<T> {
        return getObjects(x, y, distance).filterIsInstance<T>()
    }

    fun getObject(x: Double, y: Double, distance: Double): ChunkObject? {
        return getObjects(x, y, distance).firstOrNull()
    }

    inline fun <reified T : ChunkObject> getObjectByType(x: Double, y: Double, distance: Double): T? {
        return getObjects(x, y, distance).filterIsInstance<T>().firstOrNull()
    }

    fun load(world: World) {
        getObjectByPriority().forEach { it.load(ChunkContext(world, this)) }
    }

    fun update(world: World) {
        getObjectByPriority().forEach { it.update(ChunkContext(world, this)) }
    }

    fun tick(world : World, location : Pair<BigInteger, BigInteger>) {
        getObjectByPriority().forEach { it.tick(GlobalChunkContext(location, ChunkContext(world, this))) }
    }

    fun unload(world: World) {
        getObjectByPriority().forEach { it.unload(ChunkContext(world, this)) }
    }

    fun getObjectByPriority(): Iterable<ChunkObject> {
        return objects.sortedWith { a, b ->
            val priority = a.getRenderPriority().compareTo(b.getRenderPriority())
            if (priority != 0) return@sortedWith priority
            val z = a.z.compareTo(b.z)
            if (z != 0) return@sortedWith z
            val y = a.y.compareTo(b.y)
            if (y != 0) return@sortedWith y
            return@sortedWith a.x.compareTo(b.x)
        }
    }

    fun paint(
        canvas: Canvas,
        world: World,
        renderLocation: Pair<BigDecimal, BigDecimal>,
        renderSize: Pair<Double, Double>,
        renderScale: Pair<Int, Int>,
        chunkPosition: Pair<BigInteger, BigInteger>
    ) {
        val context = RenderContext(canvas, GlobalChunkContext(chunkPosition, ChunkContext(world, this)),renderLocation, renderSize, renderScale)
        getObjectByPriority().forEach { it.paint(context) }
    }
}