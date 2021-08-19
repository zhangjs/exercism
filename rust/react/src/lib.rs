use std::collections::HashMap;

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellID(usize);

/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellID(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackID(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct ComputeCell<T: Copy + PartialEq> {
    compute_func: Box<dyn Fn(&[T]) -> T>,
    dependencies: Vec<CellID>,
    callbacks: Vec<CallbackID>,
}

pub struct Reactor<'a, T: Copy + PartialEq> {
    idx: usize,
    input_cells: HashMap<InputCellID, T>,
    compute_cells: HashMap<ComputeCellID, ComputeCell<T>>,
    callbacks: HashMap<CallbackID, Box<dyn 'a + FnMut(T)>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            idx: 0,
            input_cells: HashMap::new(),
            compute_cells: HashMap::new(),
            callbacks: HashMap::new(),
        }
    }

    fn next_id(&mut self) -> usize {
        let id = self.idx;
        self.idx += 1;
        id
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        let id = InputCellID(self.next_id());
        self.input_cells.insert(id, initial);

        id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: 'static + Fn(&[T]) -> T>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        for id in dependencies {
            match id {
                CellID::Input(input) => {
                    if self.input_cells.get(&input).is_none() {
                        return Err(*id);
                    }
                }
                CellID::Compute(compute) => {
                    if self.compute_cells.get(&compute).is_none() {
                        return Err(*id);
                    }
                }
            }
        }

        let compute_cell = ComputeCell {
            compute_func: Box::new(compute_func),
            dependencies: dependencies.iter().cloned().collect(),
            callbacks: vec![],
        };
        let id = ComputeCellID(self.next_id());
        self.compute_cells.insert(id, compute_cell);

        Ok(id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            CellID::Input(input) => self.input_cells.get(&input).cloned(),
            CellID::Compute(compute) => {
                if let Some(cc) = self.compute_cells.get(&compute) {
                    let args: Vec<T> = cc
                        .dependencies
                        .iter()
                        .map(|&cell_id| self.value(cell_id).unwrap())
                        .collect();

                    Some((cc.compute_func)(args.as_slice()))
                } else {
                    None
                }
            }
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        if !self.input_cells.contains_key(&id) {
            return false;
        }

        let old_states: HashMap<_, _> = self
            .compute_cells
            .iter()
            .map(|(&id, cc)| (id, (cc, self.value(CellID::Compute(id)))))
            .collect();

        self.input_cells.insert(id, new_value);

        for (id, (cc, old)) in old_states.into_iter() {
            let new = self.value(CellID::Compute(id));
            if new != old {
                for cid in cc.callbacks.iter() {
                    let callback = self.callbacks.get_mut(cid).unwrap();
                    callback(new.unwrap());
                }
            }
        }

        true
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: 'a + FnMut(T)>(
        &mut self,
        id: ComputeCellID,
        callback: F,
    ) -> Option<CallbackID> {
        if self.compute_cells.contains_key(&id) {
            let cid = CallbackID(self.next_id());
            let cc = self.compute_cells.get_mut(&id).unwrap();
            cc.callbacks.push(cid);
            self.callbacks.insert(cid, Box::new(callback));

            Some(cid)
        } else {
            None
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        if let Some(cc) = self.compute_cells.get_mut(&cell) {
            if self.callbacks.remove(&callback).is_none() {
                return Err(RemoveCallbackError::NonexistentCallback);
            }

            let pos = cc.callbacks.iter().position(|&id| id == callback).unwrap();
            cc.callbacks.remove(pos);

            Ok(())
        } else {
            Err(RemoveCallbackError::NonexistentCell)
        }
    }
}
