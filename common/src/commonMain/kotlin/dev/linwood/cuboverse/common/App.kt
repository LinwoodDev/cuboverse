package dev.linwood.cuboverse.common

import androidx.compose.foundation.layout.Column
import androidx.compose.material.*
import androidx.compose.runtime.*
import kotlinx.coroutines.launch

@Composable
fun App() {
    var text by remember { mutableStateOf("Hello, World!") }
    val platformName = getPlatformName()
    val snackbarHostState = remember { SnackbarHostState() }
    val scope = rememberCoroutineScope()

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
            }
        }
    )
}
