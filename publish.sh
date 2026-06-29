#!/bin/bash

VERSION="$1"

if [ -z "$VERSION" ]; then
  echo "Error: VERSION is not set. Please provide it as the first argument."
  exit 1
fi

if [ -z "$MAVEN_TOKEN" ]; then
  echo "Error: MAVEN_TOKEN is not set."
  exit 1
fi

function upload() {
  curl --request POST \
    --verbose \
    --header "Authorization: Bearer $MAVEN_TOKEN" \
    --form "bundle=@build/tmp/staging-deploy.zip" \
    "https://central.sonatype.com/api/v1/publisher/upload?name=$1:$2&publishingType=AUTOMATIC"
}

upload "schema-xsd" "$VERSION"
