/// A representation of a f64 based distance matrix.
#[derive(Debug)]
pub struct DistanceMat {
    distances: Vec<Vec<f64>>,
}

impl DistanceMat {
    /// Create a new distance mat based on exising
    /// distances.
    ///
    /// # Arguments
    ///
    /// * `distances` - The distances between all indexes 0..n. The matrix
    /// is assumed to be symmetrical and the distance between an object and itself
    /// (the diagonal) should be only 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algo::distance_mat::DistanceMat;
    ///
    /// let distance_matrix = DistanceMat::new(vec![vec![0.0,1.0,2.0], vec![1.0,0.0,3.0], vec![2.0,3.0,0.0]]);
    /// ```
    pub fn new(distances: Vec<Vec<f64>>) -> Self {
        DistanceMat { distances }
    }
    /// Get the number of nodes in the distance matrix, e.g. one of its dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algo::distance_mat::DistanceMat;
    ///
    /// let distance_matrix = DistanceMat::new(vec![vec![0.0,1.0,2.0], vec![1.0,0.0,3.0], vec![2.0,3.0,0.0]]);
    /// println!("{}", distance_matrix.n_units());
    /// ```
    pub fn n_units(&self) -> usize {
        self.distances.len()
    }
    /// Given a sequence of nodes (in a `Route`-object) compute the distance for the round-
    /// trip between node 0..0
    ///
    /// # Arguments
    ///
    /// * `route` - The sequence of nodes that is visited and for which the round-trip-lenght
    /// should be computed.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algo::distance_mat::DistanceMat;
    /// use genetic_algo::route::Route;
    ///
    /// let distance_matrix = DistanceMat::new(vec![vec![0.0,1.0,2.0], vec![1.0,0.0,3.0], vec![2.0,3.0,0.0]]);
    /// println!("{}", distance_matrix.get_distance(&vec![1,0,2]));
    /// ```
    pub fn get_distance(&self, route: &[usize]) -> f64 {
        route
            .iter()
            .fold(
                // By folding the indexes we get the distances between 1-2, 2-3, ... , (n-1)-n.
                // Then we are missing n-0, therefore that's the initial value we choose in the `fold`-
                // operator.
                (self.distances[route[route.len() - 1]][route[0]], None),
                |(mut loss, last_point): (f64, Option<usize>), current_point| {
                    if let Some(last_point) = last_point {
                        loss += &self.distances[last_point][*current_point];
                    }
                    (loss, Some(*current_point))
                },
            )
            .0
    }
}

#[cfg(test)]
mod test_distance_mat {
    use super::*;
    use crate::test_utils::test_dist_mat;
    #[test]
    fn test_constructor() {
        let dist_mat = DistanceMat::new(vec![vec![0.0, 1.0], vec![1.0, 0.0]]);
        assert_eq!(dist_mat.distances, vec![vec![0.0, 1.0], vec![1.0, 0.0]]);
    }
    #[test]
    fn test_dist_same_node() {
        assert_eq!(test_dist_mat().get_distance(&vec![0, 0]), 0.0);
    }
    #[test]
    fn test_dist_two_nodes() {
        assert_eq!(test_dist_mat().get_distance(&vec![0, 1]), 2.0);
        assert_eq!(test_dist_mat().get_distance(&vec![0, 2]), 4.0);
        assert_eq!(test_dist_mat().get_distance(&vec![1, 2]), 6.0);
    }
    #[test]
    fn test_dist_three_nodes() {
        assert_eq!(test_dist_mat().get_distance(&vec![0, 1, 2]), 6.0);
        assert_eq!(test_dist_mat().get_distance(&vec![0, 2, 1]), 6.0);
    }
    #[test]
    fn test_dist_repeat_visit() {
        assert_eq!(test_dist_mat().get_distance(&[0, 2, 1, 2]), 10.0);
    }
}
