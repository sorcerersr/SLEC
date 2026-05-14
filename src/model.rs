use gloo_console::log;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

pub struct ShutterSpeed {
    pub display_text: String,
    pub speed_value: f64,
}

impl ShutterSpeed {
    fn new(display_text: String, speed_value: f64) -> ShutterSpeed {
        ShutterSpeed {
            display_text,
            speed_value,
        }
    }

    pub fn shutter_speed_array() -> [ShutterSpeed; 53] {
        let speeds: [ShutterSpeed; 53] = [
            ShutterSpeed::new("1/8000".to_owned(), 1.0 / 8000.0),
            ShutterSpeed::new("1/6400".to_owned(), 1.0 / 6400.0),
            ShutterSpeed::new("1/5000".to_owned(), 1.0 / 5000.0),
            ShutterSpeed::new("1/4000".to_owned(), 1.0 / 4000.0),
            ShutterSpeed::new("1/3200".to_owned(), 1.0 / 3200.0),
            ShutterSpeed::new("1/2500".to_owned(), 1.0 / 2500.0),
            ShutterSpeed::new("1/2000".to_owned(), 1.0 / 2000.0),
            ShutterSpeed::new("1/1600".to_owned(), 1.0 / 1600.0),
            ShutterSpeed::new("1/1250".to_owned(), 1.0 / 1250.0),
            ShutterSpeed::new("1/800".to_owned(), 1.0 / 800.0),
            ShutterSpeed::new("1/640".to_owned(), 1.0 / 640.0),
            ShutterSpeed::new("1/500".to_owned(), 1.0 / 500.0),
            ShutterSpeed::new("1/400".to_owned(), 1.0 / 400.0),
            ShutterSpeed::new("1/320".to_owned(), 1.0 / 320.0),
            ShutterSpeed::new("1/250".to_owned(), 1.0 / 250.0),
            ShutterSpeed::new("1/200".to_owned(), 1.0 / 200.0),
            ShutterSpeed::new("1/160".to_owned(), 1.0 / 160.0),
            ShutterSpeed::new("1/125".to_owned(), 1.0 / 125.0),
            ShutterSpeed::new("1/100".to_owned(), 1.0 / 100.0),
            ShutterSpeed::new("1/80".to_owned(), 1.0 / 80.0),
            ShutterSpeed::new("1/60".to_owned(), 1.0 / 60.0),
            ShutterSpeed::new("1/50".to_owned(), 1.0 / 50.0),
            ShutterSpeed::new("1/40".to_owned(), 1.0 / 40.0),
            ShutterSpeed::new("1/30".to_owned(), 1.0 / 30.0),
            ShutterSpeed::new("1/25".to_owned(), 1.0 / 25.0),
            ShutterSpeed::new("1/20".to_owned(), 1.0 / 20.0),
            ShutterSpeed::new("1/15".to_owned(), 1.0 / 15.0),
            ShutterSpeed::new("1/10".to_owned(), 1.0 / 10.0),
            ShutterSpeed::new("1/8".to_owned(), 1.0 / 8.0),
            ShutterSpeed::new("1/6".to_owned(), 1.0 / 6.0),
            ShutterSpeed::new("1/5".to_owned(), 1.0 / 5.0),
            ShutterSpeed::new("1/4".to_owned(), 1.0 / 4.0),
            ShutterSpeed::new("0.3".to_owned(), 0.3),
            ShutterSpeed::new("0.4".to_owned(), 0.4),
            ShutterSpeed::new("0.5".to_owned(), 0.5),
            ShutterSpeed::new("0.6".to_owned(), 0.6),
            ShutterSpeed::new("0.8".to_owned(), 0.8),
            ShutterSpeed::new("1".to_owned(), 1.0),
            ShutterSpeed::new("1.3".to_owned(), 1.3),
            ShutterSpeed::new("1.6".to_owned(), 1.6),
            ShutterSpeed::new("2".to_owned(), 2.0),
            ShutterSpeed::new("2.5".to_owned(), 2.5),
            ShutterSpeed::new("3.2".to_owned(), 3.2),
            ShutterSpeed::new("4".to_owned(), 4.0),
            ShutterSpeed::new("5".to_owned(), 5.0),
            ShutterSpeed::new("6".to_owned(), 6.0),
            ShutterSpeed::new("8".to_owned(), 8.0),
            ShutterSpeed::new("10".to_owned(), 10.0),
            ShutterSpeed::new("13".to_owned(), 13.0),
            ShutterSpeed::new("15".to_owned(), 15.0),
            ShutterSpeed::new("20".to_owned(), 20.0),
            ShutterSpeed::new("25".to_owned(), 25.0),
            ShutterSpeed::new("30".to_owned(), 30.0),
        ];

        speeds
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub factor: u64,
    pub fstop_reduction: f64,
    pub display_name: String,
    pub selected: bool,
    pub id: usize,
}

impl Filter {
    pub fn filter_list() -> Vec<Filter> {
        if let Ok(filters_json) = LocalStorage::get::<String>("filters") {
            if let Ok(filters) = serde_json::from_str(&filters_json) {
                filters
            } else {
                log!("failed to load filter list from LocalStorage! Using default filters");
                Filter::default_filter_list()
            }
        } else {
            let filters = Filter::default_filter_list();
            Filter::store_filter_list(&filters);
            filters
        }
    }

