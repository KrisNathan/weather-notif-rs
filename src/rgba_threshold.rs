fn check_threshold(a: u8, target: u8, threshold: u8) -> bool {
    let delta = a.abs_diff(target);
    match delta <= threshold {
        true => true,
        false => false,
    }
}

pub fn rgba8_threshold(pixel: &image::Rgba<u8>, target: image::Rgba<u8>, threshold: u8) -> bool {
    let mut conditions = [false; 4];
    for i in 0..4 {
        conditions[i] = check_threshold(pixel.0[i], target.0[i], threshold);
    }
    conditions.iter().all(|v| *v == true)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rgba8_threshold() {
        assert_eq!(
            rgba8_threshold(
                &image::Rgba::from([120, 120, 120, 255]),
                image::Rgba::from([255, 255, 255, 255]),
                255 / 2
            ),
            false
        );

        assert_eq!(
            rgba8_threshold(
                &image::Rgba::from([128, 128, 128, 255]),
                image::Rgba::from([255, 255, 255, 255]),
                255 / 2
            ),
            true
        );

        assert_eq!(
            rgba8_threshold(
                &image::Rgba::from([255, 2, 180, 255]),
                image::Rgba::from([255, 0, 180, 255]),
                255 / 2
            ),
            true
        )
    }
}
