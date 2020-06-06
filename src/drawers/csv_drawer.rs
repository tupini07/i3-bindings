use crate::config_reader::I3Binding;
use std::collections::HashMap;
use std::io;

pub fn draw(bindings: HashMap<String, Vec<I3Binding>>) {
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b',')
        .quote_style(csv::QuoteStyle::NonNumeric)
        .from_writer(io::stdout());

    for (_k, coll) in bindings {
        for b in coll {
            wtr.serialize(b).unwrap();
        }
    }

    wtr.flush().unwrap();
}
