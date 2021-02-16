package app.chargeprice.api;

public interface ClientListener<T> {
    void onSuccess(T value);
}
