use std::collections::HashMap;
use serde::{Deserialize, Serialize};


#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HojinInfoResponse {
    /// エラー情報(エラーがある場合に出力します。)
    pub errors: Option<Vec<ApiError>>,
    /// gBizINFOデータ
    #[serde(rename = "hojin-infos")]
    pub hojin_infos: Option<Vec<HojinInfo>>,
    /// リクエストid
    pub id: Option<String>,
    /// メッセージ
    pub message: Option<String>,
}

/// エラー情報
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApiError {
    /// エラー箇所(項目)
    pub item: Option<String>,
    /// エラーメッセージ
    pub message: Option<String>,
}

/// gBizINFOデータ
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HojinInfo {
    /// 全省庁統一資格の営業品目
    pub business_items: Option<Vec<String>>,
    /// 事業概要
    pub business_summary: Option<String>,
    /// 資本金
    pub capital_stock: Option<usize>,
    /// 届出・認定情報
    pub certification: Option<Vec<CertificationInfo>>,
    /// 登記記録の閉鎖等の事由
    pub close_cause: Option<String>,
    /// 登記記録の閉鎖等年月日
    pub close_date: Option<String>,
    /// 表彰情報
    pub commendation: Option<Vec<CommendationInfo>>,
    /// 企業規模詳細(女)
    pub company_size_female: Option<usize>,
    /// 企業規模詳細(男)
    pub company_size_male: Option<usize>,
    /// 企業ホームページ
    pub company_url: Option<String>,
    /// 法人番号
    pub corporate_number: Option<String>,
    /// 設立年月日
    pub date_of_establishment: Option<String>,
    /// 従業員数
    pub employee_number: Option<usize>,
    /// 財務情報
    pub finance: Option<Finance>,
    /// 創業年
    pub founding_year: Option<usize>,
    /// 法人名フリガナ
    pub kana: Option<String>,
    /// 本社所在地
    pub location: Option<String>,
    /// 法人名
    pub name: Option<String>,
    /// 法人名英語
    pub name_en: Option<String>,
    /// 法人活動情報件数
    pub number_of_activity: Option<String>,
    /// 特許情報
    pub patent: Option<Vec<PatentInfo>>,
    /// 郵便番号
    pub postal_code: Option<String>,
    /// 調達情報
    pub procurement: Option<Vec<ProcurementInfo>>,
    /// 全省庁統一資格の資格等級(物品の製造、物品の販売、役務の提供等、物品の買受け)
    pub qualification_grade: Option<String>,
    /// 法人代表者名
    pub representative_name: Option<String>,
    /// 法人代表者役職
    pub representative_position: Option<String>,
    /// ステータス
    pub status: Option<String>,
    /// 補助金情報
    pub subsidy: Option<Vec<SubsidyInfo>>,
    /// 最終更新日
    pub update_date: Option<String>,
    /// 職場情報
    pub workplace_info: Option<WorkplaceInfoBean>,
}

/// 届出・認定情報
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertificationInfo {
    /// 部門
    pub category: Option<String>,
    /// 認定日
    pub date_of_approval: Option<String>,
    /// 企業規模
    pub enterprise_scale: Option<String>,
    /// 有効期限
    pub expiration_date: Option<String>,
    /// 府省
    pub government_departments: Option<String>,
    /// 対象
    pub target: Option<String>,
    /// 届出認定等
    pub title: Option<String>,
}

impl HojinInfo {
    pub fn name(&self) -> &str {
        match &self.name {
            None => { "" }
            Some(v) => { v.as_str() }
        }
    }
   pub fn corporate_number(&self) -> &str {
        match &self.corporate_number {
            None => { "" }
            Some(v) => { v.as_str() }
        }
    }
}


/// 表彰情報
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommendationInfo {
    /// 部門
    pub category: Option<String>,
    /// 年月日
    pub date_of_commendation: Option<String>,
    /// 府省
    pub government_departments: Option<String>,
    /// 受賞対象
    pub target: Option<String>,
    /// 表彰名
    pub title: Option<String>,
}

/// 財務情報
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Finance {
    /// 会計基準
    pub accounting_standards: Option<String>,
    /// 期
    pub fiscal_year_cover_page: Option<String>,
    /// 大株主
    pub major_shareholders: Option<Vec<MajorShareholders>>,
    /// 財務
    pub management_index: Option<Vec<ManagementIndex>>,
}

/// 特許情報
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PatentInfo {
    /// 出願年月日
    pub application_date: Option<String>,
    /// 出願番号
    pub application_number: Option<String>,
    /// 分類
    ///  (Array[HashMap«string,string»], optional)
    pub classifications: Option<Vec<HashMap<String, String>>>,
    /// 特許/意匠/商標
    pub patent_type: Option<String>,
    /// 発明の名称(等)/意匠に係る物品/表示用商標
    pub title: Option<String>,
}

/// 調達情報
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProcurementInfo {
    /// 金額
    pub amount: Option<usize>,
    /// 受注日
    pub date_of_order: Option<String>,
    /// 府省
    pub government_departments: Option<String>,
    /// 連名
    pub joint_signatures: Option<Vec<String>>,
    /// 事業名
    pub title: Option<String>,
}

