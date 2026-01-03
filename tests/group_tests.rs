use ray_tracer_challenge::*;

#[cfg(test)]
mod group {
    use super::*;

    #[test]
    /// Creating a new group p. 195
    fn creating_a_group() {
        let mut g = group();
        g.set_transform(Matrix::identity4x4());

        let deref_group = g.as_any().downcast_ref::<Group>().unwrap();
        assert_eq!(deref_group.count(), 0);
    }
}
