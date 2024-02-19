#[cfg(test)]
mod lights {
    use ray_tracer_challenge::*;

    #[test]
    /// A point light has a position and intensity
    fn point_has_position_and_intensity() {
        let intensity = color(1.0, 1.0, 1.0);
        let position = point (0.0, 0.0, 0.0);

        let light = point_light(position, intensity);

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}