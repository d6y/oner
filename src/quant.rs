fn quantize<V>(column: &[V], classifications: &[V], small: i32) -> Vec<V>
where
    V: Clone,
{
    column.to_vec()
}

#[cfg(test)]
mod test_quantize {
    use super::quantize;
    #[test]
    fn test_golf_example() {
        // This example (inputs, and boundary points) comes from:
        // Nevill-Manning, Holmes & Witten (1995)  _The Development of Holte's 1R Classifier_, p. 2
        let input = [
            "64", "65", "68", "69", "70", "71", "72", "72", "75", "75", "80", "81", "83", "85",
        ];
        let class = [
            "P", "D", "P", "P", "P", "D", "P", "D", "P", "P", "D", "P", "P", "D",
        ];
        assert_eq!(input.len(), class.len());

        let i1 = ">= 64 and < 71";
        let i2 = ">= 71 and < 85";
        let i3 = ">= 85";
        let expected = [i1, i1, i1, i1, i1, i2, i2, i2, i2, i2, i2, i2, i2, i3];
        assert_eq!(expected.to_vec(), quantize(&input, &class, 3));
    }
}
