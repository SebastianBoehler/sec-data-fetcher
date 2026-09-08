use scraper::{ElementRef, Html, Selector};
use std::sync::LazyLock;

pub type Tables = Vec<Vec<Vec<String>>>;
static TABLE: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("table").expect("constant selector"));
static ROW: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("tr").expect("constant selector"));
static CELL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("th, td").expect("constant selector"));

/// Extract cell text in document order using HTML5 parsing rules.
/// Nested tables are separate results; parent cell text includes descendant text.
/// This does not reconstruct colspan/rowspan or interpret financial statements.
pub fn extract_tables(content: &str) -> Tables {
    let document = Html::parse_document(content);
    document
        .select(&TABLE)
        .map(|table| {
            table
                .select(&ROW)
                .filter(|row| belongs_to(*row, table, "table"))
                .map(|row| {
                    row.select(&CELL)
                        .filter(|cell| belongs_to(*cell, row, "tr"))
                        .map(|cell| cell.text().collect::<String>().trim().to_owned())
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn belongs_to(element: ElementRef<'_>, parent: ElementRef<'_>, name: &str) -> bool {
    element
        .ancestors()
        .filter_map(ElementRef::wrap)
        .find(|ancestor| ancestor.value().name() == name)
        .is_some_and(|ancestor| ancestor.id() == parent.id())
}
