# Mandelbrot

A simple command line utility to generate images of the infamous `Mandelbrot Set`.

## Usage

You can build the binary using `cargo build --release`. Then run the following command to run the binary:

Usage: `target/release/mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT`
Example: `target/release/mandelbrot mandel.png 1000x750 -1.20,0.35 -1,0.20`

where

- `FILE`: Path to the file that you want to write the image to. It will create one if the provided file does not exist.
- `PIXELS`: Dimensions of the image that you want to generate. Must be supplied as `WIDTHxHEIGHT`.
- `UPPERLEFT`: The upperleft corner of the complex plane that you want to render in the image. Must be supplied as `X,Y` if the complex number is `x + iy`
- `LOWERTIGHT`: Same as `UPPERRIGHT`, but gives the lower-right corner.

## Example

Using the command `target/release/mandelbrot mandel.jpeg 1200x1200 -1.20,0.35 -1.0,0.2`, you get the [sample image as shown here](./mandel.jpeg).