/// 補助金情報
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubsidyInfo {
    ///  金額
    pub amount: Option<String>,
    ///  認定日
    pub date_of_approval: Option<String>,
    ///  府省
    pub government_departments: Option<String>,
    /// 連名
    pub joint_signatures: Option<Vec<String>>,
    ///  備考
    pub note: Option<String>,
    ///  補助金財源
    pub subsidy_resource: Option<String>,
    ///  対象
    pub target: Option<String>,
    ///  補助金等
    pub title: Option<String>,
}

/// 職場情報
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WorkplaceInfoBean {
    /// 勤務基本情報
    pub base_infos: Option<WorkplaceBaseInfos>,
    /// 育児・仕事の両立に関する情報
    pub compatibility_of_childcare_and_work: Option<CompatibilityOfChildcareAndWork>,
    /// 女性の活躍に関する情報
    pub women_activity_infos: Option<WomenActivityInfos>,
}

/// 大株主
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MajorShareholders {
    ///  企業名もしくは出資者
    pub name_major_shareholders: Option<String>,
    ///  出資比率
    pub shareholding_ratio: Option<f32>,
}

/// 財務
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagementIndex {
    /// 資本金
    pub capital_stock_summary_of_business_results: Option<usize>,
    /// 資本金(単位)
    pub capital_stock_summary_of_business_results_unit_ref: Option<String>,
    /// 営業総収入
    pub gross_operating_revenue_summary_of_business_results: Option<usize>,
    /// 営業総収入（単位）
    pub gross_operating_revenue_summary_of_business_results_unit_ref: Option<String>,
    /// 純資産額
    pub net_assets_summary_of_business_results: Option<usize>,
    /// 純資産額(単位)
    pub net_assets_summary_of_business_results_unit_ref: Option<String>,
    /// 当期純利益又は当期純損失(△)
    pub net_income_loss_summary_of_business_results: Option<isize>,
    /// 当期純利益又は当期純損失(△)(単位)
    pub net_income_loss_summary_of_business_results_unit_ref: Option<String>,
    /// 正味収入保険料
    pub net_premiums_written_summary_of_business_results_ins: Option<usize>,
    /// 正味収入保険料（単位）
    pub net_premiums_written_summary_of_business_results_ins_unit_ref: Option<String>,
    /// 売上高
    pub net_sales_summary_of_business_results: Option<usize>,
    /// 売上高(単位)
    pub net_sales_summary_of_business_results_unit_ref: Option<String>,
    /// 従業員数
    pub number_of_employees: Option<usize>,
    /// 従業員数(単位)
    pub number_of_employees_unit_ref: Option<String>,
    /// 営業収益
    pub operating_revenue1_summary_of_business_results: Option<isize>,
    /// 営業収益（単位）
    pub operating_revenue1_summary_of_business_results_unit_ref: Option<String>,
    /// 営業収入
    pub operating_revenue2_summary_of_business_results: Option<isize>,
    /// 営業収入（単位）
    pub operating_revenue2_summary_of_business_results_unit_ref: Option<String>,
    /// 経常利益又は経常損失(△)
    pub ordinary_income_loss_summary_of_business_results: Option<isize>,
    /// 経常利益又は経常損失(△)(単位)
    pub ordinary_income_loss_summary_of_business_results_unit_ref: Option<String>,
    /// 経常収益
    pub ordinary_income_summary_of_business_results: Option<isize>,
    /// 経常収益（単位）
    pub ordinary_income_summary_of_business_results_unit_ref: Option<String>,
    /// 回次
    pub period: Option<String>,
    /// 総資産額
    pub total_assets_summary_of_business_results: Option<isize>,
    /// 総資産額(単位)
    pub total_assets_summary_of_business_results_unit_ref: Option<String>,
}

/// 勤務基本情報
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WorkplaceBaseInfos {
    /// 従業員の平均年齢
    pub average_age: Option<usize>,
    /// 正社員の平均継続勤務年数
    pub average_continuous_service_years: Option<usize>,
    /// 平均継続勤務年数-女性
    #[serde(rename = "average_continuous_service_years_Female")]
    pub average_continuous_service_years_female: Option<usize>,
    /// 平均継続勤務年数-男性
    #[serde(rename = "average_continuous_service_years_Male")]
    pub average_continuous_service_years_male: Option<usize>,
    /// 平均継続勤務年数種別
    pub average_continuous_service_years_type: Option<String>,
    /// 月平均所定外労働時間
    pub month_average_predetermined_overtime_hours: Option<usize>,
}

/// 育児・仕事の両立に関する情報
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompatibilityOfChildcareAndWork {
    /// 育児休業取得者数（女性）
    pub maternity_leave_acquisition_num: Option<usize>,
    /// 育児休業対象者数（女性）
    pub number_of_maternity_leave: Option<usize>,
    /// 育児休業対象者数（男性）
    pub number_of_paternity_leave: Option<usize>,
    /// 育児休業取得者数（男性
    pub paternity_leave_acquisition_num: Option<usize>,
}

/// 女性の活躍に関する情報
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WomenActivityInfos {
    /// 女性管理職人数
    pub female_share_of_manager: Option<usize>,
    /// 女性役員人数
    pub female_share_of_officers: Option<usize>,
    /// 労働者に占める女性労働者の割合
    pub female_workers_proportion: Option<f32>,
    /// 労働者に占める女性労働者の割合種別
    pub female_workers_proportion_type: Option<usize>,
    /// 管理職に占める女性の割合
    pub gender_total_of_manager: Option<f32>,
    /// 役員全体人数（男女計）
    pub gender_total_of_officers: Option<usize>,
}
