import org.jetbrains.compose.desktop.application.dsl.TargetFormat

plugins {
    kotlin("jvm")
    id("org.jetbrains.compose")
}

group = "dev.linwood.cuboverse"
version = "1.0-SNAPSHOT"

dependencies {
    implementation(compose.foundation)
}