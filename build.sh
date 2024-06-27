#!/usr/bin/env bash
case "$1" in
    hello)
        echo "$1をビルドし実行します．"
        cargo run --manifest-path ./$1/Cargo.toml
        ;;
    *)
        echo "ビルドするプロジェクトを指定してください．"
        ;;
esac