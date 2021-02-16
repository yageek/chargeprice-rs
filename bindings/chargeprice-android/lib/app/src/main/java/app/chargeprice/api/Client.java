package app.chargeprice.api;

public class Client {

    private final long innerClient;

    // Inner JNI helper
    private static native long createNewClient(String key);
    private static native void releaseClient(long ptr);


    static {
        System.loadLibrary("chargeprice_jni");
    }
    public Client(String key) throws Throwable {

        // Allocate the inner JNI client
        innerClient = createNewClient(key);
        if (innerClient == 0) {
            throw new RuntimeException();
        }
    }

    public void close() {
        releaseClient(innerClient);
    }
}