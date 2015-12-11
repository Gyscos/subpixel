`subpixel` is a command to better see the subpixels in a screenshot. It creates a new image 3 times as big, where every pixel in the original image is displayed as the three color bands: R,G,B.

To compile:

```bash
git clone https://github.com/Gyscos/subpixel && cd subpixel
cargo build --release
```

Example usage:
```
$ target/release/subpixel ~/screenshot.png
Loading file ~/screenshot.png
Creating blank canvas
Iterating
....................
Now saving image as ~/screenshot.png.subs.png
```

Performance-wise, it takes ~0.7s to process a 1080p screenshot on a i7-4790K.

Here are before/after images (scaled to have the same size):

![before](https://raw.github.com/Gyscos/subpixel/master/doc/before.png)

![after](https://raw.github.com/Gyscos/subpixel/master/doc/after.png)


Support for other pixel patterns (BGR, pentile, RGBW, ...) is currently not supported, but easy to add.
