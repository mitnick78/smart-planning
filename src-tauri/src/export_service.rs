use crate::planning_repository;
use rust_xlsxwriter::{
    Color, Format, FormatAlign, FormatBorder,
    Workbook, XlsxError,
};
use std::collections::HashMap;
use std::path::PathBuf;

const MONTH_NAMES: [&str; 12] = [
    "Janvier", "Février", "Mars", "Avril", "Mai", "Juin",
    "Juillet", "Août", "Septembre", "Octobre", "Novembre", "Décembre",
];

const DAY_ABBR: [&str; 7] = ["lun.", "mar.", "mer.", "jeu.", "ven.", "sam.", "dim."];

fn hex_to_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    let rgb = u32::from_str_radix(hex, 16).unwrap_or(0xFFFFFF);
    Color::RGB(rgb)
}

fn day_of_week(year: i32, month: u32, day: u32) -> u32 {
    let d = day as i32;
    let m = month as i32;
    let y = if m < 3 { year - 1 } else { year };
    let m = if m < 3 { m + 12 } else { m };
    let k = y % 100;
    let j = y / 100;
    let h = (d + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
    ((h + 5) % 7) as u32
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 29 } else { 28 },
        _ => 30,
    }
}

pub fn export_months_to_excel(
    year: i32,
    months: &[u32],
    output_path: &PathBuf,
) -> Result<(), XlsxError> {

    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();
    sheet.set_name(&format!("Planning {}", year))?;

    // Formats 
    let title_format = Format::new()
        .set_bold()
        .set_font_size(11.0)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_background_color(Color::RGB(0x1F4E79))
        .set_font_color(Color::White)
        .set_border(FormatBorder::Thin);

    let empty_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Left)
        .set_align(FormatAlign::VerticalCenter)
        .set_font_size(9.0)
        .set_font_color(Color::RGB(0x888888))
        .set_background_color(Color::White);

    // width columns
    // every month column has the same width, enough for "mer. 31/12  NAME LABEL"
    for (i, _) in months.iter().enumerate() {
        sheet.set_column_width(i as u16, 22.0)?;
    }

    // line 0 : title with month names
    for (col, &month) in months.iter().enumerate() {
        sheet.write_with_format(
            0, col as u16,
            &format!("{} {}", MONTH_NAMES[(month - 1) as usize].to_uppercase(), year),
            &title_format,
        )?;
    }
    sheet.set_row_height(0, 22.0)?;

    // line days: 1 to 31 (max) with entries
    // every month has its own column, so we can have up to 31 lines (for the longest month) and 12 columns (for all months)
    let max_days = 31u32;

    for day in 1..=max_days {
        let row = day as u32; // line 1 à 31
        sheet.set_row_height(row, 18.0)?;

        for (col, &month) in months.iter().enumerate() {
            let total = days_in_month(year, month);

            if day > total {
                // this day doesn't exist in this month, gray cell
                let gray_format = Format::new()
                    .set_border(FormatBorder::Thin)
                    .set_background_color(Color::RGB(0xEEEEEE));
                sheet.write_with_format(row, col as u16, "", &gray_format)?;
                continue;
            }

            let date_str = format!("{:04}-{:02}-{:02}", year, month, day);
            let dow = day_of_week(year, month, day);
            let day_abbr = DAY_ABBR[dow as usize];
            let date_label = format!("{} {:02}/{:02}", day_abbr, day, month);

            // look for entries of this day in the month
            //we charged all entries of the month at once to avoid multiple database calls, then we group them by day in a HashMap
            let entries = planning_repository::get_by_month(year, month)
                .unwrap_or_default();

            let mut day_map: HashMap<String, Vec<(String, String)>> = HashMap::new();
            for entry in &entries {
                let label = entry.label_name.clone().unwrap_or_default();
                let color = entry.label_color.clone()
                    .unwrap_or_else(|| "#FFFFFF".to_string());
                day_map.entry(entry.date.clone()).or_default()
                       .push((label, color));
            }

            if let Some(day_entries) = day_map.get(&date_str) {
                // cell with entries, we take the color of the first entry as background color, and we list all labels in the cell content
                let (_, label_color) = &day_entries[0];

                let cell_format = Format::new()
                    .set_border(FormatBorder::Thin)
                    .set_background_color(hex_to_color(label_color))
                    .set_font_color(Color::Black)
                    .set_bold()
                    .set_align(FormatAlign::Left)
                    .set_align(FormatAlign::VerticalCenter)
                    .set_font_size(9.0);

                // "mar. 01/04  NOM LABEL"
                let names: Vec<String> = day_entries.iter()
                    .map(|(name, _)| name.to_uppercase())
                    .collect();
                let content = format!("{}  {}", date_label, names.join(" / "));

                sheet.write_with_format(row, col as u16, &content, &cell_format)?;
            } else {
                // Empty cell — just the date
                sheet.write_with_format(row, col as u16, &date_label, &empty_format)?;
            }
        }
    }

    workbook.save(output_path)?;
    Ok(())
}