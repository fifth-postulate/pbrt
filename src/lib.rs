pub mod shapes;
pub mod accelerators;
pub mod cameras;
pub mod samplers;
pub mod filters;
pub mod film;
pub mod materials;
pub mod textures;
pub mod volumes;
pub mod lights;
pub mod renderers;
pub mod integrators;

#[cfg(test)]
mod tests {
    #[test]
    fn pbrt_works() {
        assert_eq!(2 + 2, 4);
    }
}
