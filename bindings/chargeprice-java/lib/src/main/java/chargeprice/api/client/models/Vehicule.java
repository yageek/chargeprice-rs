package chargeprice.api.client.models;

public class Vehicule {
    String identifier;
    String brand;
    String manufacturerIdentifier;

    public String getIdentifier() {
        return identifier;
    }

    public String getBrand() {
        return brand;
    }

    public String getManufacturerIdentifier() {
        return manufacturerIdentifier;
    }

    public Vehicule(String identifier, String brand, String manufacturerIdentifier) {
        this.identifier = identifier;
        this.brand = brand;
        this.manufacturerIdentifier = manufacturerIdentifier;
    }

    public Vehicule(long pointer) {

    }

}
