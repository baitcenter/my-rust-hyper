use super::entities::route;
use super::connectors;
use std::sync::Arc;

pub struct Router {
    service_name: String,
    local_router_service_name: Option<String>,
}

impl Router {
    pub async fn from_remote(
        http_from: Option<String>,
        mq_from: Option<String>,
        mut service: route::Route,
        host: &str
    ) -> connectors::Result<Router> {
        let sv = &mut service;
        sv.http_helth = sv.http_helth.replace("{host}", host);
        Ok(Router {
            service_name: service.service_name,
            local_router_service_name: None,
        })
    }

    pub async fn from_local(
        dc: &connectors::DataConnector,
        mut service: route::Route,
        mut local_router: route::Route,
        host: &str
    ) -> connectors::Result<Router> {
        let sv = &mut service;
        sv.http_helth = sv.http_helth.replace("{host}", host);
        for c in &mut sv.command {
            if c.http_to.is_some() {
                let v = c.http_to.as_ref().unwrap();
                c.http_to = Some(v.replace("{host}", host));
            }
        }
        for s in &mut sv.subscription {
            if s.http_to.is_some() {
                let v = s.http_to.as_ref().unwrap();
                s.http_to = Some(v.replace("{host}", host));
            }
        }
        let lr = &mut local_router;
        lr.http_helth = lr.http_helth.replace("{host}", host);
        for c in &mut lr.command {
            if c.http_to.is_some() {
                let v = c.http_to.as_ref().unwrap();
                c.http_to = Some(v.replace("{host}", host));
            }
        }
        for s in &mut lr.subscription {
            if s.http_to.is_some() {
                let v = s.http_to.as_ref().unwrap();
                s.http_to = Some(v.replace("{host}", host));
            }
        }
        let sn = service.service_name.clone();
        let rn = local_router.service_name.clone();
        dc.route.add(vec![local_router]).await?;
        debug!("add local route");
        dc.route.add(vec![service]).await?;
        debug!("add service route");
        Ok(Router {
            service_name: sn,
            local_router_service_name: Some(rn),
        })
    }

    pub async fn shutdown(&self, dc: Arc<connectors::DataConnector>) -> connectors::Result<()> {
        if self.local_router_service_name.is_some() {
            dc.route
                .remove(vec![self.local_router_service_name.as_ref().unwrap().to_string()])
                .await?;
            debug!("remove local route");
        }
        dc.route.remove(vec![self.service_name.clone()]).await?;
        debug!("remove service route");
        Ok({})
    }
}
