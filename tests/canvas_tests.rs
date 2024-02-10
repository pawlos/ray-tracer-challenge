use ray_tracer_challenge::*;

#[cfg(test)]
mod canvas {
    use super::*;

    #[test]
    /// Creating a canvas
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert!(c.pixels.iter().all(|c| *c == color(0.0,0.0,0.0)));
    }

    #[test]
    /// Writing pixels to a canvas
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10,20);
        let red = color(1.0,0.0,0.0);
        c.write_pixel(2,3, red);
        assert_eq!(c.pixel_at(2,3), red);
    }

    #[test]
    /// Constructing the PPM header
    fn constructing_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = canvas_to_ppm(c);
        assert_eq!(ppm, r#"P3
5 3
255
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
"#)
    }

    #[test]
    /// Constructing the PPM data
    fn constructing_ppm_data() {
        let mut c = Canvas::new(5,3);
        let c1 = color (1.5, 0.0, 0.0);
        let c2 = color (0.0, 0.5, 0.0);
        let c3 = color (-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let ppm = canvas_to_ppm(c);
        assert_eq!(ppm, r#"P3
5 3
255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
"#)
    }

    #[test]
    /// Splitting long lines in PPM files
    fn splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);

        for y in 0..2 {
            for x in 0..10 {
                c.write_pixel(x, y, color(1.0, 0.8, 0.6))
            }
        }

        let ppm = canvas_to_ppm(c);
        assert_eq!(ppm, r#"P3
10 2
255
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
"#)
    }

    #[test]
    /// PPM files are terminated by a newline character
    fn ppm_files_are_terminated_by_newline_character() {
        let c = Canvas::new(5,3);
        let ppm = canvas_to_ppm(c);
        assert_eq!(ppm, r#"P3
5 3
255
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
"#)
    }
}