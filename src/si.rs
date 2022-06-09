use crate::unit::Unit;

#[derive(Debug)]
pub struct Si<V = f64, const U: Unit = { Unit::default() }>(pub V);

impl<V, const U: Unit> Si<V, U> {
    // Allow reference for copy values
    pub fn num(self) -> V {
        self.0
    }
}
