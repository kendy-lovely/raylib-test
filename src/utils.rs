pub fn round_to_nearest(x: f32, a: f32, b: f32) -> f32 {
    let (min_val, max_val) = (a.min(b), a.max(b));
    let diff_min = (x - min_val).abs();
    let diff_max = (x - max_val).abs();
    if x > min_val && x < max_val {
        if diff_min > diff_max { b } else if diff_min < diff_max { a } else { a }
    } else { x }
}