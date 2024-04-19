use crate::buildah::container::*;
use crate::graphdata::request::*;

pub async fn build_image() {
    let url = "https://api.openshift.com/api/upgrades_info/graph-data".to_string();
    let g_con = ImplUpgradePathInterface {};
    g_con
        .get_graph_tar_gz(url.clone(), "/tmp/cincinnati-graph-data.tar.gz")
        .await
        .unwrap();
    let mut container = Container::from("registry.access.redhat.com/ubi9/ubi:latest");
    container
        .untar("/tmp/cincinnati-graph-data.tar.gz", "./container")
        .unwrap();
    container
        .copy("container", "/var/lib/cincinnati-graph-data/")
        .unwrap();
    container
            .config_cmd("[\"/bin/bash\", \"-c\" ,\"exec cp -rp /var/lib/cincinnati-graph-data/* /var/lib/cincinnati/graph-data\" ]")
            .unwrap();
    container.commit("openshift/graph-data:latest").unwrap();
}
