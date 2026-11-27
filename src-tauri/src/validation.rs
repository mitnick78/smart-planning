//! Validation des arguments reçus du frontend.
//!
//! Le frontend n'est jamais considéré comme fiable (CLAUDE.md §4) : chaque
//! commande Tauri valide ses arguments ici avant de toucher à la base ou au
//! système de fichiers. Les messages d'erreur sont destinés à l'utilisateur.
//!
//! Ces fonctions rejoindront la couche `services/` lors de sa création
//! (lot 3 de AUDIT.md).

use crate::models::{Label, Period};
use chrono::NaiveDate;

/// Années acceptées pour le planning, les statistiques et les exports.
pub const YEAR_MIN: i32 = 1970;
pub const YEAR_MAX: i32 = 2100;

/// Nombre maximal de mois dans un export.
pub const MAX_EXPORT_MONTHS: u32 = 60;

/// Zones de vacances scolaires reconnues.
const SCHOOL_ZONES: [&str; 3] = ["A", "B", "C"];

/// Vérifie le nom et la couleur `#RRGGBB` d'un label.
pub fn validate_label_input(name: &str, color_hex: &str) -> Result<(), String> {
    if name.trim().is_empty() {
        return Err("Le nom ne peut pas être vide".to_string());
    }
    if !Label::is_valid_color(color_hex) {
        return Err(format!(
            "Couleur invalide : {color_hex} (format attendu : #RRGGBB)"
        ));
    }
    Ok(())
}

/// Vérifie une date au format strict `YYYY-MM-DD`, dans les années acceptées.
pub fn validate_date(date: &str) -> Result<NaiveDate, String> {
    let invalid = || format!("Date invalide : {date} (format attendu : AAAA-MM-JJ)");
    // chrono accepte aussi "2026-3-5" : on impose la forme stockée en base,
    // sinon les recherches par mois (LIKE 'AAAA-MM%') ne la retrouveraient pas.
    if date.len() != 10 {
        return Err(invalid());
    }
    let parsed = NaiveDate::parse_from_str(date, "%Y-%m-%d").map_err(|_| invalid())?;
    validate_year(chrono::Datelike::year(&parsed))?;
    Ok(parsed)
}

/// Vérifie qu'une période fait partie des valeurs connues.
pub fn validate_period(period: &str) -> Result<Period, String> {
    period
        .parse::<Period>()
        .map_err(|()| format!("Période invalide : {period}"))
}

pub fn validate_year(year: i32) -> Result<(), String> {
    if (YEAR_MIN..=YEAR_MAX).contains(&year) {
        Ok(())
    } else {
        Err(format!(
            "Année invalide : {year} (attendue entre {YEAR_MIN} et {YEAR_MAX})"
        ))
    }
}

pub fn validate_year_month(year: i32, month: u32) -> Result<(), String> {
    validate_year(year)?;
    if (1..=12).contains(&month) {
        Ok(())
    } else {
        Err(format!("Mois invalide : {month} (attendu entre 1 et 12)"))
    }
}

/// Vérifie une plage de mois d'export : bornes valides, début avant la fin,
/// et taille raisonnable.
///
/// Sans cette vérification, `month_range` des exports boucle indéfiniment
/// quand le début est après la fin ou qu'un mois dépasse 12.
pub fn validate_month_range(
    start_year: i32,
    start_month: u32,
    end_year: i32,
    end_month: u32,
) -> Result<(), String> {
    validate_year_month(start_year, start_month)?;
    validate_year_month(end_year, end_month)?;

    // Années et mois déjà bornés : aucun dépassement possible.
    let start = i64::from(start_year) * 12 + i64::from(start_month);
    let end = i64::from(end_year) * 12 + i64::from(end_month);
    if start > end {
        return Err("Le mois de début doit précéder le mois de fin".to_string());
    }
    let months = end - start + 1;
    if months > i64::from(MAX_EXPORT_MONTHS) {
        return Err(format!(
            "Plage trop longue : {months} mois (maximum {MAX_EXPORT_MONTHS})"
        ));
    }
    Ok(())
}

/// Vérifie une zone de vacances scolaires (`A`, `B` ou `C`).
pub fn validate_zone(zone: &str) -> Result<(), String> {
    if SCHOOL_ZONES.contains(&zone) {
        Ok(())
    } else {
        Err(format!("Zone invalide : {zone} (attendue : A, B ou C)"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_input_rejects_blank_name_and_bad_colors() {
        assert!(validate_label_input("École", "#3B82F6").is_ok());
        assert!(validate_label_input("   ", "#3B82F6").is_err());
        for bad in ["3B82F6", "#3B82F", "#3B82F6A", "#GGGGGG", "", "#3b82f6 "] {
            assert!(validate_label_input("École", bad).is_err(), "{bad:?}");
        }
    }

    #[test]
    fn date_must_be_strict_iso_and_real() {
        assert!(validate_date("2026-03-15").is_ok());
        assert!(validate_date("2024-02-29").is_ok());
        for bad in [
            "2026-3-15",
            "2026-02-30",
            "2025-02-29",
            "15/03/2026",
            "2026-03-15T00:00",
            "",
            "1900-01-01",
        ] {
            assert!(validate_date(bad).is_err(), "{bad:?}");
        }
    }

    #[test]
    fn period_accepts_only_known_values() {
        for ok in ["matin", "après-midi", "soir", "journée"] {
            assert!(validate_period(ok).is_ok(), "{ok:?}");
        }
        for bad in ["", "Matin", "nuit", "apres-midi"] {
            assert!(validate_period(bad).is_err(), "{bad:?}");
        }
    }

    #[test]
    fn year_month_bounds() {
        assert!(validate_year_month(2026, 1).is_ok());
        assert!(validate_year_month(2026, 12).is_ok());
        assert!(validate_year_month(2026, 0).is_err());
        assert!(validate_year_month(2026, 13).is_err());
        assert!(validate_year_month(YEAR_MIN - 1, 6).is_err());
        assert!(validate_year_month(YEAR_MAX + 1, 6).is_err());
    }

    #[test]
    fn month_range_rejects_inverted_invalid_and_oversized_ranges() {
        assert!(validate_month_range(2026, 3, 2026, 3).is_ok());
        assert!(validate_month_range(2025, 11, 2026, 2).is_ok());
        // Cas qui faisaient boucler month_range à l'infini ou paniquer.
        assert!(validate_month_range(2026, 5, 2026, 3).is_err());
        assert!(validate_month_range(2027, 1, 2026, 12).is_err());
        assert!(validate_month_range(2026, 1, 2026, 13).is_err());
        assert!(validate_month_range(2026, 0, 2026, 3).is_err());
        // Taille maximale : exactement MAX_EXPORT_MONTHS passe, un de plus non.
        assert!(validate_month_range(2026, 1, 2030, 12).is_ok());
        assert!(validate_month_range(2026, 1, 2031, 1).is_err());
    }

    #[test]
    fn zone_accepts_only_a_b_c() {
        for ok in ["A", "B", "C"] {
            assert!(validate_zone(ok).is_ok());
        }
        for bad in ["", "a", "D", "Zone A", "A\" or 1=1"] {
            assert!(validate_zone(bad).is_err(), "{bad:?}");
        }
    }
}
