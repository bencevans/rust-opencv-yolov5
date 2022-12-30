# YOLOv5 Inference in Rust

1. Follow the documentation in the ultralytics repository to export your model in ONNX format.
2. Ensure you have OpenCV installed (`brew install opencv`/`choco install opencv` etc.)

Add the project as a dependency.

```
cargo add git+https://github.com/bencevans/rust-opencv-yolov5.git
```

Check the test in `src/lib.rs` for example inference.
