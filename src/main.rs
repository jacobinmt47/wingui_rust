/*!
    A very simple application that show your name in a message box.
    See `basic` for the version without the derive macro
*/


extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;


use nwd::NwgUi;
use nwg::NativeUi;
use rand_distr::{Normal, Distribution};


#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (300, 200), position: (300, 300), title: "lows at end per trend", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,

    #[nwg_control(text: "1000", size: (100, 25), position: (150, 10))]
    station_edit: nwg::TextInput,

    #[nwg_control(text: "number of stations",position:(10,10))]
    station_label: nwg::Label,

    #[nwg_control(text: "100", size: (100, 25), position: (150,45))]
    length_edit: nwg::TextInput,
    #[nwg_control(text: "length run in years",position:(10,45))]
    length_label: nwg::Label,
    #[nwg_control(text: "0.003", size: (100, 25), position: (150,80))]
    trend_edit: nwg::TextInput,
    #[nwg_control(text: "Trend per year",position:(10,80))]
    trend_label: nwg::Label,

    #[nwg_control(text: "calculate", size: (280, 60), position: (10, 120))]
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    hello_button: nwg::Button
}

impl BasicApp {
   
    fn say_hello(&self) {
        let station_count: i64 = self.station_edit.text().parse().unwrap();
        let length:i64 = self.length_edit.text().parse().unwrap();
        let trend:f64 = self.trend_edit.text().parse().unwrap();

        println!("{}",station_count);
        println!("{}",length);
        println!("{}",trend);
        let normal = Normal::new(0.0,1.0).unwrap();
        for x in 0..station_count{
            let mut min:f64 = 10.0; // very high value sure to
            for y in 0..length{
                // convert to f64 and multiply by rnadom number
                let l:f64 = y as f64;
                let t:f64 = l * trend;
                let norm_var = normal.sample(&mut rand::thread_rng());
                let z = t + norm_var;
                if z<min {
                    min = z;
                }
                if y == (length - 1) && min == z { println!("{0}, {1} ",min,x)}
               
            }
        }
    }
    
    fn say_goodbye(&self) {
        nwg::simple_message("{}","Goodbye");
        nwg::stop_thread_dispatch();
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}