use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::error::GBizApiError;
use crate::http::HttpClient;
use crate::types::HojinInfoResponse;

pub mod types;
mod http;
pub mod error;


pub struct GBizInfoApiV1 {
    api_token: Option<String>,
}


impl GBizInfoApiV1 {
    pub fn new<T: ToString>(token: T) -> GBizInfoApiV1 {
        GBizInfoApiV1 {
            api_token: Some(token.to_string()),
        }
    }

    pub(crate) fn token(&self) -> &str {
        match self.api_token.as_ref() {
            None => { "" }
            Some(v) => { v.as_str() }
        }
    }
    /// GBizInfoApiV1UpdateInfoQuery
    pub async fn hojin(&self, query: &GBizInfoApiV1HojinQuery) -> Result<HojinInfoResponse, GBizApiError> {
        HttpClient::get(self.token(), "https://info.gbiz.go.jp/hojin/v1/hojin", query).await
    }
    pub async fn corporate_number(&self, corporate_number: &str) -> Result<HojinInfoResponse, GBizApiError> {
        HttpClient::get(self.token(), format!("https://info.gbiz.go.jp/hojin/v1/hojin/{}", corporate_number).as_str(), &json!({})).await
    }
    pub async fn corporate_number_certification(&self, corporate_number: &str) -> Result<HojinInfoResponse, GBizApiError> {
        HttpClient::get(self.token(), format!("https://info.gbiz.go.jp/hojin/v1/hojin/{}/certification", corporate_number).as_str(), &json!({})).await
    }
    pub async fn corporate_number_commendation(&self, corporate_number: &str) -> Result<HojinInfoResponse, GBizApiError> {
        HttpClient::get(self.token(), format!("https://info.gbiz.go.jp/hojin/v1/hojin/{}/commendation", corporate_number).as_str(), &json!({})).await
    }
    pub async fn corporate_number_finance(&self, corporate_number: &str) -> Result<HojinInfoResponse, GBizApiError> {
        HttpClient::get(self.token(), format!("https://info.gbiz.go.jp/hojin/v1/hojin/{}/finance", corporate_number).as_str(), &json!({})).await
    }
    pub async fn corporate_number_patent(&self, corporate_number: &str) -> Result<HojinInfoResponse, GBizApiError> {
        HttpClient::get(self.token(), format!("https://info.gbiz.go.jp/hojin/v1/hojin/{}/patent", corporate_number).as_str(), &json!({})).await
    }
    pub async fn corporate_number_procurement(&self, corporate_number: &str) -> Result<HojinInfoResponse, GBizApiError> {
        HttpClient::get(self.token(), format!("https://info.gbiz.go.jp/hojin/v1/hojin/{}/procurement", corporate_number).as_str(), &json!({})).await
    }
    pub async fn corporate_number_workplace(&self, corporate_number: &str) -> Result<HojinInfoResponse, GBizApiError> {
        HttpClient::get(self.token(), format!("https://info.gbiz.go.jp/hojin/v1/hojin/{}/workplace", corporate_number).as_str(), &json!({})).await
    }

