# YOLOv5 Inference in Rust

1. Follow the documentation in the ultralytics repository to export your model in ONNX format.
2. Ensure you have OpenCV installed (`brew install opencv`/`choco install opencv` etc.)

For MacOS add the following to the build.rs file in your project.

```rs
fn main() {
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=OpenCL");
        println!("cargo:rustc-link-lib=framework=Accelerate");
    }
}
```

Add the project as a dependency.

```
cargo add git+https://github.com/bencevans/rust-opencv-yolov5.git
```

Check the test in `src/lib.rs` for example inference.
