# Ray Tracing the Next Week

This is my attempt at Ray Tracing the Next Week book by Peter Shirley.


* On x86_64 target, It'll try to use AVX2. If you get build errors, Try commenting the simd_vec3 import in `src/types/mod.rs`.
I tried changing the cfg attribute to, `all(target_arch = "x86_64", target_feature = "avx2")` but it keeps reporting that `avx2` feature is disabled on my machine?
Without AVX2, The final scene takes ~33 seconds to render at a certain setting and with AVX2 the same scene takes ~23.5 seconds.




* It also distribute the render job across all available threads. This is done by dividing the entire area into a 2D grid and then a scheduler assigns chunks to available cores.
This way, We can go quite fast and the number of chunks also plays a role in the overall performance.

With the same resolution and sample and at 144 chunks, It takes 25.10 seconds.
However, if we increase the number of chunks from 144 to say, 256, It only takes ~22.4 seconds.
Similarly At 576 chunks, It only took ~19.5 seconds.
This improvement keeps on happening until 900 chunks, After that I didn't see any significant improvement.



This happens because some chunks finish up a lot faster than others. For example, A chunk that only has some basic background and doesn't have much going on will finish a lot faster than a chunk that consists of reflections on an object. 
If the entire area has not been divided into very small chunks, You'll have a lot of idle cores towards the end because there are no more chunks left and some cores will be busy rendering a complex area of the entire image.

With that said, There is also no point in dividing the image into a million chunks because there is still some overhead associated with each chunk. 

For example, In rust(afaik), I don't have a way to tell the compiler that it's okay to share a vector across multiple threads because each thread will only write to a exclusive section in that vector. So, even though they are sharing the vector, They are not _really_ doing that. 
And because of this, Currently in this project, Each thread writes to a relatively small temporary buffer and when it's done, This buffer is copied to the correct position in the larger buffer that holds the entire image. Then there is also the overhead of spawning all the os threads. 

For each resolution there is probably a sweet spot. In my tests, At 500x500, It appears to be 900 chunks. 


[_Ray Tracing: The Next Week_](https://raytracing.github.io/books/RayTracingTheNextWeek.html)

# Renders


![[1] Motion Blur](https://user-images.githubusercontent.com/7921368/114031856-4d419b80-986b-11eb-8a56-b9ecc1785a6e.png)
[1] Motion Blur

![[2] Cornell Box](https://user-images.githubusercontent.com/7921368/114031671-1e2b2a00-986b-11eb-9528-4fd5525c43ea.png)
[2] Cornell Box


