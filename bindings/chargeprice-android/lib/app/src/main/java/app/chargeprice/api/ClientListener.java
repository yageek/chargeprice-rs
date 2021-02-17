package app.chargeprice.api;

import java.util.List;

public interface ClientListener {
    void onVehiculeSuccess(List<Vehicule> values);
    void onVehiculeError(String reason);
}
