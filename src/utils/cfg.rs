use tap::prelude::*;
use toml_edit::{Datetime, DocumentMut, Formatted, Value, visit_mut::VisitMut};


pub struct DocUpdater {
    to_be_merged: DocumentMut,
    path: Vec<String>,
}

impl DocUpdater {
    pub fn new(to_be_merged: DocumentMut) -> Self {
        let path = vec![];
        Self { to_be_merged, path }
    }

    pub fn update(&mut self, doc: &mut DocumentMut) {
        self.visit_item_mut(doc.as_item_mut());
    }

    fn current_item(&self) -> &toml_edit::Item {
        let mut item = self.to_be_merged.as_item();
        for index in &self.path {
            item = &item[index];
        }
        item
    }
}

impl toml_edit::visit_mut::VisitMut for DocUpdater {
    fn visit_boolean_mut(&mut self, node: &mut Formatted<bool>) {
        let Some(Value::Boolean(formatted)) = self.current_item().as_value().cloned() else {
            unreachable!()
        };
        // preserve comments
        *node = formatted.tap_mut(|f| f.decor_mut().clone_from(node.decor()));
    }

    fn visit_datetime_mut(&mut self, node: &mut Formatted<Datetime>) {
        let Some(Value::Datetime(formatted)) = self.current_item().as_value().cloned() else {
            unreachable!()
        };
        *node = formatted.tap_mut(|f| f.decor_mut().clone_from(node.decor()));
    }

    fn visit_float_mut(&mut self, node: &mut Formatted<f64>) {
        let Some(Value::Float(formatted)) = self.current_item().as_value().cloned() else {
            unreachable!()
        };
        *node = formatted.tap_mut(|f| f.decor_mut().clone_from(node.decor()));
    }

    fn visit_integer_mut(&mut self, node: &mut Formatted<i64>) {
        let Some(Value::Integer(formatted)) = self.current_item().as_value().cloned() else {
            unreachable!()
        };
        *node = formatted.tap_mut(|f| f.decor_mut().clone_from(node.decor()));
    }

    fn visit_string_mut(&mut self, node: &mut Formatted<String>) {
        let Some(Value::String(formatted)) = self.current_item().as_value().cloned() else {
            unreachable!()
        };
        *node = formatted.tap_mut(|f| f.decor_mut().clone_from(node.decor()));
    }

    fn visit_table_like_kv_mut(&mut self, key: toml_edit::KeyMut<'_>, node: &mut toml_edit::Item) {
        self.path.push(key.to_string());
        toml_edit::visit_mut::visit_table_like_kv_mut(self, key, node);
        self.path.pop();
    }

    fn visit_array_mut(&mut self, node: &mut toml_edit::Array) {
        let decor = node.iter().next().map(|v| v.decor().clone());

        let Some(array) = self.current_item().as_array().cloned() else {
            unreachable!()
        };
        node.clear();
        node.extend(array.into_iter().map(|mut value| {
            decor.as_ref().map(|d| value.decor_mut().clone_from(d));
            value
        }));
    }

    fn visit_array_of_tables_mut(&mut self, node: &mut toml_edit::ArrayOfTables) {
        let decor = node.iter().next().map(|t| t.decor().clone());
        node.clear();
        if let Some(array) = self.current_item().as_array() {
            node.extend(array.iter().map(|v| {
                v.as_inline_table()
                    .unwrap()
                    .clone()
                    .into_table()
                    .tap_mut(|t| {
                        decor.as_ref().map(|d| t.decor_mut().clone_from(d));
                    })
            }));
        } else {
            unreachable!();
        }
    }
}
