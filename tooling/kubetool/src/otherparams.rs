extern crate serde_yaml;
use std::collections::HashMap;

pub fn create(os: &str, version: &str, container_runtime: &str, cni_network_provider: &str, etcd_initial_cluster: &str, etcd_ip: &str, api_address: &str, install_dashboard: &str ){

    let mut data = HashMap::new();

    match os {
        "debian" => {
            let v = version.to_string();
            let package = "-00".to_string();
            let kubernetes_package_version = v + &package;

            data.insert(String::from("kubernetes::kubernetes_package_version: "), kubernetes_package_version);
        }
        "redhat" => {
            data.insert(String::from("kubernetes::kubernetes_package_version: "), version.to_string());
        }
        _ => {
            println!("The OS you've entered is not compatible with the puppetlabs-kubernetes module");
        }
    }

    match cni_network_provider {
        "weave" => {
            let cni_pod_cidr = "10.32.0.0/12".to_string();
            data.insert(String::from("kubernetes::cni_pod_cidr: "), cni_pod_cidr);
        }
        "flannel" => {
            let cni_pod_cidr = "10.244.0.0/16".to_string();
            data.insert(String::from("kubernetes::cni_pod_cidr: "), cni_pod_cidr);
        }
        "calico" => {
            let cni_pod_cidr = "192.168.0.0/16".to_string();
            data.insert(String::from("kubernetes::cni_pod_cidr: "), cni_pod_cidr);
        }
        _ => {
            println!("The CNI you've entered is not compatible with the puppetlabs-kubernetes module");
        }
    }

    let x = etcd_initial_cluster.split(",");
    let cluster
    for members in x {
        let member = members.split(":");
        let v = member.collect::<Vec<&str>>();
        let hostname = v[0];
        let ip = v[1];
    }

    data.insert(String::from("kubernetes::container_runtime: "), container_runtime.to_string());
    data.insert(String::from("kubernetes::cni_network_provider: "), cni_network_provider.to_string());
    data.insert(String::from("kubernetes::install_dashboard: "), install_dashboard.to_string());

}

