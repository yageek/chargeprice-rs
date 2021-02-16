use chargeprice::api::{VehiculeChargePort, VehiculeResponse};

#[test]
fn vehicule_deserialize() {
    let vehicule_data = include_str!("samples/vehicule.json");

    let response: VehiculeResponse =
        serde_json::from_str(vehicule_data).expect("valid deserialization");

    let vehicule = &response.data()[0];
    assert_eq!("1e49b853-36fc-47ed-9826-97828b5b2fdd", vehicule.id());
    assert_eq!("Kona 64kWh (2018)", vehicule.attributes().name());
    assert_eq!("Hyundai", vehicule.attributes().brand());
    assert_eq!(
        Some("3e49b853-36fc-47ed-9826-97828b5b2fd1"),
        vehicule.relationships().map(|d| d.manufacturer_id())
    );
}
