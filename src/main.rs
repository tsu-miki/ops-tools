use read_excel::read_excel;

fn main() {
    let input_excel_path = format!("{}/read-excel/input/company-list.xlsx", env!("CARGO_MANIFEST_DIR"));
    let global_ids = read_excel::execute(input_excel_path).unwrap();
    println!("{:?}", global_ids);
}
