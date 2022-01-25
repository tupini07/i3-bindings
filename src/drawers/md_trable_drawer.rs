#[allow(unused_imports)]
use crate::config_reader::I3Binding;
use std::collections::HashMap;

use crossterm::style::Color::*;
use termimad::*;

pub fn draw(mut bindings: HashMap<String, Vec<I3Binding>>) {
    let mut sorted_vec: Vec<_> = bindings.iter_mut().collect();
    sorted_vec.sort_by(|a, b| a.0.cmp(b.0));

    let mut binding_table: String = format!(
        "
|:-:|:-:|-
|**Category**|**Actual Binding**|**Binding mode**|
"
    );

    for (category, bindings_for_category) in sorted_vec.iter() {
        if bindings_for_category.is_empty() {
            continue;
        }
        binding_table.push_str(&String::from("|:--|:--|:-:
"));
        binding_table.push_str(&format!("|*{}*||
", category));
        for b in bindings_for_category.iter() {
            let line = format!("||{} {} {}|{}\n", b.binding_type, b.binding, b.command, b.runtype);
            binding_table.push_str(&line);
        }
    }
    binding_table.push_str(&String::from("|-"));

    println!("\n");
    let mut skin = MadSkin::default();
    skin.set_headers_fg(Red);
    skin.bold.set_fg(Yellow);
    skin.italic.set_fgbg(Magenta, rgb(30, 30, 40));
    skin.paragraph.align = Alignment::Center;
    skin.table.align = Alignment::Center;
    let (width, _) = terminal_size();
    let mut markdown = format!("Terminal width: *{}*", width);
    markdown.push_str(&binding_table);
    println!("{}", skin.term_text(&markdown));
    println!("\n");
}
