import org.jetbrains.compose.desktop.application.dsl.TargetFormat

plugins {
    kotlin("jvm")
    kotlin("plugin.serialization")
    `java-library`
    id("org.jetbrains.compose")
}

group = "dev.linwood.cuboverse"
version = "1.0-SNAPSHOT"

dependencies {
    implementation(compose.foundation)
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.5.0")
    implementation(kotlin("reflect"))
}
