pub fn bpm_time_to_real(mut num: f32, bpm: f32, shuffle: f32, shuffle_period: f32) -> f32 {
    if shuffle_period > 0f32 && ((num * (1f32 / shuffle_period)).trunc() as i32) % 2 == 1 {
        num += shuffle * shuffle_period;
    }

    if bpm > 0f32 {
        num = (num / bpm) * 60f32;
    }

    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bpm_time() {
        assert_eq!(bpm_time_to_real(10f32, 60f32, 1f32, 2f32), 12f32); // arbitrary values compared with c#
    }
}
