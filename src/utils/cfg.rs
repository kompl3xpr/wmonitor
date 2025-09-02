use toml_edit::{visit_mut::VisitMut, Datetime, DocumentMut, Formatted, Value};
use tap::prelude::*;

enum Index {
    Str(String),
    Int(usize),
}

pub struct DocUpdater {
    to_be_merged: DocumentMut,
    path: Vec<Index>,
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
            match index {
                Index::Int(i) => item = &item[i],
                Index::Str(s) => item = &item[s],
            }
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
        *node = formatted.tap_mut(|f| *f.decor_mut() = node.decor().clone());
    }

    fn visit_datetime_mut(&mut self, node: &mut Formatted<Datetime>) {
        let Some(Value::Datetime(mut formatted)) = self.current_item().as_value().cloned() else {
            unreachable!()
        };
        *formatted.decor_mut() = node.decor().clone();
        *node = formatted;
    }

    fn visit_float_mut(&mut self, node: &mut Formatted<f64>) {
        let Some(Value::Float(mut formatted)) = self.current_item().as_value().cloned() else {
            unreachable!()
        };
        *formatted.decor_mut() = node.decor().clone();
        *node = formatted;
    }

    fn visit_integer_mut(&mut self, node: &mut Formatted<i64>) {
        let Some(Value::Integer(mut formatted)) = self.current_item().as_value().cloned() else {
            unreachable!()
        };
        *formatted.decor_mut() = node.decor().clone();
        *node = formatted;
    }

    fn visit_string_mut(&mut self, node: &mut Formatted<String>) {
        let Some(Value::String(mut formatted)) = self.current_item().as_value().cloned() else {
            unreachable!()
        };
        *formatted.decor_mut() = node.decor().clone();
        *node = formatted;
    }

    fn visit_table_like_kv_mut(&mut self, key: toml_edit::KeyMut<'_>, node: &mut toml_edit::Item) {
        let key_str = key.to_string();
        self.path.push(Index::Str(key_str));
        toml_edit::visit_mut::visit_table_like_kv_mut(self, key, node);
        self.path.pop();
    }

    fn visit_array_mut(&mut self, node: &mut toml_edit::Array) {
        for (i, value) in node.iter_mut().enumerate() {
            self.path.push(Index::Int(i));
            self.visit_value_mut(value);
            self.path.pop();
        }
    }

    fn visit_array_of_tables_mut(&mut self, node: &mut toml_edit::ArrayOfTables) {
        for (i, table) in node.iter_mut().enumerate() {
            self.path.push(Index::Int(i));
            self.visit_table_mut(table);
            self.path.pop();
        }
    }
}