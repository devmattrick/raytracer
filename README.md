# ray-tracer
![Example of raytracer output](sample.png)
A simple ray tracer written in Rust following
[this tutorial](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

## running
To run this you'll need Rust and cargo installed, as well as a way to view ppm images (I'm using 
[ImageGlass](https://imageglass.org/)).

```
# run the raytracer and output to test.ppm
cargo run > out.ppm

# ...grab a coffee or something...you'll be waiting a while

# In WSL2 you can use this to automatically open in the default app
wslview out.ppm
```
