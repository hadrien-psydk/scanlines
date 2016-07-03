# scanlines
Convert an image as if it was displayed on a CRT display.
Usage:
```
scanlines INFILE [OUTFILE]
```
If OUTFILE is missing, a suffix is appended to INFILE for the new file name.
The result is saved as a PNG file.

![ScreenShot](example-scanlines.png)

To quikly view the result after a conversion (in case you tweak various parameters of the program), you can use [feh](http://feh.finalrewind.org) (an image viewer) that supports a lot of command line options:
```
sudo apt install feh
cargo run -- example.png out.png && feh --force-aliasing out.png
```