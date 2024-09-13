import * as cheerio from 'cheerio';

/**
 * Extracts all tables from the given SEC filing content.
 * @param filingContent - The HTML content of the SEC filing.
 * @returns An array of tables, where each table is represented as an array of rows and each row is an array of cells.
 */
export function extractTables(
  filingContent: string,
): Array<Array<Array<string>>> {
  const $ = cheerio.load(filingContent);
  const tables: Array<Array<Array<string>>> = [];

  $('table').each((i, table) => {
    const tableData: Array<Array<string>> = [];

    $(table)
      .find('tr')
      .each((i, row) => {
        const rowData: Array<string> = [];
        $(row)
          .find('td, th')
          .each((i, cell) => {
            rowData.push($(cell).text().trim());
          });
        tableData.push(rowData);
      });

    tables.push(tableData);
  });

  return tables;
}
