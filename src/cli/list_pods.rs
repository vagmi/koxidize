use k8s_openapi::api::core::v1::Pod;
use kube::{Client, api::{Api, ListParams}, core::ObjectList};
use anyhow::{Result, Context};

pub async fn list_pods(client: Client) -> Result<ObjectList<Pod>> {
    let pod_api: Api<Pod> = Api::all(client);
    pod_api.list(&ListParams::default()).await.context("unable to fetch pods")
}
