# monte-carlo
Learning about Monte Carlo methods.

## stack
- `rust` language
- `wgpu` binding library
- `wgsl` shader language

## plan
- [x] Follow (again) [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) implementing as much code as wgsl fragment/compute shaders
- [ ] Extend to https://raytracing.github.io/books/RayTracingTheNextWeek.html
- [ ] Explore https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html
- [ ] Move to the fascinating monte carlo/wos papers and try to generalise the wgsl compute shaders to do other (non ray tracing) monte carlo based simulations sampling e.g. https://www.cs.cmu.edu/~kmcrane/Projects/MonteCarloGeometryProcessing/index.html & https://cs.dartmouth.edu/wjarosz/publications/sawhneyseyb22gridfree.html

## samples
### [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) output (takes ~5 seconds to generate on RTX 2070 Super)
See `./basic_raytracing.wgsl`, `./basic_raytracing.rs` and `./basic_raytracing` directory for implementation.

![ray_tracing_in_one_weekend](https://user-images.githubusercontent.com/11990706/170103953-7e279469-3915-47b4-b8fa-0b748689ce7f.png)

- [ ] Refine the computation to spread over multiple frames and store the current pixel sample values in a texture.
