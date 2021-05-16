use std::ops::RangeInclusive;

/// Maps an `RangeInclusive<T>` to `RangeInclusive<U>` by applying a
/// function to the bounds.
pub fn map_range_inclusive<T, U, F>(range: RangeInclusive<T>, f: F) -> RangeInclusive<U>
where
    F: Fn(T) -> U,
{
    let (start, end) = range.into_inner();

    RangeInclusive::new(f(start), f(end))
}
