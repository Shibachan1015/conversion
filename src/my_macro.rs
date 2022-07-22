#[macro_export]
macro_rules! round {
    ($x:expr,$scale:expr) => (($x*$scale).round()/$scale)
}

