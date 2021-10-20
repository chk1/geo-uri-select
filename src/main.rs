#![windows_subsystem = "windows"]
mod regexes;

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;

use regex::Regex;
use std::env;

windows::include_bindings!();

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (300, 225), center: true, title: "Geo: Select", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnInit: [BasicApp::on_init], OnWindowClose: [BasicApp::quit] )]
    window: nwg::Window,

    #[nwg_resource]
    embed: nwg::EmbedResource,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control(text: "OpenStreetMap")]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    #[nwg_events( OnButtonClick: [BasicApp::open_osm] )]
    button_osm: nwg::Button,

    #[nwg_control(text: "Google Maps")]
    #[nwg_layout_item(layout: grid, row: 1, col: 0)]
    #[nwg_events( OnButtonClick: [BasicApp::open_gmaps] )]
    button_gmaps: nwg::Button,

    #[nwg_control(text: "Bing Maps (Web)")]
    #[nwg_layout_item(layout: grid, row: 2, col: 0)]
    #[nwg_events( OnButtonClick: [BasicApp::open_bing] )]
    button_bing: nwg::Button,

    #[nwg_control(text: "Bing Maps (App)")]
    #[nwg_layout_item(layout: grid, row: 3, col: 0)]
    #[nwg_events( OnButtonClick: [BasicApp::open_bing_native] )]
    button_bingmaps_native: nwg::Button,
    
    #[nwg_control(text: "Yandex")]
    #[nwg_layout_item(layout: grid, row: 4, col: 0)]
    #[nwg_events( OnButtonClick: [BasicApp::open_yandex] )]
    button_yandex: nwg::Button,

    #[nwg_control(text: "geo:37.786971,-122.399677")]
    #[nwg_layout_item(layout: grid, row: 5, col: 0)]
    edit_geo_uri: nwg::TextInput
}

const MAP_PROVIDER_OSM: i8 = 0;
const MAP_PROVIDER_BING: i8 = 1;
const MAP_PROVIDER_GMAPS: i8 = 2;
const MAP_PROVIDER_BING_APP: i8 = 3;
const MAP_PROVIDER_YANDEX: i8 = 4;

impl BasicApp {
    fn open_browser(&self, map_provider: i8) {
        let input = self.edit_geo_uri.text();
        let re_geo  = Regex::new(r"^geo:(?P<lat>-?[0-9]+(\.[0-9]+)?),(?P<lon>-?[0-9]+(\.[0-9]+)?)").unwrap();
        let re_bing = Regex::new(r"^bingmaps:(.+)(collection=point\.|cp=)(?P<lat>-?[0-9]+(\.[0-9]+)?)[_~](?P<lon>-?[0-9]+(\.[0-9]+)?)").unwrap();
        let cap ;
        let mut uri_is_bing = false;
        if re_geo.is_match(&input) {
            cap = re_geo.captures(&input).unwrap();
        } else if re_bing.is_match(&input) {
            cap = re_bing.captures(&input).unwrap();
            uri_is_bing = true;
        } else {
            nwg::modal_error_message(&self.window, "Error: Invalid uri", "The provided uri appears to be invalid.");
            return;
        }

        if cap.len() == 0 {
            nwg::modal_error_message(&self.window, "Error: Invalid uri", "Could not extract coordinates from the uri.");
            return;
        }
        let lat = cap.name("lat").unwrap().as_str();
        let lon = cap.name("lon").unwrap().as_str();

        // (mis-)using "1" as a return type for all match arms
        match map_provider {
            MAP_PROVIDER_OSM => {
                open::that(format!("https://www.openstreetmap.org/?mlat={}&mlon={}&zoom=13&layers=M", lat=lat, lon=lon)).unwrap(); 
                1
            },
            MAP_PROVIDER_BING => {
                open::that(format!("https://www.bing.com/maps/?v=2&cp={lat}~{lon}&style=r&lvl=13&sp=Point.{lat}_{lon}_Point", lat=lat, lon=lon)).unwrap(); 
                1
            },
            MAP_PROVIDER_GMAPS => {
                open::that(format!("https://www.google.com/maps?ll={lat},{lon}&q={lat},{lon}&hl=en&t=m&z=13", lat=lat, lon=lon)).unwrap(); 
                1
            },
            MAP_PROVIDER_BING_APP => {
                // https://docs.microsoft.com/en-us/windows/uwp/launch-resume/launch-maps-app#launch-a-uri-from-your-app
                let uri;
                if uri_is_bing {
                    // use input unchanged
                    uri = input;
                } else {
                    // use simplified call: point coordinate only
                    uri = format!("bingmaps:?lvl=16&collection=point.{lat}_{lon}_Point", lat=lat, lon=lon);
                }
                let win_uri = Windows::Foundation::Uri::CreateUri(uri).unwrap();
                let lo = Windows::System::LauncherOptions::new().unwrap();
                lo.SetTargetApplicationPackageFamilyName("Microsoft.WindowsMaps_8wekyb3d8bbwe").unwrap();
                Windows::System::Launcher::LaunchUriWithOptionsAsync(win_uri, lo).unwrap();
                1
            },
            MAP_PROVIDER_YANDEX => {
                open::that(format!("https://yandex.com/maps/?ll={lon},{lat}&z=13", lat=lat, lon=lon)).unwrap(); 
                1
            },
            _ => unreachable!()
        };
    }

    fn open_osm(&self) {
        self.open_browser(MAP_PROVIDER_OSM);
    }
    
    fn open_bing(&self) {
        self.open_browser(MAP_PROVIDER_BING);
    }

    fn open_bing_native(&self) {
        self.open_browser(MAP_PROVIDER_BING_APP);
    }

    fn open_gmaps(&self) {
        self.open_browser(MAP_PROVIDER_GMAPS);
    }

    fn open_yandex(&self) {
        self.open_browser(MAP_PROVIDER_YANDEX);
    }

    fn on_init(&self) {
        let em = &self.embed;
        self.window.set_icon(em.icon_str("app_icon", None).as_ref());
    }

    fn quit(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        _app.edit_geo_uri.set_text(&args[1]);
    }

    nwg::dispatch_thread_events();
}
