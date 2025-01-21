// Using the SolutionFragment struct and a vector that stores the order in which the elements were
// considered/visited, we can reconstruct the solution by traversing the tree from the last element

pub struct SolutionTree {
    fragments: Vec<SolutionFragment>,
}

#[derive(Clone, Copy)]
pub struct SolutionFragment {
    /// The value of the fragment which represent a series of decisions bitwise encoded
    pub value: u64,
    /// The index of the previous fragment in the tree
    previous_idx: Option<usize>,
}

impl Default for SolutionFragment {
    fn default() -> Self {
        SolutionFragment {
            value: 0,
            previous_idx: None,
        }
    }
}

impl SolutionFragment {
    pub fn new(previous_idx: Option<usize>) -> Self {
        SolutionFragment {
            value: 0,
            previous_idx,
        }
    }

    pub fn add_decision(&mut self, decision: bool) {
        self.value <<= 1;
        self.value |= decision as u64;
    }

    pub fn get_decision(&self, idx: usize) -> bool {
        (self.value >> idx) & 1 == 1
    }

    pub fn update_previous_idx(&mut self, previous_idx: Option<usize>) {
        self.previous_idx = previous_idx;
    }

    pub fn clear_value(&mut self) {
        self.value = 0;
    }

    pub fn get_previous_idx(&self) -> Option<usize> {
        self.previous_idx
    }
}

impl SolutionTree {
    pub fn new() -> Self {
        SolutionTree {
            fragments: Vec::new(),
        }
    }

    /// Pushes a new fragment to the tree and clears its content
    /// The function also updates the "previous_idx" field of the fragment so that it points to the
    /// pushed fragment
    // pub fn push_and_clear(&mut self, fragment: &mut SolutionFragment) {
    //     fragment.previous_idx = Some(self.fragments.len());
    //     self.fragments.push(*fragment);
    //     fragment.value = 0;
    // }

    /// Pushes a new fragment to the tree and returns its index
    pub fn push_fragment(&mut self, fragment: SolutionFragment) -> usize {
        let idx = self.fragments.len();
        self.fragments.push(fragment);
        idx
    }

    pub fn get_fragment(&self, idx: usize) -> Option<&SolutionFragment> {
        self.fragments.get(idx)
    }
}
