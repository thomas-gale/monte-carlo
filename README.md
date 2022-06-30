# monte-carlo
Learn about Monte Carlo methods for radiation and conduction.

## stack
- `rust` language
- `wgpu` binding library
- `wgsl` shader language

## running (you'll need decent GPU that supports float textures - tested on Nvidia RTX 2070 Super)
- Install rust https://www.rust-lang.org/tools/install
- `cargo run`
- A fixed `1280x1024` screen will be opened
- Esc to quit
- Left mouse click and drag to orbit (framerate probably will be quite low)
- Right mouse click and drag to change slice plane
- Mouse wheel to zoom

### configuration
https://github.com/thomas-gale/monte-carlo/blob/main/src/bvh_raytracing/constants.rs

## plan
### intro
- [x] Follow (again) [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) implementing as much code as wgsl fragment/compute shaders
- [x] Extend to https://raytracing.github.io/books/RayTracingTheNextWeek.html (focus on bvh section, rectangles/lights and volumes)

### mesh
- [ ] Support loading of .obj / mesh files and parse into the bvh hittables with other primitives
- [ ] Signed distance and ray intersection shader functions for triangles

### monte carlo based simulations sampling e.g. https://www.cs.cmu.edu/~kmcrane/Projects/MonteCarloGeometryProcessing/index.html & https://cs.dartmouth.edu/wjarosz/publications/sawhneyseyb22gridfree.html
- [x] Laplace walk on spheres
- [ ] Poisson walk on spheres

### code structure
- [ ] Refactor from direct wgpu impl to bevy and re-structure the code into the ECS pattern.

## pictures
![ray_tracing_in_one_weekend final scene](https://user-images.githubusercontent.com/11990706/170242871-14b1ed44-1134-4bd7-b557-69f7c788fcae.png)
![bvh preview in final scene](https://user-images.githubusercontent.com/11990706/173129836-4b0307c2-5652-423a-9786-6d6bf775c618.png)
![volume sphere in cornel box](https://user-images.githubusercontent.com/11990706/175810986-34269991-fbdd-437d-9a3c-ae0a062833f4.png)
![wos laplace sample plane](https://user-images.githubusercontent.com/11990706/176406605-579a46f0-e3e3-4bc0-9c83-afd09d81474d.png)
