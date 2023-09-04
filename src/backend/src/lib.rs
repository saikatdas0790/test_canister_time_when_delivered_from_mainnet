use std::time::{Duration, SystemTime, UNIX_EPOCH};

use ic_cdk::api::{
    self,
    management_canister::http_request::{
        self, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
    },
};

pub fn get_current_system_time_from_ic() -> SystemTime {
    UNIX_EPOCH
        .checked_add(Duration::new(
            api::time() / 1_000_000_000,
            (api::time() % 1_000_000_000) as u32,
        ))
        .expect("Getting timestamp from ic_cdk failed")
}

#[candid::candid_method(update)]
#[ic_cdk::update]
async fn test_time() {
    let current_time = get_current_system_time_from_ic();

    let request_arg = CanisterHttpRequestArgument {
        url: "https://webhook.site/4e940c88-e13b-4bed-8cfc-5b2c36d8e276".to_string(),
        headers: vec![HttpHeader {
            name: "TimeStamp".to_string(),
            value: format!("{:?}", current_time),
        }],
        max_response_bytes: Some(0),
        method: HttpMethod::POST,
        ..Default::default()
    };

    http_request::http_request(request_arg, 1_000_000_000)
        .await
        .expect("Failed to ping");
}

#[cfg(test)]
mod test;

#[ic_cdk::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    candid::export_service!();
    __export_service()
}
