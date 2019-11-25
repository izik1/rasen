use crate::Label;

use std::collections::HashMap;
#[derive(Default)]
pub struct Labeler {
    label_values: HashMap<Label, u64>,
    label_counter: usize,
}

impl Labeler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_label(&mut self) -> Label {
        let label = Label(self.label_counter);
        self.label_counter += 1;
        label
    }

    pub fn create_attached_label(&mut self, address: u64) -> Label {
        let label = self.create_label();
        self.attach_label(label, address);
        label
    }

    pub fn attach_label(&mut self, label: Label, address: u64) {
        if self.label_values.contains_key(&label) {
            todo!("handle label with already existing label")
        }

        self.label_values.insert(label, address);
    }

    pub fn resolve_label(&self, label: Label) -> Option<u64> {
        self.label_values.get(&label).map(|it| *it)
    }
}
