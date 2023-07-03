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

#[derive(PartialEq, Clone)]
pub struct Filter {
    pub factor: u64,
    pub density: f64,
    pub fstop_reduction: f64,
    pub display_name: String,
    pub selected: bool,
}

impl Filter {
    pub fn filter_array() -> [Filter; 3] {
        let filters: [Filter; 3] = [
            Filter {
                factor: 8,
                density: 0.9,
                fstop_reduction: 3.0,
                display_name: "ND8".to_owned(),
                selected: false,
            },
            Filter {
                factor: 64,
                density: 1.8,
                fstop_reduction: 6.0,
                display_name: "ND64".to_owned(),
                selected: false,
            },
            Filter {
                factor: 1024,
                density: 3.0,
                fstop_reduction: 10.0,
                display_name: "ND1000".to_owned(),
                selected: false,
            },
        ];
        filters
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}
