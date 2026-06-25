import org.gradle.api.tasks.bundling.Zip

plugins {
  `java-library`
  `maven-publish`
  signing
}

sourceSets {
  main {
    resources {
      srcDir(".")
      include("*.xsd")
    }
  }
}

java {
  withSourcesJar()
  withJavadocJar()
}


publishing {
  publications {
    create<MavenPublication>("mavenJava") {
      from(components["java"])
      pom {
        name.set("schema-xsd")
        description.set("XSD schema definitions for the schema library XML formats.")
        url.set("https://github.com/jstano/schema-xsd")
        licenses {
          license {
            name.set("MIT License")
            url.set("https://opensource.org/license/mit")
          }
        }
        developers {
          developer {
            id.set("jstano")
            name.set("Jeff Stano")
            email.set("jeff@stano.com")
          }
        }
        scm {
          connection.set("scm:git:https://github.com/jstano/schema-xsd.git")
          developerConnection.set("scm:git:ssh://git@github.com:jstano/schema-xsd.git")
          url.set("https://github.com/jstano/schema-xsd")
        }
      }
    }
  }
  repositories {
    maven {
      url = layout.buildDirectory.dir("staging-deploy").get().asFile.toURI()
    }
  }
}

signing {
  isRequired = gradle.taskGraph.hasTask("publish")
  sign(publishing.publications["mavenJava"])
}

tasks.register<Zip>("zipStagingDeploy") {
  archiveFileName.set("staging-deploy.zip")
  destinationDirectory.set(layout.buildDirectory.dir("tmp"))
  from("build/staging-deploy") {
    include("**/*")
  }
}
