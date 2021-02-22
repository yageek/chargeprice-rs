use chargeprice::api::{ChargingStationResponse, Plug, VehiculeResponse};

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

#[test]
fn charging_station_serialize() {
    let input_data = include_str!("samples/charging_station.json");

    let response: ChargingStationResponse =
        serde_json::from_str(input_data).expect("valid deseiralization");
    let station = &response.data()[0];
    assert_eq!("Spar", station.attributes().name());
    assert_eq!(10.0, station.attributes().latitude());
    assert_eq!(20.0, station.attributes().longitude());
    assert_eq!("AT", station.attributes().country());
    assert_eq!(
        "Stangersdorf-Gewerbegebiet 110 A, 8403 Lebring",
        station.attributes().address()
    );
    assert_eq!(Some(true), station.attributes().free_parking());
    assert_eq!(Some(false), station.attributes().free_charging());

    // Point
    let points = station.attributes().charge_points();
    assert!(!points.is_empty());
    let pt = &points[0];
    assert_eq!(Plug::CCS, pt.plug());
    assert_eq!(50.0, pt.power());
    assert_eq!(2, pt.count());
    assert_eq!(2, pt.available_count());

    assert_eq!(
        "ae62cd2d-f29d-4107-b087-6d4f75261cca",
        station.relationships().unwrap().operator_id()
    );
}
