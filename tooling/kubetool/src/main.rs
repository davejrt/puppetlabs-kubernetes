#[macro_use]
extern crate clap;
use clap::App;
mod otherparams;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    // let matches = App::from_yaml(yaml).get_matches();

    // _create(matches.value_of("os").unwrap(),"1.13.2","weave","https://172.17.10.101:2379","172.17.10.101","172,17.10.101","172.17.10.101","true")

    otherparams::create("debian","1.13.2","docker","weave","kube-master:172.17.10.101,kube-node-01:172.17.10.102","172.17.10.101","172,17.10.101","true")

}





