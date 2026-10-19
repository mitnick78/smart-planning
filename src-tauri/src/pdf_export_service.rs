// ============================================================
//  Smart Planning Pro — src-tauri/src/pdf_export_service.rs
//  Export PDF multi-années (plage de dates)
//  API : printpdf 0.7.0
// ============================================================

use crate::planning_repository;
use printpdf::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

const MONTH_NAMES: [&str; 12] = [
    "Janvier", "Fevrier", "Mars", "Avril", "Mai", "Juin",
    "Juillet", "Aout", "Septembre", "Octobre", "Novembre", "Decembre",
];

const DAY_ABBR: [&str; 7] = ["lun.", "mar.", "mer.", "jeu.", "ven.", "sam.", "dim."];

// A4 Paysage
const PAGE_WIDTH_MM:  f32 = 297.0;
const PAGE_HEIGHT_MM: f32 = 210.0;
const MARGIN_MM:      f32 = 10.0;

// ── Helpers ───────────────────────────────────────────────────

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

fn hex_to_rgb(hex: &str) -> (f32, f32, f32) {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 { return (0.5, 0.5, 0.5); }
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(128) as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(128) as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(128) as f32 / 255.0;
    (r, g, b)
}

fn draw_rect(
    layer: &PdfLayerReference,
    x: f32, y: f32, width: f32, height: f32,
    r: f32, g: f32, b: f32,
) {
    layer.set_fill_color(Color::Rgb(Rgb::new(r, g, b, None)));
    layer.add_rect(Rect::new(Mm(x), Mm(y), Mm(x + width), Mm(y + height)));
}

fn draw_separator(layer: &PdfLayerReference, x: f32, y1: f32, y2: f32) {
    layer.set_outline_color(Color::Rgb(Rgb::new(0.8, 0.8, 0.8, None)));
    layer.set_outline_thickness(0.3);
    layer.add_line(Line {
        points: vec![
            (Point::new(Mm(x), Mm(y1)), false),
            (Point::new(Mm(x), Mm(y2)), false),
        ],
        is_closed: false,
    });
}

// ── Export principal ──────────────────────────────────────────

pub fn export_months_to_pdf(
    start_year:  i32,
    start_month: u32,
    end_year:    i32,
    end_month:   u32,
    output_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {

    let months = month_range(start_year, start_month, end_year, end_month);

    // Titre du document
    let title = if start_year == end_year {
        format!("Planning {}", start_year)
    } else {
        format!("Planning {} - {}", start_year, end_year)
    };

    let (doc, page1, layer1) = PdfDocument::new(
        &title,
        Mm(PAGE_WIDTH_MM),
        Mm(PAGE_HEIGHT_MM),
        "Layer 1",
    );

    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font      = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;

    // Dimensions
    let usable_width  = PAGE_WIDTH_MM  - 2.0 * MARGIN_MM;
    let usable_height = PAGE_HEIGHT_MM - 2.0 * MARGIN_MM;
    let nb_months     = months.len() as f32;
    let col_width     = usable_width / nb_months;
    let header_height = 8.0;
    let row_height    = (usable_height - header_height) / 31.0;

    // Titre général
    current_layer.set_fill_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
    current_layer.use_text(
        &title,
        10.0,
        Mm(MARGIN_MM),
        Mm(PAGE_HEIGHT_MM - MARGIN_MM + 2.0),
        &font_bold,
    );

    for (month_idx, &(year, month)) in months.iter().enumerate() {
        let col_x = MARGIN_MM + (month_idx as f32 * col_width);

        // Données du mois
        let entries = planning_repository::get_by_month(year, month)
            .unwrap_or_default();

        let mut day_map: HashMap<String, Vec<(String, String)>> = HashMap::new();
        for entry in &entries {
            let label = entry.label_name.clone().unwrap_or_default();
            let color = entry.label_color.clone()
                .unwrap_or_else(|| "#CCCCCC".to_string());
            day_map.entry(entry.date.clone()).or_default()
                   .push((label, color));
        }

        // En-tête du mois
        let header_y = PAGE_HEIGHT_MM - MARGIN_MM - header_height;
        draw_rect(&current_layer, col_x, header_y, col_width - 1.0, header_height, 0.12, 0.31, 0.47);

        // Nom du mois (+ année si multi-années)
        let month_label = if start_year != end_year {
            format!("{} {}", MONTH_NAMES[(month - 1) as usize], year)
        } else {
            MONTH_NAMES[(month - 1) as usize].to_string()
        };

        current_layer.set_fill_color(Color::Rgb(Rgb::new(1.0, 1.0, 1.0, None)));
        current_layer.use_text(
            &month_label,
            7.0,
            Mm(col_x + 1.0),
            Mm(header_y + 2.5),
            &font_bold,
        );

        // Lignes des jours
        let total_days = days_in_month(year, month);

        for day in 1..=total_days {
            let row_y = PAGE_HEIGHT_MM
                - MARGIN_MM
                - header_height
                - (day as f32 * row_height);

            let date_str   = format!("{:04}-{:02}-{:02}", year, month, day);
            let dow        = day_of_week(year, month, day);
            let day_abbr   = DAY_ABBR[dow as usize];
            let date_label = format!("{} {:02}/{:02}", day_abbr, day, month);

            if let Some(day_entries) = day_map.get(&date_str) {
                let (_, color_hex) = &day_entries[0];
                let (r, g, b) = hex_to_rgb(color_hex);

                // Fond coloré
                draw_rect(&current_layer, col_x, row_y, col_width - 1.0, row_height - 0.3, r, g, b);

                // Texte noir — date + label côte à côte
                current_layer.set_fill_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));

                let names: Vec<String> = day_entries.iter()
                    .map(|(name, _)| name.to_uppercase())
                    .collect();

                let content = format!("{}  {}", date_label, names.join(" / "));
                current_layer.use_text(
                    &content,
                    7.0,
                    Mm(col_x + 0.5),
                    Mm(row_y + row_height * 0.3),
                    &font_bold,
                );

            } else {
                // Cellule vide
                draw_rect(&current_layer, col_x, row_y, col_width - 1.0, row_height - 0.3, 0.98, 0.98, 0.98);

                current_layer.set_fill_color(Color::Rgb(Rgb::new(0.6, 0.6, 0.6, None)));
                current_layer.use_text(
                    &date_label,
                    7.0,
                    Mm(col_x + 0.5),
                    Mm(row_y + row_height * 0.3),
                    &font,
                );
            }
        }

        // Séparateur entre les mois
        if month_idx < months.len() - 1 {
            draw_separator(
                &current_layer,
                col_x + col_width - 0.5,
                MARGIN_MM,
                PAGE_HEIGHT_MM - MARGIN_MM,
            );
        }
    }

    let file = File::create(output_path)?;
    doc.save(&mut BufWriter::new(file))?;

    println!("Export PDF : {:?}", output_path);
    Ok(())
}