package app.chargeprice.api;
public class Vehicule {

    private String identifier;
    private String brand;

    public Vehicule(String identifier, String brand) {
        this.identifier = identifier;
        this.brand = brand;
    }

    public String getIdentifier() {
        return identifier;
    }

    public String getBrand() {
        return brand;
    }
}
