# A RayTracer in rust 

[Tutorial](https://raytracing.github.io/books/RayTracingInOneWeekend.html#outputanimage/theppmimageformat)

![image](./image.png)

## How to run?

Generate output with 
```
cargo run --release > image.ppm
```

## Benchmaring

Rendering only:
```
hyperfine "cargo run --release"
```

Outputing image
```
hyperfine "cargo run --release > image.ppm"
```

## Downscale image
you need image magick installed. Run:
```
magick image.ppm -quality 10% image.png
```
