import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.input.key.Key
import androidx.compose.ui.input.key.KeyEvent
import androidx.compose.ui.input.key.key
import androidx.compose.ui.res.loadImageBitmap
import androidx.compose.ui.res.useResource
import dev.linwood.cubos.api.Chunk
import dev.linwood.cubos.api.Player
import dev.linwood.cubos.api.RenderContext
import dev.linwood.cubos.api.Vector3D

class CuboversePlayer(currentChunk: Chunk, vector3D: Vector3D) : Player(currentChunk, vector3D) {
    override fun getImage(context: RenderContext): ImageBitmap {
        return useResource("player.png", ::loadImageBitmap)
    }

    @OptIn(ExperimentalComposeUiApi::class)
    fun onKeyEvent(event: KeyEvent): Boolean {
        when (event.key) {
            Key.W -> {
                move(Vector3D(0f, -1f, 0f))
            }

            Key.S -> {
                move(Vector3D(0f, 1f, 0f))
            }

            Key.A -> {
                move(Vector3D(-1f, 0f, 0f))
            }

            Key.D -> {
                move(Vector3D(1f, 0f, 0f))
            }

            else -> return false
        }
        return true
    }
}
