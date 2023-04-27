pub fn multiplier(numbers:&[f64]) -> f64{
      let mut result = 1.0;
      for &num in numbers{
            result *= num;
      }
      result
}