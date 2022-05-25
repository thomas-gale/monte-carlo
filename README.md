# monte-carlo
Learn about Monte Carlo methods via making stuff.

## stack
- `rust` language
- `wgpu` binding library
- `wgsl` shader language

## plan
- [x] Follow (again) [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) implementing as much code as wgsl fragment/compute shaders
- [ ] Extend to https://raytracing.github.io/books/RayTracingTheNextWeek.html (focus on bvh section, rectangles/lights and volumes)
- [ ] Explore https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html
- [ ] Move to the fascinating monte carlo/wos papers and try to generalise the wgsl compute shaders to do other (non ray tracing) monte carlo based simulations sampling e.g. https://www.cs.cmu.edu/~kmcrane/Projects/MonteCarloGeometryProcessing/index.html & https://cs.dartmouth.edu/wjarosz/publications/sawhneyseyb22gridfree.html

## pictures
### [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
See `./basic_raytracing.wgsl`, `./basic_raytracing.rs` and `./basic_raytracing` directory for implementation.
#### On Nvidia RTX 2070 Super in ~2min: 50 bouce depth, sampling at approx 22fps (single sample per pixel) (~2700 samples)
![ray_tracing_in_one_weekend](https://user-images.githubusercontent.com/11990706/170242871-14b1ed44-1134-4bd7-b557-69f7c788fcae.png)
- [x] Refine the computation to spread over multiple frames and store the current pixel sample values in a texture.

## notes
### bvh
- Define a simple aabb (pod) struct buffer structure that allows the fixed texture/buffer allocation of a static bvh on CPU side and pass to GPU
- Write helper methods in GPU wgsl to retrieve data from texture/buffer of bvh to enable GPU side ray intersection tests
- simply store volume information in various boxes within the bvh - this will be the placeholder for generic matter (that we can keep rendering in bvh structure to higher and higher levels of detail).
- todo - verify idea.