// ============================================================
//  Smart Planning Pro — src-tauri/src/export_service.rs
//  Export Excel multi-années (plage de dates)
// ============================================================

use crate::planning_repository;
use rust_xlsxwriter::*;
use std::path::PathBuf;

const MONTH_NAMES: [&str; 12] = [
    "Janvier", "Février", "Mars", "Avril", "Mai", "Juin",
    "Juillet", "Août", "Septembre", "Octobre", "Novembre", "Décembre",
];

/// Génère la liste des (year, month) entre start et end inclus
fn month_range(
    start_year: i32, start_month: u32,
    end_year:   i32, end_month:   u32,
) -> Vec<(i32, u32)> {
    let mut result = Vec::new();
    let mut y = start_year;
    let mut m = start_month;
    loop {
        result.push((y, m));
        if y == end_year && m == end_month { break; }
        m += 1;
        if m > 12 { m = 1; y += 1; }
    }
    result
}

pub fn export_months_to_excel(
    start_year:  i32,
    start_month: u32,
    end_year:    i32,
    end_month:   u32,
    output_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {

    let months = month_range(start_year, start_month, end_year, end_month);

    let mut workbook = Workbook::new();
    let worksheet   = workbook.add_worksheet();

    // ── Formats ──────────────────────────────────────────────
    let fmt_header = Format::new()
        .set_bold()
        .set_font_color(Color::White)
        .set_background_color(Color::RGB(0x1F4E79))
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin);

    let fmt_date = Format::new()
        .set_bold()
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin);

    let fmt_empty = Format::new()
        .set_font_color(Color::RGB(0xAAAAAA))
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin);

    // ── Titre global ─────────────────────────────────────────
    let title = if start_year == end_year {
        format!("Planning {} — {} à {}",
            start_year,
            MONTH_NAMES[(start_month - 1) as usize],
            MONTH_NAMES[(end_month   - 1) as usize],
        )
    } else {
        format!("Planning {} {} → {} {}",
            MONTH_NAMES[(start_month - 1) as usize], start_year,
            MONTH_NAMES[(end_month   - 1) as usize], end_year,
        )
    };

    let fmt_title = Format::new()
        .set_bold()
        .set_font_size(14.0)
        .set_font_color(Color::RGB(0x1F4E79));

    worksheet.write_with_format(0, 0, &title, &fmt_title)?;

    // ── Une colonne par mois ──────────────────────────────────
    // Col layout : col_base + 0 = date, col_base + 1 = label
    // Chaque mois prend 2 colonnes + 1 de séparation

    let col_stride: u16 = 3; // 2 colonnes données + 1 vide

    for (month_idx, &(year, month)) in months.iter().enumerate() {
        let col_base = (month_idx as u16) * col_stride;

        // Données du mois
        let entries = planning_repository::get_by_month(year, month)
            .unwrap_or_default();

        // Index par date
        let mut day_map: std::collections::HashMap<String, Vec<(String, String)>> =
            std::collections::HashMap::new();
        for entry in &entries {
            let label = entry.label_name.clone().unwrap_or_default();
            let color = entry.label_color.clone()
                .unwrap_or_else(|| "#CCCCCC".to_string());
            day_map.entry(entry.date.clone()).or_default()
                   .push((label, color));
        }

        // En-tête du mois (ligne 1)
        let month_header = if start_year != end_year {
            format!("{} {}", MONTH_NAMES[(month - 1) as usize], year)
        } else {
            MONTH_NAMES[(month - 1) as usize].to_string()
        };
        worksheet.write_with_format(1, col_base, &month_header, &fmt_header)?;
        worksheet.write_with_format(1, col_base + 1, "Label", &fmt_header)?;

        // Largeur des colonnes
        worksheet.set_column_width(col_base,     12.0)?;
        worksheet.set_column_width(col_base + 1, 18.0)?;

        // Jours du mois
        let total_days = days_in_month(year, month);
        for day in 1..=total_days {
            let row = (day + 1) as u32; // ligne 2 = jour 1
            let date_str = format!("{:04}-{:02}-{:02}", year, month, day);
            let date_label = format!("{:02}/{:02}", day, month);

            if let Some(day_entries) = day_map.get(&date_str) {
                // Couleur de fond
                let color_hex = &day_entries[0].1;
                let bg = hex_to_xlsx_color(color_hex);

                let fmt_cell_date = Format::new()
                    .set_bold()
                    .set_align(FormatAlign::Center)
                    .set_background_color(bg)
                    .set_border(FormatBorder::Thin);

                let fmt_cell_label = Format::new()
                    .set_bold()
                    .set_background_color(bg)
                    .set_border(FormatBorder::Thin);

                let names: Vec<String> = day_entries.iter()
                    .map(|(n, _)| n.to_uppercase())
                    .collect();

                worksheet.write_with_format(row, col_base,     &date_label,      &fmt_cell_date)?;
                worksheet.write_with_format(row, col_base + 1, &names.join(" / "), &fmt_cell_label)?;
            } else {
                worksheet.write_with_format(row, col_base,     &date_label, &fmt_date)?;
                worksheet.write_with_format(row, col_base + 1, "",          &fmt_empty)?;
            }

            // Hauteur de ligne
            worksheet.set_row_height(row, 16.0)?;
        }

        // Colonne séparatrice (vide)
        worksheet.set_column_width(col_base + 2, 1.0)?;
    }

    workbook.save(output_path)?;
    println!("Export Excel : {:?}", output_path);
    Ok(())
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 29 } else { 28 },
        _ => 30,
    }
}

fn hex_to_xlsx_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 { return Color::RGB(0xCCCCCC); }
    let val = u32::from_str_radix(hex, 16).unwrap_or(0xCCCCCC);
    Color::RGB(val)
}