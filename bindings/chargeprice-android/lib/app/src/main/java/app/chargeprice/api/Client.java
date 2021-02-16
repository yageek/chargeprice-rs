package app.chargeprice.api;

public class Client {

    private final long innerClient;

    // Inner JNI helper
    private static native long createNewClient(String key);
    private static native void releaseClient(long ptr);
    private static native void loadVehicules(long ptr, ClientListener listener);

    static {
        System.loadLibrary("chargeprice_jni");
    }

    /**
     * Creates a client
     * @param key The api key to use
     * @throws Throwable
     */
    public Client(String key) throws Throwable {

        // Allocate the inner JNI client
        innerClient = createNewClient(key);
        if (innerClient == 0) {
            throw new RuntimeException();
        }
    }

    /**
     * Releases the client
     */
    public void release() {
        releaseClient(innerClient);
    }

    public void loadVehicule(ClientListener listener) {
        loadVehicules(innerClient, listener);
    }
}