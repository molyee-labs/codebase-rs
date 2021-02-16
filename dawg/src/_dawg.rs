#![cfg(feature = "none")]

pub struct Dawg<K, V> {
    bases: Pool<>,
    labels: Pool<L>,
    flags: Pool<F>,
    num_states: usize,
    num_transitions: usize,
    num_merged_states: usize,
    num_merged_transitions: usize,
    num_merging_states: usize,
}

impl Dawg {
    const fn len(&self) -> usize {
        self.keys.len()
    }

    const fn num_states(&self) -> usize {
        self.num_states
    }

    const fn num_transitions(&self) -> usize {
        self.num_states
    }

    const fn num_merged_states(&self) -> usize {
        self.num_states
    }

    const fn num_merged_transitions(&self) -> usize {
        self.num_states
    }

    const fn num_merging_states(&self) -> usize {
        self.num_states
    }

    const fn child(&self, idx: &K) -> 
}