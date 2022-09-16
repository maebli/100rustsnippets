extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};

fn main() {
    let vendor_yaml_url = "https://raw.githubusercontent.com/TheThingsNetwork/lorawan-devices/master/vendor/index.yaml";
    let vendor_yaml = reqwest::blocking::get(vendor_yaml_url).unwrap().text().unwrap();

    let vendors = YamlLoader::load_from_str(&vendor_yaml).unwrap();
    let vendors = vendors[0]["vendors"].clone();

    /* minus 1 because there is a "example" vendor */
    let vendor_count = vendors.as_vec().unwrap().len()-1;

    println!("There are currently {} vendors in the Things Network device repository. ", vendor_count);

}
