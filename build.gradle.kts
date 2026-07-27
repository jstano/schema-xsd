import com.stano.gradle.mavencentralpublish.MavenCentralPublishExtension

plugins {
  id("com.stano.base")
  id("com.stano.java-library")
  id("com.stano.maven-central-publish")
}

extensions.configure<MavenCentralPublishExtension> {
  componentName = "java"
  pomName = "schema-xsd"
  pomDescription = "XSD schema definitions for the schema library XML formats."
  pomUrl = "https://github.com/jstano/schema-xsd"
  licenseName = "MIT License"
  licenseUrl = "https://opensource.org/license/mit"
  developerId = "jstano"
  developerName = "Jeff Stano"
  developerEmail = "jeff@stano.com"
  scmConnection = "scm:git:https://github.com/jstano/schema-xsd.git"
  scmDeveloperConnection = "scm:git:ssh://git@github.com:jstano/schema-xsd.git"
  scmUrl = "https://github.com/jstano/schema-xsd"
}

sourceSets {
  main {
    resources {
      srcDir(".")
      include("*.xsd")
    }
  }
}
