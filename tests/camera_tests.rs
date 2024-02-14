#[cfg(test)]
mod camera {
    use std::f32::consts::PI;
    use ray_tracer_challenge::*;

    #[test]
    /// Constructing a camera
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI/2.0;

        let c = camera(hsize, vsize, field_of_view);

        assert_eq!(c.hsize, hsize);
        assert_eq!(c.vsize, vsize);
        assert_eq!(c.field_of_view, field_of_view);
        assert_eq!(c.transform, Matrix::identity4x4());
    }

    #[test]
    /// The pixel size for a horizontal canvas
    fn pixel_size_for_a_horizontal_canvas() {
        let c = camera(200, 125, PI/2.0);

        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    /// The pixel size for a vertical canvas
    fn pixel_size_for_a_vertical_canvas() {
        let c = camera(125, 200, PI/2.0);

        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    /// Constructing a ray through the center of the camera
    fn constructing_a_ray_through_the_center_of_the_camera() {
        let c = camera(201, 101, PI/2.0);
        let r = ray_for_pixel(&c, 100, 50);

        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, vector(0.0, 0.0, -1.0));
    }

    #[test]
    /// Constructing a ray through a corner of the canvas
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = camera(201, 101, PI/2.0);
        let r = ray_for_pixel(&c, 0, 0);

        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    /// Constructing a ray when the camera is transformed
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let mut c = camera(201, 101, PI/2.0);
        c.transform = rotation_y(PI/4.0) * translation(0.0, -2.0, 5.0);

        let r = ray_for_pixel(&c, 100, 50);

        assert_eq!(r.origin, point(0.0, 2.0, -5.0));
        assert_eq!(r.direction, vector(2.0f32.sqrt()/2.0, 0.0, -(2.0f32.sqrt()/2.0)));
    }

    #[test]
    /// Rendering a world with a camera
    fn rendering_a_world_with_a_camera() {
        let w = default_world();
        let mut c = camera(11, 11, PI/2.0);
        let from = point(0.0, 0.0, -5.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        c.transform = view_transformation(from, to, up);
        let image = render(&c, &w);

        assert_eq!(image.pixel_at(5,5), color(0.38066, 0.47583, 0.2855))
    }
}