use calamine::{DataType, Error, Reader, Xlsx, open_workbook};

pub fn get_global_ids_by_revenue(path: String) -> Result<Vec<String>, Error> {
    let mut excel: Xlsx<_> = open_workbook(path)?;
    let worksheet = excel.worksheet_range("企業リスト")?;
    let rows = worksheet.rows();
    let data_start_row = 10;
    let global_id_col = 2;
    let total_revenue_col = 4;

    let global_ids_revenue_over_1000b: Vec<String> = rows
        .skip(data_start_row)
        .filter_map(|row| {
            let revenue = row.iter().nth(total_revenue_col)?.get_float()?;
            (revenue >= 100_000.0)
                .then(|| row
                    .iter()
                    .nth(global_id_col)
                    .map(|d| d.to_string()))
                .flatten()
        })
        .collect();

    Ok(global_ids_revenue_over_1000b)
}