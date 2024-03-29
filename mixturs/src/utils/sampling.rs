use nalgebra::{RealField};
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, WeightedIndex};
use rand::Rng;

/// Random sampling k items without replacement with reservoir sampling algorithm.
///
/// # Arguments:
///
/// * `rng`: a random number generator
/// * `src`: The iterator to sample from.
/// * `dst`: The destination slice.
///
/// # Returns:
///
/// The number of items that were copied into the destination array.
///
/// # Example:
/// ```
/// use mixturs::utils::reservoir_sampling;
///
/// let mut rng = rand::thread_rng();
/// let src = vec![1, 2, 3, 4, 5];
/// let mut dst = vec![0; 3];
/// let n = reservoir_sampling(&mut rng, src.into_iter(), &mut dst);
/// assert_eq!(n, 3);
/// ```
pub fn reservoir_sampling<T: Copy, I: Iterator<Item=T>>(
    rng: &mut impl Rng,
    mut src: I,
    dst: &mut [I::Item]
) -> usize {
    let mut n = 0;
    for (dst_val,  src_val) in dst.iter_mut().zip(src.by_ref()) {
        *dst_val = src_val;
        n += 1;
    }

    let mut i = n;
    for v in src {
        let j = rng.gen_range(0..i);
        if j < dst.len() {
            dst[j] = v;
        }
        i += 1;
    }
    n
}

/// Random sampling k items without replacement with weighted reservoir sampling algorithm.
///
/// # Arguments:
///
/// * `rng`: a random number generator
/// * `src`: The source of the data.
/// * `dst`: The destination array.
///
/// # Returns:
///
/// The number of elements sampled.
///
/// # Example:
/// ```
/// use mixturs::utils::reservoir_sampling_weighted;
///
/// let mut rng = rand::thread_rng();
/// let weights = vec![0.1, 0.2, 0.3, 0.4, 0.5];
/// let mut dst = vec![0; 3];
///
/// let n = reservoir_sampling_weighted(&mut rng, weights.into_iter(), &mut dst);
/// assert_eq!(n, 3);
/// ```
pub fn reservoir_sampling_weighted<
    W: RealField + Copy + SampleUniform, I: Iterator<Item=W>
>(
    rng: &mut impl Rng,
    mut src: I,
    dst: &mut [usize]
) -> usize {
    let mut n = 0;
    let mut w_sum = W::zero();
    let mut i = 0;
    for dst_v in dst.iter_mut() {
        if let Some(w) = src.next() {
            *dst_v = i;
            w_sum += w;
            n += 1;
        } else {
            break;
        }
        i += 1;
    }

    for w in src {
        w_sum += w;
        let j = rng.gen_range(W::zero()..w_sum);
        if j < w {
            dst[rng.gen_range(0..dst.len())] = i;
        }
        i += 1;
    }
    n
}

/// Random sampling k items with replacement with weighted sampling algorithm.
///
/// # Arguments:
///
/// * `rng`: A random number generator.
/// * `src`: An iterator over the weights.
/// * `dst`: The destination array.
///
/// # Example:
/// ```
/// use mixturs::utils::replacement_sampling_weighted;
///
/// let mut rng = rand::thread_rng();
/// let weights = vec![0.1, 0.2];
/// let mut dst = vec![0; 3];
/// replacement_sampling_weighted(&mut rng, weights.into_iter(), &mut dst);
/// ```
pub fn replacement_sampling_weighted<
    W: RealField + SampleUniform + Default + for<'a> core::ops::AddAssign<&'a W>,
    I: Iterator<Item=W>
>(
    rng: &mut impl Rng,
    src: I,
    dst: &mut [usize]
) {
    let dist = WeightedIndex::new(src).unwrap();
    for (dst_v, v) in dst.iter_mut().zip(dist.sample_iter(rng)) {
        *dst_v = v;
    }
}


#[cfg(test)]
mod tests {
    use crate::utils::reservoir_sampling_weighted;

    #[test]
    fn test_reservoir_sampling_weighted() {
        let mut rng = rand::thread_rng();
        let mut dst = [0; 4];
        let src = vec![1.0, 2.0, 3.0, 4.0].into_iter();
        assert_eq!(reservoir_sampling_weighted(&mut rng, src, &mut dst), 4);
        dst.sort();
        assert_eq!(dst, [0, 1, 2, 3]);

        let src = vec![1.0, 2.0, 3.0].into_iter();
        assert_eq!(reservoir_sampling_weighted(&mut rng, src, &mut dst), 3);
        dst.sort();
        assert_eq!(dst, [0, 1, 2, 3]);
    }
}