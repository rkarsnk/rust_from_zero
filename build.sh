#!/usr/bin/env bash
case "$1" in
    hello)
        echo "$1をビルドし実行します．"
        cargo run --manifest-path ./src/hello/Cargo.toml
        ;;
    ch02)
        echo "$1をビルドし実行します．"
        cargo run --manifest-path ./src/$1/1_5/Cargo.toml
        ;;
    *)
        echo "ビルドするプロジェクトを指定してください．"
        ;;
esac