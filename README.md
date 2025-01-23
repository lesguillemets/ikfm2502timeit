# ikfm2502timeit

- 特定の場所に特定の画像のあるフレーム，の続くスパンを csv 形式で出力する
- Dependency: opencv
- まず `cargo run --release -- -f video.mov prepare` とかで参照する見本を作成
- こいつはデフォルトでは `data/va_roi.png` に保存される
- あとは `cargo run --release -- -d dir/ process` とか

- 並列化とか cuda とかはやりたいけどもう当初の目的は達したのでたぶんやらない


