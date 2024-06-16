struct AssociatedData {
    pub name: String,
    pub quality: String,
    pub timestamp: i64,
    pub value: f64,
}

struct AssetEvent {
    pub asset_name: String,
    pub associated_data: Vec<AssociatedData>,
    pub event_name: String,
    pub event_status: String,
    pub timestamp: i64,
    pub created_user: String,
}

struct Events {
    pub events: Vec<AssetEvent>,
}