pluginManagement {
  repositories {
    mavenLocal()
    gradlePluginPortal()
  }
}

plugins {
  id("com.stano.settings") version "0.1.8"
}

rootProject.name = "schema-xsd"

dependencyResolutionManagement {
  repositories {
    mavenLocal()
    mavenCentral()
  }
}
