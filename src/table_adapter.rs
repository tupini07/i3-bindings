use crate::config_reader::I3Binding;
use crate::main;
use prettytable::Table;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn build_table_from_bindings(mut bindings_map: HashMap<String, Vec<I3Binding>>) -> Table {
    // draw table
    let mut main_table = Table::new();
    main_table.set_titles(row!["Category", "Actual Binding"]);

    let mut sorted_vec: Vec<_> = bindings_map.iter_mut().collect();
    sorted_vec.sort_by(|a, b| a.0.cmp(b.0));

    for (category, bindings_for_category) in sorted_vec.iter_mut() {
        if bindings_for_category.len() == 0 {
            continue;
        }

        let mut category_table = Table::new();

        bindings_for_category.sort_by(|e, i| {
            if e.binding > i.binding {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });

        for element in bindings_for_category.iter() {
            category_table.add_row(row![element.binding_type, element.binding, element.command]);
        }

        category_table.set_format(*prettytable::format::consts::FORMAT_CLEAN);

        main_table.add_row(row![category, category_table]);
    }

    main_table
}
