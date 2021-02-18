package app.chargeprice.api;

public class Vehicule {

    private String identifier;
    private String brand;
    private String manufacturerIdentifier;

    public Vehicule(String identifier, String brand, String manufacturerIdentifier) {
        this.identifier = identifier;
        this.brand = brand;
        this.manufacturerIdentifier = manufacturerIdentifier;
    }

    public String getIdentifier() {
        return identifier;
    }

    public String getBrand() {
        return brand;
    }
    
    public String getManufacturerIdentifier() {
        return manufacturerIdentifier;
    }
}
