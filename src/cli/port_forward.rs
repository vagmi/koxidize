use std::collections::BTreeMap;
use std::io::ErrorKind;
use std::net::{SocketAddr};

use k8s_openapi::Metadata;
use k8s_openapi::api::core::v1::Pod;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
use kube::api::ListParams;
use kube::{Client, Api};

use anyhow::{Result, anyhow};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::wrappers::TcpListenerStream;
use futures::{StreamExt, TryStreamExt};

#[derive(Clone)]
struct PodForwarder {
    client: Client
}
impl PodForwarder {
    fn new(client: Client) -> PodForwarder {
        PodForwarder{client}
    }
    async fn get_nginx_pod(self) -> Result<Option<String>> {
        let pods_api: Api<Pod> = Api::namespaced(self.client.clone(), "default");
        let pods = pods_api.list(&ListParams::default().labels("app=nginx")).await?;
        match pods.items.get(0) {
            Some(_p) => Ok(_p.metadata().name.clone()),
         None => Err(anyhow!("Could not find pod with the label"))
        }
    }
    async fn port_forward(self) -> Result<()> {
        let addr = SocketAddr::from(([127,0,0,1], 8080));
        let tcp_listener = TcpListener::bind(addr).await?;
        let listener = TcpListenerStream::new(tcp_listener);
        let server = listener.take_until(tokio::signal::ctrl_c())
            .try_for_each(|conn| async {self.clone().run_forwarder(conn).await});
        server.await?;
        Ok(())
    }
    async fn run_forwarder(self,mut conn: TcpStream) -> std::io::Result<()>{
        let pod_name = self.clone().get_nginx_pod()
                                   .await.map_err(|e| std::io::Error::new(ErrorKind::Other, e))?
                                   .ok_or(std::io::Error::new(ErrorKind::Other, "podname is invalid"))?;
        let pods_api : Api<Pod> = Api::namespaced(self.client.clone(), "default");

                if let Ok(peer_addr) = conn.peer_addr() {
                    println!("peer addr {}", peer_addr);
                }
                let mut forwarder = pods_api.portforward(&pod_name, &[80]).await
                                                                        .map_err(|e| std::io::Error::new(ErrorKind::Other, e))?;
                let mut stream = forwarder.take_stream(80).ok_or(std::io::Error::new(ErrorKind::Other, "Unable to take stream"))?;
                tokio::io::copy_bidirectional(&mut conn, &mut stream).await?;
                drop(stream);
                forwarder.join().await.map_err(|e| std::io::Error::new(ErrorKind::Other, e))
    }
}

pub async fn port_forward(client: Client) -> Result<()> {
    PodForwarder::new(client).port_forward().await
}
