package dev.linwood.cuboverse.common.game

import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.res.loadImageBitmap
import androidx.compose.ui.res.useResource
import dev.linwood.cubos.api.Chunk
import dev.linwood.cubos.api.CubicEntity
import dev.linwood.cubos.api.RenderContext
import dev.linwood.cubos.api.Vector3D

class TestBlock(chunk: Chunk, vector3D: Vector3D) : CubicEntity(chunk, vector3D) {
    override fun getImage(context: RenderContext): ImageBitmap =
        image

    companion object {
        private val image = useResource("test.png", ::loadImageBitmap).apply {
            prepareToDraw()
        }
    }
}

