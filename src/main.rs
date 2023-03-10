mod destination;
mod model;
mod source;

use destination::{file::File, Destination, meilisearch::Meilisearch};
use source::{chrome::Chrome, firefox::Firefox, safari::Safari, Source};

fn main() {
    let safari_path = "/Users/o.utkin/Library/Safari/History.db";
    // export(
    //     Safari::new(safari_path).unwrap(),
    //     File::new("visits_safari.json"),
    // );
    export(
        Safari::new(safari_path).unwrap(),
        Meilisearch::new("http://localhost:7700", "IpWpkU9a6ixB3tLZha6HX-rZCAJCehlfq6roEyedk98"),
    );

    // let firefox_path = "/Users/o.utkin/Library/Application Support/Firefox/Profiles/2qs1beu4.default/places.sqlite";
    // export(
    //     Firefox::new(firefox_path).unwrap(),
    //     File::new("visits_firefox.json"),
    // );

    // let chrome_path = "/Users/o.utkin/Library/Application Support/Google/Chrome/Default/History";
    // export(
    //     Chrome::new(chrome_path).unwrap(),
    //     File::new("visits_chrome.json"),
    // );
}

fn export(src: impl Source, dst: impl Destination) {
    let visits = src.get_visits().unwrap();
    dst.push_visits(&visits).unwrap();
}
