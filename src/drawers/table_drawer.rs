use prettytable::{format, Table};

pub fn draw(mut table: Table) {
    table.set_format(
        format::FormatBuilder::new()
            .column_separator('║')
            .borders('║')
            .separators(
                &[format::LinePosition::Top],
                format::LineSeparator::new('═', '╦', '╔', '╗'),
            )
            .separators(
                &[format::LinePosition::Bottom],
                format::LineSeparator::new('═', '╩', '╚', '╝'),
            )
            .separators(
                &[format::LinePosition::Title, format::LinePosition::Intern],
                format::LineSeparator::new('─', '╫', '╟', '╢'),
            )
            .build(),
    );

    table.printstd();
}
