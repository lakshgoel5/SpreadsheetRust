use project::terminal::types::*;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinates_creation() {
        let coord = Coordinates { row: 2, col: 3 };
        assert_eq!(coord.row, 2);
        assert_eq!(coord.col, 3);
    }

    #[test]
    fn test_coordinates_display() {
        let coord = Coordinates { row: 5, col: 7 };
        let output = format!("{}", coord);
        assert_eq!(output, "(5, 7)");
    }

    #[test]
    fn test_coordinates_equality() {
        let coord1 = Coordinates { row: 1, col: 1 };
        let coord2 = Coordinates { row: 1, col: 1 };
        let coord3 = Coordinates { row: 0, col: 0 };

        assert_eq!(coord1, coord2);
        assert_ne!(coord1, coord3);
    }

    #[test]
    fn test_coordinates_clone_and_copy() {
        let coord1 = Coordinates { row: 10, col: 20 };
        let coord2 = coord1; // Copy trait in action
        let coord3 = coord1.clone(); // Clone trait

        assert_eq!(coord1, coord2);
        assert_eq!(coord1, coord3);
    }

    #[test]
    fn test_coordinates_default() {
        let coord = Coordinates::default();
        assert_eq!(coord.row, 0);
        assert_eq!(coord.col, 0);
    }
}
