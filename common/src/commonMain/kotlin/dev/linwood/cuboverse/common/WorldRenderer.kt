package dev.linwood.cuboverse.common

import androidx.compose.foundation.*
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.remember
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.drawscope.clipRect
import androidx.compose.ui.graphics.drawscope.drawIntoCanvas
import androidx.compose.ui.input.key.onKeyEvent
import androidx.compose.ui.res.loadImageBitmap
import androidx.compose.ui.res.useResource
import dev.linwood.cubos.api.*
import dev.linwood.cuboverse.common.game.CuboverseGame
import java.math.BigDecimal

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun WorldRenderer(
    renderScale: Pair<Int, Int>
) {
    val focusRequester = remember { FocusRequester() }
    val game = remember {
        CuboverseGame()
    }
    DisposableEffect(Unit) {
        focusRequester.requestFocus()
        onDispose { }
    }
    Canvas(
        modifier = Modifier.fillMaxSize()
            .onClick {
                focusRequester.requestFocus()
            }
            .focusRequester(focusRequester)
            .onKeyEvent(game::onKeyEvent)
            .focusable()
    ) {
        this.clipRect {
            drawIntoCanvas {
                game.paint(it, size)
            }
        }
    }
}