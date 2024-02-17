pub mod entity {
    #[derive(Debug)]
    pub struct RequestParams {
        pub bussiness_type: String,
        pub supplier: String,
        pub request_id: String,
        pub client_id: String,
        pub cur_page: String,
        pub page_size: String,
    }
    impl RequestParams {
        #[allow(dead_code)]
        pub fn to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
            let data: serde_json::Value = serde_json::from_str(&format!(
                r#"{{"businessType": "{}", "supplier": "{}", "requestId": "{}", "clientId": "{}", "curPage": "{}", "pageSize": "{}"}}"#,
                self.bussiness_type,
                self.supplier,
                self.request_id,
                self.client_id,
                self.cur_page,
                self.page_size
            ))?;
            //let data: serde_json::Value = serde_json::from_str(&data)?;
            Ok(data)
        }

        #[allow(dead_code)]
        pub fn new() -> RequestParams {
            RequestParams {
                bussiness_type: "".to_string(),
                supplier: "".to_string(),
                request_id: "".to_string(),
                client_id: "".to_string(),
                cur_page: "".to_string(),
                page_size: "".to_string(),
            }
        }
    }
}

pub mod api {
    use crate::entity::RequestParams;
    use reqwest::{header::HeaderMap, Client, Method};

    // 获取我的任务
    #[allow(dead_code)]
    pub async fn get_my_task_by_supplier(
        _params: RequestParams,
    ) -> Result<String, Box<dyn std::error::Error>> {

        // 设置请求头
        let mut headers = HeaderMap::new();
        headers.insert("X-HW-ID", "APP_102071_guanghonggufen".parse()?);
        headers.insert("X-HW-APPKEY", "ZVwEuLc/P3SzS8a5CR5Rpg==".parse()?);
        headers.insert("Content-Type", "application/json".parse()?);

        // 设置请求参数, 转化为json格式
        let test: serde_json::Value = match (RequestParams {
            bussiness_type: "ALL".to_string(),
            supplier: "102071".to_string(),
            request_id: "415332".to_string(),
            client_id: "惠州光弘科技股份有限公司".to_string(),
            cur_page: "1".to_string(),
            page_size: "25".to_string(),
        }.to_json()) {
            Ok(data) => data,
            Err(e) => return Err(Box::new(e)),
        };

        // 发送POST请求
        let request = Client::builder()
            .build()?
            .request(
                Method::POST,
                "https://apigw-01.huawei.com/api/services/process/task/getMyTaskBySupplier",
            )
            .headers(headers)
            .json(&test);

        // 获取响应, 并转化为String文本
        let response = request.send().await?.text().await?;

        // 返回响应
        Ok(response)
    }

    #[allow(dead_code)]
    pub async fn download_file() -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder().build()?;
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-HW-ID", "APP_102071_guanghonggufen".parse()?);
        headers.insert("X-HW-APPKEY", "ZVwEuLc/P3SzS8a5CR5Rpg==".parse()?);

        let url = format!("https://apigw-01.huawei.com/api/dlservice/downloadFile?supplier={}&workitemId={}&clientId={}&requestId={}&businessType={}", 102071, "0f7d84107d4a11eebd9a02550c0b032c___027f4c087d4b11eea64c02550c0b15b0", "惠州光弘科技股份有限公司", "11126", "ALL");
        let request = client.request(reqwest::Method::GET, url)
        .headers(headers);

        let response = request.send().await?;
        let body = response.text().await?;

        println!("{}", body);

        Ok(())
    }
}
