// serde is used for serializing and deserializing data structures, which is common in Rust applications, especially when dealing with data that needs to be stored or transmitted. In this case, it is likely being used to define data models that can be easily converted to and from formats like JSON.
use serde::{Deserialize, Serialize};


//Label
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] 
pub struct Label{
    pub id: Option<i64>,
    pub name: String,
    pub color_hex: String,
    pub created_at: Option<String>,
}

impl Label{
    pub fn new(name: &str, color_hex: &str) -> Self {
        Self {
            id: None,
            name: name.to_string(),
            color_hex: color_hex.to_string(),
            created_at: None,
        }
    }

    pub fn is_valid_color(color: &str) -> bool {
        // Check if the color string is a valid hex code (e.g., #RRGGBB)
        if color.len() != 7 || !color.starts_with('#') {
            return false;
        }
        color[1..].chars().all(|c| c.is_digit(16))
    }
}

pub enum Period {
    Morning,
    Afternoon,
    Evening,
    Day,
}

impl Period {
    pub fn to_string(&self) -> String {
        match self {
            Period::Morning => "matin".to_string(),
            Period::Afternoon => "après-midi".to_string(),
            Period::Evening => "soir".to_string(),
            Period::Day => "journée".to_string(),
        }
    }

    pub fn from_str(s: &str) -> Option<Period> {
        match s {
            "matin" => Some(Period::Morning),
            "après-midi" => Some(Period::Afternoon),
            "soir" => Some(Period::Evening),
            "journée" => Some(Period::Day),
            _ => None,
        }
    }

    pub fn emoji(&self) -> &str {
        match self {
            Period::Morning => "🌅",
            Period::Afternoon => "☀️",
            Period::Evening => "🌙",
            Period::Day => "📅",
        }
    }
}

//PlanningEntry

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] 
pub struct PlanningEntry {
    pub id: Option<i64>,
    pub date: String,
    pub period: String,
    pub label_id: i64,
    pub note: Option<String>,
    
    pub label_name: Option<String>,
    pub label_color: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl PlanningEntry {
    pub fn new(date: &str, period: &str, label_id: i64) -> Self {
        Self {
            id: None,
            date: date.to_string(),
            period: period.to_string(),
            label_id,
            note: None,
            label_name: None,
            label_color: None,
            created_at: None,
            updated_at: None,
        }
    }

    pub fn display_label(&self) -> String {
        let emoji = Period::from_str(&self.period).map_or("📅".to_string(), |p| p.emoji().to_string());

        let name = self.label_name
        .as_deref()
        .unwrap_or("Label inconnu");

        format!("{} {}", emoji, name)
    }
}

//stats

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] 
pub struct StatEntry {
    pub label_name: String,
    pub label_color: String,
    pub period: String,
    pub count: i64,
}