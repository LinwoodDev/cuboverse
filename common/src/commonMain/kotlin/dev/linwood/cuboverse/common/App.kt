package dev.linwood.cuboverse.common

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.*
import androidx.compose.runtime.*
import kotlinx.coroutines.launch
import java.math.BigDecimal

@Composable
fun App() {
    var text by remember { mutableStateOf("Hello, World!") }
    val platformName = getPlatformName()
    val snackbarHostState = remember { SnackbarHostState() }
    val scope = rememberCoroutineScope()
    val operationIncrement = BigDecimal.TEN
    var scale by remember { mutableStateOf(Pair(1, 1)) }

    Scaffold (
        snackbarHost = { SnackbarHost(snackbarHostState) },

        content = {
            Column {

                Text("Hello $platformName!")
                TextField(value = text, onValueChange = { text = it })
                Text("Typed: $text")
                Button(onClick = {
                    scope.launch {
                        snackbarHostState.showSnackbar(
                            message = "Snackbar #",
                            actionLabel = "Action on"
                        )
                        println("Snackbar closed")
                    }
                }) { Text("Do >Magic<") }
                Text("Scale: $scale")
                Row {
                    Button(onClick = { scale = Pair(scale.first + 1, scale.second) }) { Text("Zoom In X") }
                    Button(onClick = { scale = Pair(scale.first - 1, scale.second) }) { Text("Zoom Out X") }
                    Button(onClick = { scale = Pair(scale.first, scale.second + 1) }) { Text("Zoom In Y") }
                    Button(onClick = { scale = Pair(scale.first, scale.second - 1) }) { Text("Zoom Out Y") }
                }
                WorldRenderer(scale)
            }
        }
    )
}
