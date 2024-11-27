use reqwest::{header::LOCATION, Response, StatusCode};
use serde::de::DeserializeOwned;

#[derive(Debug)]
#[allow(dead_code)]
pub struct CreatedResponse<P> {
    pub location: String,
    pub payload: P,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct OkResponse<P> {
    pub payload: P,
}

impl<P> CreatedResponse<P>
where
    P: DeserializeOwned,
{
    pub async fn from(response: Response) -> Self {
        assert_eq!(
            response.status(),
            StatusCode::CREATED,
            "create operation did not respond with CREATED status code"
        );
        let location = response
            .headers()
            .get(LOCATION)
            .expect("missing location header in create response")
            .to_str()
            .unwrap()
            .to_string();
        let payload = response.json::<P>().await.unwrap();
        Self {
            location: location.to_string(),
            payload,
        }
    }

    pub fn payload(&self) -> &P {
        &self.payload
    }
}

impl<P> OkResponse<P>
where
    P: DeserializeOwned,
{
    pub async fn from(response: Response) -> Self {
        assert_eq!(
            response.status(),
            StatusCode::OK,
            "unexpected status code. the expected status is 200 OK"
        );
        let payload = response.json::<P>().await.unwrap();
        Self { payload }
    }

    pub fn payload(&self) -> &P {
        &self.payload
    }
}