    /// GBizInfoApiV1UpdateInfoQuery
    pub async fn update_info(&self, query: &GBizInfoApiV1UpdateInfoQuery) -> Result<HojinInfoResponse, GBizApiError> {
        HttpClient::get(self.token(), "https://info.gbiz.go.jp/hojin/v1/hojin/updateInfo", query).await
    }
    pub async fn update_info_certification(&self, query: &GBizInfoApiV1UpdateInfoQuery) -> Result<HojinInfoResponse, GBizApiError> {
        HttpClient::get(self.token(), "https://info.gbiz.go.jp/hojin/v1/hojin/updateInfo/certification", query).await
    }
    pub async fn update_info_commendation(&self, query: &GBizInfoApiV1UpdateInfoQuery) -> Result<HojinInfoResponse, GBizApiError> {
        HttpClient::get(self.token(), "https://info.gbiz.go.jp/hojin/v1/hojin/updateInfo/commendation", query).await
    }
    pub async fn update_info_finance(&self, query: &GBizInfoApiV1UpdateInfoQuery) -> Result<HojinInfoResponse, GBizApiError> {
        HttpClient::get(self.token(), "https://info.gbiz.go.jp/hojin/v1/hojin/updateInfo/finance", query).await
    }
    pub async fn update_info_patent(&self, query: &GBizInfoApiV1UpdateInfoQuery) -> Result<HojinInfoResponse, GBizApiError> {
        HttpClient::get(self.token(), "https://info.gbiz.go.jp/hojin/v1/hojin/updateInfo/patent", query).await
    }
    pub async fn update_info_procurement(&self, query: &GBizInfoApiV1UpdateInfoQuery) -> Result<HojinInfoResponse, GBizApiError> {
        HttpClient::get(self.token(), "https://info.gbiz.go.jp/hojin/v1/hojin/updateInfo/procurement", query).await
    }
    pub async fn update_info_workplace(&self, query: &GBizInfoApiV1UpdateInfoQuery) -> Result<HojinInfoResponse, GBizApiError> {
        HttpClient::get(self.token(), "https://info.gbiz.go.jp/hojin/v1/hojin/updateInfo/workplace", query).await
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GBizInfoApiV1HojinQuery {
    pub corporate_number: Option<String>,
    pub name: Option<String>,
    pub exist_flg: Option<String>,
    pub corporate_type: Option<String>,
    pub prefecture: Option<String>,
    pub city: Option<String>,
    pub capital_stock_from: Option<String>,
    pub capital_stock_to: Option<String>,
    pub employee_number_from: Option<String>,
    pub employee_number_to: Option<String>,
    pub founded_year: Option<String>,
    pub sales_area: Option<String>,
    pub business_item: Option<String>,
    pub unified_qualification: Option<String>,
    pub unified_qualification_sub01: Option<String>,
    pub unified_qualification_sub02: Option<String>,
    pub unified_qualification_sub03: Option<String>,
    pub unified_qualification_sub04: Option<String>,
    pub net_sales_summary_of_business_results_from: Option<String>,
    pub net_sales_summary_of_business_results_to: Option<String>,
    pub net_income_loss_summary_of_business_results_from: Option<String>,
    pub net_income_loss_summary_of_business_results_to: Option<String>,
    pub total_assets_summary_of_business_results_from: Option<String>,
    pub total_assets_summary_of_business_results_to: Option<String>,
    pub name_major_shareholders: Option<String>,
    pub average_continuous_service_years: Option<String>,
    pub average_age: Option<String>,
    pub month_average_predetermined_overtime_hours: Option<String>,
    pub female_workers_proportion: Option<String>,
    pub year: Option<String>,
    pub ministry: Option<String>,
    pub source: Option<String>,
    pub page: Option<String>,
    pub limit: Option<String>,
}

impl GBizInfoApiV1HojinQuery {
    pub fn set_limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit.to_string());
        self
    }
    pub fn set_page(&mut self, page: u32) -> &mut Self {
        self.page = Some(page.to_string());
        self
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GBizInfoApiV1UpdateInfoQuery {
    /// 検索結果のページ番号：正の整数を設定。 下限値1。
    pub page: Option<String>,
    /// 検索対象期間の開始日：yyyyMMdd形式を設定。
    pub from: Option<String>,
    /// 検索対象期間の終了日：yyyyMMdd形式を設定
    pub to: Option<String>,
}

impl GBizInfoApiV1UpdateInfoQuery {
    pub fn set_from<T:ToString>(&mut self, from: T) -> &mut Self {
        self.from = Some(from.to_string());
        self
    }
    pub fn set_to<T:ToString>(&mut self, to: T) -> &mut Self {
        self.to = Some(to.to_string());
        self
    }
    pub fn set_page(&mut self, page: u32) -> &mut Self {
        self.page = Some(page.to_string());
        self
    }
}