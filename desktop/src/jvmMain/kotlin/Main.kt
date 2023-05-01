import androidx.compose.ui.window.Window
import androidx.compose.ui.window.application
import dev.linwood.cuboverse.common.App


fun main() = application {
    Window(onCloseRequest = ::exitApplication) {
        App()
    }
}