    pub fn store_filter_list(filters: &Vec<Filter>) {
        let json_result = serde_json::to_string(&filters);
        match json_result {
            Ok(json) => {
                let storage_result = LocalStorage::set("filters", json);
                match storage_result {
                    Ok(_) => {}
                    Err(e) => {
                        log!(
                            "failed to write filter list to LocalStorage",
                            format!("{:#?}", e)
                        );
                    }
                }
            }
            Err(e) => {
                log!("failed to serialize filter list", format!("{:#?}", e));
            }
        }
    }

    pub fn default_filter_list() -> Vec<Filter> {
        let filters = vec![
            Filter {
                factor: 8,
                fstop_reduction: 3.0,
                display_name: "ND8".to_owned(),
                selected: false,
                id: 0,
            },
            Filter {
                factor: 64,
                fstop_reduction: 6.0,
                display_name: "ND64".to_owned(),
                selected: false,
                id: 1,
            },
            Filter {
                factor: 1024,
                fstop_reduction: 10.0,
                display_name: "ND1000".to_owned(),
                selected: false,
                id: 2,
            },
        ];
        filters
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn new_custom(factor: u64, fstop: f64, name: String) -> Filter {
        Filter {
            factor,
            fstop_reduction: fstop,
            display_name: name,
            selected: false,
            id: 0, // will be set by next_id when adding to a list
        }
    }

    pub fn next_id(filters: &[Filter]) -> usize {
        filters
            .iter()
            .map(|f| f.id)
            .max()
            .map(|m| m + 1)
            .unwrap_or(0)
    }

    pub fn reset_to_defaults() -> Vec<Filter> {
        Filter::default_filter_list()
    }

    pub fn remove_filter(filters: &mut Vec<Filter>, id: usize) {
        filters.retain(|f| f.id != id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shutter_speed_array_length() {
        let speeds = ShutterSpeed::shutter_speed_array();
        assert_eq!(speeds.len(), 53);
    }

    #[test]
    fn test_shutter_speed_array_values() {
        let speeds = ShutterSpeed::shutter_speed_array();

        // Define expected values for verification
        let expected_values = [
            ("1/8000", 1.0 / 8000.0),
            ("1/6400", 1.0 / 6400.0),
            ("1/5000", 1.0 / 5000.0),
            ("1/4000", 1.0 / 4000.0),
            ("1/3200", 1.0 / 3200.0),
            ("1/2500", 1.0 / 2500.0),
            ("1/2000", 1.0 / 2000.0),
            ("1/1600", 1.0 / 1600.0),
            ("1/1250", 1.0 / 1250.0),
            ("1/800", 1.0 / 800.0),
            ("1/640", 1.0 / 640.0),
            ("1/500", 1.0 / 500.0),
            ("1/400", 1.0 / 400.0),
            ("1/320", 1.0 / 320.0),
            ("1/250", 1.0 / 250.0),
            ("1/200", 1.0 / 200.0),
            ("1/160", 1.0 / 160.0),
            ("1/125", 1.0 / 125.0),
            ("1/100", 1.0 / 100.0),
            ("1/80", 1.0 / 80.0),
            ("1/60", 1.0 / 60.0),
            ("1/50", 1.0 / 50.0),
            ("1/40", 1.0 / 40.0),
            ("1/30", 1.0 / 30.0),
            ("1/25", 1.0 / 25.0),
            ("1/20", 1.0 / 20.0),
            ("1/15", 1.0 / 15.0),
            ("1/10", 1.0 / 10.0),
            ("1/8", 1.0 / 8.0),
            ("1/6", 1.0 / 6.0),
            ("1/5", 1.0 / 5.0),
            ("1/4", 1.0 / 4.0),
            ("0.3", 0.3),
            ("0.4", 0.4),
            ("0.5", 0.5),
            ("0.6", 0.6),
            ("0.8", 0.8),
            ("1", 1.0),
            ("1.3", 1.3),
            ("1.6", 1.6),
            ("2", 2.0),
            ("2.5", 2.5),
            ("3.2", 3.2),
            ("4", 4.0),
            ("5", 5.0),
            ("6", 6.0),
            ("8", 8.0),
            ("10", 10.0),
            ("13", 13.0),
            ("15", 15.0),
            ("20", 20.0),
            ("25", 25.0),
            ("30", 30.0),
        ];

        // Verify each element
        for (i, (expected_text, expected_value)) in expected_values.iter().enumerate() {
            assert_eq!(speeds[i].display_text, *expected_text);
            assert_eq!(speeds[i].speed_value, *expected_value);
        }
    }

    #[test]
    fn test_filter_default_list() {
        let filters = Filter::default_filter_list();

        assert_eq!(filters.len(), 3);

        // Check first filter
        assert_eq!(filters[0].factor, 8);
        assert_eq!(filters[0].fstop_reduction, 3.0);
        assert_eq!(filters[0].display_name, "ND8");
        assert!(!filters[0].selected);
        assert_eq!(filters[0].id, 0);

        // Check second filter
        assert_eq!(filters[1].factor, 64);
        assert_eq!(filters[1].fstop_reduction, 6.0);
        assert_eq!(filters[1].display_name, "ND64");
        assert!(!filters[1].selected);
        assert_eq!(filters[1].id, 1);

        // Check third filter
        assert_eq!(filters[2].factor, 1024);
        assert_eq!(filters[2].fstop_reduction, 10.0);
        assert_eq!(filters[2].display_name, "ND1000");
        assert!(!filters[2].selected);
        assert_eq!(filters[2].id, 2);
    }

    #[test]
    fn test_next_id_empty() {
        assert_eq!(Filter::next_id(&[]), 0);
    }

    #[test]
    fn test_next_id_with_filters() {
        let filters = Filter::default_filter_list();
        // Default list has ids 0, 1, 2 → next id is 3
        assert_eq!(Filter::next_id(&filters), 3);
    }

    #[test]
    fn test_remove_existing_filter() {
        let mut filters = Filter::default_filter_list();
        assert_eq!(filters.len(), 3);
        Filter::remove_filter(&mut filters, 0); // remove ND8
        assert_eq!(filters.len(), 2);
        assert!(filters.iter().all(|f| f.id != 0));
    }

    #[test]
    fn test_remove_nonexistent_filter() {
        let mut filters = Filter::default_filter_list();
        Filter::remove_filter(&mut filters, 99);
        assert_eq!(filters.len(), 3);
    }

    #[test]
    fn test_reset_to_defaults() {
        let defaults = Filter::default_filter_list();
        let reset = Filter::reset_to_defaults();
        assert_eq!(reset.len(), defaults.len());
        for (a, b) in defaults.iter().zip(reset.iter()) {
            assert_eq!(a.factor, b.factor);
            assert_eq!(a.fstop_reduction, b.fstop_reduction);
            assert_eq!(a.display_name, b.display_name);
        }
    }

    #[test]
    fn test_new_custom_filter() {
        let filter = Filter::new_custom(256, 8.0, "ND256".to_owned());
        assert_eq!(filter.factor, 256);
        assert_eq!(filter.fstop_reduction, 8.0);
        assert_eq!(filter.display_name, "ND256");
        assert!(!filter.selected);
        assert_eq!(filter.id, 0); // id set to 0 by new_custom
    }
}
