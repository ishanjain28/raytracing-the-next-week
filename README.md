# Ray Tracing the Next Week

This is my attempt at Ray Tracing the Next Week book by Peter Shirley.


On x86_64 target, It'll try to use AVX2. If you get build errors, Try commenting the simd_vec3 import in `src/types/mod.rs`.
I tried changing the cfg attribute to, `all(target_arch = "x86_64", target_feature = "avx2")` but it keeps reporting that `avx2` feature is disabled on my machine?


Without AVX2, The final scene takes ~33 seconds to render at a certain setting and with AVX2 the same scene takes ~23.5 seconds.

[_Ray Tracing: The Next Week_](https://raytracing.github.io/books/RayTracingTheNextWeek.html)

# Renders


![[1] Motion Blur](https://user-images.githubusercontent.com/7921368/114031856-4d419b80-986b-11eb-8a56-b9ecc1785a6e.png)
[1] Motion Blur

![[2] Cornell Box](https://user-images.githubusercontent.com/7921368/114031671-1e2b2a00-986b-11eb-9528-4fd5525c43ea.png)
[2] Cornell Box
