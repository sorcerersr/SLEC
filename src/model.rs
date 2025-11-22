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
                id: 3,
            },
        ];
        filters
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
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

        // Test all the shutter speed values and their display text
        assert_eq!(speeds[0].display_text, "1/8000");
        assert_eq!(speeds[0].speed_value, 1.0 / 8000.0);

        assert_eq!(speeds[1].display_text, "1/6400");
        assert_eq!(speeds[1].speed_value, 1.0 / 6400.0);

        assert_eq!(speeds[2].display_text, "1/5000");
        assert_eq!(speeds[2].speed_value, 1.0 / 5000.0);

        assert_eq!(speeds[3].display_text, "1/4000");
        assert_eq!(speeds[3].speed_value, 1.0 / 4000.0);

        assert_eq!(speeds[4].display_text, "1/3200");
        assert_eq!(speeds[4].speed_value, 1.0 / 3200.0);

        assert_eq!(speeds[5].display_text, "1/2500");
        assert_eq!(speeds[5].speed_value, 1.0 / 2500.0);

        assert_eq!(speeds[6].display_text, "1/2000");
        assert_eq!(speeds[6].speed_value, 1.0 / 2000.0);

        assert_eq!(speeds[7].display_text, "1/1600");
        assert_eq!(speeds[7].speed_value, 1.0 / 1600.0);

        assert_eq!(speeds[8].display_text, "1/1250");
        assert_eq!(speeds[8].speed_value, 1.0 / 1250.0);

        assert_eq!(speeds[9].display_text, "1/800");
        assert_eq!(speeds[9].speed_value, 1.0 / 800.0);

        assert_eq!(speeds[10].display_text, "1/640");
        assert_eq!(speeds[10].speed_value, 1.0 / 640.0);

        assert_eq!(speeds[11].display_text, "1/500");
        assert_eq!(speeds[11].speed_value, 1.0 / 500.0);

        assert_eq!(speeds[12].display_text, "1/400");
        assert_eq!(speeds[12].speed_value, 1.0 / 400.0);

        assert_eq!(speeds[13].display_text, "1/320");
        assert_eq!(speeds[13].speed_value, 1.0 / 320.0);

        assert_eq!(speeds[14].display_text, "1/250");
        assert_eq!(speeds[14].speed_value, 1.0 / 250.0);

        assert_eq!(speeds[15].display_text, "1/200");
        assert_eq!(speeds[15].speed_value, 1.0 / 200.0);

        assert_eq!(speeds[16].display_text, "1/160");
        assert_eq!(speeds[16].speed_value, 1.0 / 160.0);

        assert_eq!(speeds[17].display_text, "1/125");
        assert_eq!(speeds[17].speed_value, 1.0 / 125.0);

        assert_eq!(speeds[18].display_text, "1/100");
        assert_eq!(speeds[18].speed_value, 1.0 / 100.0);

        assert_eq!(speeds[19].display_text, "1/80");
        assert_eq!(speeds[19].speed_value, 1.0 / 80.0);

        assert_eq!(speeds[20].display_text, "1/60");
        assert_eq!(speeds[20].speed_value, 1.0 / 60.0);

        assert_eq!(speeds[21].display_text, "1/50");
        assert_eq!(speeds[21].speed_value, 1.0 / 50.0);

        assert_eq!(speeds[22].display_text, "1/40");
        assert_eq!(speeds[22].speed_value, 1.0 / 40.0);

        assert_eq!(speeds[23].display_text, "1/30");
        assert_eq!(speeds[23].speed_value, 1.0 / 30.0);

        assert_eq!(speeds[24].display_text, "1/25");
        assert_eq!(speeds[24].speed_value, 1.0 / 25.0);

        assert_eq!(speeds[25].display_text, "1/20");
        assert_eq!(speeds[25].speed_value, 1.0 / 20.0);

        assert_eq!(speeds[26].display_text, "1/15");
        assert_eq!(speeds[26].speed_value, 1.0 / 15.0);

        assert_eq!(speeds[27].display_text, "1/10");
        assert_eq!(speeds[27].speed_value, 1.0 / 10.0);

        assert_eq!(speeds[28].display_text, "1/8");
        assert_eq!(speeds[28].speed_value, 1.0 / 8.0);

        assert_eq!(speeds[29].display_text, "1/6");
        assert_eq!(speeds[29].speed_value, 1.0 / 6.0);

        assert_eq!(speeds[30].display_text, "1/5");
        assert_eq!(speeds[30].speed_value, 1.0 / 5.0);

        assert_eq!(speeds[31].display_text, "1/4");
        assert_eq!(speeds[31].speed_value, 1.0 / 4.0);

        assert_eq!(speeds[32].display_text, "0.3");
        assert_eq!(speeds[32].speed_value, 0.3);

        assert_eq!(speeds[33].display_text, "0.4");
        assert_eq!(speeds[33].speed_value, 0.4);

        assert_eq!(speeds[34].display_text, "0.5");
        assert_eq!(speeds[34].speed_value, 0.5);

        assert_eq!(speeds[35].display_text, "0.6");
        assert_eq!(speeds[35].speed_value, 0.6);

        assert_eq!(speeds[36].display_text, "0.8");
        assert_eq!(speeds[36].speed_value, 0.8);

        assert_eq!(speeds[37].display_text, "1");
        assert_eq!(speeds[37].speed_value, 1.0);

        assert_eq!(speeds[38].display_text, "1.3");
        assert_eq!(speeds[38].speed_value, 1.3);

        assert_eq!(speeds[39].display_text, "1.6");
        assert_eq!(speeds[39].speed_value, 1.6);

        assert_eq!(speeds[40].display_text, "2");
        assert_eq!(speeds[40].speed_value, 2.0);

        assert_eq!(speeds[41].display_text, "2.5");
        assert_eq!(speeds[41].speed_value, 2.5);

        assert_eq!(speeds[42].display_text, "3.2");
        assert_eq!(speeds[42].speed_value, 3.2);

        assert_eq!(speeds[43].display_text, "4");
        assert_eq!(speeds[43].speed_value, 4.0);

        assert_eq!(speeds[44].display_text, "5");
        assert_eq!(speeds[44].speed_value, 5.0);

        assert_eq!(speeds[45].display_text, "6");
        assert_eq!(speeds[45].speed_value, 6.0);

        assert_eq!(speeds[46].display_text, "8");
        assert_eq!(speeds[46].speed_value, 8.0);

        assert_eq!(speeds[47].display_text, "10");
        assert_eq!(speeds[47].speed_value, 10.0);

        assert_eq!(speeds[48].display_text, "13");
        assert_eq!(speeds[48].speed_value, 13.0);

        assert_eq!(speeds[49].display_text, "15");
        assert_eq!(speeds[49].speed_value, 15.0);

        assert_eq!(speeds[50].display_text, "20");
        assert_eq!(speeds[50].speed_value, 20.0);

        assert_eq!(speeds[51].display_text, "25");
        assert_eq!(speeds[51].speed_value, 25.0);

        assert_eq!(speeds[52].display_text, "30");
        assert_eq!(speeds[52].speed_value, 30.0);
    }
}
