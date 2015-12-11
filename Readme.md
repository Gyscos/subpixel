`subpixel` is a command to better see the subpixels in a screenshot. It creates a new image 3 times as big, where every pixel in the original image is displayed as the three color bands: R,G,B.

To compile:

```bash
git clone https://github.com/Gyscos/subpixel && cd subpixel
cargo build --release
```

Example usage:
```
$ target/release/subpixel ~/screenshot.png
Loading ~/screenshot.png
Loading file...
Creating blank canvas
Iterating...
....................
Now saving image as ~/screenshot.png.subs.png...
```



Support for other pixel patterns (BGR, pentile, RGBW, ...) is currently not supported, but easy to add.
