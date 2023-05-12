package dev.linwood.cubos.api

import androidx.compose.ui.graphics.Canvas
import java.math.BigDecimal
import java.math.BigInteger

typealias ChunkBuilder = (Chunk) -> Unit

object EmptyChunkBuilder : ChunkBuilder {
    override fun invoke(p1: Chunk) {
    }
}

class Chunk(val world: World, val coordinate: ChunkCoordinate) {
    private val objects = mutableListOf<Entity>()

    val chunkSize get() = world.chunkSize

    fun getObjects(): Iterable<Entity> {
        return objects
    }

    inline fun <reified T : Entity> getObjectsByType(): Iterable<T> {
        return getObjects().filterIsInstance<T>()
    }

    fun getObjects(x: Double, y: Double, distance: Double): Iterable<Entity> {
        return objects.filter {
            it.vector.first >= x - distance && it.vector.first <= x + distance &&
                    it.vector.second >= y - distance && it.vector.second <= y + distance
        }
    }

    inline fun <reified T : Entity> getObjectsByType(x: Double, y: Double, distance: Double): Iterable<T> {
        return getObjects(x, y, distance).filterIsInstance<T>()
    }

    fun getObject(x: Double, y: Double, distance: Double): Entity? {
        return getObjects(x, y, distance).firstOrNull()
    }

    inline fun <reified T : Entity> getObjectByType(x: Double, y: Double, distance: Double): T? {
        return getObjects(x, y, distance).filterIsInstance<T>().firstOrNull()
    }

    fun load() {
        getObjectsByPriority().forEach { it.load() }
    }

    fun update() {
        getObjectsByPriority().forEach { it.update() }
    }

    fun tick(location: Pair<BigInteger, BigInteger>) {
        getObjectsByPriority().forEach { it.tick() }
    }

    fun unload() {
        getObjectsByPriority().forEach { it.unload() }
    }

    fun getObjectsByPriority(): Iterable<Entity> = objects.sortedWith { a, b ->
        val priority = a.getRenderPriority().compareTo(b.getRenderPriority())
        if (priority != 0) return@sortedWith priority
        val aVec = a.vector3D
        val bVec = b.vector3D
        val z = aVec.third.compareTo(bVec.third)
        if (z != 0) return@sortedWith z
        val y = aVec.second.compareTo(bVec.second)
        if (y != 0) return@sortedWith y
        return@sortedWith aVec.first.compareTo(bVec.first)
    }

    fun paint(
        canvas: Canvas,
        renderLocation: Pair<BigDecimal, BigDecimal>,
        renderSize: Pair<Float, Float>,
        renderScale: Pair<Int, Int>
    ) {
        val context = RenderContext(
            this,
            renderLocation,
            renderSize,
            renderScale
        )
        getObjectsByPriority().forEach { it.paint(canvas, context) }
    }

    fun addEntity(entityBuilder: EntityBuilder<Entity>, pos: Vector3D): Entity {
        val entity = entityBuilder(this, pos)
        objects.add(entity)
        entity.load()
        return entity
    }

    inline fun <reified T : Entity> addEntityOfType(noinline entityBuilder: EntityBuilder<Entity>, pos: Vector3D): T {
        return addEntity(entityBuilder, pos) as T
    }

    fun addEntity(entity: Entity) {
        objects.add(entity)
    }

    fun removeEntity(entity: Entity) {
        objects.remove(entity)
        entity.unload()
    }

    fun getPositionFromVector(vector: Vector): Position {
        val chunkPos = getChunkPlace()
        return Pair(
            vector.first.toBigDecimal() + chunkPos.first.toBigDecimal(),
            vector.second.toBigDecimal() + chunkPos.second.toBigDecimal()
        );
    }

    fun getChunkPlace(): ChunkPlace {
        val globChunkPos = coordinate
        return Pair(
            globChunkPos.first * chunkSize.first.toBigInteger(),
            globChunkPos.second * chunkSize.second.toBigInteger()
        )
    }
}