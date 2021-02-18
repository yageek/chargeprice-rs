package app.chargeprice.api;

import androidx.appcompat.app.AppCompatActivity;

import android.os.Bundle;
import android.util.Log;
import android.widget.TextView;

import java.util.ArrayList;
import java.util.List;

public class MainActivity extends AppCompatActivity {

    private Client mClient;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        try {
            mClient = new Client("***REMOVED***");

            mClient.loadVehicule(new ClientListener() {
                @Override
                public void onVehiculeSuccess(ArrayList<Vehicule> values) {
                    Log.d("JavaClient", "Element:" + values);
                }

                @Override
                public void onVehiculeError(String reason) {
                    Log.e("JavaClient", reason);
                }
            });
        } catch (Throwable throwable) {
            throwable.printStackTrace();
        }
    }
}