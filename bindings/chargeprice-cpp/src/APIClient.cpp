#include "APIClient.hpp"

#include <exception>

using namespace chargeprice;

APIClient::APIClient(const std::string &key)
{
    auto client = chargeprice_create_api_client(key.c_str(), "(ffi CPP)");
    if (client == nullptr)
    {
        throw std::runtime_error("impossible to create client");
    }
    else
    {
        this->_client = client;
    }
}

APIClient::~APIClient()
{
    chargeprice_free_api_client(this->_client);
}

void APIClient::success_cb(void *context, const CAPI_Vehicule *arg, size_t length)
{
    auto func = static_cast<std::function<void(std::vector<Vehicule> success, void *error)> *>(context);

    std::vector<Vehicule> buff(length);
    CAPI_Vehicule *ptr = (CAPI_Vehicule *)arg;
    for (auto i = 0; i < length; i++)
    {
        Vehicule new_value(ptr->id, ptr->brand, ptr->manufacturer_id);
        buff.push_back(new_value);
        ptr += 1;
    }

    (*func)(buff, nullptr);
};

void APIClient::error_cb(void *context, const CAPI_FFIError *arg)
{

    auto func = static_cast<std::function<void(std::vector<Vehicule> success, void *error)> *>(context);
    std::vector<Vehicule> test;
    (*func)(test, nullptr);
};

APIClientCancellable APIClient::load_vehicules(std::function<void(std::vector<Vehicule> success, void *error)> func)
{
    auto ctx = static_cast<void *>(&func);
    CAPI_ArrayCallback_Vehicule__FFIError cb = {
        .context = ctx,
        .on_success = APIClient::success_cb,
        .on_error = APIClient::error_cb};

    return chargeprice_get_vehicles(_client, cb);
}