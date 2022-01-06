use crate::config_reader::I3Binding;
use prettytable::Table;
use std::collections::HashMap;

pub fn build_table_from_bindings(mut bindings_map: HashMap<String, Vec<I3Binding>>) -> Table {
    // draw table
    let mut main_table = Table::new();
    main_table.set_titles(row!["Category", "Actual Binding", "Binding mode"]);

    let mut sorted_vec: Vec<_> = bindings_map.iter_mut().collect();
    sorted_vec.sort_by(|a, b| a.0.cmp(b.0));

    for (category, bindings_for_category) in sorted_vec.iter() {
        if bindings_for_category.is_empty() {
            continue;
        }

        let mut category_table = bindings_for_category
            .iter()
            .fold(Table::new(), |mut acc, e| {
                acc.add_row(row![e.binding_type, e.binding, e.command]);
                acc
            });
        let mut type_table = bindings_for_category
            .iter()
            .fold(Table::new(), |mut acc, e| {
                acc.add_row(row![e.runtype]);
                acc
            });

        category_table.set_format(*prettytable::format::consts::FORMAT_CLEAN);
        type_table.set_format(*prettytable::format::consts::FORMAT_CLEAN);

        main_table.add_row(row![category, category_table, type_table]);
    }

    main_table
}
