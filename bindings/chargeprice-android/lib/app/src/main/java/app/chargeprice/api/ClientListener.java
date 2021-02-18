package app.chargeprice.api;

import java.util.ArrayList;
import java.util.List;

public interface ClientListener {
    void onVehiculeSuccess(ArrayList<Vehicule> values);
    void onVehiculeError(String reason);
}
