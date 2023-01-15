use std::collections::BTreeMap;

use k8s_openapi::{api::{apps::v1::{Deployment, DeploymentSpec}, core::v1::{PodTemplate, PodTemplateSpec, PodSpec, Container}}, apimachinery::pkg::apis::meta::v1::LabelSelector};
use kube::{Client, api::{Api, PostParams}, core::ObjectMeta};
use anyhow::{Result, Context};
use serde_json::json;

pub async fn deploy_nginx(client: Client) -> Result<Deployment> {
    let dep_api : Api<Deployment> = Api::namespaced(client, "default");
    
    let data = json!({
        "apiVersion": "apps/v1",
        "kind": "Deployment", 
        "metadata": {"name": "nginx"},
        "spec": {
            "replicas": 2,
            "selector": {"matchLabels": {"app": "nginx"}},
            "template": {
                "metadata": {"labels": {"app": "nginx"}},
                "spec": {"containers": [{"name": "nginx", "image": "nginx"}]}
            }
        }
    });
    let dep_req: Deployment = serde_json::from_value(data)?;
    dep_api.create(&PostParams::default(), &dep_req).await.context("Unable to create deployment")
}

pub async fn deploy_nginx_obj(client: Client) -> Result<Deployment> {
    let dep_api : Api<Deployment> = Api::namespaced(client, "default");
    let dep_req = Deployment {
        metadata: ObjectMeta{
            name: Some(String::from("nginx")),
            ..Default::default()
        },
        spec: Some(DeploymentSpec {
            selector: LabelSelector {
                match_labels:  Some(BTreeMap::from([(String::from("app"), String::from("nginx"))])),
                ..Default::default()
            },
            replicas: Some(2),
            template: PodTemplateSpec{
                metadata: Some(ObjectMeta{
                    labels: Some(BTreeMap::from([(String::from("app"), String::from("nginx"))])),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    containers: vec![Container{
                        name: String::from("nginx"),
                        image: Some(String::from("nginx")),
                        ..Default::default()
                    }],
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        }),
        ..Default::default()
    };

    dep_api.create(&PostParams::default(), &dep_req).await.context("Unable to create deployment")
}
