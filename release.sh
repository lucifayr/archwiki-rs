#!/usr/bin/env sh

if [ -z $1 ];then 
    echo "provide a version of the x.y.z"
    exit 1
fi

version=$1

cargo build --release \
    && git add Cargo.toml Cargo.lock \
    git commit -m "bump version to $version" \
    && git tag "v$version" && git push --follow-tags && git push github && git push --tags github \
    && glab release create "v$version" -n "v$version" && gh release create "v$version" -t "v$version" --generate-notes
