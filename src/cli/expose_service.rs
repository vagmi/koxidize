use std::collections::BTreeMap;

use anyhow::{Result, Context};
use k8s_openapi::api::core::v1::{Service, ServiceSpec, ServicePort};
use kube::{Api, Client, api::PostParams, core::ObjectMeta};
use serde_json::json;

pub async fn expose_nginx(client: Client) -> Result<Service> {
    let svc_api : Api<Service> = Api::namespaced(client, "default");
    let svc_req_json = json!({
        "apiVersion": "v1",
        "kind": "Service",
        "metadata": {"name": "nginx"},
        "spec": {
            "type": "NodePort",
            "selector": {"app": "nginx"},
            "ports": [{"protocol": "TCP", "port": 80}]
        }
    });
    let svc_req: Service = serde_json::from_value(svc_req_json)?;
    svc_api.create(&PostParams::default(), &svc_req)
        .await
        .context("Unable to create service")
}

pub async fn expose_nginx_obj(client: Client) -> Result<Service> {
    let svc_api : Api<Service> = Api::namespaced(client, "default");
    let svc_req = Service {
        metadata: ObjectMeta {
            name: Some(String::from("nginx")),
            ..Default::default()
        },
        spec: Some(ServiceSpec {
            type_: Some(String::from("NodePort")),
            selector: Some(BTreeMap::from([
              (String::from("app"), String::from("nginx"))
            ])),
            ports: Some(vec![ServicePort{
                protocol: Some("TCP".into()),
                port: 80,
                ..Default::default()
            }]),
            ..Default::default()
        }),
        ..Default::default()

    };
    svc_api.create(&PostParams::default(), &svc_req)
        .await
        .context("Unable to create service")
}
